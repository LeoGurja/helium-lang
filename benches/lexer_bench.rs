use criterion::{criterion_group, Criterion};
use helium::lexer::{Lexer, Token};

static INPUT: &str = "
  let x = 0

  fn main() { 
    for i in [1, 2, 3, 4, 5] {
      x = x + i
    }

    if x >= 6 {
      print(x)
    } else {
      print('too small')
    }

    0
  }
  
";

pub fn bench_lexer(c: &mut Criterion) {
  c.bench_function("lexer", |b| {
    b.iter(|| {
      let mut lex = Lexer::new(INPUT);
      while let Some(Token::Eof) = lex.next() {}
    })
  });
}

criterion_group!(lexer_benches, bench_lexer);
