// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::compile_prelude::*;

/// [Class Definition Table Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/chapter2#class-definition-table-format-1)
/// [GPOS Version 1.0](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#gpos-header)
#[derive(Clone, Debug)]
pub struct Gpos {
    /// Offset to ScriptList table, from beginning of GPOS table
    pub script_list_offset: OffsetMarker<Offset16, ScriptList>,
    /// Offset to FeatureList table, from beginning of GPOS table
    pub feature_list_offset: OffsetMarker<Offset16, FeatureList>,
    /// Offset to LookupList table, from beginning of GPOS table
    pub lookup_list_offset: OffsetMarker<Offset16, PositionLookupList>,
    pub feature_variations_offset: NullableOffsetMarker<Offset32, FeatureVariations>,
}

impl FontWrite for Gpos {
    fn write_into(&self, writer: &mut TableWriter) {
        (self.compute_version()).write_into(writer);
        self.script_list_offset.write_into(writer);
        self.feature_list_offset.write_into(writer);
        self.lookup_list_offset.write_into(writer);
        self.feature_variations_offset.write_into(writer);
    }
}

impl Validate for Gpos {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Gpos", |ctx| {
            ctx.in_field("script_list_offset", |ctx| {
                self.script_list_offset.validate_impl(ctx);
            });
            ctx.in_field("feature_list_offset", |ctx| {
                self.feature_list_offset.validate_impl(ctx);
            });
            ctx.in_field("lookup_list_offset", |ctx| {
                self.lookup_list_offset.validate_impl(ctx);
            });
            ctx.in_field("feature_variations_offset", |ctx| {
                self.feature_variations_offset.validate_impl(ctx);
            });
        })
    }
}

bitflags::bitflags! { # [doc = " See [ValueRecord]"] pub struct ValueFormat : u16 { # [doc = " Includes horizontal adjustment for placement"] const X_PLACEMENT = 0x0001 ; # [doc = " Includes vertical adjustment for placement"] const Y_PLACEMENT = 0x0002 ; # [doc = " Includes horizontal adjustment for advance"] const X_ADVANCE = 0x0004 ; # [doc = " Includes vertical adjustment for advance"] const Y_ADVANCE = 0x0008 ; # [doc = " Includes Device table (non-variable font) / VariationIndex"] # [doc = " table (variable font) for horizontal placement"] const X_PLACEMENT_DEVICE = 0x0010 ; # [doc = " Includes Device table (non-variable font) / VariationIndex"] # [doc = " table (variable font) for vertical placement"] const Y_PLACEMENT_DEVICE = 0x0020 ; # [doc = " Includes Device table (non-variable font) / VariationIndex"] # [doc = " table (variable font) for horizontal advance"] const X_ADVANCE_DEVICE = 0x0040 ; # [doc = " Includes Device table (non-variable font) / VariationIndex"] # [doc = " table (variable font) for vertical advance"] const Y_ADVANCE_DEVICE = 0x0080 ; } }

impl FontWrite for ValueFormat {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}

/// [Anchor Tables](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-tables)
/// position one glyph with respect to another.
#[derive(Clone, Debug)]
pub enum AnchorTable {
    Format1(AnchorFormat1),
    Format2(AnchorFormat2),
    Format3(AnchorFormat3),
}

impl FontWrite for AnchorTable {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
            Self::Format3(item) => item.write_into(writer),
        }
    }
}

impl Validate for AnchorTable {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
            Self::Format3(item) => item.validate_impl(ctx),
        }
    }
}

/// [Anchor Table Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-table-format-1-design-units): Design Units
#[derive(Clone, Debug)]
pub struct AnchorFormat1 {
    /// Horizontal value, in design units
    pub x_coordinate: i16,
    /// Vertical value, in design units
    pub y_coordinate: i16,
}

impl FontWrite for AnchorFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
    }
}

impl Validate for AnchorFormat1 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// [Anchor Table Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-table-format-2-design-units-plus-contour-point): Design Units Plus Contour Point
#[derive(Clone, Debug)]
pub struct AnchorFormat2 {
    /// Horizontal value, in design units
    pub x_coordinate: i16,
    /// Vertical value, in design units
    pub y_coordinate: i16,
    /// Index to glyph contour point
    pub anchor_point: u16,
}

