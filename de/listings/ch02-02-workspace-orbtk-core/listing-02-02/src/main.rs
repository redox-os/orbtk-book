use orbtk:: {
    prelude::*,
    shell::{Key, KeyEvent},
    //widgets::behaviors::MouseBehavior,
};

// ANCHOR: Localization
static LOCALIZATION_DE_DE: &str = include_str!("../assets/localization/dictionary_de_DE.ron");
// ANCHOR_END: Localization

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
                            if key_event.key == Key::A(true) {
                                self.handle_ctrl_a(ctx);
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
    fn action(&mut self, action: KeyboardAction) {
        self.action = Some(action);
    }

    fn handle_ctrl_a(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Ctrl+a`
        println!("Handle: `Key::Ctr` + `Key:A`")
    }

    fn handle_down_key(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Down`
        println!("Handle: `Key::Down`")
    }

    fn handle_enter_key(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Enter`
        println!("Handle: `Key::Enter`")
    }

    fn handle_left_key(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Left`
        println!("Handle: `Key::Left`")
    }

    fn handle_right_key(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Key::Right`
        println!("Handle: `Key::Right`")
    }

    fn handle_up_key(&mut self, _ctx: &mut Context<'_>) {
        // here goes your rust code to act on `Up`
        println!("Handle: `Key::Up`")
    }

    fn request_focus(&mut self, _ctx: &mut Context<'_>) {
        println!("You did request focus!")
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
    let de_de = RonLocalization::create()
        .language("de_DE")
        .dictionary("de_DE", LOCALIZATION_DE_DE)
        .build();
    // ANCHOR_END: Language

    // use this only if you want to run it as web application.
    orbtk::initialize();

    // ANCHOR: Application
    Application::new()
        .localization(de_de)
        // ANCHOR_END: Application
        .window(|ctx| {
            Window::new()
                .title("OrbTk-Book - Chapter 2.2.2")
                .position((100.0, 100.0))
                .size(450.0, 140.0)
                .child(
                    Stack::new()
                        .spacing(16)
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
                            Container::new()
                                .padding((64, 0, 32, 0))
                                .child(
                                    Stack::new()
                                        .spacing(16)
                                        .orientation("horizontal")
                                        .v_align("left")
                                        .child(
                                            TextBlock::new()
                                                .font_size(22)
                                                .h_align("left")
                                                .text("How are you today?")
                                                .v_align("center")
                                                .build(ctx)
                                        )
                                        .child(
                                            TextBox::new()
                                                .font_size(22)
                                                .h_align("left")
                                                .v_align("center")
                                                .build(ctx)
                                        )
                                        .build(ctx)
                                )
                                .build(ctx)
                        )
                        .build(ctx)
                )
                .build(ctx)
        })
        .run();
}
