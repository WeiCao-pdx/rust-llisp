mod lib;

fn main() {
  let mut a = String::from("
  (if (> (val x) 0)
    (set! z (f (+ (* a (val x)) b))))
  ");
  println!("{:?}", lib::tokenizer::tokenizer(&mut a));
}
