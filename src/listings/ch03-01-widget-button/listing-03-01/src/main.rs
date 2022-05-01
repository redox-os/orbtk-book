//!
//! OrbTk-Book: Annotated widget listing
//!

// ANCHOR: All
// ANCHOR: Use
use orbtk::{
    prelude::*,
    widgets::themes::theme_orbtk::{
        {colors, material_icons_font},
        theme_default_dark,
    },
};
// ANCHOR_END: Use

// ANCHOR: Constants
pub static ID_BUTTON_CONTAINER: &str = "ButtonContainer";
pub static ID_BUTTON_CHECK: &str = "ButtonCheck";
pub static ID_BUTTON_TEXT_BLOCK_HEADER: &str = "ButtonTextBlock Header";
pub static ID_BUTTON_TEXT_BLOCK_1: &str = "ButtonTextBlock 1";
pub static ID_BUTTON_TEXT_BLOCK_2: &str = "ButtonTextBlock 2";
pub static ID_BUTTON_TEXT_BLOCK_3: &str = "ButtonTextBlock 3";
pub static ID_BUTTON_ICONONLY: &str = "ButtonIcononly";
pub static ID_BUTTON_TEXTONLY: &str = "ButtonTextonly";
pub static ID_BUTTON_UNCHECK: &str = "ButtonUncheck";
pub static ID_BUTTON_STACK: &str = "ButtonStack";
pub static ID_BUTTON_STYLED: &str = "ButtonStyled";
pub static ID_BUTTON_VIEW: &str = "ButtonView";
pub static ID_WINDOW: &str = "button_Window";
// ANCHOR_END: Constants

// ANCHOR: Main
fn main() {
    // Asure correct initialization, if compiling as a web application
    orbtk::initialize();

    // ANCHOR: Application_New
    Application::new()
        // ANCHOR: Theme
        .theme(
            // ANCHOR: Theme_Name
            theme_default_dark()
            // ANCHOR_END: Theme_Name
        )
        // ANCHOR_END: Theme
        // ANCHOR: Window
        .window(|ctx| {
            // ANCHOR: Window_New
            Window::new()
                .id(ID_WINDOW)
                .name(ID_WINDOW)
                .title("OrbTk-Book - Widget Button")
                .position((100.0, 100.0))
                .size(260.0, 400.0)
                .resizable(true)
                // ANCHOR: Child_ButtonView
                .child(
                    ButtonView::new()
                        .id(ID_BUTTON_VIEW)
                        .name(ID_BUTTON_VIEW)
                        .min_width(120.0)
                        .build(ctx),
                )
                // ANCHOR_END: Child_ButtonView
                .build(ctx)
            // ANCHOR_END: Window_New
        })
        // ANCHOR_END: Window
        // ANCHOR: Application_Run
        .run()
        // ANCHOR_END: Application_Run
    // ANCHOR_END: Application_New
}
// ANCHOR_END: Main

// ANCHOR: View
// ANCHOR: MacroCall_View
// Represents a button widgets.
widget!(ButtonView {});
// ANCHOR_END: MacroCall_View

