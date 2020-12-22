use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::{HashMap, HashSet, BTreeSet};
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use std::iter;
use seahash::SeaHasher;
use ahash::{AHashSet};

const N: usize = 100000;

pub fn hash_ints_benchmark(c: &mut Criterion) {
    c.bench_function("Hash numbers benchmark", |b| b.iter(hash_ints));
}

pub fn hash_str_benchmark(c: &mut Criterion) {
    c.bench_function("Hash strings benchmark", |b| b.iter(hash_str));
}

pub fn hash_str_long_benchmark(c: &mut Criterion) {
    c.bench_function("Hash long strings benchmark", |b| b.iter(hash_long_str));
}

pub fn fast_hash_str_long_benchmark(c: &mut Criterion) {
    c.bench_function("Fast Hash long strings benchmark", |b| b.iter(fast_hash_long_str));
}

pub fn tree_ints_benchmark(c: &mut Criterion) {
    c.bench_function("BTree numbers benchmark", |b| b.iter(tree_ints));
}

pub fn tree_str_benchmark(c: &mut Criterion) {
    c.bench_function("BTree strings benchmark", |b| b.iter(tree_str));
}

pub fn tree_str_long_benchmark(c: &mut Criterion) {
    c.bench_function("BTree long strings benchmark", |b| b.iter(tree_long_str));
}

fn hash_ints() {
    let mut set = HashSet::new();
    for _ in 0..N {
        let x: i32 = rand::random();
        set.get(&x);
        set.insert(x);
    }
}

fn hash_str() {
    let mut set = HashSet::new();
    let mut rng = thread_rng();
    for _ in 0..N {
        let x: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(7)
            .collect();
        set.get(&x);
        set.insert(x);
    }
}

fn hash_long_str() {
    let mut set = HashSet::new();
    let mut rng = thread_rng();
    for _ in 0..N {
        let x: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect();
        set.get(&x);
        set.insert(x);
    }
}

fn fast_hash_long_str() {
    let mut set = AHashSet::new();
    let mut rng = thread_rng();
    for _ in 0..N {
        let x: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect();
        set.get(&x);
        set.insert(x);
    }
}


fn tree_ints() {
    let mut set = BTreeSet::new();
    for _ in 0..N {
        let x: i32 = rand::random();
        set.get(&x);
        set.insert(x);
    }
}

fn tree_str() {
    let mut set = BTreeSet::new();
    let mut rng = thread_rng();
    for _ in 0..N {
        let x: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(7)
            .collect();
        set.get(&x);
        set.insert(x);
    }
}

fn tree_long_str() {
    let mut set = BTreeSet::new();
    let mut rng = thread_rng();
    for _ in 0..N {
        let x: String = iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .map(char::from)
            .take(100)
            .collect();
        set.get(&x);
        set.insert(x);
    }
}

criterion_group!(benches,
    hash_ints_benchmark,
    tree_ints_benchmark,
    hash_str_benchmark,
    tree_str_benchmark,
    hash_str_long_benchmark,
    fast_hash_str_long_benchmark,
    tree_str_long_benchmark,
);
criterion_main!(benches);