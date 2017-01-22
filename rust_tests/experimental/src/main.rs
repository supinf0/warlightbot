


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
///  match
///  return code
///

use std::io;

fn main() {
    println!(" fun1() returns {} as a value",fun1());
    readline_test();
//     overflow_test_io();
}

// start readline test for stdin, print a String
fn readline_test() {
  println!("EOF (Ctrl+D) for quit");
  loop {
    let mut a = String::new();
    match io::stdin().read_line(&mut a) //.expect("Failed to read line");
    {
      Ok(0) => {break;}, // end of file (EOF) reached
      Ok(n) => {
        println!("read {} bytes",n);
      },
      Err(_) => { println!("failed to read a line"); }
    }
    println!("input: {}",a);
  }
}
// end readline

fn fun1() -> i32 {
 2
}

// start overflow
// remember: behaviour affected by --release, can be modified by other flags
fn overflow_test_io() {
    let a:i8 = overflow_test();
    println!(" overflow_test() returns {} as a value",a);
}

fn overflow_test() -> i8 {
  let a:i8 = 125;
  a+12  // note: this behaves different if compiled with --release or not
}
// end overflow


