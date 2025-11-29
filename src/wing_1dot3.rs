use core::f32;

use crate::runge;

fn lift(alpha: f32) -> f32 {
    if alpha >= f32::consts::PI / -10.0 && alpha <= f32::consts::PI / 10.0  {
        2.0 * f32::consts::PI * alpha
    } else { 0.0 }
}

fn gen_func(beta: f32, u: f32, a0: f32) -> Box<dyn Fn(f32,f32,f32) -> f32> {
    Box::new(move |z, v, t| {
        -(2.0 * f32::consts::PI).powi(2) * z 
        + beta * lift(a0 - f32::atan(v/u)) * u * (u.powi(2) + v.powi(2)).sqrt()
    })
}

pub fn sim(u0: f32, t_max: f32, alpha: f32) {
    let beta = 0.00183;
    let dt = 0.1;
    let mut t = 0.0;
    let mut z = 0.0;
    let mut v = 0.0;
    let du = u0;
    let mut u = 0.0;
    u = u + du;

    let f = gen_func(beta, u, alpha);

    while t < t_max {
        let al = alpha - f32::atan(v/u);
        println!("t: {}, z: {}, v: {}, ad: {}", t, z, v, al.to_degrees());
        (z,v) = runge::runge(z, v, t, dt, &f);
        t += dt;
    } 
}

// todo: more exercises   s