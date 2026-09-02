use crate::estimator::Estimator;
use crate::controller::Controller;

pub struct ControlStack {
    pub estimator: Estimator,
    pub controller: Controller,
}