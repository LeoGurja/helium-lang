use crate::helium;
use crate::object::{Object, Type};

#[test]
fn book_name() {
  let input = String::from(
    "
    let name = 'Helium'
    let age = 1
    let inspirations = ['monkey', 'javascript', 'ruby']
    let book = {
      'title': 'Writing A Compiler In Go',
      'author': 'Thorsten Ball',
      'prequel': 'Writing An Interpreter In Go'
    }

    fn printBookName(book) {
      let title = book['title']
      let author = book['author']
      author + ' - ' + title
    }

    printBookName(book)
  ",
  );

  let result = helium::run(input).unwrap();

  assert_eq!(
    result,
    Object::new(Type::String(String::from(
      "Thorsten Ball - Writing A Compiler In Go"
    )))
  )
}

#[test]
fn fibonacci() {
  let input = String::from(
    "
    fn fibonacci(x) {
      if x == 0 {
        0
      } else if x == 1 {
        1
      } else {
        fibonacci(x - 1) + fibonacci(x - 2)
      }
    }

    let numbers = [1, 1 + 1, 4 - 1, 2 * 2, 2 + 3, 12 / 2];
    map(numbers, fibonacci)
  ",
  );

  let result = helium::run(input).unwrap();

  assert_eq!(
    result,
    Object::new(Type::Array(vec![
      Object::new(Type::Integer(1)),
      Object::new(Type::Integer(1)),
      Object::new(Type::Integer(2)),
      Object::new(Type::Integer(3)),
      Object::new(Type::Integer(5)),
      Object::new(Type::Integer(8))
    ]))
  )
}
