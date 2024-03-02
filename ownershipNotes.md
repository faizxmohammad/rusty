`Ownership is a unique feature of rust, which allows users to write memory safe code, without a garbage collector.`

#### features of ownership model:
1. Control over memory.
2. Error free.
3. Faster runtime.
4. Smaller program size.

#### cons of ownership:
1. Slower write time. (€fighting with borrower checker)
2. Learning cureve.



# RULES OF OWNER SHIP MODEL:
1. Each value in  Rust has a variable that's called its owner.
2. There can be only one owner at a time.
3. When the owner gets out of scope, the value will be dropped.


```ownership in variable:```
 ```Rust
    // case 1:
    let x = 21;
    let y = x;
    println!("your x value is: {}",x);  

    // case 2:
    let str = String::from("new string");
    let str2 = str;
    println!("{}", str)

```

`Let's analyze both cases:`
#### case 1: 
In this case, x is a simple integer, which implements the Copy trait. When you assign x to y, a copy of the value is made because integers are simple and cheap to copy. Therefore, x retains ownership of its value, and you can still use it after assigning its value to y. So, this code works fine.

#### case 2:

In this case, str is a String, which represents a heap-allocated string and does not implement the Copy trait. When you assign str to str2, ownership of the string data is transferred from str to str2. This means str no longer owns the string data, and if you try to use str afterward, Rust will consider it invalid because its ownership has been moved. Hence, attempting to use str after transferring ownership (let str2 = str;) results in a compile-time error.

If you want to clone the string data instead of moving ownership, you can use the .clone() method:
 ```Rust
let str = String::from("new string");
let str2 = str.clone(); // --> This will create a new copy of the string data on the heap, and both str and str2 will own their separate copies.
println!("{}", str);
```
final verdict: The str will be dropped/deleted from stack and heap memory


`Ownership in Functions:`
 ```Rust
fn main(){
   let new_str = String::from("my new String");
   my_function(new_str);
   // now the below line throws an error because the string ownership is now passed to the function,
   // because the ownership is transferred to another function, it gets removed from stack and heap memory
    println!("{}", new_str);
}

fn my_function(str:String){
    print!("{}", str);
}
```


## We can take back ownership using the following way:

```Rust
fn main(){
   let mut new_str = String::from("my new String");
   new_str = my_function(new_str);
   println!("{}", new_str);
   
}

fn my_function(str:String) -> String{
    print!("{}", str);
    str
}
```


`This is a tedious process of giving and taking back the ownership, to solve this we use references:`

 #### handling ownerships using references:
 ```Rust
 fn main(){
   let string: String = String::from("hello world");
   reference_function(&string);
   println!("{}", string);
 }
 fn reference_function(str: &String)->usize{
    str.len()
}
```

`Here instead of passing the actual values to the function, we pass the references to the function.`

###### Note: References are immutable by default. Ou cannot modified referred variables.

 ```Rust
 fn main(){
   let string: String = String::from("hello world");
   reference_function(&string);
   println!("{}", string);
 }
 fn reference_function(str: &String)->usize{
    str.push_str("welcome") // this wont work
    str.len()
}
```

#### To resolve this we pass mutable reference to the function
```Rust
 fn main(){
   let mut string: String = String::from("hello world");
   reference_function(&mut string);
   println!("{}", string);
 }
 fn reference_function(str: &mut String) ->usize{
    str.push_str("welcome") // this wont work
    str.len()
}
```

