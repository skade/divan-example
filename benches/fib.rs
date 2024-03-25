use divan_example;
use divan::AllocProfiler;

// #[global_allocator]
// static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci_rec(n: u64) {
    divan_example::fibonacci_rec(n);
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci_iter_naive(n: u64) {
    divan_example::fibonacci_iter_naive(n);
}

#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci_iter_opt(n: u64) {
    divan_example::fibonacci_iter_opt(n);
}  