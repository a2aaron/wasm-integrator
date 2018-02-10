#![feature(conservative_impl_trait, generators, generator_trait)]

use std::ops::Generator;

fn main() {
    println!("Hello, world!");
    let funct = |t: f64, x: f64| -> f64 { t*x };
    let mut x_gen = Euler{}.integrate_to(&funct, 1.1, 0.0, 5.06, 0.1);
    for _ in 0..10 {
        println!("{:?}", x_gen);
    }
}

trait Integrator {
    fn step<F: Fn(f64, f64) -> f64>(&self, f: &F, x0: f64, t0: f64, dt: f64) -> f64;

    fn integrate<'a, F: Fn(f64, f64) -> f64>(&'a self, f: &'a F, x0: f64, t0: f64, dt: f64) -> Box<Generator<Return = f64, Yield = f64> + 'a>;

    fn integrate_to<F: Fn(f64, f64) -> f64>(&self, f: &F, x0: f64, t0: f64, t_end: f64, dt: f64) -> f64;
}


struct Euler {
}

impl Integrator for Euler {
    fn step<F: Fn(f64, f64) -> f64>(&self, f: &F, x0: f64, t0: f64, dt: f64) -> f64 {
        x0 + dt * f(t0, x0)
    }

    fn integrate<'a, F: Fn(f64, f64) -> f64>(&'a self, f: &'a F, mut x0: f64, mut t0: f64, dt: f64) -> Box<Generator<Return = f64, Yield = f64> + 'a> {
        Box::new(move || {
            loop {
            let x_next = self.step(f, x0, t0, dt);
            yield x_next;
            x0 = x_next;
            t0 = t0 + dt;
            }
        })
    }

    fn integrate_to<F: Fn(f64, f64) -> f64>(&self, f: &F, mut x0: f64, mut t0: f64, t_end: f64, dt: f64) -> f64 {
        while t0 + dt < t_end {
            x0 = self.step(f, x0, t0, dt);
            t0 = t0 + dt;
        }
        // Take one last small step to land right on t_end
        let t_diff = t_end - t0;
        self.step(f, x0, t0, t_diff)
    }
}
