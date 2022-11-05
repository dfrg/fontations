fn main() {
    use glyph_load::*;

    let font_path = "c:/work\\content\\fonts\\noto\\Arimo\\Arimo-Bold.ttf";
    let font_path = "c:/work\\content\\fonts\\noto\\NotoMusic\\NotoMusic-Regular.ttf";
    let gid = 105;

    let font_path =
        "c:/work\\content\\fonts\\noto\\NotoSerifMyanmar\\NotoSerifMyanmar-CondensedBlack.ttf";
    let gid = 6;

    let font_path =
        "c:/work\\content\\fonts\\noto\\NotoLoopedThaiUI\\NotoLoopedThaiUI-SemiCondensedThin.ttf";
    let gid = 0;

    let font_path = "c:/work\\content\\fonts\\noto\\Cousine\\Cousine-Bold.ttf";
    let gid = 372;

    let font_path = "c:/work\\content\\fonts\\noto\\NotoRashiHebrew\\NotoRashiHebrew-Regular.ttf";
    let font_path = "c:/work\\content\\fonts\\notofonts\\fonts\\NotoSansTelugu\\hinted\\ttf\\NotoSansTelugu-Black.ttf";
    let gid = 5;

    let font_size = 16.0;

    let file_data = std::fs::read(font_path).unwrap();
    let font_data = read_fonts::FontData::new(&file_data);
    let font = read_fonts::FontRef::new(font_data).unwrap();

    let mut loader = truetype::Loader::new();
    let mut state = truetype::LoaderState::new(
        &font,
        None,
        &[],
        font_size,
        Some(truetype::HinterMode::Subpixel),
    )
    .unwrap();

    let outline = loader.load(&mut state, font_types::GlyphId::new(gid));
    println!("{:?}", &outline);
}
