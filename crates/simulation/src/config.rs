/// This file contains the simulation configuration for the plant dynamics configuration
/// NOT anything that would be done for Monte Carlo simulatino
/// i.e. state space models

use nalgebra::{SVector, SMatrix};
use dynamics::models::state_space_model::{LTVSystem, StateSpace};

pub const NX: usize = 3;
pub const NU: usize = 3;
pub const NY: usize = 3;

pub struct Plant;

impl LTVSystem<NX, NU, NY> for Plant{
    fn a(&self, t: f64) -> SMatrix<f64, NX, NX> {
        let k = 1.0 + 0.1 * t.sin(); // time-varying stiffness
        SMatrix::<f64, NX, NX>::new(
            0.0, 1.0, 0.0,
            -k,  -0.2, 0.0,
            0.0, 0.0, 0.0,
        )
    }
    fn b(&self, _t: f64) -> SMatrix<f64, NX, NU> { 
        SMatrix::<f64, NX, NU>::new(
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
        ) 
    }
    fn c(&self, _t: f64) -> SMatrix<f64, NY, NX> { 
        SMatrix::<f64, NY, NX>::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        )
    }
    fn d(&self, _t: f64) -> SMatrix<f64, NY, NU> { SMatrix::<f64, NY, NU>::zeros() }
}



pub type ControlVector = SVector<f64, NU>;
pub type StateVector = SVector<f64, NX>;