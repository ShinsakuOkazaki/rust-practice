fn main() {
    // create vector
    let v1: Vec<i32> = Vec::new();
    // initiate vector with elements
    let v2 = vec![1, 2, 3, 4];

    //Updateing a Vector
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("v3 elements: {:?}", v3);

    let v = vec![1, 2, 3, 4, 5];

    // get reference of index 2 of v to third.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // .get takes Option<&T>.
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // type of error of accessing vector.
    let v = vec![1, 2, 3, 4, 5];
    // this will crash program, because it references nonexistent element.
    //let does_not_exist = &v[100];

    // This returns None.
    let does_not_exist = v.get(100);

    // accessing elements of vector with immutable references of each element of vector. 
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // changeing elements of vector with mutable references of each elements of vector.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }


    // when we want to have diffenrent type within a vector, use enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10, 12),
    ];


}
