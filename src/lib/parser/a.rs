
#[derive(Debug)]
struct Point<T>{
  x:T, 
  y:T, 
}

trait Cp{
  fn copy(&self) -> Self;
}
trait Plus<T>{
  fn plus(&self, data: &T) -> T;
}

impl<T: Cp> Cp for Point<T>{
  fn copy(&self) -> Point<T> {
    Point{x:self.x.copy(), y:self.y.copy()}
  }
}
impl<T: Cp+Plus<T>> Plus<Point<T>> for Point<T>{
  fn plus(&self, data: &Point<T>) -> Point<T> {
    Point{
      x: self.x.copy().plus(&data.x.copy()), 
      y: self.y.copy().plus(&data.y.copy()), 
    }
  }
}

impl Cp for i32{
  fn copy(&self) -> i32{
    *self
  }
}
impl Plus<i32> for i32{
  fn plus(&self, data: &i32) -> i32 {
    *self + *data
  }
}

fn main() {
  let a = Point{x:5 as i32, y:6 as i32};
  let b = a.copy();
  let c = a.plus(&b);
  println!("a:{:?}", a);
  println!("b:{:?}", b);
  println!("c:{:?}", c);
}

