use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use cses_3311_grid_coloring::{Grid, greedy::solve_greedy, backtracking::solve_backtracking};

fn generate_random_grid(n: usize, m: usize) -> Grid {
    let mut seed = 12345u64;
    let mut rand = || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        seed
    };
    let chars = [b'A', b'B', b'C', b'D'];
    let mut raw_buffer = Vec::with_capacity(n * m);
    for _ in 0..(n * m) {
        raw_buffer.push(chars[(rand() % 4) as usize]);
    }
    Grid { n, m, raw_buffer }
}

fn bench_grid_coloring(c: &mut Criterion) {
    let mut group = c.benchmark_group("Grid Coloring");
    
    // Benchmark Greedy Scan on scaling inputs
    for &size in &[10, 100, 500] {
        let grid = generate_random_grid(size, size);
        group.bench_with_input(BenchmarkId::new("Greedy Scan", size), &grid, |b, g| {
            b.iter(|| solve_greedy(g));
        });
    }

    // Benchmark Backtracking DFS on smaller inputs to prevent stack overflow
    // (since recursion depth is N * M, a 30x30 grid has recursion depth of 900)
    for &size in &[10, 30] {
        let grid = generate_random_grid(size, size);
        group.bench_with_input(BenchmarkId::new("Backtracking DFS", size), &grid, |b, g| {
            b.iter(|| solve_backtracking(g));
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_grid_coloring);
criterion_main!(benches);
