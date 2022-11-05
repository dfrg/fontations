use super::cache::{Cache, CacheSlot};
use super::{math::*, HinterMode};
use super::{Outline, Point};
use font_types::{F2Dot14, GlyphId};
use read_fonts::tables::{
    glyf::{Anchor, CompositeGlyphFlags, Glyf, Glyph},
    hmtx::Hmtx,
    loca::Loca,
};
use read_fonts::TableProvider;

const DEFAULT_CACHE_MAX_SIZE: usize = 8;
const RECURSE_LIMIT: usize = 32;

/// Loader for TrueType glyphs.
pub struct Loader {
    unscaled: Vec<Point>,
    original: Vec<Point>,
    deltas: Vec<Point>,
    cache: Cache,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            unscaled: vec![],
            original: vec![],
            deltas: vec![],
            cache: Cache::new(DEFAULT_CACHE_MAX_SIZE),
        }
    }
}

impl Loader {
    /// Creates a new TrueType glyph loader.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the specified TrueType glyph using the prepared state.
    pub fn load(&mut self, state: &mut LoaderState, glyph_id: GlyphId) -> Option<Outline> {
        let mut outline = Outline::default();
        if self.load_to(state, glyph_id, &mut outline) {
            Some(outline)
        } else {
            None
        }
    }

    /// Loads the specified TrueType glyph using the prepared state into a
    /// preallocated outline.
    pub fn load_to(
        &mut self,
        state: &mut LoaderState,
        glyph_id: GlyphId,
        outline: &mut Outline,
    ) -> bool {
        outline.points.clear();
        outline.tags.clear();
        outline.contours.clear();
        outline.is_scaled = false;
        self.unscaled.clear();
        self.original.clear();
        self.deltas.clear();
        state.inner.hint_failed = false;
        if glyph_id.to_u16() >= state.inner.data.glyph_count {
            return false;
        }
        if self
            .load_impl(&mut state.inner, glyph_id, outline, 0)
            .is_some()
        {
            let pp0x = state.inner.phantom[0].x;
            if pp0x != 0 {
                for p in &mut outline.points {
                    p.x -= pp0x;
                }
            }
            outline.is_scaled = state.inner.have_scale;
            true
        } else {
            false
        }
    }
}

