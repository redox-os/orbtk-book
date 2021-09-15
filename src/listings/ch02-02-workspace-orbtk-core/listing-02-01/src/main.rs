use orbtk::prelude::*;

/*
 * if you prefere to see a german translation
 * static LOCALIZATION_DE_DE: &str = include_str!("../assets/localization/dictionary_de_DE.ron");
 */

// ANCHOR: Localization
static LOCALIZATION_ES_ES: &str = include_str!("../assets/localization/dictionary_es_ES.ron");
// ANCHOR_END: Localization

fn main() {
    // if no dictionary is set for the default language e.g. english
    // the content of the text property will be drawn.
    /* disabled german translation file
     * let _de_de = RonLocalization::create()
     *    .language("de_DE")
     *    .dictionary("de_DE", LOCALIZATION_DE_DE)
     *    .build();
     */
    // ANCHOR: Language
    let es_es = RonLocalization::create()
        .language("es_ES")
        .dictionary("es_ES", LOCALIZATION_ES_ES)
        .build();
    // ANCHOR_END: Language

    // use this only if you want to run it as web application.
    orbtk::initialize();

    // ANCHOR: Application
    Application::new()
        .localization(es_es)
        // ANCHOR_END: Application
        .window(|ctx| {
            Window::new()
                .title("OrbTk-Book - Chapter 2.2")
                .position((100.0, 100.0))
                .size(450.0, 140.0)
                .child(
                    Stack::new()
                        .spacing(8)
                        .v_align("center")
                        .child(
                            TextBlock::new()
                                .font_size(28)
                                .h_align("center")
                                .text("Hey OrbTk User!")
                                .v_align("center")
                                .build(ctx)
                        )
                        .child(
                            TextBlock::new()
                                .font_size(28)
                                .h_align("center")
                                .text("Do you like it?")
                                .v_align("center")
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
