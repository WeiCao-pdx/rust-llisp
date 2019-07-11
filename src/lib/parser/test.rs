use std::fmt::*;

#[derive(Debug)]
struct Point {
  x: i32, 
  y: i32
}

impl Point{
  fn swap(&mut self) {
    let temp = self.x;
    self.x = self.y;
    self.y = temp;
  }
}

fn main() {
  let mut a = Point{x: 5, y: 3};
  a.swap();
  println!("{:?}", a);
}
