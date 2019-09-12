// fn largest<T>(list:&[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest        
// }

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    // This throws error
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest($number_list);
    // print!("The largest number is {}", result);

    // let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest(&char_list);
    // print!("The largest char is {}", result);

    let integer_and_float = Point{ x: 5, y: 4.0};
    print!("point of intefer and float: {:#?}", integer_and_float);
}
