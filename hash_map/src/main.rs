use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);
    println!("score: {:?}", score);    

    // other way to construct hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![20, 60];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let score = scores.get(&team_name);
    println!("score: {:?}", score);  

    //Copy trait,like i32, the values are copied into the hash map. 
    //For owned values like String, the values will be moved and the hash map will be owner of 
    //those values.


    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        // or_insert returns mutable reference (&mut V) to value of for this key
        // we store it to immutable reference (count). To access mutable reference of value
        // dereference with *count
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
