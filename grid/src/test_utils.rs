#[cfg(test)]
use nu_ansi_term::Color;

#[cfg(test)]
pub enum DebugGridTileColor {
    Header,
    White,
    Red,
    Green,
    Dimmed,
}
#[cfg(test)]
impl DebugGridTileColor {
    pub const BG_COL: Color = Color::RGB(40, 40, 40);

    pub fn colored(&self, tile_char: char) -> String {
        self.style().paint(tile_char.to_string()).to_string()
    }

    pub fn prefix(&self) -> String {
        self.style().prefix().to_string()
    }

    fn style(&self) -> nu_ansi_term::Style {
        (match self {
            DebugGridTileColor::Header => Color::RGB(170, 170, 170),
            DebugGridTileColor::White => Color::RGB(255, 255, 255),
            DebugGridTileColor::Red => Color::Red,
            DebugGridTileColor::Green => Color::Green,
            DebugGridTileColor::Dimmed => Color::RGB(170, 170, 170),
        })
        .on(Self::BG_COL)
    }
}

#[cfg(test)]
pub(crate) struct TestGridUtils {}
#[cfg(test)]
impl TestGridUtils {
    pub const TILE_SIZE: u16 = 96;
    pub const TEST_LVL_6X3: &'static str = "
    ......
    .####.
    ......
    ";
}
