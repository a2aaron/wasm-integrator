//#[feature(conservative_impl_trait, generators, generator_trait)]

fn main() {
    println!("Hello, world!");
    let funct = |t: f64| -> f64 { t };

    let mut y_this = 1.0;
    let mut t_this = 0.0;
    for _ in 0..10 {
        let (y_next, t_next) = euler_step(&funct, y_this, 0.1, t_this);
        println!("{} {}", y_next, t_next);
        y_this = y_next;
        t_this = t_next;
    }
}


fn euler_step<F: Fn(f64) -> f64>(f: &F, x0: f64, dt: f64, t: f64) -> (f64, f64) {
    (x0 + dt * f(t), t + dt)
}
