use font_types::GlyphId;
use freetype::face::LoadFlag;
use glyph_load::{read_fonts, truetype};
use std::path::PathBuf;
use std::time::Duration;

fn benchmark() {
    let font_paths = glob::glob("c:/work/content/fonts/notofonts/fonts/**/hinted/ttf/*.ttf")
        .unwrap()
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>();

    let mut unhinted = vec![];
    let mut hinted = vec![];

    let font_size = 16u32;

    let ft = freetype::Library::init().unwrap();

    let mut loader = truetype::Loader::new();
    let mut outline = truetype::Outline::default();

    let mut id = 0u64;

    for font_path in &font_paths {
        if let Ok(data) = std::fs::read(&font_path) {
            if let Ok(face) = ft.new_memory_face2(&data[..], 0) {
                let font_data = read_fonts::FontData::new(&data);
                let font = read_fonts::FontRef::new(font_data).unwrap();
                let glyph_count = face.num_glyphs() as u16;

                id += 1;

                // Unhinted

                let mut start = std::time::Instant::now();
                face.set_pixel_sizes(font_size, font_size);
                for gid in 0..glyph_count {
                    face.load_glyph(gid as _, LoadFlag::NO_HINTING | LoadFlag::NO_BITMAP);
                }
                let freetype_time = start.elapsed();

                let mut start = std::time::Instant::now();
                let mut loader_state =
                    truetype::LoaderState::new(&font, Some(id), &[], font_size as _, None).unwrap();
                for gid in 0..glyph_count {
                    loader.load_to(&mut loader_state, GlyphId::new(gid), &mut outline);
                }
                let oxidize_time = start.elapsed();

                unhinted.push(BenchmarkItem {
                    path: font_path.clone(),
                    freetype: freetype_time,
                    oxidize: oxidize_time,
                });

                // Hinted

                let mut start = std::time::Instant::now();
                face.set_pixel_sizes(font_size, font_size);
                for gid in 0..glyph_count {
                    face.load_glyph(
                        gid as _,
                        LoadFlag::NO_AUTOHINT | LoadFlag::TARGET_LCD | LoadFlag::NO_BITMAP,
                    );
                }
                let freetype_time = start.elapsed();

                let mut start = std::time::Instant::now();
                let mut loader_state = truetype::LoaderState::new(
                    &font,
                    Some(id),
                    &[],
                    font_size as _,
                    Some(truetype::HinterMode::Subpixel),
                )
                .unwrap();
                for gid in 0..glyph_count {
                    loader.load_to(&mut loader_state, GlyphId::new(gid), &mut outline);
                }
                let oxidize_time = start.elapsed();

                hinted.push(BenchmarkItem {
                    path: font_path.clone(),
                    freetype: freetype_time,
                    oxidize: oxidize_time,
                });
            }
        }
    }

    fn print_benchmark_items(items: &[BenchmarkItem]) {
        for item in items {
            let name = item.path.file_name().unwrap().to_string_lossy();
            let ox = item.oxidize;
            let ft = item.freetype;
            let ratio = ox.as_secs_f64() / ft.as_secs_f64();
            let ox_str = format!("{}", ox.as_secs_f64() * 1000.);
            let ft_str = format!("{}", ft.as_secs_f64() * 1000.);
            let ratio_str = format!("{:.3}", ratio);
            println!("{},{},{},{}", name, ox_str, ft_str, ratio_str);
        }
    }

    println!("[Unhinted]");
    // println!("{:<80} {:10} {:10} {}", "", "oxidize", "freetype", "oxidize/freetype");
    print_benchmark_items(&unhinted);

    println!("[Hinted]");
    // println!("{:<80} {:10} {:10} {}", "", "oxidize", "freetype", "oxidize/freetype");
    print_benchmark_items(&hinted);
}

struct BenchmarkItem {
    pub path: PathBuf,
    pub freetype: Duration,
    pub oxidize: Duration,
}

