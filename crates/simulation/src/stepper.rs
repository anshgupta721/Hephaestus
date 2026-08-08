



pub fn sim_step(dt: float){ // need to add initial conditions, 

    let plant_state_n_minus_1 = Dynamics.sense(dt);
    let plant_state_n = Dynamics.step(dt, plant_state_n_minus_1);

    let onboard_state_n_minus_1 = Onboard_Software.sense();
    let onboard_state_n = Onboard_Software.estimate();
    let onboard_controls_n = Onboard_Software.control();
}
