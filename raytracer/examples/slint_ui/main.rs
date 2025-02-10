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
    let mut selected = SharedString::new();


    main_window.set_items(model.clone());

    main_window.on_selection_changed(move |value| {
        selected = value;
    });
    let window_handle = main_window.as_weak();
    let progress_function = |i:f32| {
        if let Some(main_window) = window_handle.upgrade() {
            main_window.set_progress(i);
            main_window.set_is_enabled(i == 100.0);

            // if i == 100 {
            //     main_window.set_image()
            // }
        }
    };
    main_window.on_render_clicked(move ||{
        if let Some(main_window) = window_handle.upgrade() {
            main_window.set_is_enabled(false);
            // scenes::render_scene_buffer(scenes::name_to_scene(selected.to_string().as_str()),
            //         progress_function);
            //
        }
    });

    main_window.run()
}