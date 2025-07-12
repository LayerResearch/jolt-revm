use revm_guest as guest;
use spinners::{Spinner, Spinners};

macro_rules! step {
    ($msg:expr, $action:expr) => {{
        let mut sp = Spinner::new(Spinners::Dots9, $msg.to_string());
        let result = $action;
        sp.stop_with_message(format!("✓ {}", $msg));
        result
    }};
}

pub fn fib() {
    let target_dir = "/tmp/jolt-guest-targets";

    let program = step!("Compiling guest code", { guest::compile_fib(target_dir) });

    let prover_preprocessing = step!("Preprocessing prover", {
        guest::preprocess_prover_fib(&program)
    });

    let verifier_preprocessing = step!("Preprocessing verifier", {
        guest::preprocess_verifier_fib(&program)
    });

    let prove_fib = step!("Building prover", {
        guest::build_prover_fib(program, prover_preprocessing)
    });

    let verify_fib = step!("Building verifier", {
        guest::build_verifier_fib(verifier_preprocessing)
    });

    let (output, proof) = step!("Proving", { prove_fib(50) });
    assert!(output >= 1);

    let is_valid = step!("Verifying", { verify_fib(50, output, proof) });
    assert!(is_valid);
}

pub fn exec() {
    let target_dir = "/tmp/jolt-guest-targets";

    let program = step!("Compiling guest code", { guest::compile_exec(target_dir) });

    let prover_preprocessing = step!("Preprocessing prover", {
        guest::preprocess_prover_exec(&program)
    });

    let verifier_preprocessing = step!("Preprocessing verifier", {
        guest::preprocess_verifier_exec(&program)
    });

    let prove_exec = step!("Building prover", {
        guest::build_prover_exec(program, prover_preprocessing)
    });

    let verify_exec = step!("Building verifier", {
        guest::build_verifier_exec(verifier_preprocessing)
    });

    let (output, proof) = step!("Proving", { prove_exec(50) });
    assert!(output >= 1);

    let is_valid = step!("Verifying", { verify_exec(50, output, proof) });
    assert!(is_valid);
}

fn main() {
    println!("Fibonacci");
    fib();
    println!("Transaction Execution");
    exec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        exec();
    }
}
