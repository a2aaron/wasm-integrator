#![feature(conservative_impl_trait, generators, generator_trait, never_type)]

extern crate stdweb;

use stdweb::web::{document, INode, Node};

use std::ops::{GeneratorState, Generator, Add, Mul};

fn to_iter<G, Y>(gen: G) -> impl Iterator<Item=Y>
    where G: Generator<Yield=Y>
{
    (0..).scan(gen, |state, _| {
        match state.resume() {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    })
}

fn main() {
    stdweb::initialize();
    let root = document().query_selector("body").unwrap();
    write_table(
        root.as_node(),
        Some(("Time", "RK4 Position")),
        to_iter(RK4.integrate(|t, x| -t * x, 1.1, 0.0, 0.1))
            .take(11)
            .enumerate()
            .map(|(i, x)| (i as f64 * 0.1, x)),
    );
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

struct RK4;

impl<N: Mul<f64, Output = N> + Add<Output = N> + Copy> Integrator<N> for RK4 {
    fn step<F: Fn(f64, N) -> N>(&self, f: F, x0: N, t0: f64, dt: f64) -> N {
        let k1 = f(t0, x0);
        let k2 = f(t0 + dt / 2.0, x0 + k1 * (dt / 2.0));
        let k3 = f(t0 + dt / 2.0, x0 + k2 * (dt / 2.0));
        let k4 = f(t0 + dt, x0 + k3 * dt);
        x0 + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0)
    }
}

fn write_table<H, R, I>(node: &Node, header: Option<H>, rows: I)
    where H: Into<TableRow>, R: Into<TableRow>, I: IntoIterator<Item=R>
{
    let table = document().create_element("table");
    if let Some(header) = header {
        let tr = document().create_element("tr");
        for cell in &header.into().cells {
            let th = document().create_element("th");
            th.append_child(cell);
            tr.append_child(&th);
        }
        table.append_child(&tr);
    }

    for row in rows {
        let tr = document().create_element("tr");
        for cell in &row.into().cells {
            let td = document().create_element("td");
            td.append_child(cell);
            tr.append_child(&td);
        }
        table.append_child(&tr);
    }

    node.append_child(&table);
}

struct TableRow {
    cells: Vec<Node>,
}

impl<'a, 'b> Into<TableRow> for (&'a str, &'b str) {
    fn into(self) -> TableRow {
        TableRow { cells: vec![
            document().create_text_node(self.0).as_node().clone(),
            document().create_text_node(self.1).as_node().clone(),
        ] }
    }
}

impl Into<TableRow> for (f64, f64) {
    fn into(self) -> TableRow {
        TableRow { cells: vec![
            document().create_text_node(&format!("{:.6}", self.0)).as_node().clone(),
            document().create_text_node(&format!("{:.6}", self.1)).as_node().clone(),
        ] }
    }
}
