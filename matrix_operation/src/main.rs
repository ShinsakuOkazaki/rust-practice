fn main() {
    //let a = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    //println!("{:?}", a[1][1]);
    let a = vec![vec![0.0, 1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0, 7.0, 8.0]];
    let b = vec![vec![0.0, 1.0, 2.0], vec![3.0, 4.0, 5.0], vec![6.0, 7.0, 8.0]];
    
    let c = add(&a, &b);
    println!("{:?}", &c);

    
}

fn add(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let a_m = a[0].len();
    let a_n = a.len();

    let b_m = b[0].len();
    let b_n = b.len();

    if a_m != b_m || a_n != b_n {
        panic!("Dimentions are not matched!!")
    }

    let mut c = vec![vec![0.0; a_m]; b_n];
    for i in 0..a_m {
        for j in 0..b_n {
            c[i][j] = &a[i][j] + &b[i][j]
        }
    }
    return c;
}

fn mul

// fn add(a: &[[f32]], b: &[[f32]]) {
//     let a_m: i32 = a[0].len();
//     let a_n: i32 = a.len();

//     let  b_m: i32 = b[0].len();
//     let b_n: i32 = b.len();

//     if a_m != b_m || a_n != b_n {
//         panic!("Dimentions are not matched!!")
//     }

//     let mut c = [[0.0; a_m]; b_n];
//     for i in 0..a_m {
//         for j in 0..b_n {
//             c[i][j] = a[i][j] + b[i][j];
//         }
//     }

// }
 
