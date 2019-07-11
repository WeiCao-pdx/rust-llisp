use LList::*;
use TokenData::*;

enum TokenData {
  Token(String), 
  List(Box<LList>), 
  /* This type wraps String and List, the purpose is to 
   * make these two types be a same type.
   * e.g: ["a", "b, ["c"]] is allowed.
   * It's used to store parse tree.
   * Consider this lisp code :
   * (define b (+ a 2))
   * after the code is tokenized: ["(", "define", "b", "(", "+", "a", "2", ")", ")"]
   * after the tokens are parsed: ["define", "b", ["+", "a", "2"]]*/
}

enum LList {
  Cons(TokenData, Box<LList>), 
  Nil, 
  /* Consider a list: [1, 2, 3]
   * In Lisp programming language , this list is defined by:
   * cons(1, cons(2, cons(3, nil)))*/
}

struct LinkList{
  this: LList, 
  /* To make it more eaiser to use, the LList should be wrapped */
}

impl LList {
  fn new() -> LList {
    Nil
  }
  fn prepend(&self, data: TokenData) -> LList{
    /* Add a item to the head of list 
     * e.g: add cons("3", nil) to cons("1", cons("2", nil))
     * it will become to 
     *    cons(cons("3", nil), cons("1", cons("2", nil)))*/
    Cons(data, Box::new(self.copy()))
  }
  fn append(&self, data: TokenData) -> LList{
    /* Add a item to the rear of list*/
    match *self{
      Cons(ref head, ref tail) => {
        let left = match head {
          Token(x) => Token(x.to_string()), 
          List(ref x) => List(Box::new(x.copy())), 
        }; // 这里可以简化
        Cons(left, Box::new(tail.append(data)))
      }, 
      Nil => {
        Cons(data, Box::new(Nil))
      }
    }
  }
  fn copy(&self) -> LList {
    /* Copy the whole list */
    match *self {
      Cons(ref head, ref tail) => {
        let left = match head {
          Token(x) => Token(x.to_string()), 
          List(ref x) => List(Box::new(x.copy())), 
        };
        let right = tail.copy();
        Cons(left, Box::new(right))
      }, 
      Nil => {
        Nil
      }
    }
  }
  fn display(&self){
    print!("[");
    self._display();
    println!("]");
  }
  fn _display(&self){
    match *self {
      Cons(ref head, ref tail) => {
        match head {
          Token(x) => print!("{}, ", x), 
          List(ref x) => {
            print!("[");
            x._display();
            print!("], ");
          }
        };
        tail._display();
      }, 
      Nil => {
        print!("^");
      }
    }
  }
}
trait ToToken {
  fn to_tokendata(&self) -> TokenData;
}
impl ToToken for String{
  fn to_tokendata(&self) -> TokenData{
    Token(self.to_string())
  }
}
impl ToToken for &str{
  fn to_tokendata(&self) -> TokenData{
    Token(self.to_string())
  }
}
impl ToToken for LinkList{
  fn to_tokendata(&self) -> TokenData{
    List(Box::new(self.this.copy()))
  }
}
impl ToToken for LList{
  fn to_tokendata(&self) -> TokenData{
    List(Box::new(self.copy()))
  }
}
impl LinkList{
  fn new() -> LinkList{
    LinkList{this: LList::new()}
  }
  fn prepend<T: ToToken>(&mut self, data: T) {
    self.this = self.this.prepend(data.to_tokendata());
  }
  fn append<T: ToToken>(&mut self, data: T) {
    self.this = self.this.append(data.to_tokendata());
  }
  fn display(&self){
    self.this.display();
  }
  /*
  fn reversed(&self) -> LinkList{
    LinkList{this: self.this.reversed()}
  }
  */
  fn copy(&self) -> LinkList{
    LinkList{this: self.this.copy()}
  }

}
fn main() {
  let mut a = LinkList::new();
  a.prepend("aaa");
  a.prepend("bbb".to_string());
  a.display();
  let mut b = a.copy();
  b.prepend(a.copy());
  b.display();
  let mut c = b.copy();
  c.append(a.copy());
  c.display();
}
