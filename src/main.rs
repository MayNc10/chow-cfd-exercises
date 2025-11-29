use chow_cfd_exercises::{free_fall_1dot1::{free_fall, run_example}, wing_1dot3};

fn main() {
    println!("Hello, world!");
    //run_example();
    wing_1dot3::sim(25.0, 5.0, 10.0_f32.to_radians());
}
