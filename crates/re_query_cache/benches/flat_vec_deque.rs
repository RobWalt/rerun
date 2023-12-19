use criterion::{criterion_group, criterion_main, Criterion};

use itertools::Itertools as _;

use re_query_cache::FlatVecDeque;

// TODO: all of this should be core_benchmarks

// ---

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

criterion_group!(
    benches,
    range,
    insert,
    insert_range,
    insert_with,
    remove,
    remove_range
);
criterion_main!(benches);

// ---

// `cargo test` also runs the benchmark setup code, so make sure they run quickly:
#[cfg(debug_assertions)]
mod constants {
    pub const INITIAL_VALUES_PER_ENTRY: usize = 1;
    pub const INITIAL_NUM_ENTRIES: usize = 1;
    pub const ADDED_VALUES_PER_ENTRY: usize = 1;
    pub const ADDED_NUM_ENTRIES: usize = 1;
}

#[cfg(not(debug_assertions))]
mod constants {
    pub const INITIAL_VALUES_PER_ENTRY: usize = 1000;
    pub const INITIAL_NUM_ENTRIES: usize = 100;
    pub const ADDED_VALUES_PER_ENTRY: usize = 1000;
    pub const ADDED_NUM_ENTRIES: usize = 5;
}

#[allow(clippy::wildcard_imports)]
use self::constants::*;

// ---

fn range(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(
        (ADDED_NUM_ENTRIES * ADDED_VALUES_PER_ENTRY) as _,
    ));

    {
        group.bench_function("range/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let v: FlatVecDeque<i64> = base.clone();
                v.range(0..ADDED_NUM_ENTRIES)
                    .map(ToOwned::to_owned)
                    .collect_vec()
            });
        });
        group.bench_function("range/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let v: FlatVecDeque<i64> = base.clone();
                v.range(
                    INITIAL_NUM_ENTRIES / 2 - ADDED_NUM_ENTRIES / 2
                        ..INITIAL_NUM_ENTRIES / 2 + ADDED_NUM_ENTRIES / 2,
                )
                .map(ToOwned::to_owned)
                .collect_vec()
            });
        });
        group.bench_function("range/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let v: FlatVecDeque<i64> = base.clone();
                v.range(INITIAL_NUM_ENTRIES - ADDED_NUM_ENTRIES..INITIAL_NUM_ENTRIES)
                    .map(ToOwned::to_owned)
                    .collect_vec()
            });
        });
    }
}

fn insert(c: &mut Criterion) {
    let added = (0..ADDED_VALUES_PER_ENTRY as i64).collect_vec();

    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(added.len() as _));

    {
        group.bench_function("insert/empty", |b| {
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = FlatVecDeque::new();
                v.insert(0, added.clone().into_iter());
                v
            });
        });
    }

    {
        group.bench_function("insert/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert(0, added.clone().into_iter());
                v
            });
        });
        group.bench_function("insert/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert(INITIAL_NUM_ENTRIES / 2, added.clone().into_iter());
                v
            });
        });
        group.bench_function("insert/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert(INITIAL_NUM_ENTRIES, added.clone().into_iter());
                v
            });
        });
    }
}

fn insert_range(c: &mut Criterion) {
    let added = (0..ADDED_NUM_ENTRIES as i64)
        .map(|_| (0..ADDED_VALUES_PER_ENTRY as i64).collect_vec())
        .collect_vec();

    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(
        (ADDED_NUM_ENTRIES * ADDED_VALUES_PER_ENTRY) as _,
    ));

    {
        group.bench_function("insert_range/empty", |b| {
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = FlatVecDeque::new();
                v.insert_range(0, added.clone());
                v
            });
        });
    }

    {
        group.bench_function("insert_range/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_range(0, added.clone());
                v
            });
        });
        group.bench_function("insert_range/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_range(INITIAL_NUM_ENTRIES / 2, added.clone());
                v
            });
        });
        group.bench_function("insert_range/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_range(INITIAL_NUM_ENTRIES, added.clone());
                v
            });
        });
    }
}

fn insert_with(c: &mut Criterion) {
    let mut added: FlatVecDeque<i64> = FlatVecDeque::new();
    for i in 0..ADDED_NUM_ENTRIES {
        added.insert(
            i,
            (0..ADDED_VALUES_PER_ENTRY as i64).collect_vec().into_iter(),
        );
    }

    let added = FlatVecDeque::from_vecs(
        std::iter::repeat_with(|| (0..ADDED_VALUES_PER_ENTRY as i64).collect_vec())
            .take(ADDED_NUM_ENTRIES),
    );

    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(
        (ADDED_NUM_ENTRIES * ADDED_VALUES_PER_ENTRY) as _,
    ));

    {
        group.bench_function("insert_with/empty", |b| {
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = FlatVecDeque::new();
                v.insert_with(0, added.clone());
                v
            });
        });
    }

    {
        group.bench_function("insert_with/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_with(0, added.clone());
                v
            });
        });
        group.bench_function("insert_with/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_with(INITIAL_NUM_ENTRIES / 2, added.clone());
                v
            });
        });
        group.bench_function("insert_with/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.insert_with(INITIAL_NUM_ENTRIES, added.clone());
                v
            });
        });
    }
}

fn remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(1));

    {
        group.bench_function("remove/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove(0);
                v
            });
        });
        group.bench_function("remove/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove(INITIAL_NUM_ENTRIES / 2);
                v
            });
        });
        group.bench_function("remove/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove(INITIAL_NUM_ENTRIES - 1);
                v
            });
        });
    }
}

fn remove_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("flat_vec_deque");
    group.throughput(criterion::Throughput::Elements(
        (ADDED_NUM_ENTRIES * ADDED_VALUES_PER_ENTRY) as _,
    ));

    {
        group.bench_function("remove_range/prefilled/front", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove_range(0..ADDED_NUM_ENTRIES);
                v
            });
        });
        group.bench_function("remove_range/prefilled/middle", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove_range(
                    INITIAL_NUM_ENTRIES / 2 - ADDED_NUM_ENTRIES / 2
                        ..INITIAL_NUM_ENTRIES / 2 + ADDED_NUM_ENTRIES / 2,
                );
                v
            });
        });
        group.bench_function("remove_range/prefilled/back", |b| {
            let base = create_prefilled();
            b.iter(|| {
                let mut v: FlatVecDeque<i64> = base.clone();
                v.remove_range(INITIAL_NUM_ENTRIES - ADDED_NUM_ENTRIES..INITIAL_NUM_ENTRIES);
                v
            });
        });
    }
}

// ---

fn create_prefilled() -> FlatVecDeque<i64> {
    FlatVecDeque::from_vecs(
        std::iter::repeat_with(|| (0..INITIAL_VALUES_PER_ENTRY as i64).collect_vec())
            .take(INITIAL_NUM_ENTRIES),
    )
}