// Loading
impl Loader {
    fn load_impl(
        &mut self,
        state: &mut LoaderStateInner,
        glyph_id: GlyphId,
        outline: &mut Outline,
        recurse: usize,
    ) -> Option<()> {
        if recurse > RECURSE_LIMIT {
            return None;
        }
        let glyph = state.data.glyph(glyph_id)?;
        let glyph = match glyph {
            Some(glyph) => glyph,
            // This is a valid empty glyph
            None => return Some(()),
        };
        let bounds = [glyph.x_min(), glyph.x_max(), glyph.y_min(), glyph.y_max()];
        self.setup(state, bounds, glyph_id, recurse);
        let point_base = outline.points.len();
        let contour_base = outline.contours.len();
        match glyph {
            Glyph::Simple(simple) => {
                let end_pts = simple.end_pts_of_contours();
                let contour_count = end_pts.len();
                let contour_end = contour_base + contour_count;
                outline
                    .contours
                    .extend(end_pts.iter().map(|end_pt| end_pt.get()));
                let mut point_count = outline.contours.last().copied().unwrap_or(0) as usize + 1;
                for point in simple.points() {
                    outline
                        .points
                        .push(Point::new(point.x as i32, point.y as i32));
                    outline.tags.push(point.on_curve as u8);
                }
                let ins = simple.instructions();
                self.push_phantom(state, outline);
                point_count += 4;
                let point_end = point_base + point_count;
                // TODO: variations
                // if state.vary {
                //     self.unscaled.clear();
                //     self.unscaled.resize(point_count, Point::new(0, 0));
                //     self.original.clear();
                //     self.original.resize(point_count, Point::new(0, 0));
                //     if state.data.deltas(
                //         state.coords,
                //         glyph_id,
                //         &self.scaled[point_base..],
                //         &mut self.tags[point_base..],
                //         &self.contours[contour_base..],
                //         &mut self.unscaled[..],
                //         &mut self.original[..],
                //     ) {
                //         for (d, p) in self.original[..point_count]
                //             .iter()
                //             .zip(self.scaled[point_base..].iter_mut())
                //         {
                //             p.x += d.x;
                //             p.y += d.y;
                //         }
                //     }
                // }
                let hinted = state.hint && !ins.is_empty();
                if hinted {
                    self.unscaled.clear();
                    self.unscaled
                        .extend_from_slice(&outline.points[point_base..]);
                }
                if state.have_scale {
                    let scale = state.scale;
                    for p in &mut outline.points[point_base..] {
                        p.x = mul(p.x, scale);
                        p.y = mul(p.y, scale);
                    }
                    self.save_phantom(state, outline, point_base, point_count);
                }
                if hinted {
                    self.original.clear();
                    self.original
                        .extend_from_slice(&outline.points[point_base..point_end]);
                    for p in &mut outline.points[point_end - 4..] {
                        p.x = round(p.x);
                        p.y = round(p.y);
                    }
                    if !self.hint(state, outline, point_base, contour_base, ins, false) {
                        state.hint_failed = true;
                    }
                }
                if point_base != 0 {
                    for c in &mut outline.contours[contour_base..contour_end] {
                        *c += point_base as u16;
                    }
                }
                outline.points.truncate(outline.points.len() - 4);
                outline.tags.truncate(outline.tags.len() - 4);
            }
            Glyph::Composite(composite) => {
                if state.have_scale {
                    let scale = state.scale;
                    for p in state.phantom.iter_mut() {
                        p.x = mul(p.x, scale);
                        p.y = mul(p.y, scale);
                    }
                }
                // TODO: variations
                // let delta_base = self.deltas.len();
                // let mut have_deltas = false;
                // let count = composite.components().count();
                // self.deltas.resize(delta_base + count, Point::new(0, 0));
                // if state.data.composite_deltas(
                //     state.coords,
                //     glyph_id,
                //     &mut self.deltas[delta_base..],
                // ) {
                //     have_deltas = true;
                // }
                for component in composite.components() {
                    let phantom = state.phantom;
                    let start_point = outline.points.len();
                    self.load_impl(state, component.glyph, outline, recurse + 1)?;
                    let end_point = outline.points.len();
                    if !component
                        .flags
                        .contains(CompositeGlyphFlags::USE_MY_METRICS)
                    {
                        state.phantom = phantom;
                    }
                    fn f2dot14_to_fixed(x: F2Dot14) -> i32 {
                        i16::from_be_bytes(x.to_be_bytes()) as i32 * 4
                    }
                    let xx = f2dot14_to_fixed(component.transform.xx);
                    let yx = f2dot14_to_fixed(component.transform.yx);
                    let xy = f2dot14_to_fixed(component.transform.xy);
                    let yy = f2dot14_to_fixed(component.transform.yy);
                    let have_xform = component.flags.intersects(
                        CompositeGlyphFlags::WE_HAVE_A_SCALE
                            | CompositeGlyphFlags::WE_HAVE_AN_X_AND_Y_SCALE
                            | CompositeGlyphFlags::WE_HAVE_A_TWO_BY_TWO,
                    );
                    if have_xform {
                        for p in &mut outline.points[start_point..end_point] {
                            let (x, y) = transform(p.x, p.y, xx, yx, xy, yy);
                            p.x = x;
                            p.y = y;
                        }
                    }
                    let anchor = component.anchor;
                    let (dx, dy) = match anchor {
                        Anchor::Offset { x, y } => {
                            let (mut dx, mut dy) = (x as i32, y as i32);
                            if have_xform
                                && component.flags
                                    & (CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                                        | CompositeGlyphFlags::UNSCALED_COMPONENT_OFFSET)
                                    == CompositeGlyphFlags::SCALED_COMPONENT_OFFSET
                            {
                                dx = mul(dx, hypot(xx, xy));
                                dy = mul(dy, hypot(yy, yx));
                            }
                            // TODO: variations
                            // if have_deltas {
                            //     let d = self.deltas[delta_base + i];
                            //     dx += d.x;
                            //     dy += d.y;
                            // }
                            if state.have_scale {
                                dx = mul(dx, state.scale);
                                dy = mul(dy, state.scale);
                                if state.hint
                                    && component
                                        .flags
                                        .contains(CompositeGlyphFlags::ROUND_XY_TO_GRID)
                                {
                                    dy = round(dy);
                                }
                            }
                            (dx, dy)
                        }
                        Anchor::Point { base, component } => {
                            let (a1, a2) = (base as usize, component as usize);
                            let pi1 = point_base + a1;
                            let pi2 = start_point + a2;
                            if pi1 >= outline.points.len() || pi2 >= outline.points.len() {
                                println!(
                                    "a1: {a1}, a2: {a2}, pi1: {pi1}, pi2: {pi2}, len: {}",
                                    outline.points.len()
                                );
                                return None;
                            }
                            let p1 = outline.points.get(pi1)?;
                            let p2 = outline.points.get(pi2)?;
                            (p1.x.wrapping_sub(p2.x), p1.y.wrapping_sub(p2.y))
                        }
                    };
                    if dx != 0 || dy != 0 {
                        // println!("outline: {:?}", outline);
                        // println!(">>>>>>>>>>>>>>>>>>>>>>>>>>   anchor offset: {:?}", (dx, dy));
                        for p in &mut outline.points[start_point..end_point] {
                            p.x += dx;
                            p.y += dy;
                        }
                        // println!("outline: {:?}", outline);
                    }
                }
                if state.hint {
                    let ins = composite.instructions().unwrap_or_default();
                    // TODO: variations
                    // self.deltas.resize(delta_base, Point::new(0, 0));
                    if !ins.is_empty() {
                        self.push_phantom(state, outline);
                        self.unscaled.clear();
                        self.unscaled
                            .extend_from_slice(&outline.points[point_base..]);
                        self.original.clear();
                        self.original
                            .extend_from_slice(&outline.points[point_base..]);
                        let point_end = outline.points.len();
                        for p in &mut outline.points[point_end - 4..] {
                            p.x = round(p.x);
                            p.y = round(p.y);
                        }
                        for t in &mut outline.tags[point_base..] {
                            *t &= !(0x08 | 0x10);
                        }
                        if !self.hint(state, outline, point_base, contour_base, ins, true) {
                            state.hint_failed = true;
                        }
                        outline.points.truncate(outline.points.len() - 4);
                        outline.tags.truncate(outline.tags.len() - 4);
                    }
                }
            }
        }
        Some(())
    }
}

