extern crate ndarray;
extern crate rand;

use ndarray::{Array1, ArrayView1};
use rand::distributions::{Distribution, Normal};

fn main() {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0);
    for _ in 0..50 {
        let v = normal.sample(&mut rng);
        
    }
    
}
