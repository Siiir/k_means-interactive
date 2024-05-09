use slint::ComponentHandle;

fn main() {
    // Construct
    let gui = k_means::new_gui();
    // Run
    gui.run().unwrap();
}