// Hinting
impl Loader {
    fn hint(
        &mut self,
        state: &mut LoaderStateInner,
        outline: &mut Outline,
        point_base: usize,
        contour_base: usize,
        ins: &[u8],
        is_composite: bool,
    ) -> bool {
        let slot = match state.slot {
            Some(slot) => slot,
            None => {
                match self.cache.prepare(
                    state.id,
                    &state.data,
                    state.coords,
                    state.ppem,
                    state.scale,
                    state.hinter_mode.unwrap_or(HinterMode::default()),
                ) {
                    Some(slot) => {
                        state.slot = Some(slot);
                        slot
                    }
                    None => {
                        state.hint = false;
                        return false;
                    }
                }
            }
        };
        self.cache.hint(
            &state.data,
            state.coords,
            slot,
            &mut self.unscaled[..],
            &mut self.original[..],
            &mut outline.points[..],
            &mut outline.tags[..],
            &mut outline.contours[..],
            &mut state.phantom[..],
            point_base,
            contour_base,
            ins,
            is_composite,
        );
        true
    }
}

// Per-component setup.
impl Loader {
    fn setup(
        &mut self,
        state: &mut LoaderStateInner,
        bounds: [i16; 4],
        glyph_id: GlyphId,
        recurse: usize,
    ) {
        let lsb = state.data.lsb(glyph_id);
        let advance = state.data.advance_width(glyph_id) as i32;
        let vadvance = 0;
        let tsb = 0;
        state.phantom[0].x = (bounds[0] - lsb) as i32;
        state.phantom[0].y = 0;
        state.phantom[1].x = state.phantom[0].x + advance as i32;
        state.phantom[1].y = 0;
        state.phantom[2].x = advance as i32 / 2;
        state.phantom[2].y = (bounds[3] + tsb) as i32;
        state.phantom[3].x = advance as i32 / 2;
        state.phantom[3].y = state.phantom[2].y - vadvance;
        if recurse == 0 && state.have_scale {
            state.xmin = mul(bounds[0] as i32, state.scale);
            state.xmax = mul(bounds[2] as i32, state.scale);
            state.lsb = mul(lsb as i32, state.scale);
        }
        state.advance = mul(advance, state.scale);
    }

