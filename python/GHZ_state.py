from qiskit import QuantumCircuit
from qiskit.primitives import StatevectorSampler
import numpy as np

# 1. prepare quantum state |000> + i |111>

qc_1 = QuantumCircuit(3) # quantum circuit with 3 qubits
qc_1.h(0) # create an equal superposition using Hadamard gate
qc_1.p(np.pi/2, 0) # add a quantum phase
qc_1.cx(0, 1) # controlled not-gate on qubit 1
qc_1.cx(0, 2) # controlled not-gate on qubit 2

# 2. Measure all qubits

qc_measured = qc_1.measure_all(inplace=False)


# 3. Execute

sampler = StatevectorSampler()
job = sampler.run([qc_measured], shots=1000)
result = job.result()
print(f" > Counts: {result[0].data['meas'].get_counts()}")