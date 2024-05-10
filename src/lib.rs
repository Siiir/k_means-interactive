use std::rc::Rc;

use rand::Rng;

slint::include_modules!();

pub mod group;

pub use point_src::PointSrc;
pub mod point_src;
pub mod state_serv;
pub mod util;

pub fn new_gui() -> MainWindow {
    // Construct main window
    let main_win = MainWindow::new().unwrap();
    // Init logic
    {
        let point_src = Rc::<crate::point_src::IdentifiableRandPoints>::default();
        {
            let point_info = main_win.global::<PointInfo>();
            point_info.on_x(point_src.clone().coord_x_getter_for_i32_idx());
            point_info.on_y(point_src.clone().coord_y_getter_for_i32_idx());
        }
        let logic = main_win.global::<Logic>();
        {
            let _future_use_of_point_src = point_src;
            logic.on_rand_fract_float(|| rand::thread_rng().gen());
            logic.on_rand_natural_int(|| rand::thread_rng().gen::<i32>() & i32::MAX);
            logic.on_group_color(group::color);
            logic.on_group_color_with_alpha(group::color_with_alpha);
            logic.on_loosely_parse_int(crate::util::loosely_parse_int);
            // Dirty functions
            logic.on_algorithm_step(|step_count| println!("todo: perform {step_count} steps."));
        }
    } // Return
    main_win
}
