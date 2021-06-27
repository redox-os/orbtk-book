use orbtk::prelude::*;

// ANCHOR: Localization
static LOCALIZATION_DE_DE: &str = include_str!("../assets/localization/dictionary_de_DE.ron");
static LOCALIZATION_ES_ES: &str = include_str!("../assets/localization/dictionary_es_ES.ron");
// ANCHOR_END: Localization

fn main() {
    // if no dictionary is set for the default language e.g. english
    // the content of the text property will be drawn.
    // ANCHOR: Language
    let _de_de = RonLocalization::create()
        .language("de_DE")
        .dictionary("de_DE", LOCALIZATION_DE_DE)
        .build();
    let _es_es = RonLocalization::create()
        .language("es_ES")
        .dictionary("es_ES", LOCALIZATION_ES_ES)
        .build();
    // ANCHOR_END: Language

    // use this only if you want to run it as web application.
    orbtk::initialize();

    // ANCHOR: Application
    Application::new()
        .localization(_es_es)
        // ANCHOR_END: Application
        .window(|ctx| {
            Window::new()
                .title("OrbTk - Example Localization")
                .position((100.0, 100.0))
                .size(420.0, 240.0)
                .child(
                    Stack::new()
                        .spacing(8)
                        .v_align("center")
                        .child(
                            TextBlock::new()
                                .text("Hey OrbTK User!")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx)
                        )
                        .child(
                            TextBlock::new()
                                .text("Do you like it?")
                                .h_align("center")
                                .v_align("center")
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
