use bb84::bb84_states::{random_bit, BB84State, MeasurementBasis};

pub fn generate_bb84_state(bit: bool, basis: MeasurementBasis) -> BB84State {
    match (bit, basis) {
        (false, MeasurementBasis::Basis1) => BB84State::QubitZero,
        (true, MeasurementBasis::Basis1) => BB84State::QubitOne,
        (false, MeasurementBasis::Basis2) => BB84State::QubitMinus,
        (true, MeasurementBasis::Basis2) => BB84State::QubitPlus,
    }
}

pub fn measure_bb84_state(state: BB84State, basis: MeasurementBasis) -> bool {
    match (state, basis) {
        (BB84State::QubitZero, MeasurementBasis::Basis1) => false,
        (BB84State::QubitOne, MeasurementBasis::Basis1) => true,
        (BB84State::QubitMinus, MeasurementBasis::Basis2) => false,
        (BB84State::QubitPlus, MeasurementBasis::Basis2) => true,
        _ => rand::random(),
    }
}

pub fn simulate_eve_attack(state: BB84State) -> BB84State {
    let eve_basis = MeasurementBasis::random();
    let intercepted_bit = measure_bb84_state(state, eve_basis);
    generate_bb84_state(intercepted_bit, eve_basis)
}

fn main() {
    println!("--- BB84 Statistical Attack Analysis ---");

    let total_iterations = 1000;
    let mut matched_bases_count = 0;
    let mut error_count = 0;

    for _ in 0..total_iterations {
        // 1. Alice
        let alice_bit = random_bit();
        let alice_basis = MeasurementBasis::random();
        let alice_state = generate_bb84_state(alice_bit, alice_basis);

        // 2. Eve (Active Intercept-Resend)
        let state_after_eve = simulate_eve_attack(alice_state);

        // 3. Bob
        let bob_basis = MeasurementBasis::random();
        let bob_bit = measure_bb84_state(state_after_eve, bob_basis);

        // 4. Analysis (Sifting)
        if alice_basis == bob_basis {
            matched_bases_count += 1;
            if alice_bit != bob_bit {
                error_count += 1;
            }
        }
    }

    let qber = (error_count as f64 / matched_bases_count as f64) * 100.0;
    println!("Total Trials: {}", total_iterations);
    println!("Bits where Bases Matched: {}", matched_bases_count);
    println!("Detected Bit Errors: {}", error_count);
    println!("Calculated QBER: {:.2}%", qber);

    if qber > 20.0 {
        println!("CONCLUSION: Heavy Eavesdropping Detected. Key discarded.");
    } else {
        println!("CONCLUSION: Secure Channel. Proceeding to Privacy Amplification.");
    }
}
