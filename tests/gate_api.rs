use stabilizer_ch_form_rust::StabilizerCHForm;
use stabilizer_ch_form_rust::prelude::*;

#[test]
fn test_to_statevector() {
    let mut state = StabilizerCHForm::new(2);
    state.apply_h(0);
    state.apply_cz(0, 1);
    let statevector = state.to_statevector();
    let expected = vec![
        num_complex::Complex64::new(0.7071067811865475, 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(0.7071067811865475, 0.0),
    ];
    for (a, b) in statevector.iter().zip(expected.iter()) {
        assert!((a - b).norm() < 1e-10);
    }
}