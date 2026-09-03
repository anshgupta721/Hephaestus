"""Run the simulation entry point."""
import numpy as np
import matplotlib.pyplot as plt
from pyfrontend import pysim_runner  # pylint: disable=no-name-in-module

if __name__ == "__main__":
    x_0 = np.zeros(3, dtype=np.float64)
    u_0 = np.zeros(3, dtype=np.float64)
    t, states, _ = pysim_runner(x_0, u_0, (0.0, 10.0), 0.01)
    plt.plot(t, states[:, 0])
    plt.show()
