use hello_rust::hash_table::{HashTable, DefaultHashTableEntity, DefaultHasher, DefaultHashTableGrower};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};

fn bench_with_monotonic_increase(c: &mut Criterion) {
    static BASIC_UNIT: i32 = 1024;

    let mut group = c.benchmark_group("monotonic_insert");
    for insert_size in [BASIC_UNIT, 2 * BASIC_UNIT, 4 * BASIC_UNIT, 8 * BASIC_UNIT, 16 * BASIC_UNIT].iter() {
        group.throughput(Throughput::Elements(*insert_size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(insert_size), insert_size, |bencher, &size| {
            bencher.iter(|| {
                let mut cur_index: i32 = 0;
                let iterator = std::iter::from_fn(move || {
                    cur_index += 1;
                    if cur_index < size as i32
                    {
                        Some(cur_index)
                    } else {
                        None
                    }
                });

                let inserted = true;
                let mut hash_table = HashTable::<i32, DefaultHashTableEntity, DefaultHasher, DefaultHashTableGrower>::new();
                iterator.for_each(|index| { hash_table.insert_key(index, inserted); });
            });
        });
    }
    group.finish();

    // std::iter::from_fn()
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_with_monotonic_increase);
criterion_main!(benches);