    fn push_phantom(&mut self, state: &mut LoaderStateInner, outline: &mut Outline) {
        for i in 0..4 {
            outline.points.push(state.phantom[i]);
            outline.tags.push(0);
        }
    }

    fn save_phantom(
        &mut self,
        state: &mut LoaderStateInner,
        outline: &mut Outline,
        point_base: usize,
        point_count: usize,
    ) {
        for i in 0..4 {
            state.phantom[3 - i] = outline.points[point_base + point_count - i - 1];
        }
    }
}

/// Configured state for loading a TrueType glyph from a font.
pub struct LoaderState<'a> {
    inner: LoaderStateInner<'a>,
}

impl<'a> LoaderState<'a> {
    /// Creates a new glyph loader state for the specified font and
    /// configuration settings.
    pub fn new(
        font: &impl TableProvider<'a>,
        font_id: Option<u64>,
        variation_coords: &'a [i16],
        pixels_per_em: f32,
        hinter_mode: Option<HinterMode>,
    ) -> Option<Self> {
        Some(Self {
            inner: LoaderStateInner::new(
                font,
                font_id,
                variation_coords,
                pixels_per_em,
                hinter_mode,
            )?,
        })
    }
}

pub struct LoaderStateInner<'a> {
    pub data: LoaderFont<'a>,
    pub id: Option<u64>,
    pub coords: &'a [i16],
    pub slot: Option<CacheSlot>,
    pub have_scale: bool,
    pub ppem: u16,
    pub scale: i32,
    pub hinter_mode: Option<HinterMode>,
    pub hint: bool,
    pub hint_failed: bool,
    pub vary: bool,
    pub xmin: i32,
    pub xmax: i32,
    pub lsb: i32,
    pub advance: i32,
    pub phantom: [Point; 4],
}

impl<'a> LoaderStateInner<'a> {
    fn new(
        font: &impl TableProvider<'a>,
        id: Option<u64>,
        coords: &'a [i16],
        size: f32,
        hint: Option<HinterMode>,
    ) -> Option<Self> {
        let data = LoaderFont::new(font)?;
        let axis_count = data.axis_count;
        let size = size.abs();
        let ppem = size as u16;
        let upem = data.units_per_em;
        let (have_scale, scale) = if size != 0. && upem != 0 {
            (true, div((size * 64.) as i32, upem as i32))
        } else {
            (false, 0)
        };
        Some(Self {
            data,
            id,
            coords,
            slot: None,
            have_scale,
            ppem,
            scale,
            hinter_mode: hint,
            hint: hint.is_some(),
            hint_failed: false,
            vary: axis_count != 0 && !coords.is_empty(), // && data.gvar.is_some(),
            xmin: 0,
            xmax: 0,
            lsb: 0,
            advance: 0,
            phantom: Default::default(),
        })
    }
}

