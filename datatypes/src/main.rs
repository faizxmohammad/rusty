fn main() {
    // println!("Hello, world!");
    // const COUNT:u32 = 100_000;
    // let COUNTER = 100_000_000;

    // let val:u32 = 32;
    // let val:i32 = 33;
    // println!("{}",val);
    // println!{"Your count is {}",COUNTER}

    // --------------- Scalar datatypes --------------
    // Integers
    // Floating point numbers
    // Booleans
    // Characters

    // -------------- Compound Datatypes --------------

    // Tuples
    let tuple = ("This is a string", 34); // tuples are fixed size

    // getting values of a tuple
    // 1.Destructuring
    // 2. Using fot(.) operator

    // via destructuring

    let (str, num) = tuple;
    println!("{}", str);

    // vai dot operator
    let value = tuple.1;
    println!("{}", value);

    // arrays: fixed size list
    let arr = [23, 34, 45];

    println!("printing array elements");
    let mut i = 0;
    while i < arr.len() {
        println!("{}", arr[i]);
        i += 1;
    }

    for j in 0..arr.len() {
        println!("{}", arr[j]);
    }

    // functions
    let sum = my_function(2,4);
    println!("sum of the function is: {}", sum);


    // terenary operation:
    const condition:bool = false;
    let finalValue:i32 = if my_function(-2,-4) > 0 {6} else {-6};
    println!("{}", finalValue);

    //loops

    let mut ptr = 0;
    let result =  loop {
        ptr += 1;

        if ptr == 10{
            break ptr;
        }
    };

    println!("ptr value after loop is {}", result);

}

fn my_function(x: i32, y: i32) -> i32 {
    println!("{}", x);
    println!("{}", y);
    x + y
}
