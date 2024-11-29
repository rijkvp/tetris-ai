use criterion::{black_box, criterion_group, criterion_main, Criterion};

// test if copying an [usize; 10] is faster than referencing it

fn copy_array(arr: [usize; 10]) -> usize {
    arr.iter().sum()
}

fn reference_array(arr: &[usize]) -> usize {
    arr.iter().sum()
}

fn benchmark(c: &mut Criterion) {
    let array = [1; 10];
    c.bench_function("copy_array", |b| b.iter(|| copy_array(black_box(array))));
    c.bench_function("reference_array", |b| {
        b.iter(|| reference_array(black_box(&array)))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
