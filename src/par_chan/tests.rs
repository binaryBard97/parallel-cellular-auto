use std::io::{self, Cursor, Write};
use std::sync::{Arc, Mutex};

use crate::par_chan;
use crate::seq;

struct ArcMutex<T>(Arc<Mutex<T>>);
impl<T> ArcMutex<T> {
    fn new(x: T) -> Self {
        ArcMutex(Arc::new(Mutex::new(x)))
    }
}
impl<T> Clone for ArcMutex<T> {
    fn clone(&self) -> Self {
        ArcMutex(Arc::clone(&self.0))
    }
}
impl<W> Write for ArcMutex<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut w = self.0.lock().unwrap();
        w.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        let mut w = self.0.lock().unwrap();
        w.flush()
    }
}

fn mk_test(
    threads: usize,
    rule: [bool; 8],
    size: usize,
    steps: usize,
    indices: Vec<usize>,
    visualize: bool,
) {
    let (mut output_seq, output_par) = if visualize {
        (Some(Vec::<u8>::new()), Some(Vec::<u8>::new()))
    } else {
        (None, None)
    };
    let write_seq = output_seq.as_mut().map(Cursor::new);
    let write_par = output_par.map(|v| ArcMutex::new(Cursor::new(v)));
    let res_seq = seq::run_eca(rule, size, steps, indices.clone(), write_seq);
    let res_par = par_chan::run_eca(
        threads,
        rule,
        size,
        steps,
        indices.clone(),
        write_par.clone(),
    );
    assert_eq!(
        res_seq,
        res_par,
        "par_chan::run_eca({}, {:?}, {}, {}, vec!{:?}, {})",
        threads,
        rule,
        size,
        steps,
        indices.as_slice(),
        visualize
    );
    assert_eq!(
        output_seq.map(|s| String::from_utf8(s).unwrap()),
        write_par.map(|amw| {
            let cursor = amw.0.lock().unwrap();
            String::from(std::str::from_utf8(cursor.get_ref()).unwrap())
        }),
        "par_chan::run_eca({}, {:?}, {}, {}, vec!{:?}, {})",
        threads,
        rule,
        size,
        steps,
        indices.as_slice(),
        visualize
    );
}

const RULE_30: [bool; 8] = [false, true, true, true, true, false, false, false];
const RULE_110: [bool; 8] = [false, true, true, true, false, true, true, false];

mod run {
    mod rule_30 {
        use super::super::mk_test;
        use super::super::RULE_30;

        #[test]
        fn test01() {
            mk_test(1, RULE_30, 19, 10, vec![0], false)
        }

        #[test]
        fn test02() {
            mk_test(2, RULE_30, 19, 10, vec![18], false)
        }

        #[test]
        fn test03() {
            mk_test(3, RULE_30, 19, 10, vec![9], false)
        }

        #[test]
        fn test04() {
            mk_test(
                4,
                RULE_30,
                19,
                10,
                vec![0, 18, 2, 16, 4, 14, 6, 12, 8, 10],
                false,
            )
        }

        #[test]
        fn test05() {
            mk_test(
                5,
                RULE_30,
                19,
                10,
                vec![17, 15, 13, 11, 9, 7, 5, 3, 1],
                false,
            )
        }

        #[test]
        fn test06() {
            mk_test(1, RULE_30, 199, 100, vec![0], false)
        }

        #[test]
        fn test07() {
            mk_test(2, RULE_30, 199, 100, vec![198], false)
        }

        #[test]
        fn test08() {
            mk_test(3, RULE_30, 199, 100, vec![99], false)
        }

        #[test]
        fn test09() {
            mk_test(4, RULE_30, 199, 100, vec![2, 0, 1, 3, 4], false)
        }

        #[test]
        fn test10() {
            mk_test(5, RULE_30, 199, 100, vec![0, 100, 99, 98], false)
        }

        #[test]
        fn test11() {
            mk_test(1, RULE_30, 199, 0, vec![0, 100, 99, 98], false)
        }

        #[test]
        fn test12() {
            mk_test(2, RULE_30, 199, 0, vec![2, 0, 1, 3, 4], false)
        }

        #[test]
        fn test13() {
            mk_test(3, RULE_30, 199, 0, vec![99], false)
        }

        #[test]
        fn test14() {
            mk_test(4, RULE_30, 199, 0, vec![198], false)
        }

        #[test]
        fn test15() {
            mk_test(5, RULE_30, 199, 0, vec![0], false)
        }

