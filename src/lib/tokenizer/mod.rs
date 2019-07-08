pub fn tokenize(a_str:&mut String) -> Vec<String>{
  spliter(&pre_process(a_str), ' ')
}
//---------------------------------------------------
pub fn find_symbol(a_str:&String, pos: i64, symbol:char) -> i64{
  let mut index = pos;
  loop{
    if a_str.chars().nth(index as usize) == None {
      break;
    } else {
      if a_str.chars().nth(index as usize).unwrap() != symbol{
        index += 1;
      } else {
        break;
      }
    }
  }
  index
}
pub fn find_symbol_is_not(a_str:&String, pos: i64, symbol:char) -> i64{
  let mut index = pos;
  loop{
    if a_str.chars().nth(index as usize) == None {
      break;
    } else {
      if a_str.chars().nth(index as usize).unwrap() == symbol{
        index += 1;
      } else {
        break;
      }
    }
  }
  index
}
//---------------------------------------------------
pub fn pre_process(a_str:&String) -> String{
  let mut a_str = a_str.clone();
  loop{
    match a_str.chars().nth(0){
      None => break, 
      Some(' ') => a_str.remove(0),
      Some(_) => break, 
    };
  }
  let mut idxs: Vec<i32> = Vec::new();
  let mut index = 0;
  loop{
    match a_str.chars().nth(index){
      None => break, 
      Some('(') | Some(')') => {idxs.push(index as i32); index+=1},
      Some(_) => index += 1, 
    };
  }
  idxs.reverse();
  for i in 0..idxs.len(){
    a_str.insert(idxs[i] as usize +1, ' ');
    a_str.insert(idxs[i] as usize, ' ');
  }
  a_str
  //println!("{}", a_str);
}

pub fn spliter(a_str: &String, symbol: char) -> Vec<String>{
  let mut result:Vec<String> = Vec::new();
  let length = a_str.len() as i64;
  let mut index = find_symbol_is_not(a_str, 0, ' ');;

  while index < length{
    let end = find_symbol(a_str, index, symbol);
    let lex = ||->String{
      let mut substr = String::new();
      for i in index..end{
        let ch =  a_str.chars().nth(i as usize);
        if ch != None{
          substr.push(ch.unwrap());
        } else {
          break;
        }
      }
      substr
    }();
    if lex != "\n"{
      result.push(lex);
    }
    index = find_symbol_is_not(a_str, end, ' ');
  }
  result
}
