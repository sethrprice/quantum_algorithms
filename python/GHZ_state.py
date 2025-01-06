from qiskit import QuantumCircuit
import numpy as np

# prepare quantum state |000> + i |111>

qc_1 = QuantumCircuit(3) # quantum circuit with 3 qubits
qc_1.h(0) # create an equal superposition using Hadamard gate
qc_1.p(np.pi/2, 0) # add a quantum phase
qc_1.cx(0, 1) # controlled not-gate on qubit 1
qc_1.cx(0, 2) # controlled not-gate on qubit 2