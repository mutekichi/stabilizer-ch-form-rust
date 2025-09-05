use crate::api::representation::{CliffordCircuit, CliffordGate};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Parses an OpenQASM 2.0 string into a `CliffordCircuit`.
///
/// This is a simplified parser that supports `qreg` declarations and
/// a subset of Clifford gates (h, s, x, z, cx, cz).
/// It ignores comments, headers, and includes.
/// **Note:** `measure` operations are detected and ignored, with a warning printed to stderr.
///
/// # Arguments
/// * `qasm_str` - A string slice containing the OpenQASM 2.0 circuit description.
///
/// # Returns
/// A `Result` containing the parsed `CliffordCircuit` or a `String` error message.
pub fn from_qasm_str(qasm_str: &str) -> Result<CliffordCircuit, String> {
    // Lazy static regex for better performance if called multiple times
    lazy_static::lazy_static! {
        static ref QREG_RE: Regex = Regex::new(r"qreg\s+([a-zA-Z][a-zA-Z0-9_]*)\s*\[\s*(\d+)\s*\]\s*;").unwrap();
        static ref GATE1_RE: Regex = Regex::new(r"([a-z]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;").unwrap();
        static ref GATE2_RE: Regex = Regex::new(r"([a-z]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\],\s*([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;").unwrap();
    }

    let mut n_qubits: Option<usize> = None;
    let mut gates = Vec::new();

    for (line_num, line_content) in qasm_str.lines().enumerate() {
        let line = line_content.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some(caps) = QREG_RE.captures(line) {
            if n_qubits.is_some() {
                return Err("Multiple qreg declarations are not supported.".to_string());
            }
            let size = caps[2]
                .parse::<usize>()
                .map_err(|_| "Invalid qreg size".to_string())?;
            n_qubits = Some(size);
            continue;
        }

        if line.starts_with("OPENQASM") || line.starts_with("include") {
            continue;
        }

        // Explicitly detect and ignore 'measure' operations
        if line.starts_with("measure") {
            eprintln!(
                "[Warning] Line {}: `measure` operation is ignored by the parser.",
                line_num + 1
            );
            continue;
        }

        // 2-qubit gates must be checked first, as they are more specific
        if let Some(caps) = GATE2_RE.captures(line) {
            let gate_name = &caps[1];
            let control = caps[3]
                .parse::<usize>()
                .map_err(|_| format!("Invalid control qubit index in line: {}", line))?;
            let target = caps[5]
                .parse::<usize>()
                .map_err(|_| format!("Invalid target qubit index in line: {}", line))?;

            let gate = match gate_name {
                "cx" => CliffordGate::CX(control, target),
                "cz" => CliffordGate::CZ(control, target),
                _ => return Err(format!("Unsupported 2-qubit gate: {}", gate_name)),
            };
            gates.push(gate);
        } else if let Some(caps) = GATE1_RE.captures(line) {
            let gate_name = &caps[1];
            let qarg = caps[3]
                .parse::<usize>()
                .map_err(|_| format!("Invalid qubit index in line: {}", line))?;

            let gate = match gate_name {
                "h" => CliffordGate::H(qarg),
                "s" => CliffordGate::S(qarg),
                "x" => CliffordGate::X(qarg),
                "z" => CliffordGate::Z(qarg),
                _ => return Err(format!("Unsupported 1-qubit gate: {}", gate_name)),
            };
            gates.push(gate);
        } else {
            return Err(format!("Unrecognized or malformed line: {}", line));
        }
    }

    if let Some(n) = n_qubits {
        Ok(CliffordCircuit { n_qubits: n, gates })
    } else {
        Err("qreg declaration not found in QASM string.".to_string())
    }
}

pub fn from_qasm_file<P: AsRef<Path>>(path: P) -> Result<CliffordCircuit, String> {
    let qasm_content = fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Failed to read file '{}': {}", path.as_ref().display(), e))?;

    from_qasm_str(&qasm_content)
}