impl FontWrite for AnchorFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
        self.anchor_point.write_into(writer);
    }
}

impl Validate for AnchorFormat2 {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// [Anchor Table Format 3](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#anchor-table-format-3-design-units-plus-device-or-variationindex-tables): Design Units Plus Device or VariationIndex Tables
#[derive(Clone, Debug)]
pub struct AnchorFormat3 {
    /// Horizontal value, in design units
    pub x_coordinate: i16,
    /// Vertical value, in design units
    pub y_coordinate: i16,
    /// Offset to Device table (non-variable font) / VariationIndex
    /// table (variable font) for X coordinate, from beginning of
    /// Anchor table (may be NULL)
    pub x_device_offset: NullableOffsetMarker<Offset16, Device>,
    /// Offset to Device table (non-variable font) / VariationIndex
    /// table (variable font) for Y coordinate, from beginning of
    /// Anchor table (may be NULL)
    pub y_device_offset: NullableOffsetMarker<Offset16, Device>,
}

impl FontWrite for AnchorFormat3 {
    fn write_into(&self, writer: &mut TableWriter) {
        (3 as u16).write_into(writer);
        self.x_coordinate.write_into(writer);
        self.y_coordinate.write_into(writer);
        self.x_device_offset.write_into(writer);
        self.y_device_offset.write_into(writer);
    }
}

impl Validate for AnchorFormat3 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("AnchorFormat3", |ctx| {
            ctx.in_field("x_device_offset", |ctx| {
                self.x_device_offset.validate_impl(ctx);
            });
            ctx.in_field("y_device_offset", |ctx| {
                self.y_device_offset.validate_impl(ctx);
            });
        })
    }
}

/// [Mark Array Table](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-array-table)
#[derive(Clone, Debug)]
pub struct MarkArray {
    /// Array of MarkRecords, ordered by corresponding glyphs in the
    /// associated mark Coverage table.
    pub mark_records: Vec<MarkRecord>,
}

impl FontWrite for MarkArray {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.mark_records)).unwrap().write_into(writer);
        self.mark_records.write_into(writer);
    }
}

impl Validate for MarkArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkArray", |ctx| {
            ctx.in_field("mark_records", |ctx| {
                if self.mark_records.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.mark_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkArray]
#[derive(Clone, Debug)]
pub struct MarkRecord {
    /// Class defined for the associated mark.
    pub mark_class: u16,
    /// Offset to Anchor table, from beginning of MarkArray table.
    pub mark_anchor_offset: OffsetMarker<Offset16, AnchorTable>,
}

impl FontWrite for MarkRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.mark_class.write_into(writer);
        self.mark_anchor_offset.write_into(writer);
    }
}

impl Validate for MarkRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkRecord", |ctx| {
            ctx.in_field("mark_anchor_offset", |ctx| {
                self.mark_anchor_offset.validate_impl(ctx);
            });
        })
    }
}

/// [Lookup Type 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-1-single-adjustment-positioning-subtable): Single Adjustment Positioning Subtable
#[derive(Clone, Debug)]
pub enum SinglePos {
    Format1(SinglePosFormat1),
    Format2(SinglePosFormat2),
}

impl FontWrite for SinglePos {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
}

impl Validate for SinglePos {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
        }
    }
}

/// [Single Adjustment Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#single-adjustment-positioning-format-1-single-positioning-value): Single Positioning Value
#[derive(Clone, Debug)]
pub struct SinglePosFormat1 {
    /// Offset to Coverage table, from beginning of SinglePos subtable.
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Defines positioning value(s) — applied to all glyphs in the
    /// Coverage table.
    pub value_record: ValueRecord,
}

impl FontWrite for SinglePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage_offset.write_into(writer);
        (self.compute_value_format()).write_into(writer);
        self.value_record.write_into(writer);
    }
}

impl Validate for SinglePosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SinglePosFormat1", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
        })
    }
}

/// [Single Adjustment Positioning Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#single-adjustment-positioning-format-2-array-of-positioning-values): Array of Positioning Values
#[derive(Clone, Debug)]
pub struct SinglePosFormat2 {
    /// Offset to Coverage table, from beginning of SinglePos subtable.
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Array of ValueRecords — positioning values applied to glyphs.
    pub value_records: Vec<ValueRecord>,
}