        #[test]
        fn test16() {
            mk_test(1, RULE_30, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test17() {
            mk_test(2, RULE_30, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test18() {
            mk_test(3, RULE_30, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test19() {
            mk_test(4, RULE_30, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test20() {
            mk_test(1, RULE_30, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test21() {
            mk_test(2, RULE_30, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test22() {
            mk_test(3, RULE_30, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test23() {
            mk_test(4, RULE_30, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test24() {
            mk_test(1, RULE_30, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test25() {
            mk_test(2, RULE_30, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test26() {
            mk_test(3, RULE_30, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test27() {
            mk_test(4, RULE_30, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test28() {
            mk_test(1, RULE_30, 1, 100, vec![0], false)
        }

        #[test]
        fn test29() {
            mk_test(2, RULE_30, 1, 100, vec![0], false)
        }

        #[test]
        fn test30() {
            mk_test(3, RULE_30, 1, 100, vec![0], false)
        }

        #[test]
        fn test31() {
            mk_test(4, RULE_30, 1, 100, vec![0], false)
        }

        #[test]
        fn test32() {
            mk_test(1, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test33() {
            mk_test(2, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test34() {
            mk_test(3, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test35() {
            mk_test(4, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test36() {
            mk_test(5, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test37() {
            mk_test(1, RULE_30, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test38() {
            mk_test(2, RULE_30, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test39() {
            mk_test(3, RULE_30, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test40() {
            mk_test(4, RULE_30, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test41() {
            mk_test(5, RULE_30, 3999, 1000, vec![3998], false)
        }
    }

    mod rule_110 {
        use super::super::mk_test;
        use super::super::RULE_110;

        #[test]
        fn test01() {
            mk_test(1, RULE_110, 19, 10, vec![0], false)
        }

        #[test]
        fn test02() {
            mk_test(2, RULE_110, 19, 10, vec![18], false)
        }

        #[test]
        fn test03() {
            mk_test(3, RULE_110, 19, 10, vec![9], false)
        }

        #[test]
        fn test04() {
            mk_test(
                4,
                RULE_110,
                19,
                10,
                vec![0, 18, 2, 16, 4, 14, 6, 12, 8, 10],
                false,
            )
        }

        #[test]
        fn test05() {
            mk_test(
                5,
                RULE_110,
                19,
                10,
                vec![17, 15, 13, 11, 9, 7, 5, 3, 1],
                false,
            )
        }

        #[test]
        fn test06() {
            mk_test(1, RULE_110, 199, 100, vec![0], false)
        }

        #[test]
        fn test07() {
            mk_test(2, RULE_110, 199, 100, vec![198], false)
        }

        #[test]
        fn test08() {
            mk_test(3, RULE_110, 199, 100, vec![99], false)
        }

        #[test]
        fn test09() {
            mk_test(4, RULE_110, 199, 100, vec![2, 0, 1, 3, 4], false)
        }

        #[test]
        fn test10() {
            mk_test(5, RULE_110, 199, 100, vec![0, 100, 99, 98], false)
        }

        #[test]
        fn test11() {
            mk_test(1, RULE_110, 199, 0, vec![0, 100, 99, 98], false)
        }

        #[test]
        fn test12() {
            mk_test(2, RULE_110, 199, 0, vec![2, 0, 1, 3, 4], false)
        }

        #[test]
        fn test13() {
            mk_test(3, RULE_110, 199, 0, vec![99], false)
        }

        #[test]
        fn test14() {
            mk_test(4, RULE_110, 199, 0, vec![198], false)
        }

        #[test]
        fn test15() {
            mk_test(5, RULE_110, 199, 0, vec![0], false)
        }

        #[test]
        fn test16() {
            mk_test(1, RULE_110, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test17() {
            mk_test(2, RULE_110, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test18() {
            mk_test(3, RULE_110, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test19() {
            mk_test(4, RULE_110, 4, 100, vec![2, 0, 1, 3], false)
        }

        #[test]
        fn test20() {
            mk_test(1, RULE_110, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test21() {
            mk_test(2, RULE_110, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test22() {
            mk_test(3, RULE_110, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test23() {
            mk_test(4, RULE_110, 3, 100, vec![2, 0, 1], false)
        }

        #[test]
        fn test24() {
            mk_test(1, RULE_110, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test25() {
            mk_test(2, RULE_110, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test26() {
            mk_test(3, RULE_110, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test27() {
            mk_test(4, RULE_110, 2, 100, vec![0, 1], false)
        }

        #[test]
        fn test28() {
            mk_test(1, RULE_110, 1, 100, vec![0], false)
        }

        #[test]
        fn test29() {
            mk_test(2, RULE_110, 1, 100, vec![0], false)
        }

        #[test]
        fn test30() {
            mk_test(3, RULE_110, 1, 100, vec![0], false)
        }

        #[test]
        fn test31() {
            mk_test(4, RULE_110, 1, 100, vec![0], false)
        }

        #[test]
        fn test32() {
            mk_test(1, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test33() {
            mk_test(2, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test34() {
            mk_test(3, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test35() {
            mk_test(4, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test36() {
            mk_test(5, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test37() {
            mk_test(1, RULE_110, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test38() {
            mk_test(2, RULE_110, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test39() {
            mk_test(3, RULE_110, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test40() {
            mk_test(4, RULE_110, 3999, 1000, vec![3998], false)
        }

        #[test]
        fn test41() {
            mk_test(5, RULE_110, 3999, 1000, vec![3998], false)
        }
    }
}

mod vis {
    mod rule_30 {
        use super::super::mk_test;
        use super::super::RULE_30;

        #[test]
        fn test01() {
            mk_test(1, RULE_30, 19, 10, vec![0], true)
        }

        #[test]
        fn test02() {
            mk_test(5, RULE_30, 19, 10, vec![18], true)
        }

        #[test]
        fn test03() {
            mk_test(5, RULE_30, 39, 20, vec![19], true)
        }

        #[test]
        fn test04() {
            mk_test(5, RULE_30, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test05() {
            mk_test(5, RULE_30, 3999, 1000, vec![3998], false)
        }
    }
    mod rule_110 {
        use super::super::mk_test;
        use super::super::RULE_110;

        #[test]
        fn test01() {
            mk_test(1, RULE_110, 19, 10, vec![0], true)
        }

        #[test]
        fn test02() {
            mk_test(5, RULE_110, 19, 10, vec![18], true)
        }

        #[test]
        fn test03() {
            mk_test(5, RULE_110, 39, 20, vec![19], true)
        }

        #[test]
        fn test04() {
            mk_test(5, RULE_110, 1999, 1000, vec![0], false)
        }

        #[test]
        fn test05() {
            mk_test(5, RULE_110, 3999, 1000, vec![3998], false)
        }
    }
}
