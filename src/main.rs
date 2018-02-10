extern crate stdweb;

use stdweb::web::{document, INode, Node};

fn main() {
    stdweb::initialize();
    let root = document().query_selector("body").unwrap();

    write_list(root.as_node(), (0..10).scan((1.0, 0.0), |&mut (ref mut x, ref mut t), _| {
        let x0 = *x;
        let (next_x, next_t) = euler_step(&|_, x| -x, *x, 0.1, *t);
        *x = next_x;
        *t = next_t;
        Some(format!("{:.04}", x0))
    }));
}


fn write_list<S: AsRef<str>, I: IntoIterator<Item=S>>(node: &Node, items: I) {
    let list = document().create_element("ul");
    for item in items {
        let list_item = document().create_element("li");
        list_item.set_text_content(item.as_ref());
        list.append_child(&list_item);
    }
    node.append_child(&list);
}


fn euler_step<F: Fn(f64, f64) -> f64>(f: &F, x0: f64, dt: f64, t: f64) -> (f64, f64) {
    (x0 + dt * f(t, x0), t + dt)
}