impl FontWrite for SinglePosFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.coverage_offset.write_into(writer);
        (self.compute_value_format()).write_into(writer);
        (array_len(&self.value_records)).unwrap().write_into(writer);
        self.value_records.write_into(writer);
    }
}

impl Validate for SinglePosFormat2 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SinglePosFormat2", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
        })
    }
}

/// [Lookup Type 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#lookup-type-1-single-adjustment-positioning-subtable): Single Adjustment Positioning Subtable
#[derive(Clone, Debug)]
pub enum PairPos {
    Format1(PairPosFormat1),
    Format2(PairPosFormat2),
}

impl FontWrite for PairPos {
    fn write_into(&self, writer: &mut TableWriter) {
        match self {
            Self::Format1(item) => item.write_into(writer),
            Self::Format2(item) => item.write_into(writer),
        }
    }
}

impl Validate for PairPos {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        match self {
            Self::Format1(item) => item.validate_impl(ctx),
            Self::Format2(item) => item.validate_impl(ctx),
        }
    }
}

/// [Pair Adjustment Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#pair-adjustment-positioning-format-1-adjustments-for-glyph-pairs): Adjustments for Glyph Pairs
#[derive(Clone, Debug)]
pub struct PairPosFormat1 {
    /// Offset to Coverage table, from beginning of PairPos subtable.
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Array of offsets to PairSet tables. Offsets are from beginning
    /// of PairPos subtable, ordered by Coverage Index.
    pub pair_set_offsets: Vec<OffsetMarker<Offset16, PairSet>>,
}

impl FontWrite for PairPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage_offset.write_into(writer);
        (self.compute_value_format1()).write_into(writer);
        (self.compute_value_format2()).write_into(writer);
        (array_len(&self.pair_set_offsets))
            .unwrap()
            .write_into(writer);
        self.pair_set_offsets.write_into(writer);
    }
}

impl Validate for PairPosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PairPosFormat1", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("pair_set_offsets", |ctx| {
                if self.pair_set_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.pair_set_offsets.validate_impl(ctx);
            });
        })
    }
}

/// Part of [PairPosFormat1]
#[derive(Clone, Debug)]
pub struct PairSet {
    /// Array of PairValueRecords, ordered by glyph ID of the second
    /// glyph.
    pub pair_value_records: Vec<PairValueRecord>,
}

impl FontWrite for PairSet {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.pair_value_records))
            .unwrap()
            .write_into(writer);
        self.pair_value_records.write_into(writer);
    }
}

impl Validate for PairSet {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// Part of [PairSet]
#[derive(Clone, Debug)]
pub struct PairValueRecord {
    /// Glyph ID of second glyph in the pair (first glyph is listed in
    /// the Coverage table).
    pub second_glyph: u16,
    /// Positioning data for the first glyph in the pair.
    pub value_record1: ValueRecord,
    /// Positioning data for the second glyph in the pair.
    pub value_record2: ValueRecord,
}

impl FontWrite for PairValueRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.second_glyph.write_into(writer);
        self.value_record1.write_into(writer);
        self.value_record2.write_into(writer);
    }
}

