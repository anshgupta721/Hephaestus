use numpy::{IntoPyArray, PyArray1, PyArray2, PyReadonlyArray1};
use numpy::ndarray::Array2;
use pyo3::prelude::*;

use onboard_software::config::{NU, ControlVector};
use onboard_software::control_stack::ControlStack;
use simulation::config::{NX, StateVector};
use simulation::stepper::sim_stepper;

// struct TrialResult {
//     t: Vec<f64>,
//     x: Vec<StateVector>,
//     u: Vec<ControlVector>,
// }


#[pyfunction]
#[pyo3(signature = (x_0, u_0, t_span, dt, seed=None))]
pub fn pysim_runner<'py>(
    py: Python<'py>,
    x_0: PyReadonlyArray1<'py, f64>,
    u_0: PyReadonlyArray1<'py, f64>,
    t_span: [f64; 2],
    dt: f64,
    seed: Option<u64>,
) -> PyResult<(Py<PyArray1<f64>>, Py<PyArray2<f64>>, Py<PyArray2<f64>>)>{

    // TODO: add a check eventually to make sure x0.len == NX and u0.len == NU
    let x_0 = StateVector::from_row_slice(x_0.as_slice()?);
    let u_0 = ControlVector::from_row_slice(u_0.as_slice()?);
    // Instantiate the control stack
    let gnc: ControlStack = ControlStack::new(/*Configure the estimator here in future*/);
    // run the stepper
    let (t_hist, x_hist, u_hist) = sim_stepper(x_0, u_0, t_span, dt, gnc);
    let t_arr = t_hist.into_pyarray(py).into();
    // Take t_hist vector and vectors of nalgebra arrays (x_hist and u_hist) and convert them to numpy arrays
    let n = x_hist.len();
    let x_flat: Vec<f64> = x_hist.iter().flat_map(|x| x.as_slice().iter().copied()).collect();
    let x_arr = Array2::from_shape_vec((n, NX), x_flat).unwrap().into_pyarray(py).into();
    
    let m = u_hist.len();
    let u_flat: Vec<f64> = u_hist.iter().flat_map(|x| x.as_slice().iter().copied()).collect();
    let u_arr = Array2::from_shape_vec((m, NU), u_flat).unwrap().into_pyarray(py).into();
    


    Ok((t_arr, x_arr, u_arr))
}

// #[pyfunction]
// #[pyo3(signature = (x_0, u_0, t_span, dt, n_runs, x0_std, process_noise_std, seed=None))]
// fn monte_carlo_sim(
//     py: Python<'py>,
//     x_0: [f64; NX],
//     u_0: [f64]
// )
