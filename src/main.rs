use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

use clap::{Parser, ValueEnum};
#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
enum Mode {
    Seq,
    Par,
}
#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
struct Args {
    /// Execution mode
    #[clap(short, long, value_enum, default_value_t = Mode::Seq)]
    mode: Mode,
    /// Number of threads to use in parallel execution mode
    #[clap(short, long, default_value_t = 4, value_parser = pos_usize_parse)]
    threads: usize,
    /// Update rule
    #[clap(short, long, default_value = "01101110", value_parser = rule_parse)]
    rule: [bool; 8],
    /// Size of the grid
    #[clap(short('n'), long, default_value_t = 19, value_parser = pos_usize_parse)]
    size: usize,
    /// Number of steps to execute
    #[clap(short, long, default_value_t = 10)]
    steps: usize,
    /// Indicies of initially populated cells [default: SIZE-1]
    #[clap(short, long, use_value_delimiter(true), value_name = "INDEX,...", value_parser = index_parse)]
    indices: Option<Vec<usize>>,
    /// Whether or not to visualize the execution
    #[clap(short, long)]
    visualize: bool,
}

use std::str::FromStr;
use std::sync::atomic;
use std::time::Instant;

fn main() {
    let args = {
        // First parse; does not check that `indices` are in bounds.
        let args = Args::parse();
        // `--size` argument has been validated.
        SIZE.store(args.size, atomic::Ordering::Release);
        // Second parse; checks that `indices` are in bounds.
        Args::parse()
    };
    let mode = args.mode;
    let threads = args.threads;
    let rule = args.rule;
    let size = args.size;
    let steps = args.steps;
    let indices = match args.indices {
        None => vec![size - 1],
        Some(indices) => indices,
    };
    let visualize = args.visualize;
    let write = if visualize {
        Some(std::io::stdout())
    } else {
        None
    };

    let start = Instant::now();
    let ((min_popcnt, min_popcnt_step), (max_popcnt, max_popcnt_step), final_popcnt) = match mode {
        Mode::Seq => eca::seq::run_eca(rule, size, steps, indices, write),
        Mode::Par => eca::par::run_eca(threads, rule, size, steps, indices, write),
    };
    let elapsed = {
        let elapsed = start.elapsed();
        elapsed.as_secs_f64() + elapsed.subsec_nanos() as f64 / 1.0e9
    };
    let stats = GLOBAL.stats();

    let step_width = steps.to_string().len();
    let size_width = size.to_string().len();
    println!(
        "Minimum popcount: {:>size_width$} at step {:>step_width$}",
        min_popcnt,
        min_popcnt_step,
        size_width = size_width,
        step_width = step_width
    );
    println!(
        "Maximum popcount: {:>size_width$} at step {:>step_width$}",
        max_popcnt,
        max_popcnt_step,
        size_width = size_width,
        step_width = step_width
    );
    println!(
        "  Final popcount: {:>size_width$} at step {:>step_width$}",
        final_popcnt,
        steps,
        size_width = size_width,
        step_width = step_width
    );
    println!("    Running time: {:>13.10} seconds", elapsed);
    println!("       Allocated: {:>13} bytes", stats.bytes_allocated);
    println!("     Deallocated: {:>13} bytes", stats.bytes_deallocated);
    println!("     Reallocated: {:>13} bytes", stats.bytes_reallocated);
}

fn pos_usize_parse(s: &str) -> Result<usize, String> {
    match usize::from_str(s) {
        Ok(t) => {
            if t > 0 {
                Ok(t)
            } else {
                Err(String::from("must be greater than 0"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn rule_parse(s: &str) -> Result<[bool; 8], String> {
    if s.len() != 8 {
        return Err(String::from("rule must be exactly 8 characters"));
    }
    let mut rule = [false; 8];
    for (i, c) in s.chars().enumerate() {
        match c {
            '0' => {}
            '1' => rule[7 - i] = true,
            _ => return Err(String::from("invalid digit found in rule")),
        }
    }
    Ok(rule)
}

// A hack to validate indices against the size of the grid,
// since the `index_parse` function must be a top-level function.
static SIZE: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

fn index_parse(s: &str) -> Result<usize, String> {
    match usize::from_str(s) {
        Ok(i) => {
            // On first parsing of command-line arguments, `SIZE` will be zero;
            // on second parsing, `SIZE` will be positive.
            let size = SIZE.load(atomic::Ordering::Acquire);
            if size > 0 && i >= size {
                Err(format!("index must be less than SIZE ({})", size))
            } else {
                Ok(i)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
