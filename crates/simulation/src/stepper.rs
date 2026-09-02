use dynamics::models::state_space_model::StateSpace;
use onboard_software::control_stack::ControlStack;
use onboard_software::config::SensorVector;
use crate::config::{Plant, ControlVector, StateVector, NX, NU, NY};


pub fn sim_dynamics(t: f64, plant: &StateSpace<Plant, NX, NU, NY>, u_prev: ControlVector, x_prev: StateVector, dx_prev: StateVector) -> StateVector{ // need to add initial conditions, 
    // replace with rk4 eventually
    plant.derivative(t, x_prev, u_prev, dx_prev)
}

pub fn sensor_dynamics(x: StateVector) -> SensorVector{
    SensorVector::zeros()
}

pub fn sim_stepper(x_0: StateVector, u_0: ControlVector, t: f64, t_span: [f64; 2], dt: f64, mut gnc: ControlStack){

    let plant = StateSpace::new(Plant);
    let mut t = t_span[0];
    let mut x = x_0;
    let sensor_0 = sensor_dynamics(x);
    gnc.estimator.initialize(sensor_0);
    while t < t_span[1]{
        // initial state and initial sensor data should be passed in
        

        let sensor_n = sensor_dynamics(x);
        let onboard_state_n = gnc.estimator.estimate(sensor_n);// sensor_state_minus_1
        let onboard_controls_n = gnc.controller.control(); //onboard_state_n


        t += dt;
    }

}

