use std::collections::LinkedList;
use TokenData::*;

pub enum TokenData{
  Str(String), 
  /*
  Int(i64), 
  Float(f64), 
  */
  List(LinkedList<TokenData>), 
}
impl TokenData{
  pub fn display(&self){
    match self{
      Str(x) => {
        println!("{}", x);
      }, 
      List(x) => {
        x.display();
      }
    }
  }
}

trait ToTokenData{
  fn to_tokendata(&self) -> TokenData;
}
trait TokenList{
  fn deepcopy(&self) -> LinkedList<TokenData>;
  fn append_td<T: ToTokenData>(&mut self, data: T);
  fn display(&self);
  fn _display(&self);
}

impl ToTokenData for String{
  fn to_tokendata(&self) -> TokenData{
    Str(self.to_string())
  }
}
impl ToTokenData for &str{
  fn to_tokendata(&self) -> TokenData{
    Str(self.to_string())
  }
}
impl ToTokenData for LinkedList<TokenData>{
  fn to_tokendata(&self) -> TokenData{
    List(self.deepcopy())
  }
}
impl TokenList for LinkedList<TokenData>{
  fn deepcopy(&self) -> LinkedList<TokenData>{
    let mut temp:LinkedList<TokenData> = LinkedList::new();
    let mut iter = self.iter();
    loop{
      let item = iter.next();
      match item{
        Some(x) => {
          match x {
            Str(y) => {
              temp.push_back(y.to_tokendata());
            }, 
            List(y) => {
              temp.push_back(List(y.deepcopy()));
            }
          }
        }, 
        None => {
          break
        }
      }
    }
    temp
  }
  fn append_td<T: ToTokenData>(&mut self, data: T){
    self.push_back(data.to_tokendata());
  }
  fn display(&self){
    print!("[");
    self._display();
    println!("]");
  }
  fn _display(&self){
    let mut iter = self.iter();
    loop{
      let item = iter.next();
      match item{
        Some(x) => {
          match x {
            Str(y) => {
              print!("{} ", y);
            }, 
            List(y) => { 
              print!("[");
              y._display();
              print!("] ");
            }
          }
        }, 
        None => {
          break;
        }
      }
    }
  }
}
fn read_tokens(tokens: &mut Vec<String>) -> TokenData{
  let mut temp: LinkedList<TokenData> = LinkedList::new();
  let a_token = tokens.pop();
  match a_token {
    Some(x) => {
      if x == String::from("(") {
        while tokens[tokens.len()-1] != String::from(")"){
          temp.push_back(read_tokens(tokens));
        }
        let _ = tokens.pop();
        List(temp)
      } else {
        Str(x)
      }
    }, 
    None => {
      List(temp)
    }, 
  }
}
pub fn parse(tokens: &mut Vec<String>) -> TokenData{
  tokens.reverse();
  let result = read_tokens(tokens);
  result
}
