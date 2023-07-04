slint::include_modules!();
fn main() {
    let ui = MainWindow::new().unwrap();
    ui.global::<State>().on_launch(|id| {
        println!("id: {}", id);
    });
    ui.run().unwrap();
}