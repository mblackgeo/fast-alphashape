use _fast_alphashape::alphashape_edges;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

pub fn criterion_benchmark(c: &mut Criterion) {
    let arr = Array::random((1000, 2), Uniform::new(-90.0, 90.0));

    c.bench_function("1k points", |b| {
        b.iter(|| alphashape_edges(black_box(arr.view()), black_box(1.0)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