/// Contains the tables and limits necessary for loading, scaling and hinting
/// a TrueType glyph.
#[derive(Clone)]
pub struct LoaderFont<'a> {
    pub glyf: Glyf<'a>,
    pub loca: Loca<'a>,
    pub hmtx: Hmtx<'a>,
    pub fpgm: &'a [u8],
    pub prep: &'a [u8],
    pub cvt: &'a [font_types::BigEndian<i16>],
    pub units_per_em: u16,
    pub glyph_count: u16,
    pub max_storage: u16,
    pub max_stack: u16,
    pub max_function_defs: u16,
    pub max_instruction_defs: u16,
    pub max_twilight: u16,
    pub axis_count: u16,
}

impl<'a> LoaderFont<'a> {
    fn new(font: &impl TableProvider<'a>) -> Option<Self> {
        use font_types::Tag;
        let glyf = font.glyf().ok()?;
        let loca = font.loca(None).ok()?;
        let hmtx = font.hmtx().ok()?;
        let upem = font.head().ok()?.units_per_em();
        let fpgm = font
            .data_for_tag(Tag::new(b"fpgm"))
            .map(|data| data.read_array(0..data.len()).unwrap())
            .unwrap_or_default();
        let prep = font
            .data_for_tag(Tag::new(b"prep"))
            .map(|data| data.read_array(0..data.len()).unwrap())
            .unwrap_or_default();
        let cvt = font
            .data_for_tag(Tag::new(b"cvt"))
            .and_then(|data| data.read_array(0..data.len()).ok())
            .unwrap_or_default();
        let maxp = font.maxp().ok()?;
        let glyph_count = maxp.num_glyphs();
        // TODO: variations
        let axis_count = 0;
        Some(Self {
            glyf,
            loca,
            hmtx,
            fpgm,
            prep,
            cvt,
            glyph_count,
            units_per_em: upem,
            max_storage: maxp.max_storage().unwrap_or(0),
            max_stack: maxp.max_stack_elements().unwrap_or(0),
            max_function_defs: maxp.max_function_defs().unwrap_or(0),
            max_instruction_defs: maxp.max_instruction_defs().unwrap_or(0),
            max_twilight: maxp.max_twilight_points().unwrap_or(0),
            axis_count,
        })
    }

    fn glyph(&self, gid: GlyphId) -> Option<Option<Glyph<'a>>> {
        self.loca.get_glyf(gid, &self.glyf).ok()
    }

    fn advance_width(&self, gid: GlyphId) -> u16 {
        let default_advance = self
            .hmtx
            .h_metrics()
            .last()
            .map(|metric| metric.advance_width())
            .unwrap_or(0);
        self.hmtx
            .h_metrics()
            .get(gid.to_u16() as usize)
            .map(|metric| metric.advance_width())
            .unwrap_or(default_advance)
    }

    fn lsb(&self, gid: GlyphId) -> i16 {
        let gid_index = gid.to_u16() as usize;
        self.hmtx
            .h_metrics()
            .get(gid_index)
            .map(|metric| metric.lsb())
            .unwrap_or_else(|| {
                self.hmtx
                    .left_side_bearings()
                    .get(gid_index.saturating_sub(self.hmtx.h_metrics().len()))
                    .map(|lsb| lsb.get())
                    .unwrap_or(0)
            })
    }

    pub(crate) fn scale_cvt(&self, scale: Option<i32>, scaled_cvt: &mut Vec<i32>) {
        if scaled_cvt.len() < self.cvt.len() {
            scaled_cvt.resize(self.cvt.len(), 0);
        }
        for (src, dest) in self.cvt.iter().zip(scaled_cvt.iter_mut()) {
            *dest = src.get() as i32 * 64;
        }
        if let Some(scale) = scale {
            let scale = scale >> 6;
            for value in &mut scaled_cvt[..] {
                *value = mul(*value, scale);
            }
        }
    }
}
