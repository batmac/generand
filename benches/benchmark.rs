use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use generand::generate::*;

fn bench_generate(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate");
    //let dict = "😘👍🥳🎾🥹😇🙃".chars().collect();
    let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-".to_string();
    //let sc = s.chars();
    let dict:Vec<char> = s.chars().collect();
    //let iter = dict.iter();
    for i in [1, 32, 100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("vec", i), i, |b, i| {
            b.iter(|| generate_vec(&dict, *i))
        });
        group.bench_with_input(BenchmarkId::new("str", i), i, |b, i| {
            b.iter(|| generate_old(&s, *i))
        });
        group.bench_with_input(BenchmarkId::new("vec to str", i), i, |b, i| {
            b.iter(|| generate_old(&s, *i))
        });
        group.bench_with_input(BenchmarkId::new("for", i), i, |b, i| {
            b.iter(|| generate_vec_for(&dict, *i))
        });
        group.bench_with_input(BenchmarkId::new("generic", i), i, |b, i| {
            b.iter(|| generate_generic(s.chars().collect::<Vec<_>>().into_iter(), *i))
        });

/*         group.bench_with_input(BenchmarkId::new("vec_clone_lately", i), i, |b, i| {
            b.iter(|| generate_vec_clone_lately(&dict, *i))
        });
        group.bench_with_input(BenchmarkId::new("vec_for", i), i, |b, i| {
            b.iter(|| generate_vec_for(&dict, *i))
        }); */
    }
    group.finish();
}

criterion_group!(benches, bench_generate);
criterion_main!(benches);
