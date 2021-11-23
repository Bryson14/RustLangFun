extern crate rayon;
use rayon::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn sqrts(floats: Vec<f64>) -> Vec<f64> {
    floats.par_iter().map(|&f| expensive_operation(f)).collect()
}

fn sqrts_seq(floats: Vec<f64>) -> Vec<f64> {
    floats.iter().map(|&f| expensive_operation(f)).collect()
}

fn expensive_operation(time: f64) -> f64 {
    sleep(Duration::from_millis(time as u64));
    (time * time).into()
}

fn main() {
    let v: Vec<f64> = (0..100).map(f64::from).collect();
    let v2 = v.clone();

    let now = Instant::now();
    sqrts(v);
    let parallel = now.elapsed().as_millis();

    let now = Instant::now();
    sqrts_seq(v2);
    let seq = now.elapsed().as_millis();
    println!("parallel {}, seqential {}", parallel, seq);
}