impl Validate for PairValueRecord {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// [Pair Adjustment Positioning Format 2](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#pair-adjustment-positioning-format-2-class-pair-adjustment): Class Pair Adjustment
#[derive(Clone, Debug)]
pub struct PairPosFormat2 {
    /// Offset to Coverage table, from beginning of PairPos subtable.
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to ClassDef table, from beginning of PairPos subtable
    /// — for the first glyph of the pair.
    pub class_def1_offset: OffsetMarker<Offset16, ClassDef>,
    /// Offset to ClassDef table, from beginning of PairPos subtable
    /// — for the second glyph of the pair.
    pub class_def2_offset: OffsetMarker<Offset16, ClassDef>,
    pub class1_records: Vec<Class1Record>,
}

impl FontWrite for PairPosFormat2 {
    fn write_into(&self, writer: &mut TableWriter) {
        (2 as u16).write_into(writer);
        self.coverage_offset.write_into(writer);
        (self.compute_value_format1()).write_into(writer);
        (self.compute_value_format2()).write_into(writer);
        self.class_def1_offset.write_into(writer);
        self.class_def2_offset.write_into(writer);
        (self.compute_class1_count()).write_into(writer);
        (self.compute_class2_count()).write_into(writer);
        self.class1_records.write_into(writer);
    }
}

impl Validate for PairPosFormat2 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("PairPosFormat2", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("class_def1_offset", |ctx| {
                self.class_def1_offset.validate_impl(ctx);
            });
            ctx.in_field("class_def2_offset", |ctx| {
                self.class_def2_offset.validate_impl(ctx);
            });
            ctx.in_field("class1_records", |ctx| {
                self.class1_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [PairPosFormat2]
#[derive(Clone, Debug)]
pub struct Class1Record {
    /// Array of Class2 records, ordered by classes in classDef2.
    pub class2_records: Vec<Class2Record>,
}

impl FontWrite for Class1Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.class2_records.write_into(writer);
    }
}

impl Validate for Class1Record {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Class1Record", |ctx| {
            ctx.in_field("class2_records", |ctx| {
                self.class2_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [PairPosFormat2]
#[derive(Clone, Debug)]
pub struct Class2Record {
    /// Positioning for first glyph — empty if valueFormat1 = 0.
    pub value_record1: ValueRecord,
    /// Positioning for second glyph — empty if valueFormat2 = 0.
    pub value_record2: ValueRecord,
}

impl FontWrite for Class2Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.value_record1.write_into(writer);
        self.value_record2.write_into(writer);
    }
}

impl Validate for Class2Record {
    fn validate_impl(&self, _ctx: &mut ValidationCtx) {}
}

/// [Cursive Attachment Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#cursive-attachment-positioning-format1-cursive-attachment): Cursvie attachment
#[derive(Clone, Debug)]
pub struct CursivePosFormat1 {
    /// Offset to Coverage table, from beginning of CursivePos subtable.
    pub coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Array of EntryExit records, in Coverage index order.
    pub entry_exit_record: Vec<EntryExitRecord>,
}

impl FontWrite for CursivePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.coverage_offset.write_into(writer);
        (array_len(&self.entry_exit_record))
            .unwrap()
            .write_into(writer);
        self.entry_exit_record.write_into(writer);
    }
}

impl Validate for CursivePosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("CursivePosFormat1", |ctx| {
            ctx.in_field("coverage_offset", |ctx| {
                self.coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("entry_exit_record", |ctx| {
                if self.entry_exit_record.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.entry_exit_record.validate_impl(ctx);
            });
        })
    }
}

/// Part of [CursivePosFormat1]
#[derive(Clone, Debug)]
pub struct EntryExitRecord {
    /// Offset to entryAnchor table, from beginning of CursivePos
    /// subtable (may be NULL).
    pub entry_anchor_offset: NullableOffsetMarker<Offset16, AnchorTable>,
    /// Offset to exitAnchor table, from beginning of CursivePos
    /// subtable (may be NULL).
    pub exit_anchor_offset: NullableOffsetMarker<Offset16, AnchorTable>,
}

impl FontWrite for EntryExitRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.entry_anchor_offset.write_into(writer);
        self.exit_anchor_offset.write_into(writer);
    }
}

impl Validate for EntryExitRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("EntryExitRecord", |ctx| {
            ctx.in_field("entry_anchor_offset", |ctx| {
                self.entry_anchor_offset.validate_impl(ctx);
            });
            ctx.in_field("exit_anchor_offset", |ctx| {
                self.exit_anchor_offset.validate_impl(ctx);
            });
        })
    }
}

/// [Mark-to-Base Attachment Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-to-base-attachment-positioning-format-1-mark-to-base-attachment-point): Mark-to-base Attachment Point
#[derive(Clone, Debug)]
pub struct MarkBasePosFormat1 {
    /// Offset to markCoverage table, from beginning of MarkBasePos
    /// subtable.
    pub mark_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to baseCoverage table, from beginning of MarkBasePos
    /// subtable.
    pub base_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to MarkArray table, from beginning of MarkBasePos
    /// subtable.
    pub mark_array_offset: OffsetMarker<Offset16, MarkArray>,
    /// Offset to BaseArray table, from beginning of MarkBasePos
    /// subtable.
    pub base_array_offset: OffsetMarker<Offset16, BaseArray>,
}

impl FontWrite for MarkBasePosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.mark_coverage_offset.write_into(writer);
        self.base_coverage_offset.write_into(writer);
        (self.compute_mark_class_count()).write_into(writer);
        self.mark_array_offset.write_into(writer);
        self.base_array_offset.write_into(writer);
    }
}

