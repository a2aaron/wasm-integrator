#![feature(conservative_impl_trait, generators, generator_trait, never_type)]

use std::ops::{Generator, Add, Mul};

fn main() {
    println!("Hello, world!");
    let funct = |t: f64, x: f64| -> f64 { t*x };
    let x_gen = Euler.integrate_to(&funct, 1.1, 0.0, 5.06, 0.1);
    for _ in 0..10 {
        println!("{:?}", x_gen);
    }
}

trait Integrator<N: Mul<f64, Output = N> + Add<Output = N> + Copy> {
    fn step<F: Fn(f64, N) -> N>(&self, f: F, x0: N, t0: f64, dt: f64) -> N;

    fn integrate<'a, F>(&'a self, f: F, mut x0: N, mut t0: f64, dt: f64)
        -> Box<Generator<Return = !, Yield = N> + 'a>
        where N: 'a, F: Fn(f64, N) -> N + 'a
    {
        Box::new(move || {
            loop {
                let x_next = self.step(&f, x0, t0, dt);
                x0 = x_next;
                yield x_next;
                t0 = t0 + dt;
            }
        })
    }

    fn integrate_to<F: Fn(f64, N) -> N>(&self, f: F, mut x0: N, mut t0: f64, t_end: f64, dt: f64) -> N {
        while t0 + dt < t_end {
            x0 = self.step(&f, x0, t0, dt);
            t0 = t0 + dt;
        }
        // Take one last small step to land right on t_end
        let t_diff = t_end - t0;
        self.step(&f, x0, t0, t_diff)
    }
}


struct Euler;

impl<N: Mul<f64, Output = N> + Add<Output = N> + Copy> Integrator<N> for Euler {
    fn step<F: Fn(f64, N) -> N>(&self, f: F, x0: N, t0: f64, dt: f64) -> N {
        x0 + f(t0, x0) * dt
    }
}

