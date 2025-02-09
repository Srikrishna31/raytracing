extern crate scenes;

use scenes::Scenes;
use slint::{SharedString, ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError>{
    let main_window = MainWindow::new()?;
    let scenes = scenes::get_scenes()
                                        .iter()
                                        .map(|scene| {SharedString::from(*scene)})
                                        .collect::<Vec<SharedString>>();
    let model = ModelRc::new(VecModel::from(scenes));
    main_window.set_items(model);

    main_window.run()
}