impl Validate for MarkBasePosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkBasePosFormat1", |ctx| {
            ctx.in_field("mark_coverage_offset", |ctx| {
                self.mark_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("base_coverage_offset", |ctx| {
                self.base_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("mark_array_offset", |ctx| {
                self.mark_array_offset.validate_impl(ctx);
            });
            ctx.in_field("base_array_offset", |ctx| {
                self.base_array_offset.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkBasePosFormat1]
#[derive(Clone, Debug)]
pub struct BaseArray {
    /// Array of BaseRecords, in order of baseCoverage Index.
    pub base_records: Vec<BaseRecord>,
}

impl FontWrite for BaseArray {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.base_records)).unwrap().write_into(writer);
        self.base_records.write_into(writer);
    }
}

impl Validate for BaseArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseArray", |ctx| {
            ctx.in_field("base_records", |ctx| {
                self.base_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [BaseArray]
#[derive(Clone, Debug)]
pub struct BaseRecord {
    /// Array of offsets (one per mark class) to Anchor tables. Offsets
    /// are from beginning of BaseArray table, ordered by class
    /// (offsets may be NULL).
    pub base_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl FontWrite for BaseRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.base_anchor_offsets.write_into(writer);
    }
}

impl Validate for BaseRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("BaseRecord", |ctx| {
            ctx.in_field("base_anchor_offsets", |ctx| {
                self.base_anchor_offsets.validate_impl(ctx);
            });
        })
    }
}

/// [Mark-to-Ligature Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-to-ligature-attachment-positioning-format-1-mark-to-ligature-attachment): Mark-to-Ligature Attachment
#[derive(Clone, Debug)]
pub struct MarkLigPosFormat1 {
    /// Offset to markCoverage table, from beginning of MarkLigPos
    /// subtable.
    pub mark_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to ligatureCoverage table, from beginning of MarkLigPos
    /// subtable.
    pub ligature_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to MarkArray table, from beginning of MarkLigPos
    /// subtable.
    pub mark_array_offset: OffsetMarker<Offset16, MarkArray>,
    /// Offset to LigatureArray table, from beginning of MarkLigPos
    /// subtable.
    pub ligature_array_offset: OffsetMarker<Offset16, LigatureArray>,
}

impl FontWrite for MarkLigPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.mark_coverage_offset.write_into(writer);
        self.ligature_coverage_offset.write_into(writer);
        (self.compute_mark_class_count()).write_into(writer);
        self.mark_array_offset.write_into(writer);
        self.ligature_array_offset.write_into(writer);
    }
}

