#[macro_use(array, s)]
extern crate ndarray;
extern crate rand;
extern crate ndarray_linalg;

//use ndarray::{Array, Axis, ArrayBase, stack};
use ndarray::prelude::*;
use ndarray::stack;
//use ndarray_linalg::Solve;
use rand::{thread_rng, Rng};
use rand::distributions::{Standard};

use ndarray_linalg::Solve;

fn main() {

    // data creation 
    let rng = thread_rng();
    let mut x0: Vec<f32> = rng.sample_iter(&Standard).take(100).collect();
    let mut x1: Vec<f32> = rng.sample_iter(&Standard).take(100).collect();
    let mut x2: Vec<f32> = rng.sample_iter(&Standard).take(100).collect();
    let y0: Vec<f32> = rng.sample_iter(&Standard).take(100).collect();
    x0.append(&mut x1);
    x0.append(&mut x2);
    let x = Array::from_shape_vec((100, 3), x0).unwrap();
    let y = Array::from_vec(y0);
    
    // Train and Test data 
    let x_train = x.slice(s![0..80, ..]).to_owned();
    let y_train = y.slice(s![0..80]).to_owned();
    let x_test = x.slice(s![80..100, ..]).to_owned();
    let y_test = y.slice(s![80..100]).to_owned();
    
    // Train model
    let mut model = LinearRegression::new();
    model.fit(&x_train, &y_train);
    let prediction = model.predict(&x_test);
    let w = model.w();
    println!("The predictions are {:#?}", prediction);
    println!("The parameters are {:#?}", w);
}

// Object Oriented Program
pub struct LinearRegression {
    w: Array<f32, Ix1>,
}

impl LinearRegression { 

    pub fn new() -> LinearRegression {
        LinearRegression {
            w: Array::from_vec(vec![0.]),
        }
    }

    pub fn w(&self) -> &Array<f32, Ix1> {&self.w}
    
    pub fn fit(&mut self, x: &Array<f32, Ix2>, y: &Array<f32, Ix1>) {
        let x_len = x.len_of(Axis(0));
        let ones_arr = Array::from_elem((x_len, 1), 1.);
        // Insert column of all 1 to position of first column
        let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
        let a = xtil.t().dot(&xtil);
        let b = xtil.t().dot(y);
        let w = a.solve_into(b).unwrap();
        self.w =  w.clone();
    }

    pub fn predict(&self, x: &Array<f32, Ix2>) -> Array<f32, Ix1> {
        let x_len = x.len_of(Axis(0));
        let ones_arr = Array::from_elem((x_len, 1), 1.);
        let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
        let prediction = xtil.dot(&self.w);
        prediction
    }
}

