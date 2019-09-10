fn main() {
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str takes string slice
    // and do not take ownership
    let mut s2 = String::from("foo");
    let s3 = "bar";
    s2.push_str(s3);
    // if the ownership has been taken, will get error.
    println!("s3 is still {}", s3);

    //push method takes a single character
    let mut s4 = String::from("lo");
    s4.push('l');
    println!("s4 is {}", s4);

    //concatenation + and format!
    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6; // s1 has been moved and can no longer be used
    println!("s7 is {}", s7);

    //format!
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");

    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("{}", s11);

    // indexing into String

    // throw error, String does not support int index slicing
    //let s12 = String::from("hello");
    //let h = s12[0];


    // String len method counts length of bytes
    // and String is stored encoded in UTF-8
    let len = String::from("Hola").len();
    println!("Length of Hola: {}", len);
    //expecting 12 
    let len2 = String::from("Здравствуйте").len();
    println!("Length of Здравствуйте: {}", len2);

    // throw error becaue String is stored in byte
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // println!("First char of Здравствуйте: {}", answer);

    // Rust does not allow indexing to String,
    // because String indexing does not garantee O(1) run time.
    
    // Slicing Strings
    // we need specify byte starting and ending point to slice
    let hello = "Здравствуйте";
    let s = &hello[0..4]; //first 4 bytes which is '3д' here.
    println!("First 4 bytes: {}", s);

    // way to interacte individual Unicide
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}
