


/// List of things you need to learn:
///  higher order functions (?)
///  print to stderr
///  random numbers
///  iterators/for loop
///  traits
///  use multiple .rs files
///  tuples
///  function pointers
///  loop lables
///
///
///

fn main() {
    println!("Hello, world!");
    println!(" fun1() returns {} as a value",fun1());
    let a:i8 = overflow_test();
    println!(" overflow_test() returns {} as a value",a);
}


fn fun1() -> i32 {
 2
}

fn overflow_test() -> i8 {
  let a:i8 = 125;
  a+12  // note: this behaves different if compiled with --release or not
}


