//! TrueType glyph loader.

mod cache;
mod hint;
mod load;
mod math;

use super::PathSink;

pub use hint::HinterMode;
pub use load::{Loader, LoaderState};

/// Point in a TrueType outline.
///
/// Note that the coordinates in a point are either in font units or fixed
/// point (26.6) depending on whether the outline was scaled while loading.
/// See [Outline::is_scaled].
#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Point {
    /// X cooordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Point {
    /// Creates a new point with the specified x and y coordinates.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// TrueType outline.
#[derive(Default, PartialEq, Eq, Debug)]
pub struct Outline {
    /// Set of points that define the shape of the outline.
    pub points: Vec<Point>,
    /// Set of tags (one per point).
    pub tags: Vec<u8>,
    /// Index of the end points for each contour in the outline.
    pub contours: Vec<u16>,
    /// True if the loader applied a scale, in which case the points are in
    /// 26.6 fixed point format. Otherwise, they are in integral font units.
    pub is_scaled: bool,
}

impl Outline {
    /// Creates a new empty outline.
    pub fn new() -> Self {
        Self::default()
    }

    /// Empties the outline.
    pub fn clear(&mut self) {
        self.points.clear();
        self.tags.clear();
        self.contours.clear();
        self.is_scaled = false;
    }

    /// Builds a path representation of the outline by calling the
    /// appropriate methods on the provided sink.
    pub fn path(&self, sink: &mut impl PathSink) -> bool {
        #[inline(always)]
        fn conv(p: Point, s: f32) -> (f32, f32) {
            (p.x as f32 * s, p.y as f32 * s)
        }
        const TAG_MASK: u8 = 0x3;
        const CONIC: u8 = 0x0;
        const ON: u8 = 0x1;
        const CUBIC: u8 = 0x2;
        let s = if self.is_scaled { 1. / 64. } else { 1. };
        let points = &self.points;
        let tags = &self.tags;
        let mut count = 0usize;
        let mut last_was_close = false;
        for c in 0..self.contours.len() {
            let mut cur = if c > 0 {
                self.contours[c - 1] as usize + 1
            } else {
                0
            };
            let mut last = self.contours[c] as usize;
            if last < cur || last >= points.len() {
                return false;
            }
            let mut v_start = points[cur];
            let v_last = points[last];
            let mut tag = tags[cur] & TAG_MASK;
            if tag == CUBIC {
                return false;
            }
            let mut step_point = true;
            if tag == CONIC {
                if tags[last] & TAG_MASK == ON {
                    v_start = v_last;
                    last -= 1;
                } else {
                    v_start.x = (v_start.x + v_last.x) / 2;
                    v_start.y = (v_start.y + v_last.y) / 2;
                }
                step_point = false;
            }
            let p = conv(v_start, s);
            if count > 0 && !last_was_close {
                sink.close();
            }
            sink.move_to(p.0, p.1);
            count += 1;
            last_was_close = false;
            // let mut do_close = true;
            while cur < last {
                if step_point {
                    cur += 1;
                }
                step_point = true;
                tag = tags[cur] & TAG_MASK;
                match tag {
                    ON => {
                        let p = conv(points[cur], s);
                        sink.line_to(p.0, p.1);
                        count += 1;
                        last_was_close = false;
                        continue;
                    }
                    CONIC => {
                        let mut do_close_conic = true;
                        let mut v_control = points[cur];
                        while cur < last {
                            cur += 1;
                            let point = points[cur];
                            tag = tags[cur] & TAG_MASK;
                            if tag == ON {
                                let c = conv(v_control, s);
                                let p = conv(point, s);
                                sink.quad_to(c.0, c.1, p.0, p.1);
                                count += 1;
                                last_was_close = false;
                                do_close_conic = false;
                                break;
                            }
                            if tag != CONIC {
                                return false;
                            }
                            let v_middle = Point::new(
                                (v_control.x + point.x) / 2,
                                (v_control.y + point.y) / 2,
                            );
                            let c = conv(v_control, s);
                            let p = conv(v_middle, s);
                            sink.quad_to(c.0, c.1, p.0, p.1);
                            count += 1;
                            last_was_close = false;
                            v_control = point;
                        }
                        if do_close_conic {
                            let c = conv(v_control, s);
                            let p = conv(v_start, s);
                            sink.quad_to(c.0, c.1, p.0, p.1);
                            count += 1;
                            last_was_close = false;
                            //                        do_close = false;
                            break;
                        }
                        continue;
                    }
                    _ => {
                        if cur + 1 > last || (tags[cur + 1] & TAG_MASK != CUBIC) {
                            return false;
                        }
                        let c0 = conv(points[cur], s);
                        let c1 = conv(points[cur + 1], s);
                        cur += 2;
                        if cur <= last {
                            let p = conv(points[cur], s);
                            sink.curve_to(c0.0, c0.1, c1.0, c1.1, p.0, p.1);
                            count += 1;
                            last_was_close = false;
                            continue;
                        }
                        let p = conv(v_start, s);
                        sink.curve_to(c0.0, c0.1, c1.0, c1.1, p.0, p.1);
                        count += 1;
                        last_was_close = false;
                        // do_close = false;
                        break;
                    }
                }
            }
            if count > 0 && !last_was_close {
                sink.close();
                last_was_close = true;
            }
        }
        true
    }
}
