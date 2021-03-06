// http://rosettacode.org/wiki/Roots_of_unity
#![feature(core)]

extern crate num;
use num::complex::{Complex, Complex32};
use std::f32::consts;

#[cfg(not(test))]
fn main() {
    let degree = 3us;

    for root in &roots_of_unity(degree) {
        println!("{}", root);
    }
}


fn roots_of_unity(degree: usize) -> Vec<Complex32> {
    (0..degree).map(|el|
        Complex::<f32>::from_polar(&1f32, &(2f32 * consts::PI * (el as f32) / (degree as f32))))
        .collect::<Vec<Complex32>>()
}

#[test]
fn test_result() {
    let expected = vec![ Complex::new(1f32, 0.),
        Complex::new(-0.5, 0.866025),
        Complex::new(-0.5, -0.866025)
    ];

    for (root, &exp) in roots_of_unity(3us).iter().zip(expected.iter()) {
        assert!((*root - exp).norm() < 1e-6);
    }
}
