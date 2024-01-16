use std::io::Write;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

pub fn run_eca<W>(
    threads: usize,
    rule: [bool; 8],
    size: usize,
    steps: usize,
    indices: Vec<usize>,
    write: Option<W>,
) -> ((usize, usize), (usize, usize), usize)
where
    W: Write + Send + 'static,
{
    assert!(threads >= 1);
    assert!(size >= 1);
    assert!(indices.iter().all(|i| *i < size));

    let indices = Arc::new(indices);

    let visualize = write.is_some();

    let mut worker_handles = Vec::with_capacity(threads);
    let threads = std::cmp::min(threads, size);
    let mut sizes = vec![size / threads; threads];
    sizes
        .iter_mut()
        .take(size % threads)
        .for_each(|size| *size += 1);
    let (write_send, mut write_recv) = mpsc::channel();
    let mut l_sends = Vec::with_capacity(threads);
    let mut r_recvs = Vec::with_capacity(threads);
    let mut r_sends = Vec::with_capacity(threads);
    let mut l_recvs = Vec::with_capacity(threads);
    for _ in 0..threads {
        let (l_send, r_recv) = mpsc::channel();
        l_sends.push(l_send);
        r_recvs.push(r_recv);

        let (r_send, l_recv) = mpsc::channel();
        r_sends.push(r_send);
        l_recvs.push(l_recv);
    }
    r_recvs.rotate_right(1);
    l_recvs.rotate_left(1);
    let mut popcnt_recvs = Vec::with_capacity(threads);
    let mut offset = 0;
    for _ in 0..threads {
        let size = sizes.pop().unwrap();
        let indices = Arc::clone(&indices);
        let (write_send_next, write_recv_next) = mpsc::channel();
        let l_send = l_sends.pop().unwrap();
        let r_recv = r_recvs.pop().unwrap();
        let r_send = r_sends.pop().unwrap();
        let l_recv = l_recvs.pop().unwrap();
        let (popcnt_send, popcnt_recv) = mpsc::channel();
        popcnt_recvs.push(popcnt_recv);
        let worker_handle = thread::spawn(move || {
            let mut curr = vec![false; size];
            let mut curr_popcnt = 0;
            for i in indices.iter() {
                if offset <= *i && *i < offset + size && !curr[*i - offset] {
                    curr[*i - offset] = true;
                    curr_popcnt += 1;
                }
            }

            let mut next = vec![false; size];

            let mut step = 0;
            loop {
                popcnt_send.send(curr_popcnt).unwrap();
                if visualize {
                    let mut write: W = write_recv.recv().unwrap();
                    for b in curr.iter() {
                        write!(write, "{}", if *b { '■' } else { '□' }).unwrap_or_default();
                    }
                    write_send_next.send(write).unwrap();
                }

                step += 1;
                if step > steps {
                    break;
                }

                l_send.send(curr[0]).unwrap();
                r_send.send(curr[size - 1]).unwrap();
                let curr_l = || l_recv.recv().unwrap();
                let curr_r = || r_recv.recv().unwrap();

                let mut next_popcnt = 0;
                for i in 0..size {
                    let mut rule_idx = 0;
                    if if i == 0 { curr_l() } else { curr[i - 1] } {
                        rule_idx += 4;
                    }
                    if curr[i] {
                        rule_idx += 2;
                    }
                    if if i == size - 1 { curr_r() } else { curr[i + 1] } {
                        rule_idx += 1;
                    }
                    next[i] = rule[rule_idx];
                    if rule[rule_idx] {
                        next_popcnt += 1;
                    }
                }
                std::mem::swap(&mut curr, &mut next);
                curr_popcnt = next_popcnt;
            }
        });
        write_recv = write_recv_next;
        worker_handles.push(worker_handle);
        offset += size;
    }

    let mut min_popcnt = usize::MAX;
    let mut max_popcnt = usize::MIN;
    let mut min_popcnt_step = 0;
    let mut max_popcnt_step = 0;
    let mut curr_popcnt;

    let step_width = steps.to_string().len();
    let size_width = size.to_string().len();
    let mut write = write;

    let mut step = 0;
    loop {
        curr_popcnt = 0;
        for popcnt_recv in popcnt_recvs.iter() {
            curr_popcnt += popcnt_recv.recv().unwrap();
        }
        if curr_popcnt < min_popcnt {
            min_popcnt = curr_popcnt;
            min_popcnt_step = step;
        }
        if curr_popcnt > max_popcnt {
            max_popcnt = curr_popcnt;
            max_popcnt_step = step;
        }

        match write {
            None => (),
            Some(mut w) => {
                write!(w, "{:>width$} ", step, width = step_width).unwrap_or_default();
                write_send.send(w).unwrap();
                w = write_recv.recv().unwrap();
                writeln!(w, " {:>width$}", curr_popcnt, width = size_width).unwrap_or_default();
                write = Some(w);
            }
        }

        step += 1;
        if step > steps {
            break;
        }
    }

    worker_handles
        .into_iter()
        .for_each(|worker_handle| worker_handle.join().unwrap());

    (
        (min_popcnt, min_popcnt_step),
        (max_popcnt, max_popcnt_step),
        curr_popcnt,
    )
}

#[cfg(test)]
mod tests;
