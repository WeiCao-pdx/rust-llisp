mod lib;

fn main() {
  let mut a = String::from(
   "(set! x*2 (* x 2))"
  );
  let mut b = lib::tokenizer::tokenize(&mut a);
  let c = lib::parser::parse(&mut b);
  c.display();
}

