#![feature(conservative_impl_trait, generators, generator_trait)]

use std::ops::{Generator, Add, Mul};

fn main() {
    println!("Hello, world!");
    let funct = |t: f64, x: f64| -> f64 { t*x };
    let x_gen = Euler.integrate_to(&funct, 1.1, 0.0, 5.06, 0.1);
    for _ in 0..10 {
        println!("{:?}", x_gen);
    }
}

trait Integrator<F: Mul<f64, Output = F> + Add<Output = F> + Copy> {
    fn step(&self, f: &Fn(f64, F) -> F, x0: F, t0: f64, dt: f64) -> F;

    fn integrate<'a>(&'a self, f: &'a Fn(f64, F) -> F, mut x0: F, mut t0: f64, dt: f64) -> Box<Generator<Return = F, Yield = F> + 'a> {
        Box::new(move || {
            loop {
                let x_next = self.step(f, x0, t0, dt);
                x0 = x_next;
                yield x_next;
                t0 = t0 + dt;
            }
        })
    }

    fn integrate_to(&self, f: &Fn(f64, F) -> F, mut x0: F, mut t0: f64, t_end: f64, dt: f64) -> F {
        while t0 + dt < t_end {
            x0 = self.step(f, x0, t0, dt);
            t0 = t0 + dt;
        }
        // Take one last small step to land right on t_end
        let t_diff = t_end - t0;
        self.step(f, x0, t0, t_diff)
    }
}


struct Euler;

impl<F: Mul<f64, Output = F> + Add<Output = F> + Copy> Integrator<F> for Euler {
    fn step(&self, f: &Fn(f64, F) -> F, x0: F, t0: f64, dt: f64) -> F {
        x0 + f(t0, x0) * dt
    }
}
