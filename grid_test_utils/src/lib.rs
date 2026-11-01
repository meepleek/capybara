use nu_ansi_term::Color;

pub enum DebugGridTileColor {
    Header,
    White,
    Red,
    Green,
    Dimmed,
}
impl DebugGridTileColor {
    pub const BG_COL: Color = Color::Rgb(40, 40, 40);

    pub fn colored(&self, tile_char: char) -> String {
        self.style().paint(tile_char.to_string()).to_string()
    }

    pub fn prefix(&self) -> String {
        self.style().prefix().to_string()
    }

    fn style(&self) -> nu_ansi_term::Style {
        (match self {
            DebugGridTileColor::Header => Color::Rgb(170, 170, 170),
            DebugGridTileColor::White => Color::Rgb(255, 255, 255),
            DebugGridTileColor::Red => Color::Red,
            DebugGridTileColor::Green => Color::Green,
            DebugGridTileColor::Dimmed => Color::Rgb(170, 170, 170),
        })
        .on(Self::BG_COL)
    }
}

pub struct TestGridUtils {}
impl TestGridUtils {
    pub const TILE_SIZE: u16 = 96;
    pub const TEST_LVL_6X3: &'static str = "
    ......
    .####.
    ......
    ";
}
