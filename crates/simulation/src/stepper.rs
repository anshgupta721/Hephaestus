use crate::config::{NX, NY, Plant, StateVector};
use dynamics::integrators::rk4::rk4;
use dynamics::models::state_space_model::StateSpace;
use onboard_software::config::NU;
use onboard_software::config::{ControlVector, EstimatorVector, SensorVector};
use onboard_software::control_stack::ControlStack;

pub fn sensor_dynamics(x: StateVector) -> SensorVector {
    x
}

pub fn sim_stepper(
    x_0: StateVector,
    u_0: ControlVector,
    t_span: [f64; 2],
    dt: f64,
    mut gnc: ControlStack,
) -> (Vec<f64>, Vec<StateVector>, Vec<ControlVector>) {
    let plant = StateSpace::new(Plant);
    let mut t = t_span[0];
    let mut x = x_0;
    let mut u = u_0;
    let sensor_0 = sensor_dynamics(x);
    let dx0 = plant.derivative(t_span[0], x_0, u_0, StateVector::zeros());
    gnc.estimator.initialize(sensor_0);

    let n_steps: usize = ((t_span[1] - t_span[0]) / dt).ceil() as usize + 1;
    let mut t_hist = Vec::with_capacity(n_steps);
    let mut x_hist = Vec::with_capacity(n_steps);
    let mut u_hist = Vec::with_capacity(n_steps);

    while t < t_span[1] {
        // Simulation loop!
        // Dynamics update -> Sensor update -> estimator update -> controller update ->
        x = rk4(
            |t: f64, x: StateVector, u: ControlVector, dx0: StateVector| {
                plant.derivative(t, x, u, dx0)
            },
            t,
            dt,
            x,
            u,
            dx0,
        );

        let sensor_n = sensor_dynamics(x);
        let estimator_n = gnc.estimator.estimate(sensor_n);
        u = gnc.controller.control(estimator_n);

        t += dt;
        t_hist.push(t);
        x_hist.push(x);
        u_hist.push(u);
    }
    (t_hist, x_hist, u_hist)
}