impl Template for ButtonView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.id(ID_BUTTON_VIEW)
            .name(ID_BUTTON_VIEW)
            // ANCHOR: Child_Container_Definition
            .child(
                Container::new()
                    .id(ID_BUTTON_CONTAINER)
                    .name(ID_BUTTON_CONTAINER)
                    .background(colors::BOMBAY_COLOR)
                    .border_brush(colors::BOMBAY_COLOR)
                    .border_width(2)
                    // padding definition:
                    // as touple clockwise (left, top, right, bottom)
                    .padding((36, 16, 36, 16))
                    .min_width(140.0)
            // ANCHOR_END: Child_Container_Definition
                    // ANCHOR_END: Child_Stack_Definition
                    .child(
                        Stack::new()
                            .id(ID_BUTTON_STACK)
                            .name(ID_BUTTON_STACK)
                            .spacing(8)
                            // ANCHOR: Child_TextBlock
                            .child(
                                // ANCHOR: TextBlock
                                TextBlock::new()
                                    .id(ID_BUTTON_TEXT_BLOCK_HEADER)
                                    .name(ID_BUTTON_TEXT_BLOCK_HEADER)
                                    .font_size(18)
                                    // generic color names:
                                    // constants from crate `utils` -> colors.txt
                                    .foreground("black")
                                    .text("Available button stylings")
                                    .build(ctx),
                                // ANCHOR: Text_Block
                            )
                            // ANCHOR_END: Child_TextBlock
                            .child(
                                TextBlock::new()
                                    .id(ID_BUTTON_TEXT_BLOCK_1)
                                    .name(ID_BUTTON_TEXT_BLOCK_1)
                                    .font_size(14)
                                    // generic color name : rgb value
                                    .foreground("#3b434a")
                                    .text("Only Text")
                                    .build(ctx),
                            )
                            // ANCHOR: Child_Button_TextOnly
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_TEXTONLY)
                                    .name(ID_BUTTON_TEXTONLY)
                                    .enabled(false)
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .text("Disabled")
                                    .build(ctx),
                            )
                            // ANCHOR_END: Child_Button_TextOnly
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_CHECK)
                                    .name(ID_BUTTON_CHECK)
                                    .text("Push me!")
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .build(ctx),
                            )
                            .child(
                                TextBlock::new()
                                    .id(ID_BUTTON_TEXT_BLOCK_2)
                                    .name(ID_BUTTON_TEXT_BLOCK_2)
                                    .font_size(14)
                                    // theme color names:
                                    // constants from crate `orbtk_widgets`
                                    //  -> src/themes/themes_<theme_name>/colors.rs
                                    .foreground(colors::BRIGHT_GRAY_COLOR)
                                    //.foreground("#3b434a")
                                    .text("Only Icon")
                                    .build(ctx),
                            )
                            // ANCHOR: Child_Button_IconOnly
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_CHECK)
                                    .name(ID_BUTTON_CHECK)
                                    .enabled(false)
                                    .icon(material_icons_font::MD_CHECK)
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .on_enter(|_, _| {
                                        println!("Enter Button boundries");
                                    })
                                    .on_leave(|_, _| {
                                        println!("Leave Button boundries");
                                    })
                                    .build(ctx),
                            )
                            // ANCHOR_END: Child_Button_IconOnly
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_ICONONLY)
                                    .name(ID_BUTTON_ICONONLY)
                                    .style("button_primary")
                                    .icon(material_icons_font::MD_360)
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .pressed(true)
                                    .build(ctx),
                            )
                            // ANCHOR_END: Child_Button_IconOnly
                            .child(
                                TextBlock::new()
                                    .id(ID_BUTTON_TEXT_BLOCK_3)
                                    .name(ID_BUTTON_TEXT_BLOCK_3)
                                    .font_size(14)
                                    // theme color names:
                                    // constants from crate `orbtk_widgets`
                                    //  -> src/themes/themes_<theme_name>/colors.rs
                                    .foreground(colors::BRIGHT_GRAY_COLOR)
                                    .text("Icon and Text")
                                    .build(ctx),
                            )
                            // ANCHOR_END: Child_Button_IconOnly
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_CHECK)
                                    .name(ID_BUTTON_CHECK)
                                    .enabled(false)
                                    .icon(material_icons_font::MD_CHECK)
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .text("Checked")
                                    .on_enter(|_, _| {
                                        println!("Enter Button boundries");
                                    })
                                    .on_leave(|_, _| {
                                        println!("Leave Button boundries");
                                    })
                                    .build(ctx),
                            )
                            // ANCHOR: Child_Button_Styled
                            .child(
                                Button::new()
                                    .id(ID_BUTTON_STYLED)
                                    .name(ID_BUTTON_STYLED)
                                    .style("button_primary")
                                    .icon(material_icons_font::MD_360)
                                    .max_width(180.0)
                                    .min_width(90.0)
                                    .pressed(true)
                                    .text("From style")
                                    .build(ctx),
                            )
                            .build(ctx),
                            // ANCHOR_END: Child_Button_Styled
                    )
                    .build(ctx),
            )
    }
}
// ANCHOR_END: ButtonView

// ANCHOR_END: All
