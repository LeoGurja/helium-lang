use criterion::criterion_main;
#[cfg(test)]
mod lexer_bench;

criterion_main!(lexer_bench::lexer_benches);
