extern crate stdweb;

use stdweb::web::{document, INode, Node};

fn main() {
    stdweb::initialize();
    let root = document().query_selector("body").unwrap();

    write_table(
        root.as_node(),
        Some(("Time", "Position")),
        (0..11).scan((1.0, 0.0), |state, _| {
            let (x, t) = *state;
            *state = euler_step(&|_, x| -x, state.0, 0.1, state.1);
            Some((t, x))
        }),
    );
}

struct TableRow {
    cells: Vec<Node>,
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


fn euler_step<F: Fn(f64, f64) -> f64>(f: &F, x0: f64, dt: f64, t: f64) -> (f64, f64) {
    (x0 + dt * f(t, x0), t + dt)
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
