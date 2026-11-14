# Hephaestus

A Rust simulation and GNC (guidance, navigation and control) testbed for spacecraft in lunar orbit, with Python bindings for running trials and plotting results.

The plant propagates a spacecraft under a degree-8 lunar spherical-harmonic gravity field, using JPL ephemerides for the Moon's orientation. An onboard-software crate holds the estimator and controller that will run in the loop. The Rust core does the heavy lifting; a thin PyO3 module exposes it to Python. This repository is used to prove out a lot of my math for the GRAIL NESC Lunar Checkcase.

> **Status:** early development. The plant dynamics and simulation loop work end to end. The estimator, controller and navigation model are placeholders and don't do anything yet (see [Project status](#project-status)).

## Features

- Orbit propagation in the Moon-centered inertial frame (`MOON_J2000`), with gravity evaluated in the Moon body-fixed principal-axes frame (`MOON_PA_DE440`)
- Degree/order 8 spherical-harmonic gravity, returning both acceleration and the gravity-gradient tensor
- Fixed-step RK4 integration
- Frame rotations from JPL DE440 ephemeris and lunar orientation kernels via [ANISE](https://crates.io/crates/anise)
- Epoch handling with [hifitime](https://crates.io/crates/hifitime)
- Python entry point (`pysim_runner`) returning NumPy arrays, plus an example script that plots the orbit and writes a CSV

## Repository layout

```
Hephaestus/
├── Cargo.toml                    # Workspace manifest and shared dependencies
├── crates/
│   ├── simulation/               # Plant dynamics and the simulation stepper
│   │   ├── src/config.rs         #   Constants, gravity coefficients, Plant::derivative
│   │   ├── src/stepper.rs        #   sim_stepper: the simulation loop
│   │   └── data/                 #   DE440s ephemeris, lunar orientation and constants kernels
│   ├── onboard_software/         # Flight-software side of the loop
│   │   └── src/                  #   estimator, controller, control_stack, config
│   └── pyfrontend/               # PyO3 bindings, built with maturin
│       └── src/python_bindings.rs
├── pysim/
│   └── run_sim.py                # Example: run a trial, save plots and CSV
└── .github/workflows/            # Rust build/test, clippy, pylint
```

## Prerequisites

- **Rust** with edition 2024 support (Rust 1.85 or newer) and Cargo
- **Python** 3.9 or newer
- **[maturin](https://www.maturin.rs/)** to build the Python extension (`pip install maturin`)
- Network access to GitHub on the first build: some dependencies (`math`, `dynamics`, `sensors`) are git dependencies from the author's [Azimuth](https://github.com/anshgupta721/Azimuth) repository. `dynamics` is pinned to the `feature/hessian_shm` branch.

## Getting started

Clone the repo and build the Rust workspace:

```bash
git clone https://github.com/anshgupta721/Hephaestus.git
cd Hephaestus
cargo build
cargo test
```

Build and install the Python module into a virtual environment:

```bash
python -m venv .venv
source .venv/bin/activate        # Windows: .venv\Scripts\activate
pip install maturin numpy matplotlib

cd crates/pyfrontend
maturin develop --release
cd ../..
```

Run the example simulation from the repository root:

```bash
python pysim/run_sim.py
```

This propagates an 8-hour trajectory at a 1 s step, saves three figures to `plots/` (state history, gravity-gradient trace, 3D orbit), opens the plot windows, and writes `sim_output.csv`.

## Usage

```python
import numpy as np
from pyfrontend import pysim_runner

epoch_0 = "2025-07-15T11:23:27.30 UTC"

# [x, y, z, vx, vy, vz, <9 gravity-gradient entries>]  (km, km/s)
x_0 = np.array([
    -194.33826150101773, 824.8947002999065, 1653.703391999927,
    0.08094043359034313, -1.4478938749999684, 0.731723312100025,
    *np.zeros(9),
])

t, states, estimates = pysim_runner(epoch_0, x_0, (0.0, 28800.0), 1.0)
```

### `pysim_runner(epoch_0, x_0, t_span, dt, seed=None)`

| Argument  | Description |
|-----------|-------------|
| `epoch_0` | Start epoch as a string parsed by hifitime, e.g. `"2025-07-15T11:23:27.30 UTC"` |
| `x_0`     | Initial state, a NumPy array of exactly 15 floats (see below) |
| `t_span`  | `(t_start, t_end)` in seconds, relative to `epoch_0` |
| `dt`      | Fixed integration step in seconds |
| `seed`    | Reserved for future Monte Carlo runs; currently unused |

**Returns** `(t, states, estimates)`:

- `t`: 1D array of time stamps (s)
- `states`: array of shape `(N, 15)`, the plant state history
- `estimates`: array of shape `(N, 3)`, the estimator output history (currently always zeros)

An invalid epoch string or an `x_0` of the wrong length raises `ValueError`.

### State vector

The plant state has 15 elements, in kilometers and seconds:

| Indices | Contents |
|---------|----------|
| 0–2     | Position `x, y, z` in the Moon-centered inertial frame (km) |
| 3–5     | Velocity `vx, vy, vz` (km/s) |
| 6–14    | Nine entries (row-major 3×3) whose time derivative is set to the gravity-gradient tensor at the current position |

The example initial state is an approximately circular low lunar orbit.

## How it works

Each step of `sim_stepper` runs the following loop:

1. Advance the plant one step with RK4. For each derivative evaluation, rotate the position into the Moon body-fixed frame at the current epoch, evaluate spherical-harmonic gravity there, and rotate the acceleration back to the inertial frame.
2. Form the sensor measurement (currently the true position and velocity, with no noise).
3. Run the estimator's `predict` and `update`.
4. Record time, true state and estimate.

The controller call is stubbed out in the loop, so the simulation is currently open loop.

## Project status

Working:

- Lunar gravity plant with gradient output, frame handling and RK4 propagation
- Python bindings and the example plotting script
- CI for Rust build/test, clippy and pylint

In progress / placeholder:

- `Estimator` (Kalman-style predict/update) holds state and covariance but performs no filtering
- `Controller` returns a zero control vector, and the control input isn't yet applied to the plant
- The `navigation` model in `onboard_software/src/config.rs` returns zero matrices
- Monte Carlo support (`monte_carlo_sim`) is sketched in comments only, and `seed` is not yet used
- Sensor models: the sensor function is a pass-through of the true state

## Data files

`crates/simulation/data/` ships the kernels the simulation loads at runtime:

| File | Purpose |
|------|---------|
| `de440s.bsp` | JPL DE440 (short-span) planetary ephemeris |
| `moon_pa_de440_200625.bpc` | High-precision Moon principal-axes orientation |
| `pck11.pca` | Planetary constants kernel |

Paths are resolved at compile time relative to the `simulation` crate, so build from within the workspace.

## Continuous integration

GitHub Actions workflows in `.github/workflows/`:

- **Rust**: `cargo build` and `cargo test` on pushes and pull requests to `main`
- **rust-clippy analyze**: clippy lints uploaded as code-scanning results
- **Pylint**: lints the Python sources on every push