fn main() {
    benchmark();
    return;

    let font_paths = glob::glob("c:/work/content/fonts/notofonts/fonts/**/hinted/ttf/*.ttf")
        .unwrap()
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>();

    // let font_paths = ["c:/work\\content\\fonts\\notofonts\\fonts\\NotoLoopedThai\\hinted\\ttf\\NotoLoopedThai-Black.ttf"];

    // let font_paths = glob::glob("c:/windows/fonts/corbel.ttf")
    // .unwrap()
    // .filter_map(|x| x.ok())
    // .collect::<Vec<_>>();

    let ftlib = freetype::Library::init().unwrap();
    let mut loader = glyph_load::truetype::Loader::new();
    let mut outline = glyph_load::truetype::Outline::default();
    let mut ft_outline = glyph_load::truetype::Outline::default();

    let font_size = 16u32;
    let hint = true;

    // let font_size = 0u32;
    // let hint = false;

    let load_flags = if hint {
        LoadFlag::NO_AUTOHINT | LoadFlag::TARGET_LCD | LoadFlag::NO_BITMAP | LoadFlag::PEDANTIC
    } else {
        LoadFlag::NO_AUTOHINT | LoadFlag::NO_HINTING | LoadFlag::NO_BITMAP
    };

    let hinter_mode = if hint {
        Some(truetype::HinterMode::Subpixel)
    } else {
        None
    };

    'outer: for font_path in &font_paths {
        if let Ok(data) = std::fs::read(&font_path) {
            if let Ok(face) = ftlib.new_memory_face2(&data[..], 0) {
                println!("[{:?}]", &font_path);
                let font_data = glyph_load::read_fonts::FontData::new(&data);
                let font = glyph_load::read_fonts::FontRef::new(font_data).unwrap();

                let mut load_flags = load_flags;
                let glyph_count = face.num_glyphs() as u16;
                if font_size != 0 {
                    face.set_pixel_sizes(font_size, font_size);
                } else {
                    load_flags |= freetype::face::LoadFlag::NO_SCALE
                }

                for gid in 0..glyph_count {
                    let mut hinter_mode = hinter_mode;

                    // println!("gid: {}", gid);
                    if (face.load_glyph(gid as _, load_flags)).is_err() {
                        face.load_glyph(
                            gid as _,
                            LoadFlag::NO_AUTOHINT | LoadFlag::NO_HINTING | LoadFlag::NO_BITMAP,
                        );
                        hinter_mode = None;
                    }
                    let mut state = glyph_load::truetype::LoaderState::new(
                        &font,
                        None,
                        &[],
                        font_size as _,
                        hinter_mode,
                    )
                    .unwrap();
                    loader.load_to(&mut state, font_types::GlyphId::new(gid), &mut outline);
                    for tag in &mut outline.tags {
                        *tag = *tag & 1;
                    }
                    copy_outline(
                        &face.glyph().outline().unwrap(),
                        &mut ft_outline,
                        outline.is_scaled,
                    );
                    if outline != ft_outline {
                        println!(
                            "FAILED: {:?}, gid: {}, lens: ({}, {})",
                            font_path,
                            gid,
                            ft_outline.points.len(),
                            outline.points.len()
                        );
                        println!("{:?}", ft_outline);
                        println!("{:?}", outline);
                        continue 'outer;
                    }
                }
            }
        }
    }
}

fn cmp_outline(a: &glyph_load::truetype::Outline, b: &freetype::outline::Outline) -> bool {
    if a.points.len() != b.points().len()
        || a.tags.len() != b.tags().len()
        || a.contours.len() != b.contours().len()
    {
        return false;
    }
    let b_points = b.points();
    let b_tags = b.tags();
    let b_contours = b.contours();

    for (i, (x, y)) in a.points.iter().zip(b.points()).enumerate() {
        if x.x != y.x as _ {
            return false;
        }
        if x.y != y.y as _ {
            return false;
        }
    }
    true
}

fn copy_outline(
    from: &freetype::outline::Outline,
    to: &mut glyph_load::truetype::Outline,
    is_scaled: bool,
) {
    use glyph_load::truetype::Point;
    to.clear();
    to.points
        .extend(from.points().iter().map(|p| Point::new(p.x as _, p.y as _)));
    to.tags.extend(from.tags().iter().map(|x| *x as u8 & 1));
    to.contours
        .extend(from.contours().iter().map(|x| *x as u16));
    to.is_scaled = is_scaled;
}
