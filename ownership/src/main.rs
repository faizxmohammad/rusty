fn main() {
//     RULES OF OWNERSHIP MODEL:
// 1. Each value in  Rust has a variable that's called its owner.
// 2. There can be only one owner at a time.
// 3. When the owner gets out of scope, the value will be dropped.
{
    let mut s = "hello".to_string();
    // perform some operations on s
    s += "I am faiz";
    println!("{}",s);

    let mut str = String::from("hello");
    str = str + "I am faiz";

    println!("{}", str);

    // Memory and allocation

    // case 1:
    let x = 21;
    let y = x;
    println!("your x value is: {}",x);  

    // case 2:
    let str = String::from("new string");
    let str2 = str;
   // println!("{}", str) // throws a compile time error because str is moved to str2



   // ownerships in functions
   let new_str = String::from("my new String");
   my_function(new_str);
   // now the below line throws an error because the string ownership is now passed to the function,
   // because the ownership is transferred to another function, it gets removed from stack and heap memory
   // println!("{}", new_str);


   // handling ownerships using references:
   let mut string: String = String::from("hello world");
   reference_function(&mut string);
   println!("{}", string);  
}

}


fn my_function(str:String){
    println!("{}", str);
}

fn reference_function(str: &mut String)->usize{
    str.push_str("Welcome");
    str.len()
}