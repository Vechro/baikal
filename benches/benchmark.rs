use baikal::conversion_table::ConversionTable;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("replace units", |b| {
        b.iter(|| ConversionTable::replace_units(black_box("56 gb / 6 + 4kib * 5 -4 mb + 4 B".to_string())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
