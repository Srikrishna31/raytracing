// mod scenes;

use raytracing::{
    load_configuration, render
};

use std::rc::Rc;



fn main() {
    let settings = load_configuration().expect("Couldnot read settings");
    render(settings, |i: f64| eprintln!("{i:.2}% completed"));
}




