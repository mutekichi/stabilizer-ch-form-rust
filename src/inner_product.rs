use super::StabilizerCHForm;
use crate::amplitude::Amplitude;

impl StabilizerCHForm {
    /// Computes the amplitude <0...0|φ> for the stabilizer state φ.
    /// 
    /// NOTE: The amplitude includes the phase factor, but not ω.
    /// See around eq.(55) of arXiv:1808.00128 for details.
    pub fn amplitude_at_zero(&self) -> Amplitude {
        for j in 0..self.n {
            if !self.vec_v[j] && self.vec_s[j] {
                return Amplitude::Zero;
            }
        }
        let weight_v = self.vec_v.iter().filter(|&&x| x).count();

        Amplitude::NonZero {
            phase: self.phase_factor,
            r: weight_v,
        }
    }

    /// Computes the amplitude <s|φ> for the stabilizer state φ and bitstring state s.
    /// 
    /// NOTE: This implementation might be inefficient.
    pub fn amplitude_at_computational_basis(&self, s: &ndarray::Array1<bool>) -> Amplitude {
        if s.len() != self.n {
            panic!("Input bitstring length does not match number of qubits.");
        }

        let mut ch_form_clone= self.clone();
        for (i, &bit) in s.iter().enumerate() {
            if bit {
                ch_form_clone.apply_x(i);
            }
        }

        ch_form_clone.amplitude_at_zero()
    }
}