impl Validate for MarkLigPosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkLigPosFormat1", |ctx| {
            ctx.in_field("mark_coverage_offset", |ctx| {
                self.mark_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("ligature_coverage_offset", |ctx| {
                self.ligature_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("mark_array_offset", |ctx| {
                self.mark_array_offset.validate_impl(ctx);
            });
            ctx.in_field("ligature_array_offset", |ctx| {
                self.ligature_array_offset.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkLigPosFormat1]
#[derive(Clone, Debug)]
pub struct LigatureArray {
    /// Array of offsets to LigatureAttach tables. Offsets are from
    /// beginning of LigatureArray table, ordered by ligatureCoverage
    /// index.
    pub ligature_attach_offsets: Vec<OffsetMarker<Offset16, LigatureAttach>>,
}

impl FontWrite for LigatureArray {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.ligature_attach_offsets))
            .unwrap()
            .write_into(writer);
        self.ligature_attach_offsets.write_into(writer);
    }
}

impl Validate for LigatureArray {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigatureArray", |ctx| {
            ctx.in_field("ligature_attach_offsets", |ctx| {
                if self.ligature_attach_offsets.len() > (u16::MAX as usize) {
                    ctx.report("array excedes max length");
                }
                self.ligature_attach_offsets.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkLigPosFormat1]
#[derive(Clone, Debug)]
pub struct LigatureAttach {
    /// Array of Component records, ordered in writing direction.
    pub component_records: Vec<ComponentRecord>,
}

impl FontWrite for LigatureAttach {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.component_records))
            .unwrap()
            .write_into(writer);
        self.component_records.write_into(writer);
    }
}

impl Validate for LigatureAttach {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("LigatureAttach", |ctx| {
            ctx.in_field("component_records", |ctx| {
                self.component_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkLigPosFormat1]
#[derive(Clone, Debug)]
pub struct ComponentRecord {
    /// Array of offsets (one per class) to Anchor tables. Offsets are
    /// from beginning of LigatureAttach table, ordered by class
    /// (offsets may be NULL).
    pub ligature_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl FontWrite for ComponentRecord {
    fn write_into(&self, writer: &mut TableWriter) {
        self.ligature_anchor_offsets.write_into(writer);
    }
}

impl Validate for ComponentRecord {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("ComponentRecord", |ctx| {
            ctx.in_field("ligature_anchor_offsets", |ctx| {
                self.ligature_anchor_offsets.validate_impl(ctx);
            });
        })
    }
}

/// [Mark-to-Mark Attachment Positioning Format 1](https://docs.microsoft.com/en-us/typography/opentype/spec/gpos#mark-to-mark-attachment-positioning-format-1-mark-to-mark-attachment): Mark-to-Mark Attachment
#[derive(Clone, Debug)]
pub struct MarkMarkPosFormat1 {
    /// Offset to Combining Mark Coverage table, from beginning of
    /// MarkMarkPos subtable.
    pub mark1_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to Base Mark Coverage table, from beginning of
    /// MarkMarkPos subtable.
    pub mark2_coverage_offset: OffsetMarker<Offset16, CoverageTable>,
    /// Offset to MarkArray table for mark1, from beginning of
    /// MarkMarkPos subtable.
    pub mark1_array_offset: OffsetMarker<Offset16, MarkArray>,
    /// Offset to Mark2Array table for mark2, from beginning of
    /// MarkMarkPos subtable.
    pub mark2_array_offset: OffsetMarker<Offset16, Mark2Array>,
}

impl FontWrite for MarkMarkPosFormat1 {
    fn write_into(&self, writer: &mut TableWriter) {
        (1 as u16).write_into(writer);
        self.mark1_coverage_offset.write_into(writer);
        self.mark2_coverage_offset.write_into(writer);
        (self.compute_mark_class_count()).write_into(writer);
        self.mark1_array_offset.write_into(writer);
        self.mark2_array_offset.write_into(writer);
    }
}

impl Validate for MarkMarkPosFormat1 {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("MarkMarkPosFormat1", |ctx| {
            ctx.in_field("mark1_coverage_offset", |ctx| {
                self.mark1_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("mark2_coverage_offset", |ctx| {
                self.mark2_coverage_offset.validate_impl(ctx);
            });
            ctx.in_field("mark1_array_offset", |ctx| {
                self.mark1_array_offset.validate_impl(ctx);
            });
            ctx.in_field("mark2_array_offset", |ctx| {
                self.mark2_array_offset.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkMarkPosFormat1]Class2Record
#[derive(Clone, Debug)]
pub struct Mark2Array {
    /// Array of Mark2Records, in Coverage order.
    pub mark2_records: Vec<Mark2Record>,
}

impl FontWrite for Mark2Array {
    fn write_into(&self, writer: &mut TableWriter) {
        (array_len(&self.mark2_records)).unwrap().write_into(writer);
        self.mark2_records.write_into(writer);
    }
}

impl Validate for Mark2Array {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Mark2Array", |ctx| {
            ctx.in_field("mark2_records", |ctx| {
                self.mark2_records.validate_impl(ctx);
            });
        })
    }
}

/// Part of [MarkMarkPosFormat1]
#[derive(Clone, Debug)]
pub struct Mark2Record {
    /// Array of offsets (one per class) to Anchor tables. Offsets are
    /// from beginning of Mark2Array table, in class order (offsets may
    /// be NULL).
    pub mark2_anchor_offsets: Vec<NullableOffsetMarker<Offset16, AnchorTable>>,
}

impl FontWrite for Mark2Record {
    fn write_into(&self, writer: &mut TableWriter) {
        self.mark2_anchor_offsets.write_into(writer);
    }
}

impl Validate for Mark2Record {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Mark2Record", |ctx| {
            ctx.in_field("mark2_anchor_offsets", |ctx| {
                self.mark2_anchor_offsets.validate_impl(ctx);
            });
        })
    }
}
