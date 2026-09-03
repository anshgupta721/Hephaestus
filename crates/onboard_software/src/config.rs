use nalgebra::SVector;

pub const ESTIMATOR_STATES: usize = 3;
pub const NU: usize = 3;

pub type ControlVector = SVector<f64, NU>;
pub type EstimatorVector = SVector<f64, ESTIMATOR_STATES>;
pub type SensorVector = SVector<f64, 3>;
