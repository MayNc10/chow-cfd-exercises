pub fn reynolds(velocity: f64, diameter: f64, k_viscosity: f64) -> f64 {
    velocity * diameter / k_viscosity
}

pub fn sphere_cd(reynolds: f64) -> f64 {
    if reynolds == 0.0 {
        0.0
    } else if reynolds <= 1.0 {
        24.0 / reynolds // stokes formula
    } else if reynolds <= 400.0 {
        25.0 / (reynolds.powf(0.646))
    } else if reynolds <= 3E5 {
        0.5
    } else if reynolds <= 2E6 {
        0.000366 * reynolds.powf(0.4275)
    } else {
        0.18
    }
}

pub fn dvdt(velocity: f64, diameter: f64, k_viscosity: f64, sphere_density: f64, 
    fluid_density: f64, gravity: f64) -> f64 
{
    let reynolds = reynolds(velocity, diameter, k_viscosity);
    let cd = sphere_cd(reynolds);
    let rho_bar = fluid_density / sphere_density;
    let a = 1.0 + 0.5 * rho_bar;
    let b = (1.0 - rho_bar) * gravity;
    let c = (0.75 * rho_bar) / diameter;
    (1.0 / a) * (b - c * cd * velocity.abs() * velocity)
}

pub fn free_fall_rk4(
    z: f64,
    velocity: f64,
    diameter: f64,
    k_viscosity: f64,
    sphere_density: f64,
    fluid_density: f64,
    gravity: f64,
    h_step: f64,
) -> (f64, f64) {
    let d1z = h_step * velocity;
    let d1v = h_step * dvdt(velocity, diameter, k_viscosity, sphere_density, fluid_density, gravity);
    let d2z = h_step * (velocity + 0.5 * d1v);
    let d2v = h_step * dvdt(velocity + 0.5 * d1v, diameter, k_viscosity, sphere_density, fluid_density, gravity);
    let d3z = h_step * (velocity + 0.5 * d2v);
    let d3v = h_step * dvdt(velocity + 0.5 * d2v, diameter, k_viscosity, sphere_density, fluid_density, gravity);
    let d4z = h_step * (velocity + d3v);
    let d4v = h_step * dvdt(velocity + d3v, diameter, k_viscosity, sphere_density, fluid_density, gravity);
    let z_new = z + (d1z + 2.0 * d2z + 2.0 * d3z + d4z) / 6.0;
    let v_new = velocity + (d1v + 2.0 * d2v + 2.0 * d3v + d4v) / 6.0;
    (z_new, v_new)
}

pub fn free_fall(
    z_i: f64,
    velocity_i: f64,
    diameter: f64,
    k_viscosity: f64,
    sphere_density: f64,
    fluid_density: f64,
    gravity: f64,
    h_step: f64,
    time: f64,
) {
    let mut z = z_i;
    let mut v = velocity_i;
    println!("Initial: z = {}, v = {}", z, v);
    let steps = (time / h_step).ceil() as usize;
    for step in 0..steps {  
        let (z_new, v_new) = free_fall_rk4(
            z,
            v,
            diameter,
            k_viscosity,
            sphere_density,
            fluid_density,
            gravity,
            h_step,
        );
        println!("Step {}, time = {}, z = {}, v = {}", step + 1, (step + 1) as f64 * h_step,  z_new, v_new);
        z = z_new;
        v = v_new;
    }
}

pub fn run_example() {
    free_fall(0.0, 0.0, 0.01, 0.0000149, 8000.0, 1.22, 9.8, 0.1, 5.0);
}