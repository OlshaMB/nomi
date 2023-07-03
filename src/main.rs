slint::include_modules!();
fn main() {
    let ui = MainWindow::new().unwrap();
    let app = ui;
    
    ui.run().unwrap();
}