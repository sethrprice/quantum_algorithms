use quantum::complex::Complex;
use quantum::computer::QuantumComputer;
use quantum::gate::Gate;
use quantum::gates;
use quantum::m_real;
use quantum::matrix::Matrix;

// use algorithms::deutsch;
// use computer::QuantumComputer;
// use gates;

fn main() {
    // identity
    let mut quantum_computer = QuantumComputer::new(3);
    quantum_computer.initialize(5);
    quantum_computer.apply(gates::identity(3));
    quantum_computer.collapse();
    assert_eq!(5, quantum_computer.value());

    // coin flip with hadamard gate
    let mut c2 = QuantumComputer::new(1);
    c2.initialize(0);
    c2.apply(gates::hadamard(1));
    c2.collapse();
    let result = if c2.value() == 1 { "heads" } else { "tails" };
    println!("coin flipped: {}!", result);

    // Deutsch algorithm
    let const_fn = |_x: i32| 1;
    let bal_fn = |x: i32| x;
    let c3 = QuantumComputer::new(2);
    let c3_clone = QuantumComputer::new(2);
    let const_result = deutsch_algorithm(c3, const_fn);
    let bal_result = deutsch_algorithm(c3_clone, bal_fn);
    println!(
        "constant result: {}\nbalanced result: {}",
        const_result, bal_result
    );
}

macro_rules! m_one {
    ( $item:tt ) => {
        1
    };
}

macro_rules! m_rec(
  ( [ $($row:tt),* ] [$($i:expr),*]) => ({
     let _rows = 0 $(+ m_one!($row) )*;
     let _cols = (0 $(+ m_one!($i))*) / _rows;

     assert_eq!(_rows, _cols);

     Matrix::new_from_elements(_rows, vec![$($i),*])
  })
);

macro_rules! m_real {
  ( $( $( $i:expr ),* );* ) => ( m_rec!([$([$(Complex::new($i as f64, 0f64)),*]),*]
                                        [$($(Complex::new($i as f64, 0f64)),*),*]) )
}

// Deutsch Algorithm

fn deutsch_algorithm(mut computer: QuantumComputer, f: fn(i32) -> i32) -> String {
    // initialise computer to |01>
    computer.initialize(1);

    // Apply Hadamard gate to both qubits
    computer.apply(gates::hadamard(2));

    // build Deutsch gate
    let mut matrix = Matrix::identity(4);
    let exchange = m_real![0, 1;
                           1, 0];
    if f(0) == 1 {
        matrix.embed(&exchange, 0, 0);
    }

    if f(1) == 1 {
        matrix.embed(&exchange, 2, 2);
    }
    let deutsch_gate = Gate::new(2, matrix);

    // apply deutsch gate
    // |a, b > -> |a, b + f(a) >
    computer.apply(deutsch_gate);

    // apply Hadamard again
    computer.apply(gates::hadamard(2));

    // measure first qubit
    computer.collapse();

    if computer.value() == 1 {
        return "constant".into();
    } else {
        return "balanced".into();
    }
}

// Deutsch-Jozsa Algorithm

// Grover’s Search Algorithm

// Quantum Teleportation

// Bell State Preparation
