pub mod truetype;

pub use read_fonts;

/// Interface for extracting a basic outline as a path from a glyph loader.
pub trait PathSink {
    fn move_to(&mut self, x: f32, y: f32);
    fn line_to(&mut self, x: f32, y: f32);
    fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32);
    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32);
    fn close(&mut self);
}
