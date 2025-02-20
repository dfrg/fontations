//! Writing and modifying OpenType tables
//!
//! This crate provides a collection of types correlating to what is described
//! in the [OpenType spec][spec], along with the logic to serialize these types
//! into binary font tables. It is a companion to [`read-fonts`], which provides
//! efficient zero-allocation parsing of these types. It is intended to be used
//! as the basis for font engineering tools such as compilers.
//!
//! ## 'write' versus 'read' types
//!
//! Both `write-fonts` and [`read-fonts`] make heavy use of code generation, and
//! they have a similar structure, where a `tables` module contains a submodule for
//! each supported [table][table-directory], and that module contains items for
//! each table, record, flagset or enum described in the spec. This means that
//! there are (for instance) two distinct `ValueRecord` types, one defined in
//! `read_fonts::tables::gpos`, and one defined in `write_fonts::tables::gpos`.
//!
//! The reason for the distinct types is that it allows us to dramatically
//! simplify the scope of `read-fonts`; the types in that crate are generally
//! just typed views into raw slices of bytes and cannot be modified, whereas
//! the types in `write-fonts` are generally familiar Rust structs and enums.
//!
//! ## Loading and modifying fonts
//!
//! Although `write-fonts` does not contain any parsing logic, it does offer the
//! [`FromTableRef`] and [`ToOwnedTable`] traits (similar to `std`'s `From` & `Into`)
//! for converting from a `read_fonts` type to its `write-fonts` equivalent.
//! This means that you can read an existing font table into `write-fonts`;
//! under the hood we will use `read-fonts` to parse the font, and then convert
//! that to the `write-fonts` version. In general you do not need to think about
//! this conversion yourself; tables implement the [`FontRead`] trait from
//! `read-fonts`, which handles the reading + conversion logic for you.
//!
//! When loading and modifying fonts, you will likely need to interact with both
//! `write-fonts` and `read-fonts` directly. To avoid having to manage both of
//! these dependencies, there is a "read" feature on `write-fonts` that reexports
//! `read-fonts` as `read` at the crate root:
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! write-fonts = { version = "*", features = ["read"] }
//! ```
//!
//! ```no_compile
//! // main.rs
//! use write_fonts::read::FontRef;
//! ```
//!
//! ## Writing subtables
//!
//! A font table commonly contains some set of subtables which are referenced
//! in the font binary as offsets relative to the position (within the file) of
//! the parent table; and these subtables can themselves contain subtables, and
//! so on. We refer to the entire structure of tables as the 'table graph'.
//! A consequence of this structure is that compiling
//! a table is not as simple as just sequentially writing out the bytes of each
//! field; it also involves computing an ordering for the subtables, determining
//! their position in the final binary, and correctly writing that position in
//! the appropriate location in any tables that reference that subtable.
//!
//! As most subtable positions (offsets) are stored as 16-bit integers,
//! it is possible in certain cases that offsets overflow. The task of finding
//! a suitable ordering for each table in the table graph is called "table packing".
//! `write-fonts` handles the packing of tables at serialization time, based
//! on the [hb-repacker] implementation from [HarfBuzz].
//!
//! # Examples
//!
//! Create an 'hhea' table
//! ```no_run
//! use write_fonts::{tables::hhea::Hhea, types::{FWord, UfWord}};
//!
//! let my_table = Hhea {
//!     ascender: FWord::new(700),
//!     descender: FWord::new(-195),
//!     line_gap: FWord::new(0),
//!     advance_width_max: UfWord::new(1200),
//!     min_left_side_bearing: FWord::new(-80),
//!     min_right_side_bearing: FWord::new(-420),
//!     x_max_extent: FWord::new(1122),
//!     caret_slope_rise: 1,
//!     caret_slope_run: 0,
//!     caret_offset: 0,
//!     number_of_long_metrics: 301,
//! };
//!
//! let _bytes = write_fonts::dump_table(&my_table).expect("failed to write bytes");
//! ```
//!
//! Read/modify/write an existing font
//! ```no_run
//! # let path_to_my_font_file = std::path::Path::new("");
//! # fn seconds_since_font_epoch() -> LongDateTime { todo!() }
//! use read_fonts::{FontRef, TableProvider};
//! use write_fonts::{
//!     from_obj::ToOwnedTable,
//!     tables::head::Head,
//!     types::LongDateTime,
//!     FontBuilder,
//! };
//! let font_bytes = std::fs::read(path_to_my_font_file).unwrap();
//! let font = FontRef::new(&font_bytes).expect("failed to read font data");
//! let mut head: Head = font.head().expect("missing 'head' table").to_owned_table();
//! head.modified  = seconds_since_font_epoch();
//! let new_bytes = FontBuilder::new()
//!     .add_table(&head)
//!     .unwrap() // errors if we can't compile 'head', unlikely here
//!     .copy_missing_tables(font)
//!     .build();
//! std::fs::write("mynewfont.ttf", &new_bytes).unwrap();
//! ```
//!
//! [`read-fonts`]: https://docs.rs/read-fonts/
//! [spec]: https://learn.microsoft.com/en-us/typography/opentype/spec/
//! [table-directory]: https://learn.microsoft.com/en-us/typography/opentype/spec/otff#table-directory
//! [`FontRead`]: read_fonts::FontRead
//! [hb-repacker]: https://github.com/harfbuzz/harfbuzz/blob/main/docs/repacker.md
//! [HarfBuzz]: https://harfbuzz.github.io
//! [`FromTableRef`]: from_obj::FromTableRef
//! [`ToOwnedTable`]: from_obj::ToOwnedTable

mod collections;
pub mod error;
mod font_builder;
pub mod from_obj;
mod graph;
mod offsets;
pub mod pens;
mod round;
mod table_type;
pub mod tables;
mod util;
pub mod validate;
mod write;

#[cfg(test)]
mod codegen_test;
#[cfg(test)]
mod hex_diff;

pub use font_builder::{BuilderError, FontBuilder};
pub use offsets::{NullableOffsetMarker, OffsetMarker};
pub use round::OtRound;
pub use write::{dump_table, FontWrite, TableWriter};

/// Rexport of the common font types
pub extern crate font_types as types;
/// Reexport the read_fonts crate, if requested
#[cfg(feature = "read")]
pub extern crate read_fonts as read;

/// types used in autogenerated code.
pub(crate) mod codegen_prelude {
    use std::num::TryFromIntError;

    pub use super::from_obj::{FromObjRef, FromTableRef, ToOwnedObj, ToOwnedTable};
    pub use super::offsets::{NullableOffsetMarker, OffsetMarker, WIDTH_16, WIDTH_24, WIDTH_32};
    pub use super::table_type::TableType;
    pub use super::validate::{Validate, ValidationCtx};
    pub use super::write::{FontWrite, TableWriter};
    pub use std::collections::BTreeSet;
    pub use types::*;

    pub use read_fonts::{
        FontData, FontRead, FontReadWithArgs, ReadArgs, ReadError, ResolveOffset, TopLevelTable,
    };

    /// checked conversion to u16
    pub fn array_len<T: super::collections::HasLen>(s: &T) -> Result<u16, TryFromIntError> {
        s.checked_len()
    }

    pub fn plus_one(val: &usize) -> Result<u16, TryFromIntError> {
        val.saturating_add(1).try_into()
    }
}
