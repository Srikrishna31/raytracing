// mod scenes;

use raytracing::{
    load_configuration, ProgressCallback, write_image
};

use std::rc::Rc;



fn main() {
    let cb: ProgressCallback = Rc::new(|i: f64| eprintln!("{i:.2}% completed"));
    let settings = load_configuration().expect("Couldnot read settings");
    write_image(settings, cb)
}




