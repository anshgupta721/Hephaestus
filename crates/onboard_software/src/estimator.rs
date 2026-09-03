use crate::config::{ESTIMATOR_STATES, EstimatorVector, SensorVector};
use nalgebra::{SMatrix, SVector};

pub struct Estimator {
    x: EstimatorVector,
    p: SMatrix<f64, ESTIMATOR_STATES, ESTIMATOR_STATES>,
}

impl Estimator {
    pub fn new() -> Estimator {
        Estimator {
            x: EstimatorVector::zeros(),
            p: SMatrix::<f64, ESTIMATOR_STATES, ESTIMATOR_STATES>::zeros(),
        }
    }

    pub fn initialize(&self, sensor_data: SensorVector /* Fill in with sensor parameters*/) {}

    pub fn estimate(
        &self,
        sensor_data: SensorVector, /* Fill in with sensors*/
    ) -> EstimatorVector {
        self.x
    }
}
