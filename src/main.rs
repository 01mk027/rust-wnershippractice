fn concatenate_string(string1: &str, string2: &str) -> String
{
    let mut result:String = String::new();
    result.push_str(string1);
    result.push_str(string2);
    return result;
}


fn main() {
   let string1: String = String::from("Hello ");
   let string2: String = String::from("World!");
   let result = concatenate_string(&string1, &string2);
   println!("{}", result); 
}


