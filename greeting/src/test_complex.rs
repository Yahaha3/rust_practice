#[cfg(test)]
use crate::Complex;
use num::Complex;

pub fn test_complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
 
    println!("{} + {}i", result.re, result.im)
}