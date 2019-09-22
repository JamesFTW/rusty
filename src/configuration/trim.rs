pub fn trim(s:String) -> String {
  let mut char_vec: Vec<char> = s.chars().collect();
  char_vec.remove(0);

  let veclen = char_vec.len() - 1;
  char_vec.remove(veclen);

  let string: String = char_vec.into_iter().collect();
  return string;
}