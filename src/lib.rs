use rand::Rng;

slint::include_modules!();

pub mod group;
pub mod util;

pub fn new_gui() -> MainWindow {
    // Construct main window
    let main_win = MainWindow::new().unwrap();
    // Init logic
    {
        let logic = main_win.global::<Logic>();
        // Pure functions
        logic.on_rand_fract_float(|| rand::thread_rng().gen());
        logic.on_rand_natural_int(|| rand::thread_rng().gen::<i32>() & i32::MAX);
        logic.on_group_color(group::color);
        logic.on_group_color_with_alpha(group::color_with_alpha);
        logic.on_loosely_parse_int(crate::util::loosely_parse_int);
        // Dirty functions
        logic.on_algorithm_step(|step_count| println!("todo: perform {step_count} steps."));
    } // Return
    main_win
}
