use ndarray::{Array1, Array2};
use num_complex::Complex64;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn assert_eq_complex_array1(a: &Array1<Complex64>, b: &Array1<Complex64>) {
    assert_eq!(a.len(), b.len(), "Arrays have different lengths.");
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (x - y).norm();
        assert!(
            diff <= 1e-8,
            "Arrays differ at index {}: |{} - {}| = {} > {}",
            i,
            x,
            y,
            diff,
            1e-8
        );
    }
}

#[allow(dead_code)]
pub fn load_statevector_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Array1<Complex64>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut vec_data = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        let parts: Vec<&str> = line_content.split(',').collect();
        if parts.len() == 2 {
            let real = parts[0]
                .trim()
                .parse::<f64>()
                .expect("Failed to parse real part");
            let imag = parts[1]
                .trim()
                .parse::<f64>()
                .expect("Failed to parse imag part");
            vec_data.push(Complex64::new(real, imag));
        }
    }
    Ok(Array1::from(vec_data))
}

/// Prints a boolean vector (Array1<bool>) to the console in a readable format (e.g., [0, 1, 0]).
#[allow(dead_code)]
pub fn pretty_print_vec(name: &str, vec: &Array1<bool>) {
    let s: String = vec
        .iter()
        .map(|&b| if b { '1' } else { '0' })
        .collect::<String>();
    println!("{}: [{}]", name, s);
}

/// Prints a boolean matrix (Array2<bool>) to the console in a readable format.
#[allow(dead_code)] // This is a debug utility, so allow it to be unused in some tests
pub fn pretty_print_mat(name: &str, mat: &Array2<bool>) {
    println!("{}: [", name);
    for row in mat.rows() {
        let s: String = row.iter().map(|&b| if b { '1' } else { '0' }).collect();
        println!("  {}", s);
    }
    println!("]");
}
