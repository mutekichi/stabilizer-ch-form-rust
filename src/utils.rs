use num_complex::Complex64;

pub fn compare_complex(a: Complex64, b: Complex64, epsilon: f64) -> bool {
    (a.re - b.re).abs() < epsilon && (a.im - b.im).abs() < epsilon
}