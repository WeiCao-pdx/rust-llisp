mod lib;

fn main() {
  let mut a = String::from("
  (if (> (val x) 0)
    (set! z (f (+ (* a (val x)) b))))
  ");
  lib::tokenizer::pre_process(&mut a);
  lib::tokenizer::spliter(&a, ' ');
}
