use cursive::views::{Dialog, TextView};

fn main() {
    // Creates the cursive root - required for every application.
    let mut win = cursive::default();

    // Creates a dialog with a single "Quit" button
    let dlg: Dialog  = Dialog::around(TextView::new("Hello Dialog!"))
        .title("uish")
        .button("Quit", |s| s.quit());

    // Add Dialog to the root Window
    win.add_layer(dlg);

    // Starts the event loop.
    win.run();
}