#![feature(conservative_impl_trait, generators, generator_trait)]

use std::ops::Generator;

fn main() {
    println!("Hello, world!");
    let funct = |t: f64, x: f64| -> f64 { t*x };

    let mut x_this = 1.0;
    let mut t_this = 0.0;
    for _ in 0..10 {
        let (x_next, t_next) = euler_step(&funct, x_this, 0.1, t_this);
        println!("{} {}", x_next, t_next);
        x_this = x_next;
        t_this = t_next;
    }
}

trait Integrator {
    fn step(&self, dt: f64);

    fn integrate<F: Fn(f64, f64) -> f64>(&self, f: &F, x0: f64, t0: f64) -> Generator<Return = f64, Yield = f64>;

    fn integrate_to<F: Fn(f64, f64) -> f64>(&self, f: &F, x0: f64, t0: f64, t_end: f64);
}


fn euler_step<F: Fn(f64, f64) -> f64>(f: &F, x0: f64, dt: f64, t: f64) -> (f64, f64) {
    (x0 + dt * f(t, x0), t + dt)
}
