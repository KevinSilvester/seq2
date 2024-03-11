use criterion::{black_box, criterion_group, criterion_main, Criterion};
use seq2::lexer::Lexer;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lexer", |b| b.iter(|| {
        let mut lexer = Lexer::new(black_box("{1..=20, s:1, m:*10-(200 ^ 5)}, -1, -200000000, -3, -2, -3, {1..=3, s:2, m:+2}, (200 ^ 2 + 1)"));
        let _ = lexer.lex();
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
