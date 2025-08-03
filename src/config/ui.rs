use ratatui::style::{Color};
use ratatui::widgets::Padding;

pub const BACKGROUND_COLOR: Color = Color::Rgb(22, 22, 22);
pub const POPUP_BACKGROUND_COLOR: Color = Color::Rgb(18, 18, 18);
pub const BORDER_COLOR: Color = Color::Rgb(111, 111, 111);
pub const POPUP_BORDER_COLOR: Color = Color::Rgb(80, 80, 80);
pub const HIGHLIGHT_COLOR: Color = Color::Rgb(255, 165, 0);
pub const TEXT_COLOR: Color = Color::Rgb(200, 200, 200);
pub const TITLE_TEXT_COLOR: Color = Color::Rgb(255, 255, 255);
pub const BLOCK_PADDING: Padding = Padding::new(2, 2, 2, 2);
// pub const BLOCK_MARGIN: Padding = Padding::new(2, 2, 2 , 2);
pub const HIGHLIGHT_SYMBOL: &str = "┃ ";