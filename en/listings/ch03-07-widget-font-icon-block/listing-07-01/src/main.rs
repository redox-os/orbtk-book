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
pub static ID_FONT_ICON_BLOCK_CONTAINER: &str = "FontIconBlockContainer";
pub static ID_FONT_ICON_BLOCK_VIEW: &str = "FontIconBlockView";
pub static ID_FONT_ICON_BLOCK_STACK: &str = "FontIconBlockStack";
pub static ID_FONT_ICON_BLOCK_TEXT_BLOCK_HEADER: &str = "FontIconBlockTextBlockHeader";
pub static ID_FONT_ICON_BLOCK_STYLED: &str = "FontIconBlockStyled";
pub static ID_WINDOW: &str = "FontIconBlock_Window";
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
                .title("OrbTk-Book - Widget FontIconBlock")
                .position((100.0, 100.0))
                .size(290.0, 200.0)
                .resizable(true)
                // ANCHOR: Child_FontIconBlockView
                .child(
                    FontIconBlockView::new()
                        .id(ID_FONT_ICON_BLOCK_VIEW)
                        .name(ID_FONT_ICON_BLOCK_VIEW)
                        .min_width(120.0)
                        .build(ctx),
                )
                // ANCHOR_END: Child_FontIconBlockView
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
// Represents a FontIconBlock widgets.
widget!(FontIconBlockView {});
// ANCHOR_END: MacroCall_View

// ANCHOR: Template_FontIconBlockView
impl Template for FontIconBlockView {
    fn template(self, _id: Entity, ctx: &mut BuildContext) -> Self {
        self.id(ID_FONT_ICON_BLOCK_VIEW)
            .name(ID_FONT_ICON_BLOCK_VIEW)
            // ANCHOR: Child_Container_Definition
            .child(
                Container::new()
                    .id(ID_FONT_ICON_BLOCK_CONTAINER)
                    .name(ID_FONT_ICON_BLOCK_CONTAINER)
                    //.background(colors::BOMBAY_COLOR)
                    .background("lightgray")
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
                            .id(ID_FONT_ICON_BLOCK_STACK)
                            .name(ID_FONT_ICON_BLOCK_STACK)
                            .spacing(8)
                            // ANCHOR: Child_TextBlock
                            .child(
                                // ANCHOR: TextBlock
                                TextBlock::new()
                                    .id(ID_FONT_ICON_BLOCK_TEXT_BLOCK_HEADER)
                                    .name(ID_FONT_ICON_BLOCK_TEXT_BLOCK_HEADER)
                                    .font_size(18)
                                    // generic color names:
                                    // constants from crate `utils` -> colors.txt
                                    .foreground("black")
                                    .text("Font icon block")
                                    .build(ctx),
                                // ANCHOR: Text_Block
                            )
                            // ANCHOR: Child_FontIconBlock
                            .child(
                                FontIconBlock::new()
                                    .id(ID_FONT_ICON_BLOCK_STYLED)
                                    .name(ID_FONT_ICON_BLOCK_STYLED)
                                    //.style("font-icon-block")
                                    .h_align("center")
                                    .icon(material_icons_font::MD_360)
                                    .icon_brush("repeating-linear-gradient(0.25turn, rgba(255, 255, 0, 0.6), dodgerblue, deepskyblue)")
                                    .icon_font("MaterialIcons-Regular")
                                    .icon_size(128.0)
                                    .build(ctx),
                            )
                            .build(ctx),
                            // ANCHOR_END: Child_FontIconBlock
                    )
                    .build(ctx),
            )
    }
}
// ANCHOR_END: Template_FontIconBlockView

// ANCHOR_END: All
