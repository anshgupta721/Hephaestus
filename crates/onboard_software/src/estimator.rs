use nalgebra::{SVector, SMatrix};
use crate::config::{ESTIMATOR_STATES, SensorVector};


pub struct Estimator{
    x: SVector<f64, {ESTIMATOR_STATES}>,
    p: SMatrix<f64, {ESTIMATOR_STATES}, {ESTIMATOR_STATES}>,
}

impl Estimator{
    pub fn initialize(&self, sensor_data: SensorVector /* Fill in with sensor parameters*/ ){
        
    }

    pub fn estimate(&self, sensor_data: SensorVector /* Fill in with sensors*/ ){
        
    }
}