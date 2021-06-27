// ANCHOR: all
// ANCHOR: use
use orbtk::prelude::*;
// ANCHOR_END: use

// ANCHOR: main
fn main() {
    // ANCHOR_END: main
    // ANCHOR: initialize
    // use this only if you want to run it as web application.
    orbtk::initialize();
    // ANCHOR_END: initialize

    // ANCHOR: Application
    Application::new()
        // ANCHOR_END: Application
        // ANCHOR: Window
        .window(|ctx| {
            Window::new()
                .title("OrbTk-Book - Chapter 1.2")
                .position((100.0, 100.0))
                .size(420.0, 140.0)
                // ANCHOR_END: Window
                // ANCHOR: Child
                .child(
                    TextBlock::new()
                        // ANCHOR: Properties
                        .font_size(28)
                        .h_align("center")
                        .text("Hey OrbTk!")
                        .v_align("center")
                        // ANCHOR_END: Properties
                        .build(ctx)
                )
                // ANCHOR_END: Child
                // ANCHOR: Build
                .build(ctx)
                // ANCHOR_END: Build
        })
        // ANCHOR: Run
        .run();
        // ANCHOR_END: Run
}
// ANCHOR_END: all
