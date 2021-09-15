use orbtk:: {
    prelude::*,
    shell::{Key, KeyEvent},
    widgets::behaviors::MouseBehavior,
};

// [KeyboardState]

#[derive(Clone)]
enum KeyboardAction {
    Key(KeyEvent),
    RequestFocus,
}

#[derive(AsAny, Default)]
struct KeyboardState {
    action: Option<KeyboardAction>,
    event_adapter: EventAdapter,
}

// Implementation of KeyboardActions for trait `State`
impl State for KeyboardState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action.clone() {
            match action {
                KeyboardAction::Key(key_event) => {
                    match key_event.key {
                        Key::Control => {
                            // Ctrl+'a'
                            if key_event.Key::A(false) {
                                self.handle_control_a(ctx);
                            }
                        }
                        Key::Down => {
                            self.handle_down_key(ctx);
                        }
                        Key::Enter => {
                            self.handle_enter_key(ctx);
                        }
                        Key::Left => {
                            self.handle_left_key(ctx);
                        }
                        Key::Right => {
                            self.handle_right_key(ctx);
                         }
                         Key::Up => {
                             self.handle_up_key(ctx);
                         }
                         _ => {}
                     }
                 }
                 KeyboardAction::RequestFocus => {
                     self.request_focus(ctx);
                 }
             }
             self.action = None;
         }
    }
}

// associated functions
impl KeyboardState {
    fn action(&mut self, action: DirectoryListAction) {
        self.action = Some(action);
    }

    fn handle_down_key(&mut self, ctx: &mut Context<'_>) {
        /// here goes your rust code to act on `Key::Down`
        println!("Handle: `Key::Down`")
    }

    fn handle_enter_key(&mut self, ctx: &mut Context<'_>) {
        /// here goes your rust code to act on `Key::Enter`
        println!("Handle: `Key::Enter`")
    }

    fn handle_left_key(&mut self, ctx: &mut Context<'_>) {
        /// here goes your rust code to act on `Key::Left`
        println!("Handle: `Key::Left`")
    }

    fn handle_right_key(&mut self, ctx: &mut Context<'_>) {
        /// here goes your rust code to act on `Key::Right`
        println!("Handle: `Key::Right`")
    }

    fn handle_up_key(&mut self, ctx: &mut Context<'_>) {
        /// here goes your rust code to act on `Key::Up`
        println!("Handle: `Key::Up`")
    }
}

 // [KeyboardView]
 // ... your view code

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
