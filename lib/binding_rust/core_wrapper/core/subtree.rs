use crate::core::util::*;
use crate::core::*;
use :: c2rust_bitfields;
use std::os;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type _IO_lock_t = ();
use crate::core::util::libc::{dup, fclose, fdopen, fputc, fputs, FILE};
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type TSStateId = uint16_t;
pub type TSSymbol = uint16_t;
pub type TSFieldId = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub states: *const bool,
    pub symbol_map: *const TSSymbol,
    pub create: Option<unsafe extern "C" fn() -> *mut libc::c_void>,
    pub destroy: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scan: Option<unsafe extern "C" fn(*mut libc::c_void, *mut TSLexer, *const bool) -> bool>,
    pub serialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char) -> libc::c_uint>,
    pub deserialize:
        Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_uint) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub count: uint8_t,
    pub reusable: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_: uint8_t,
    pub child_count: uint8_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: int16_t,
    pub production_id: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub type_: uint8_t,
    pub state: TSStateId,
    pub extra: bool,
    pub repetition: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExternalScannerState {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub length: uint32_t,
}
type C2RustUnnamed_4 = crate::core::util::LongShortData;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SubtreeInlineData {
    #[bitfield(name = "is_inline", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "visible", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "named", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "extra", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "has_changes", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "is_missing", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "is_keyword", ty = "bool", bits = "6..=6")]
    pub is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [u8; 1],
    pub symbol: uint8_t,
    pub parse_state: uint16_t,
    pub padding_columns: uint8_t,
    #[bitfield(name = "padding_rows", ty = "uint8_t", bits = "0..=3")]
    #[bitfield(name = "lookahead_bytes", ty = "uint8_t", bits = "4..=7")]
    pub padding_rows_lookahead_bytes: [u8; 1],
    pub padding_bytes: uint8_t,
    pub size_bytes: uint8_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct SubtreeHeapData {
    pub ref_count: uint32_t,
    pub padding: Length,
    pub size: Length,
    pub lookahead_bytes: uint32_t,
    pub error_cost: uint32_t,
    pub child_count: uint32_t,
    pub symbol: TSSymbol,
    pub parse_state: TSStateId,
    #[bitfield(name = "visible", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "named", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "extra", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "fragile_left", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "fragile_right", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "has_changes", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "has_external_tokens", ty = "bool", bits = "6..=6")]
    #[bitfield(
        name = "has_external_scanner_state_change",
        ty = "bool",
        bits = "7..=7"
    )]
    #[bitfield(name = "depends_on_column", ty = "bool", bits = "8..=8")]
    #[bitfield(name = "is_missing", ty = "bool", bits = "9..=9")]
    #[bitfield(name = "is_keyword", ty = "bool", bits = "10..=10")]
    pub visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_has_external_scanner_state_change_depends_on_column_is_missing_is_keyword:
        [u8; 2],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
    pub c2rust_unnamed: C2RustUnnamed_5,
}
type C2RustUnnamed_5 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_7 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub union Subtree {
    pub data: SubtreeInlineData,
    pub ptr: *const SubtreeHeapData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union MutableSubtree {
    pub data: SubtreeInlineData,
    pub ptr: *mut SubtreeHeapData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtreeArray {
    pub contents: *mut Subtree,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MutableSubtreeArray {
    pub contents: *mut MutableSubtree,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubtreePool {
    pub free_trees: MutableSubtreeArray,
    pub tree_stack: MutableSubtreeArray,
}
type C2RustUnnamed_8 = crate::core::util::StackElement<*mut StackEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry {
    pub tree: *mut Subtree,
    pub edit: Edit,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edit {
    pub start: Length,
    pub old_end: Length,
    pub new_end: Length,
}
#[inline]
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.symbol as libc::c_int
    } else {
        (*self_0.ptr).symbol as libc::c_int
    }) as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).visible() as libc::c_int
    } else {
        (*self_0.ptr).visible() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_named(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).named() as libc::c_int
    } else {
        (*self_0.ptr).named() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_extra(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).extra() as libc::c_int
    } else {
        (*self_0.ptr).extra() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_changes(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).has_changes() as libc::c_int
    } else {
        (*self_0.ptr).has_changes() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_missing(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).is_missing() as libc::c_int
    } else {
        (*self_0.ptr).is_missing() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_parse_state(mut self_0: Subtree) -> TSStateId {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.parse_state as libc::c_int
    } else {
        (*self_0.ptr).parse_state as libc::c_int
    }) as TSStateId;
}
#[inline]
unsafe extern "C" fn ts_subtree_lookahead_bytes(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).lookahead_bytes() as libc::c_uint
    } else {
        (*self_0.ptr).lookahead_bytes
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_alloc_size(mut child_count: uint32_t) -> size_t {
    return (child_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<Subtree>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<SubtreeHeapData>() as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn ts_subtree_leaf_symbol(mut self_0: Subtree) -> TSSymbol {
    if (self_0.data).is_inline() {
        return self_0.data.symbol as TSSymbol;
    }
    if (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).symbol;
    }
    return (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .first_leaf
        .symbol;
}
#[inline]
unsafe extern "C" fn ts_subtree_leaf_parse_state(mut self_0: Subtree) -> TSStateId {
    if (self_0.data).is_inline() {
        return self_0.data.parse_state;
    }
    if (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).parse_state;
    }
    return (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .first_leaf
        .parse_state;
}
#[inline]
unsafe extern "C" fn ts_subtree_padding(mut self_0: Subtree) -> Length {
    if (self_0.data).is_inline() {
        let mut result: Length = {
            let mut init = Length {
                bytes: self_0.data.padding_bytes as uint32_t,
                extent: {
                    let mut init = TSPoint {
                        row: (self_0.data).padding_rows() as uint32_t,
                        column: self_0.data.padding_columns as uint32_t,
                    };
                    init
                },
            };
            init
        };
        return result;
    } else {
        return (*self_0.ptr).padding;
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_size(mut self_0: Subtree) -> Length {
    if (self_0.data).is_inline() {
        let mut result: Length = {
            let mut init = Length {
                bytes: self_0.data.size_bytes as uint32_t,
                extent: {
                    let mut init = TSPoint {
                        row: 0 as libc::c_int as uint32_t,
                        column: self_0.data.size_bytes as uint32_t,
                    };
                    init
                },
            };
            init
        };
        return result;
    } else {
        return (*self_0.ptr).size;
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_total_size(mut self_0: Subtree) -> Length {
    return length_add(ts_subtree_padding(self_0), ts_subtree_size(self_0));
}
#[inline]
unsafe extern "C" fn ts_subtree_total_bytes(mut self_0: Subtree) -> uint32_t {
    return (ts_subtree_total_size(self_0)).bytes;
}
#[inline]
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr).child_count
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_repeat_depth(mut self_0: Subtree) -> uint32_t {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth as libc::c_int
    }) as uint32_t;
}
#[inline]
unsafe extern "C" fn ts_subtree_visible_descendant_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0
        || (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .visible_descendant_count
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_error_cost(mut self_0: Subtree) -> uint32_t {
    if ts_subtree_missing(self_0) {
        return (110 as libc::c_int + 500 as libc::c_int) as uint32_t;
    } else {
        return if (self_0.data).is_inline() as libc::c_int != 0 {
            0 as libc::c_int as libc::c_uint
        } else {
            (*self_0.ptr).error_cost
        };
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_dynamic_precedence(mut self_0: Subtree) -> int32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0
        || (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .dynamic_precedence
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_production_id(mut self_0: Subtree) -> uint16_t {
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint {
        return (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id;
    } else {
        return 0 as libc::c_int as uint16_t;
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_fragile_left(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).fragile_left() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_fragile_right(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).fragile_right() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_external_tokens(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).has_external_tokens() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_has_external_scanner_state_change(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).has_external_scanner_state_change() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_depends_on_column(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).depends_on_column() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_is_error(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_subtree_from_mut(mut self_0: MutableSubtree) -> Subtree {
    let mut result: Subtree = Subtree {
        data: SubtreeInlineData {
            is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [0; 1],
            symbol: 0,
            parse_state: 0,
            padding_columns: 0,
            padding_rows_lookahead_bytes: [0; 1],
            padding_bytes: 0,
            size_bytes: 0,
        },
    };
    result.data = self_0.data;
    return result;
}
#[inline]
unsafe extern "C" fn ts_subtree_to_mut_unsafe(mut self_0: Subtree) -> MutableSubtree {
    let mut result: MutableSubtree = MutableSubtree {
        data: SubtreeInlineData {
            is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [0; 1],
            symbol: 0,
            parse_state: 0,
            padding_columns: 0,
            padding_rows_lookahead_bytes: [0; 1],
            padding_bytes: 0,
            size_bytes: 0,
        },
    };
    result.data = self_0.data;
    return result;
}
#[inline]
unsafe extern "C" fn length_sub(mut len1: Length, mut len2: Length) -> Length {
    let mut result: Length = Length {
        bytes: 0,
        extent: TSPoint { row: 0, column: 0 },
    };
    result.bytes = (len1.bytes).wrapping_sub(len2.bytes);
    result.extent = point_sub(len1.extent, len2.extent);
    return result;
}
#[inline]
unsafe extern "C" fn point__new(mut row: libc::c_uint, mut column: libc::c_uint) -> TSPoint {
    let mut result: TSPoint = {
        let mut init = TSPoint {
            row: row,
            column: column,
        };
        init
    };
    return result;
}
#[inline]
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new((a.row).wrapping_add(b.row), b.column);
    } else {
        return point__new(a.row, (a.column).wrapping_add(b.column));
    };
}
#[inline]
unsafe extern "C" fn point_sub(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if a.row > b.row {
        return point__new((a.row).wrapping_sub(b.row), a.column);
    } else {
        return point__new(
            0 as libc::c_int as libc::c_uint,
            (a.column).wrapping_sub(b.column),
        );
    };
}
#[inline]
unsafe extern "C" fn array__grow(
    mut self_0: *mut VoidArray,
    mut count: uint32_t,
    mut element_size: size_t,
) {
    let mut new_size: uint32_t = ((*self_0).size).wrapping_add(count);
    if new_size > (*self_0).capacity {
        let mut new_capacity: uint32_t =
            ((*self_0).capacity).wrapping_mul(2 as libc::c_int as libc::c_uint);
        if new_capacity < 8 as libc::c_int as libc::c_uint {
            new_capacity = 8 as libc::c_int as uint32_t;
        }
        if new_capacity < new_size {
            new_capacity = new_size;
        }
        array__reserve(self_0, element_size, new_capacity);
    }
}
#[inline]
unsafe extern "C" fn length_add(mut len1: Length, mut len2: Length) -> Length {
    let mut result: Length = Length {
        bytes: 0,
        extent: TSPoint { row: 0, column: 0 },
    };
    result.bytes = (len1.bytes).wrapping_add(len2.bytes);
    result.extent = point_add(len1.extent, len2.extent);
    return result;
}
#[inline]
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    crate::core::alloc::ts_free((*self_0).contents);
    let ref mut fresh4 = (*self_0).contents;
    *fresh4 = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn array__reserve(
    mut self_0: *mut VoidArray,
    mut element_size: size_t,
    mut new_capacity: uint32_t,
) {
    if new_capacity > (*self_0).capacity {
        if !((*self_0).contents).is_null() {
            let ref mut fresh5 = (*self_0).contents;
            *fresh5 = crate::core::alloc::ts_realloc(
                (*self_0).contents,
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            let ref mut fresh6 = (*self_0).contents;
            *fresh6 = crate::core::alloc::ts_malloc(
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        }
        (*self_0).capacity = new_capacity;
    }
}
#[inline]
unsafe extern "C" fn length_zero() -> Length {
    let mut result: Length = {
        let mut init = Length {
            bytes: 0 as libc::c_int as uint32_t,
            extent: {
                let mut init = TSPoint {
                    row: 0 as libc::c_int as uint32_t,
                    column: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
    return result;
}
#[inline]
unsafe extern "C" fn length_saturating_sub(mut len1: Length, mut len2: Length) -> Length {
    if len1.bytes > len2.bytes {
        return length_sub(len1, len2);
    } else {
        return length_zero();
    };
}
#[inline]
unsafe extern "C" fn ts_language_alias_sequence(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
) -> *const TSSymbol {
    return if production_id != 0 {
        &*((*self_0).alias_sequences).offset(
            production_id.wrapping_mul((*self_0).max_alias_sequence_length as libc::c_uint)
                as isize,
        ) as *const TSSymbol
    } else {
        0 as *const TSSymbol
    };
}
#[inline]
unsafe extern "C" fn ts_language_field_map(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
    mut start: *mut *const TSFieldMapEntry,
    mut end: *mut *const TSFieldMapEntry,
) {
    if (*self_0).field_count == 0 as libc::c_int as libc::c_uint {
        *start = 0 as *const TSFieldMapEntry;
        *end = 0 as *const TSFieldMapEntry;
        return;
    }
    let mut slice: TSFieldMapSlice = *((*self_0).field_map_slices).offset(production_id as isize);
    *start = &*((*self_0).field_map_entries).offset(slice.index as isize) as *const TSFieldMapEntry;
    *end = (&*((*self_0).field_map_entries).offset(slice.index as isize) as *const TSFieldMapEntry)
        .offset(slice.length as libc::c_int as isize);
}
#[inline]
unsafe extern "C" fn ts_language_write_symbol_as_dot_string(
    mut self_0: *const TSLanguage,
    mut f: *mut FILE,
    mut symbol: TSSymbol,
) {
    let mut name: *const libc::c_char = ts_language_symbol_name(self_0, symbol);
    let mut chr: *const libc::c_char = name;
    while *chr != 0 {
        match *chr as libc::c_int {
            34 | 92 => {
                fputc('\\' as i32, f);
                fputc(*chr as libc::c_int, f);
            }
            10 => {
                fputs(b"\\n\0" as *const u8 as *const libc::c_char, f);
            }
            9 => {
                fputs(b"\\t\0" as *const u8 as *const libc::c_char, f);
            }
            _ => {
                fputc(*chr as libc::c_int, f);
            }
        }
        chr = chr.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_init(
    mut self_0: *mut ExternalScannerState,
    mut data: *const libc::c_char,
    mut length: libc::c_uint,
) {
    (*self_0).length = length;
    if length as libc::c_ulong > ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong {
        let ref mut fresh7 = (*self_0).c2rust_unnamed.long_data;
        *fresh7 = crate::core::alloc::ts_malloc(length as size_t) as *mut libc::c_char;
        std::ptr::copy_nonoverlapping(
            data as *const libc::c_void,
            (*self_0).c2rust_unnamed.long_data as *mut libc::c_void,
            length as libc::c_ulong,
        );
    } else {
        std::ptr::copy_nonoverlapping(
            data as *const libc::c_void,
            ((*self_0).c2rust_unnamed.short_data).as_mut_ptr() as *mut libc::c_void,
            length as libc::c_ulong,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_copy(
    mut self_0: *const ExternalScannerState,
) -> ExternalScannerState {
    let mut result: ExternalScannerState = *self_0;
    if (*self_0).length as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
    {
        result.c2rust_unnamed.long_data =
            crate::core::alloc::ts_malloc((*self_0).length as size_t) as *mut libc::c_char;
        std::ptr::copy_nonoverlapping(
            (*self_0).c2rust_unnamed.long_data as *const libc::c_void,
            result.c2rust_unnamed.long_data as *mut libc::c_void,
            (*self_0).length as libc::c_ulong,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_delete(mut self_0: *mut ExternalScannerState) {
    if (*self_0).length as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
    {
        crate::core::alloc::ts_free((*self_0).c2rust_unnamed.long_data as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_data(
    mut self_0: *const ExternalScannerState,
) -> *const libc::c_char {
    if (*self_0).length as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong
    {
        return (*self_0).c2rust_unnamed.long_data;
    } else {
        return ((*self_0).c2rust_unnamed.short_data).as_ptr();
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_external_scanner_state_eq(
    mut self_0: *const ExternalScannerState,
    mut buffer: *const libc::c_char,
    mut length: libc::c_uint,
) -> bool {
    return (*self_0).length == length
        && if std::slice::from_raw_parts(
            ts_external_scanner_state_data(self_0) as *const u8,
            length as libc::c_ulong,
        ) == std::slice::from_raw_parts(buffer as *const u8, length as libc::c_ulong)
        {
            0
        } else {
            1
        } == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_copy(
    mut self_0: SubtreeArray,
    mut dest: *mut SubtreeArray,
) {
    (*dest).size = self_0.size;
    (*dest).capacity = self_0.capacity;
    let ref mut fresh8 = (*dest).contents;
    *fresh8 = self_0.contents;
    if self_0.capacity > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh9 = (*dest).contents;
        *fresh9 = crate::core::alloc::ts_calloc(
            self_0.capacity as size_t,
            ::std::mem::size_of::<Subtree>() as libc::c_ulong,
        ) as *mut Subtree;
        std::ptr::copy_nonoverlapping(
            self_0.contents as *const libc::c_void,
            (*dest).contents as *mut libc::c_void,
            (self_0.size as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<Subtree>() as libc::c_ulong),
        );
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < self_0.size {
            ts_subtree_retain(*((*dest).contents).offset(i as isize));
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_clear(
    mut pool: *mut SubtreePool,
    mut self_0: *mut SubtreeArray,
) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).size {
        ts_subtree_release(pool, *((*self_0).contents).offset(i as isize));
        i = i.wrapping_add(1);
    }
    (*self_0).size = 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_delete(
    mut pool: *mut SubtreePool,
    mut self_0: *mut SubtreeArray,
) {
    ts_subtree_array_clear(pool, self_0);
    array__delete(self_0 as *mut VoidArray);
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_remove_trailing_extras(
    mut self_0: *mut SubtreeArray,
    mut destination: *mut SubtreeArray,
) {
    (*destination).size = 0 as libc::c_int as uint32_t;
    while (*self_0).size > 0 as libc::c_int as libc::c_uint {
        let mut last: Subtree = *((*self_0).contents)
            .offset(((*self_0).size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        if !ts_subtree_extra(last) {
            break;
        }
        let ref mut fresh10 = (*self_0).size;
        *fresh10 = (*fresh10).wrapping_sub(1);
        array__grow(
            destination as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<Subtree>() as libc::c_ulong,
        );
        let ref mut fresh11 = (*destination).size;
        let fresh12 = *fresh11;
        *fresh11 = (*fresh11).wrapping_add(1);
        *((*destination).contents).offset(fresh12 as isize) = last;
    }
    ts_subtree_array_reverse(destination);
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_array_reverse(mut self_0: *mut SubtreeArray) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut limit: uint32_t = ((*self_0).size).wrapping_div(2 as libc::c_int as libc::c_uint);
    while i < limit {
        let mut reverse_index: size_t = ((*self_0).size)
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_sub(i) as size_t;
        let mut swap: Subtree = *((*self_0).contents).offset(i as isize);
        *((*self_0).contents).offset(i as isize) =
            *((*self_0).contents).offset(reverse_index as isize);
        *((*self_0).contents).offset(reverse_index as isize) = swap;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_pool_new(mut capacity: uint32_t) -> SubtreePool {
    let mut self_0: SubtreePool = {
        let mut init = SubtreePool {
            free_trees: {
                let mut init = MutableSubtreeArray {
                    contents: 0 as *mut MutableSubtree,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            tree_stack: {
                let mut init = MutableSubtreeArray {
                    contents: 0 as *mut MutableSubtree,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
    array__reserve(
        &mut self_0.free_trees as *mut MutableSubtreeArray as *mut VoidArray,
        ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
        capacity,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_pool_delete(mut self_0: *mut SubtreePool) {
    if !((*self_0).free_trees.contents).is_null() {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).free_trees.size {
            crate::core::alloc::ts_free(
                (*((*self_0).free_trees.contents).offset(i as isize)).ptr as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
        array__delete(&mut (*self_0).free_trees as *mut MutableSubtreeArray as *mut VoidArray);
    }
    if !((*self_0).tree_stack.contents).is_null() {
        array__delete(&mut (*self_0).tree_stack as *mut MutableSubtreeArray as *mut VoidArray);
    }
}
unsafe extern "C" fn ts_subtree_pool_allocate(
    mut self_0: *mut SubtreePool,
) -> *mut SubtreeHeapData {
    if (*self_0).free_trees.size > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh13 = (*self_0).free_trees.size;
        *fresh13 = (*fresh13).wrapping_sub(1);
        return (*((*self_0).free_trees.contents).offset(*fresh13 as isize)).ptr;
    } else {
        return crate::core::alloc::ts_malloc(
            ::std::mem::size_of::<SubtreeHeapData>() as libc::c_ulong
        ) as *mut SubtreeHeapData;
    };
}
unsafe extern "C" fn ts_subtree_pool_free(
    mut self_0: *mut SubtreePool,
    mut tree: *mut SubtreeHeapData,
) {
    if (*self_0).free_trees.capacity > 0 as libc::c_int as libc::c_uint
        && ((*self_0).free_trees.size).wrapping_add(1 as libc::c_int as libc::c_uint)
            <= 32 as libc::c_int as libc::c_uint
    {
        array__grow(
            &mut (*self_0).free_trees as *mut MutableSubtreeArray as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
        );
        let ref mut fresh14 = (*self_0).free_trees.size;
        let fresh15 = *fresh14;
        *fresh14 = (*fresh14).wrapping_add(1);
        *((*self_0).free_trees.contents).offset(fresh15 as isize) = MutableSubtree { ptr: tree };
    } else {
        crate::core::alloc::ts_free(tree as *mut libc::c_void);
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_can_inline(
    mut padding: Length,
    mut size: Length,
    mut lookahead_bytes: uint32_t,
) -> bool {
    return padding.bytes < 255 as libc::c_int as libc::c_uint
        && padding.extent.row < 16 as libc::c_int as libc::c_uint
        && padding.extent.column < 255 as libc::c_int as libc::c_uint
        && size.extent.row == 0 as libc::c_int as libc::c_uint
        && size.extent.column < 255 as libc::c_int as libc::c_uint
        && lookahead_bytes < 16 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_leaf(
    mut pool: *mut SubtreePool,
    mut symbol: TSSymbol,
    mut padding: Length,
    mut size: Length,
    mut lookahead_bytes: uint32_t,
    mut parse_state: TSStateId,
    mut has_external_tokens: bool,
    mut depends_on_column: bool,
    mut is_keyword: bool,
    mut language: *const TSLanguage,
) -> Subtree {
    let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(language, symbol);
    let mut extra: bool = symbol as libc::c_int == 0 as libc::c_int;
    let mut is_inline: bool = symbol as libc::c_int <= 255 as libc::c_int
        && !has_external_tokens
        && ts_subtree_can_inline(padding, size, lookahead_bytes) as libc::c_int != 0;
    if is_inline {
        return Subtree {
            data: {
                let mut init = SubtreeInlineData {
                    is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [0; 1],
                    padding_rows_lookahead_bytes: [0; 1],
                    symbol: symbol as uint8_t,
                    parse_state: parse_state,
                    padding_columns: padding.extent.column as uint8_t,
                    padding_bytes: padding.bytes as uint8_t,
                    size_bytes: size.bytes as uint8_t,
                };
                init.set_is_inline(1 as libc::c_int != 0);
                init.set_visible(metadata.visible);
                init.set_named(metadata.named);
                init.set_extra(extra);
                init.set_has_changes(0 as libc::c_int != 0);
                init.set_is_missing(0 as libc::c_int != 0);
                init.set_is_keyword(is_keyword);
                init.set_padding_rows(padding.extent.row as uint8_t);
                init.set_lookahead_bytes(lookahead_bytes as uint8_t);
                init
            },
        };
    } else {
        let mut data: *mut SubtreeHeapData = ts_subtree_pool_allocate(pool);
        *data = {
            let mut init = SubtreeHeapData { visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_has_external_scanner_state_change_depends_on_column_is_missing_is_keyword : [0 ; 2] , c2rust_padding : [0 ; 2] , ref_count : 1 as libc :: c_int as uint32_t , padding : padding , size : size , lookahead_bytes : lookahead_bytes , error_cost : 0 as libc :: c_int as uint32_t , child_count : 0 as libc :: c_int as uint32_t , symbol : symbol , parse_state : parse_state , c2rust_unnamed : C2RustUnnamed_5 { c2rust_unnamed : { let mut init = C2RustUnnamed_6 { visible_child_count : 0 , named_child_count : 0 , visible_descendant_count : 0 , dynamic_precedence : 0 , repeat_depth : 0 , production_id : 0 , first_leaf : { let mut init = C2RustUnnamed_7 { symbol : 0 as libc :: c_int as TSSymbol , parse_state : 0 as libc :: c_int as TSStateId , } ; init } , } ; init } , } , } ;
            init.set_visible(metadata.visible);
            init.set_named(metadata.named);
            init.set_extra(extra);
            init.set_fragile_left(0 as libc::c_int != 0);
            init.set_fragile_right(0 as libc::c_int != 0);
            init.set_has_changes(0 as libc::c_int != 0);
            init.set_has_external_tokens(has_external_tokens);
            init.set_has_external_scanner_state_change(0 as libc::c_int != 0);
            init.set_depends_on_column(depends_on_column);
            init.set_is_missing(0 as libc::c_int != 0);
            init.set_is_keyword(is_keyword);
            init
        };
        return Subtree { ptr: data };
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_set_symbol(
    mut self_0: *mut MutableSubtree,
    mut symbol: TSSymbol,
    mut language: *const TSLanguage,
) {
    let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(language, symbol);
    if ((*self_0).data).is_inline() {
        if (symbol as libc::c_int) < 255 as libc::c_int {
        } else {
            panic!();
        }
        (*self_0).data.symbol = symbol as uint8_t;
        let ref mut fresh16 = (*self_0).data;
        (*fresh16).set_named(metadata.named);
        let ref mut fresh17 = (*self_0).data;
        (*fresh17).set_visible(metadata.visible);
    } else {
        (*(*self_0).ptr).symbol = symbol;
        let ref mut fresh18 = *(*self_0).ptr;
        (*fresh18).set_named(metadata.named);
        let ref mut fresh19 = *(*self_0).ptr;
        (*fresh19).set_visible(metadata.visible);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_error(
    mut pool: *mut SubtreePool,
    mut lookahead_char: int32_t,
    mut padding: Length,
    mut size: Length,
    mut bytes_scanned: uint32_t,
    mut parse_state: TSStateId,
    mut language: *const TSLanguage,
) -> Subtree {
    let mut result: Subtree = ts_subtree_new_leaf(
        pool,
        -(1 as libc::c_int) as TSSymbol,
        padding,
        size,
        bytes_scanned,
        parse_state,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        language,
    );
    let mut data: *mut SubtreeHeapData = result.ptr as *mut SubtreeHeapData;
    (*data).set_fragile_left(1 as libc::c_int != 0);
    (*data).set_fragile_right(1 as libc::c_int != 0);
    (*data).c2rust_unnamed.lookahead_char = lookahead_char;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_clone(mut self_0: Subtree) -> MutableSubtree {
    let mut alloc_size: size_t = ts_subtree_alloc_size((*self_0.ptr).child_count);
    let mut new_children: *mut Subtree = crate::core::alloc::ts_malloc(alloc_size) as *mut Subtree;
    let mut old_children: *mut Subtree = if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as *mut Subtree
    } else {
        (self_0.ptr as *mut Subtree).offset(-((*self_0.ptr).child_count as isize))
    };
    std::ptr::copy_nonoverlapping(
        old_children as *const libc::c_void,
        new_children as *mut libc::c_void,
        alloc_size,
    );
    let mut result: *mut SubtreeHeapData = &mut *new_children
        .offset((*self_0.ptr).child_count as isize)
        as *mut Subtree as *mut SubtreeHeapData;
    if (*self_0.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*self_0.ptr).child_count {
            ts_subtree_retain(*new_children.offset(i as isize));
            i = i.wrapping_add(1);
        }
    } else if (*self_0.ptr).has_external_tokens() {
        (*result).c2rust_unnamed.external_scanner_state =
            ts_external_scanner_state_copy(&(*self_0.ptr).c2rust_unnamed.external_scanner_state);
    }
    ::std::ptr::write_volatile(
        &mut (*result).ref_count as *mut uint32_t,
        1 as libc::c_int as uint32_t,
    );
    return MutableSubtree { ptr: result };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_make_mut(
    mut pool: *mut SubtreePool,
    mut self_0: Subtree,
) -> MutableSubtree {
    if (self_0.data).is_inline() {
        return MutableSubtree { data: self_0.data };
    }
    if (*self_0.ptr).ref_count == 1 as libc::c_int as libc::c_uint {
        return ts_subtree_to_mut_unsafe(self_0);
    }
    let mut result: MutableSubtree = ts_subtree_clone(self_0);
    ts_subtree_release(pool, self_0);
    return result;
}
unsafe extern "C" fn ts_subtree__compress(
    mut self_0: MutableSubtree,
    mut count: libc::c_uint,
    mut language: *const TSLanguage,
    mut stack: *mut MutableSubtreeArray,
) {
    let mut initial_stack_size: libc::c_uint = (*stack).size;
    let mut tree: MutableSubtree = self_0;
    let mut symbol: TSSymbol = (*tree.ptr).symbol;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < count {
        if (*tree.ptr).ref_count > 1 as libc::c_int as libc::c_uint
            || (*tree.ptr).child_count < 2 as libc::c_int as libc::c_uint
        {
            break;
        }
        let mut child: MutableSubtree = ts_subtree_to_mut_unsafe(
            *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(0 as libc::c_int as isize),
        );
        if (child.data).is_inline() as libc::c_int != 0
            || (*child.ptr).child_count < 2 as libc::c_int as libc::c_uint
            || (*child.ptr).ref_count > 1 as libc::c_int as libc::c_uint
            || (*child.ptr).symbol as libc::c_int != symbol as libc::c_int
        {
            break;
        }
        let mut grandchild: MutableSubtree = ts_subtree_to_mut_unsafe(
            *if (child.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (child.ptr as *mut Subtree).offset(-((*child.ptr).child_count as isize))
            }
            .offset(0 as libc::c_int as isize),
        );
        if (grandchild.data).is_inline() as libc::c_int != 0
            || (*grandchild.ptr).child_count < 2 as libc::c_int as libc::c_uint
            || (*grandchild.ptr).ref_count > 1 as libc::c_int as libc::c_uint
            || (*grandchild.ptr).symbol as libc::c_int != symbol as libc::c_int
        {
            break;
        }
        *if (tree.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
        }
        .offset(0 as libc::c_int as isize) = ts_subtree_from_mut(grandchild);
        *if (child.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (child.ptr as *mut Subtree).offset(-((*child.ptr).child_count as isize))
        }
        .offset(0 as libc::c_int as isize) =
            *if (grandchild.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (grandchild.ptr as *mut Subtree).offset(-((*grandchild.ptr).child_count as isize))
            }
            .offset(
                ((*grandchild.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            );
        *if (grandchild.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (grandchild.ptr as *mut Subtree).offset(-((*grandchild.ptr).child_count as isize))
        }
        .offset(
            ((*grandchild.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = ts_subtree_from_mut(child);
        array__grow(
            stack as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
        );
        let ref mut fresh20 = (*stack).size;
        let fresh21 = *fresh20;
        *fresh20 = (*fresh20).wrapping_add(1);
        *((*stack).contents).offset(fresh21 as isize) = tree;
        tree = grandchild;
        i = i.wrapping_add(1);
    }
    while (*stack).size > initial_stack_size {
        let ref mut fresh22 = (*stack).size;
        *fresh22 = (*fresh22).wrapping_sub(1);
        tree = *((*stack).contents).offset(*fresh22 as isize);
        let mut child_0: MutableSubtree = ts_subtree_to_mut_unsafe(
            *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(0 as libc::c_int as isize),
        );
        let mut grandchild_0: MutableSubtree = ts_subtree_to_mut_unsafe(
            *if (child_0.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (child_0.ptr as *mut Subtree).offset(-((*child_0.ptr).child_count as isize))
            }
            .offset(
                ((*child_0.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint)
                    as isize,
            ),
        );
        ts_subtree_summarize_children(grandchild_0, language);
        ts_subtree_summarize_children(child_0, language);
        ts_subtree_summarize_children(tree, language);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_balance(
    mut self_0: Subtree,
    mut pool: *mut SubtreePool,
    mut language: *const TSLanguage,
) {
    (*pool).tree_stack.size = 0 as libc::c_int as uint32_t;
    if ts_subtree_child_count(self_0) > 0 as libc::c_int as libc::c_uint
        && (*self_0.ptr).ref_count == 1 as libc::c_int as libc::c_uint
    {
        array__grow(
            &mut (*pool).tree_stack as *mut MutableSubtreeArray as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
        );
        let ref mut fresh23 = (*pool).tree_stack.size;
        let fresh24 = *fresh23;
        *fresh23 = (*fresh23).wrapping_add(1);
        *((*pool).tree_stack.contents).offset(fresh24 as isize) = ts_subtree_to_mut_unsafe(self_0);
    }
    while (*pool).tree_stack.size > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh25 = (*pool).tree_stack.size;
        *fresh25 = (*fresh25).wrapping_sub(1);
        let mut tree: MutableSubtree = *((*pool).tree_stack.contents).offset(*fresh25 as isize);
        if (*tree.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth as libc::c_int > 0 as libc::c_int
        {
            let mut child1: Subtree = *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(0 as libc::c_int as isize);
            let mut child2: Subtree = *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(
                ((*tree.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            );
            let mut repeat_delta: libc::c_long = ts_subtree_repeat_depth(child1) as libc::c_long
                - ts_subtree_repeat_depth(child2) as libc::c_long;
            if repeat_delta > 0 as libc::c_int as libc::c_long {
                let mut n: libc::c_uint = repeat_delta as libc::c_uint;
                let mut i: libc::c_uint = n.wrapping_div(2 as libc::c_int as libc::c_uint);
                while i > 0 as libc::c_int as libc::c_uint {
                    ts_subtree__compress(tree, i, language, &mut (*pool).tree_stack);
                    n = n.wrapping_sub(i);
                    i = i.wrapping_div(2 as libc::c_int as libc::c_uint);
                }
            }
        }
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*tree.ptr).child_count {
            let mut child: Subtree = *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(i_0 as isize);
            if ts_subtree_child_count(child) > 0 as libc::c_int as libc::c_uint
                && (*child.ptr).ref_count == 1 as libc::c_int as libc::c_uint
            {
                array__grow(
                    &mut (*pool).tree_stack as *mut MutableSubtreeArray as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
                );
                let ref mut fresh26 = (*pool).tree_stack.size;
                let fresh27 = *fresh26;
                *fresh26 = (*fresh26).wrapping_add(1);
                *((*pool).tree_stack.contents).offset(fresh27 as isize) =
                    ts_subtree_to_mut_unsafe(child);
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_summarize_children(
    mut self_0: MutableSubtree,
    mut language: *const TSLanguage,
) {
    if !(self_0.data).is_inline() {
    } else {
        panic!();
    }
    (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .named_child_count = 0 as libc::c_int as uint32_t;
    (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .visible_child_count = 0 as libc::c_int as uint32_t;
    (*self_0.ptr).error_cost = 0 as libc::c_int as uint32_t;
    (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth = 0 as libc::c_int as uint16_t;
    (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .visible_descendant_count = 0 as libc::c_int as uint32_t;
    (*self_0.ptr).set_has_external_tokens(0 as libc::c_int != 0);
    (*self_0.ptr).set_depends_on_column(0 as libc::c_int != 0);
    (*self_0.ptr).set_has_external_scanner_state_change(0 as libc::c_int != 0);
    (*self_0.ptr)
        .c2rust_unnamed
        .c2rust_unnamed
        .dynamic_precedence = 0 as libc::c_int;
    let mut structural_index: uint32_t = 0 as libc::c_int as uint32_t;
    let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
        language,
        (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id as uint32_t,
    );
    let mut lookahead_end_byte: uint32_t = 0 as libc::c_int as uint32_t;
    let mut children: *const Subtree = if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as *mut Subtree
    } else {
        (self_0.ptr as *mut Subtree).offset(-((*self_0.ptr).child_count as isize))
    };
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0.ptr).child_count {
        let mut child: Subtree = *children.offset(i as isize);
        if (*self_0.ptr).size.extent.row == 0 as libc::c_int as libc::c_uint
            && ts_subtree_depends_on_column(child) as libc::c_int != 0
        {
            (*self_0.ptr).set_depends_on_column(1 as libc::c_int != 0);
        }
        if ts_subtree_has_external_scanner_state_change(child) {
            (*self_0.ptr).set_has_external_scanner_state_change(1 as libc::c_int != 0);
        }
        if i == 0 as libc::c_int as libc::c_uint {
            (*self_0.ptr).padding = ts_subtree_padding(child);
            (*self_0.ptr).size = ts_subtree_size(child);
        } else {
            (*self_0.ptr).size = length_add((*self_0.ptr).size, ts_subtree_total_size(child));
        }
        let mut child_lookahead_end_byte: uint32_t = ((*self_0.ptr).padding.bytes)
            .wrapping_add((*self_0.ptr).size.bytes)
            .wrapping_add(ts_subtree_lookahead_bytes(child));
        if child_lookahead_end_byte > lookahead_end_byte {
            lookahead_end_byte = child_lookahead_end_byte;
        }
        if ts_subtree_symbol(child) as libc::c_int
            != -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
        {
            let ref mut fresh28 = (*self_0.ptr).error_cost;
            *fresh28 = (*fresh28 as libc::c_uint).wrapping_add(ts_subtree_error_cost(child))
                as uint32_t as uint32_t;
        }
        let mut grandchild_count: uint32_t = ts_subtree_child_count(child);
        if (*self_0.ptr).symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
            || (*self_0.ptr).symbol as libc::c_int
                == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
        {
            if !ts_subtree_extra(child)
                && !(ts_subtree_is_error(child) as libc::c_int != 0
                    && grandchild_count == 0 as libc::c_int as libc::c_uint)
            {
                if ts_subtree_visible(child) {
                    let ref mut fresh29 = (*self_0.ptr).error_cost;
                    *fresh29 = (*fresh29 as libc::c_uint)
                        .wrapping_add(100 as libc::c_int as libc::c_uint)
                        as uint32_t as uint32_t;
                } else if grandchild_count > 0 as libc::c_int as libc::c_uint {
                    let ref mut fresh30 = (*self_0.ptr).error_cost;
                    *fresh30 = (*fresh30 as libc::c_uint).wrapping_add(
                        (100 as libc::c_int as libc::c_uint).wrapping_mul(
                            (*child.ptr)
                                .c2rust_unnamed
                                .c2rust_unnamed
                                .visible_child_count,
                        ),
                    ) as uint32_t as uint32_t;
                }
            }
        }
        let ref mut fresh31 = (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .dynamic_precedence;
        *fresh31 += ts_subtree_dynamic_precedence(child);
        let ref mut fresh32 = (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .visible_descendant_count;
        *fresh32 = (*fresh32 as libc::c_uint)
            .wrapping_add(ts_subtree_visible_descendant_count(child)) as uint32_t
            as uint32_t;
        if !alias_sequence.is_null()
            && *alias_sequence.offset(structural_index as isize) as libc::c_int != 0 as libc::c_int
            && !ts_subtree_extra(child)
        {
            let ref mut fresh33 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_descendant_count;
            *fresh33 = (*fresh33).wrapping_add(1);
            let ref mut fresh34 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_child_count;
            *fresh34 = (*fresh34).wrapping_add(1);
            if (ts_language_symbol_metadata(
                language,
                *alias_sequence.offset(structural_index as isize),
            ))
            .named
            {
                let ref mut fresh35 = (*self_0.ptr)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .named_child_count;
                *fresh35 = (*fresh35).wrapping_add(1);
            }
        } else if ts_subtree_visible(child) {
            let ref mut fresh36 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_descendant_count;
            *fresh36 = (*fresh36).wrapping_add(1);
            let ref mut fresh37 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_child_count;
            *fresh37 = (*fresh37).wrapping_add(1);
            if ts_subtree_named(child) {
                let ref mut fresh38 = (*self_0.ptr)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .named_child_count;
                *fresh38 = (*fresh38).wrapping_add(1);
            }
        } else if grandchild_count > 0 as libc::c_int as libc::c_uint {
            let ref mut fresh39 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_child_count;
            *fresh39 = (*fresh39 as libc::c_uint).wrapping_add(
                (*child.ptr)
                    .c2rust_unnamed
                    .c2rust_unnamed
                    .visible_child_count,
            ) as uint32_t as uint32_t;
            let ref mut fresh40 = (*self_0.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .named_child_count;
            *fresh40 = (*fresh40 as libc::c_uint)
                .wrapping_add((*child.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count)
                as uint32_t as uint32_t;
        }
        if ts_subtree_has_external_tokens(child) {
            (*self_0.ptr).set_has_external_tokens(1 as libc::c_int != 0);
        }
        if ts_subtree_is_error(child) {
            (*self_0.ptr).set_fragile_right(1 as libc::c_int != 0);
            (*self_0.ptr).set_fragile_left((*self_0.ptr).fragile_right());
            (*self_0.ptr).parse_state =
                (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as TSStateId;
        }
        if !ts_subtree_extra(child) {
            structural_index = structural_index.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*self_0.ptr).lookahead_bytes = lookahead_end_byte
        .wrapping_sub((*self_0.ptr).size.bytes)
        .wrapping_sub((*self_0.ptr).padding.bytes);
    if (*self_0.ptr).symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
        || (*self_0.ptr).symbol as libc::c_int
            == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        let ref mut fresh41 = (*self_0.ptr).error_cost;
        *fresh41 = (*fresh41 as libc::c_uint).wrapping_add(
            (500 as libc::c_int as libc::c_uint)
                .wrapping_add(
                    (1 as libc::c_int as libc::c_uint).wrapping_mul((*self_0.ptr).size.bytes),
                )
                .wrapping_add(
                    (30 as libc::c_int as libc::c_uint).wrapping_mul((*self_0.ptr).size.extent.row),
                ),
        ) as uint32_t as uint32_t;
    }
    if (*self_0.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        let mut first_child: Subtree = *children.offset(0 as libc::c_int as isize);
        let mut last_child: Subtree = *children.offset(
            ((*self_0.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        );
        (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .first_leaf
            .symbol = ts_subtree_leaf_symbol(first_child);
        (*self_0.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .first_leaf
            .parse_state = ts_subtree_leaf_parse_state(first_child);
        if ts_subtree_fragile_left(first_child) {
            (*self_0.ptr).set_fragile_left(1 as libc::c_int != 0);
        }
        if ts_subtree_fragile_right(last_child) {
            (*self_0.ptr).set_fragile_right(1 as libc::c_int != 0);
        }
        if (*self_0.ptr).child_count >= 2 as libc::c_int as libc::c_uint
            && !(*self_0.ptr).visible()
            && !(*self_0.ptr).named()
            && ts_subtree_symbol(first_child) as libc::c_int == (*self_0.ptr).symbol as libc::c_int
        {
            if ts_subtree_repeat_depth(first_child) > ts_subtree_repeat_depth(last_child) {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth =
                    (ts_subtree_repeat_depth(first_child))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as uint16_t;
            } else {
                (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.repeat_depth =
                    (ts_subtree_repeat_depth(last_child))
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as uint16_t;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_node(
    mut symbol: TSSymbol,
    mut children: *mut SubtreeArray,
    mut production_id: libc::c_uint,
    mut language: *const TSLanguage,
) -> MutableSubtree {
    let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(language, symbol);
    let mut fragile: bool = symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
        || symbol as libc::c_int
            == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int;
    let mut new_byte_size: size_t = ts_subtree_alloc_size((*children).size);
    if ((*children).capacity as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<Subtree>() as libc::c_ulong)
        < new_byte_size
    {
        let ref mut fresh42 = (*children).contents;
        *fresh42 = crate::core::alloc::ts_realloc(
            (*children).contents as *mut libc::c_void,
            new_byte_size,
        ) as *mut Subtree;
        (*children).capacity = new_byte_size
            .wrapping_div(::std::mem::size_of::<Subtree>() as libc::c_ulong)
            as uint32_t;
    }
    let mut data: *mut SubtreeHeapData = &mut *((*children).contents)
        .offset((*children).size as isize) as *mut Subtree
        as *mut SubtreeHeapData;
    *data = {
        let mut init = SubtreeHeapData { visible_named_extra_fragile_left_fragile_right_has_changes_has_external_tokens_has_external_scanner_state_change_depends_on_column_is_missing_is_keyword : [0 ; 2] , c2rust_padding : [0 ; 2] , ref_count : 1 as libc :: c_int as uint32_t , padding : Length { bytes : 0 , extent : TSPoint { row : 0 , column : 0 } , } , size : Length { bytes : 0 , extent : TSPoint { row : 0 , column : 0 } , } , lookahead_bytes : 0 , error_cost : 0 , child_count : (* children) . size , symbol : symbol , parse_state : 0 , c2rust_unnamed : C2RustUnnamed_5 { c2rust_unnamed : { let mut init = C2RustUnnamed_6 { visible_child_count : 0 , named_child_count : 0 , visible_descendant_count : 0 as libc :: c_int as uint32_t , dynamic_precedence : 0 , repeat_depth : 0 , production_id : production_id as uint16_t , first_leaf : { let mut init = C2RustUnnamed_7 { symbol : 0 as libc :: c_int as TSSymbol , parse_state : 0 as libc :: c_int as TSStateId , } ; init } , } ; init } , } , } ;
        init.set_visible(metadata.visible);
        init.set_named(metadata.named);
        init.set_extra(false);
        init.set_fragile_left(fragile);
        init.set_fragile_right(fragile);
        init.set_has_changes(0 as libc::c_int != 0);
        init.set_has_external_tokens(false);
        init.set_has_external_scanner_state_change(0 as libc::c_int != 0);
        init.set_depends_on_column(false);
        init.set_is_missing(false);
        init.set_is_keyword(0 as libc::c_int != 0);
        init
    };
    let mut result: MutableSubtree = MutableSubtree { ptr: data };
    ts_subtree_summarize_children(result, language);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_error_node(
    mut children: *mut SubtreeArray,
    mut extra: bool,
    mut language: *const TSLanguage,
) -> Subtree {
    let mut result: MutableSubtree = ts_subtree_new_node(
        -(1 as libc::c_int) as TSSymbol,
        children,
        0 as libc::c_int as libc::c_uint,
        language,
    );
    (*result.ptr).set_extra(extra);
    return ts_subtree_from_mut(result);
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_new_missing_leaf(
    mut pool: *mut SubtreePool,
    mut symbol: TSSymbol,
    mut padding: Length,
    mut lookahead_bytes: uint32_t,
    mut language: *const TSLanguage,
) -> Subtree {
    let mut result: Subtree = ts_subtree_new_leaf(
        pool,
        symbol,
        padding,
        length_zero(),
        lookahead_bytes,
        0 as libc::c_int as TSStateId,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        language,
    );
    if (result.data).is_inline() {
        (result.data).set_is_missing(1 as libc::c_int != 0);
    } else {
        let ref mut fresh43 = *(result.ptr as *mut SubtreeHeapData);
        (*fresh43).set_is_missing(1 as libc::c_int != 0);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_retain(mut self_0: Subtree) {
    if (self_0.data).is_inline() {
        return;
    }
    if (*self_0.ptr).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        panic!();
    }
    atomic_inc(&(*self_0.ptr).ref_count as *const uint32_t as *mut uint32_t);
    if (*self_0.ptr).ref_count != 0 as libc::c_int as libc::c_uint {
    } else {
        panic!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_release(mut pool: *mut SubtreePool, mut self_0: Subtree) {
    if (self_0.data).is_inline() {
        return;
    }
    (*pool).tree_stack.size = 0 as libc::c_int as uint32_t;
    if (*self_0.ptr).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        panic!();
    }
    if atomic_dec(&(*self_0.ptr).ref_count as *const uint32_t as *mut uint32_t)
        == 0 as libc::c_int as libc::c_uint
    {
        array__grow(
            &mut (*pool).tree_stack as *mut MutableSubtreeArray as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
        );
        let ref mut fresh44 = (*pool).tree_stack.size;
        let fresh45 = *fresh44;
        *fresh44 = (*fresh44).wrapping_add(1);
        *((*pool).tree_stack.contents).offset(fresh45 as isize) = ts_subtree_to_mut_unsafe(self_0);
    }
    while (*pool).tree_stack.size > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh46 = (*pool).tree_stack.size;
        *fresh46 = (*fresh46).wrapping_sub(1);
        let mut tree: MutableSubtree = *((*pool).tree_stack.contents).offset(*fresh46 as isize);
        if (*tree.ptr).child_count > 0 as libc::c_int as libc::c_uint {
            let mut children: *mut Subtree = if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            };
            let mut i: uint32_t = 0 as libc::c_int as uint32_t;
            while i < (*tree.ptr).child_count {
                let mut child: Subtree = *children.offset(i as isize);
                if !(child.data).is_inline() {
                    if (*child.ptr).ref_count > 0 as libc::c_int as libc::c_uint {
                    } else {
                        panic!();
                    }
                    if atomic_dec(&(*child.ptr).ref_count as *const uint32_t as *mut uint32_t)
                        == 0 as libc::c_int as libc::c_uint
                    {
                        array__grow(
                            &mut (*pool).tree_stack as *mut MutableSubtreeArray as *mut VoidArray,
                            1 as libc::c_int as uint32_t,
                            ::std::mem::size_of::<MutableSubtree>() as libc::c_ulong,
                        );
                        let ref mut fresh47 = (*pool).tree_stack.size;
                        let fresh48 = *fresh47;
                        *fresh47 = (*fresh47).wrapping_add(1);
                        *((*pool).tree_stack.contents).offset(fresh48 as isize) =
                            ts_subtree_to_mut_unsafe(child);
                    }
                }
                i = i.wrapping_add(1);
            }
            crate::core::alloc::ts_free(children as *mut libc::c_void);
        } else {
            if (*tree.ptr).has_external_tokens() {
                ts_external_scanner_state_delete(
                    &mut (*tree.ptr).c2rust_unnamed.external_scanner_state,
                );
            }
            ts_subtree_pool_free(pool, tree.ptr);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_compare(mut left: Subtree, mut right: Subtree) -> libc::c_int {
    if (ts_subtree_symbol(left) as libc::c_int) < ts_subtree_symbol(right) as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (ts_subtree_symbol(right) as libc::c_int) < ts_subtree_symbol(left) as libc::c_int {
        return 1 as libc::c_int;
    }
    if ts_subtree_child_count(left) < ts_subtree_child_count(right) {
        return -(1 as libc::c_int);
    }
    if ts_subtree_child_count(right) < ts_subtree_child_count(left) {
        return 1 as libc::c_int;
    }
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut n: uint32_t = ts_subtree_child_count(left);
    while i < n {
        let mut left_child: Subtree = *if (left.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (left.ptr as *mut Subtree).offset(-((*left.ptr).child_count as isize))
        }
        .offset(i as isize);
        let mut right_child: Subtree = *if (right.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (right.ptr as *mut Subtree).offset(-((*right.ptr).child_count as isize))
        }
        .offset(i as isize);
        match ts_subtree_compare(left_child, right_child) {
            -1 => return -(1 as libc::c_int),
            1 => return 1 as libc::c_int,
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_subtree_set_has_changes(mut self_0: *mut MutableSubtree) {
    if ((*self_0).data).is_inline() {
        let ref mut fresh49 = (*self_0).data;
        (*fresh49).set_has_changes(1 as libc::c_int != 0);
    } else {
        let ref mut fresh50 = *(*self_0).ptr;
        (*fresh50).set_has_changes(1 as libc::c_int != 0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_edit(
    mut self_0: Subtree,
    mut input_edit: *const TSInputEdit,
    mut pool: *mut SubtreePool,
) -> Subtree {
    let mut stack: C2RustUnnamed_8 = {
        let mut init = C2RustUnnamed_8 {
            contents: 0 as *mut StackEntry,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    array__grow(
        &mut stack as *mut C2RustUnnamed_8 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackEntry>() as libc::c_ulong,
    );
    let fresh51 = stack.size;
    stack.size = (stack.size).wrapping_add(1);
    *(stack.contents).offset(fresh51 as isize) = {
        let mut init = StackEntry {
            tree: &mut self_0,
            edit: {
                let mut init = Edit {
                    start: {
                        let mut init = Length {
                            bytes: (*input_edit).start_byte,
                            extent: (*input_edit).start_point,
                        };
                        init
                    },
                    old_end: {
                        let mut init = Length {
                            bytes: (*input_edit).old_end_byte,
                            extent: (*input_edit).old_end_point,
                        };
                        init
                    },
                    new_end: {
                        let mut init = Length {
                            bytes: (*input_edit).new_end_byte,
                            extent: (*input_edit).new_end_point,
                        };
                        init
                    },
                };
                init
            },
        };
        init
    };
    while stack.size != 0 {
        stack.size = (stack.size).wrapping_sub(1);
        let mut entry: StackEntry = *(stack.contents).offset(stack.size as isize);
        let mut edit: Edit = entry.edit;
        let mut is_noop: bool =
            edit.old_end.bytes == edit.start.bytes && edit.new_end.bytes == edit.start.bytes;
        let mut is_pure_insertion: bool = edit.old_end.bytes == edit.start.bytes;
        let mut invalidate_first_row: bool = ts_subtree_depends_on_column(*entry.tree);
        let mut size: Length = ts_subtree_size(*entry.tree);
        let mut padding: Length = ts_subtree_padding(*entry.tree);
        let mut total_size: Length = length_add(padding, size);
        let mut lookahead_bytes: uint32_t = ts_subtree_lookahead_bytes(*entry.tree);
        let mut end_byte: uint32_t = (total_size.bytes).wrapping_add(lookahead_bytes);
        if edit.start.bytes > end_byte
            || is_noop as libc::c_int != 0 && edit.start.bytes == end_byte
        {
            continue;
        }
        if edit.old_end.bytes <= padding.bytes {
            padding = length_add(edit.new_end, length_sub(padding, edit.old_end));
        } else if edit.start.bytes < padding.bytes {
            size = length_saturating_sub(size, length_sub(edit.old_end, padding));
            padding = edit.new_end;
        } else if edit.start.bytes == padding.bytes && is_pure_insertion as libc::c_int != 0 {
            padding = edit.new_end;
        } else if edit.start.bytes < total_size.bytes
            || edit.start.bytes == total_size.bytes && is_pure_insertion as libc::c_int != 0
        {
            size = length_add(
                length_sub(edit.new_end, padding),
                length_saturating_sub(total_size, edit.old_end),
            );
        }
        let mut result: MutableSubtree = ts_subtree_make_mut(pool, *entry.tree);
        if (result.data).is_inline() {
            if ts_subtree_can_inline(padding, size, lookahead_bytes) {
                result.data.padding_bytes = padding.bytes as uint8_t;
                (result.data).set_padding_rows(padding.extent.row as uint8_t);
                result.data.padding_columns = padding.extent.column as uint8_t;
                result.data.size_bytes = size.bytes as uint8_t;
            } else {
                let mut data: *mut SubtreeHeapData = ts_subtree_pool_allocate(pool);
                ::std::ptr::write_volatile(
                    &mut (*data).ref_count as *mut uint32_t,
                    1 as libc::c_int as uint32_t,
                );
                (*data).padding = padding;
                (*data).size = size;
                (*data).lookahead_bytes = lookahead_bytes;
                (*data).error_cost = 0 as libc::c_int as uint32_t;
                (*data).child_count = 0 as libc::c_int as uint32_t;
                (*data).symbol = result.data.symbol as TSSymbol;
                (*data).parse_state = result.data.parse_state;
                (*data).set_visible((result.data).visible());
                (*data).set_named((result.data).named());
                (*data).set_extra((result.data).extra());
                (*data).set_fragile_left(0 as libc::c_int != 0);
                (*data).set_fragile_right(0 as libc::c_int != 0);
                (*data).set_has_changes(0 as libc::c_int != 0);
                (*data).set_has_external_tokens(0 as libc::c_int != 0);
                (*data).set_depends_on_column(0 as libc::c_int != 0);
                (*data).set_is_missing((result.data).is_missing());
                (*data).set_is_keyword((result.data).is_keyword());
                result.ptr = data;
            }
        } else {
            (*result.ptr).padding = padding;
            (*result.ptr).size = size;
        }
        ts_subtree_set_has_changes(&mut result);
        *entry.tree = ts_subtree_from_mut(result);
        let mut child_left: Length = Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        };
        let mut child_right: Length = length_zero();
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut n: uint32_t = ts_subtree_child_count(*entry.tree);
        while i < n {
            let mut child: *mut Subtree =
                &mut *if ((*entry.tree).data).is_inline() as libc::c_int != 0 {
                    0 as *mut Subtree
                } else {
                    ((*entry.tree).ptr as *mut Subtree)
                        .offset(-((*(*entry.tree).ptr).child_count as isize))
                }
                .offset(i as isize) as *mut Subtree;
            let mut child_size: Length = ts_subtree_total_size(*child);
            child_left = child_right;
            child_right = length_add(child_left, child_size);
            if !((child_right.bytes).wrapping_add(ts_subtree_lookahead_bytes(*child))
                < edit.start.bytes)
            {
                if (child_left.bytes > edit.old_end.bytes
                    || child_left.bytes == edit.old_end.bytes
                        && child_size.bytes > 0 as libc::c_int as libc::c_uint
                        && i > 0 as libc::c_int as libc::c_uint)
                    && (!invalidate_first_row
                        || child_left.extent.row > (*(*entry.tree).ptr).padding.extent.row)
                {
                    break;
                }
                let mut child_edit: Edit = {
                    let mut init = Edit {
                        start: length_saturating_sub(edit.start, child_left),
                        old_end: length_saturating_sub(edit.old_end, child_left),
                        new_end: length_saturating_sub(edit.new_end, child_left),
                    };
                    init
                };
                if child_right.bytes > edit.start.bytes
                    || child_right.bytes == edit.start.bytes
                        && is_pure_insertion as libc::c_int != 0
                {
                    edit.new_end = edit.start;
                } else {
                    child_edit.old_end = child_edit.start;
                    child_edit.new_end = child_edit.start;
                }
                array__grow(
                    &mut stack as *mut C2RustUnnamed_8 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<StackEntry>() as libc::c_ulong,
                );
                let fresh52 = stack.size;
                stack.size = (stack.size).wrapping_add(1);
                *(stack.contents).offset(fresh52 as isize) = {
                    let mut init = StackEntry {
                        tree: child,
                        edit: child_edit,
                    };
                    init
                };
            }
            i = i.wrapping_add(1);
        }
    }
    array__delete(&mut stack as *mut C2RustUnnamed_8 as *mut VoidArray);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_last_external_token(mut tree: Subtree) -> Subtree {
    if !ts_subtree_has_external_tokens(tree) {
        return Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
    }
    while (*tree.ptr).child_count > 0 as libc::c_int as libc::c_uint {
        let mut i: uint32_t =
            ((*tree.ptr).child_count).wrapping_sub(1 as libc::c_int as libc::c_uint);
        while i.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
            let mut child: Subtree = *if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            }
            .offset(i as isize);
            if ts_subtree_has_external_tokens(child) {
                tree = child;
                break;
            } else {
                i = i.wrapping_sub(1);
            }
        }
    }
    return tree;
}
unsafe extern "C" fn ts_subtree__write_char_to_string(
    mut str: *mut libc::c_char,
    mut n: size_t,
    mut chr: int32_t,
) -> size_t {
    if chr == -(1 as libc::c_int) {
        return snwrite!(str, n as usize, "INVALID",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    } else if chr == '\0' as i32 {
        return snwrite!(str, n as usize, "'\\0'",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    } else if chr == '\n' as i32 {
        return snwrite!(str, n as usize, "'\\n'",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    } else if chr == '\t' as i32 {
        return snwrite!(str, n as usize, "'\\t'",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    } else if chr == '\r' as i32 {
        return snwrite!(str, n as usize, "'\\r'",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    } else if (0 as libc::c_int) < chr
        && chr < 128 as libc::c_int
        && ((chr as u8).is_ascii_graphic() || chr == ' ' as i32)
    {
        return snwrite!(str, n as usize, "'{}'", chr as u8 as char).unwrap_or(usize::MAX)
            as os::raw::c_int as size_t;
    } else {
        return snwrite!(str, n as usize, "{}", chr).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    };
}
static mut ROOT_FIELD: *const libc::c_char = b"__ROOT__\0" as *const u8 as *const libc::c_char;
unsafe extern "C" fn ts_subtree__write_to_string(
    mut self_0: Subtree,
    mut string: *mut libc::c_char,
    mut limit: size_t,
    mut language: *const TSLanguage,
    mut include_all: bool,
    mut alias_symbol: TSSymbol,
    mut alias_is_named: bool,
    mut field_name: *const libc::c_char,
) -> size_t {
    if (self_0.ptr).is_null() {
        return snwrite!(string, limit as usize, "(NULL)",).unwrap_or(usize::MAX) as os::raw::c_int
            as size_t;
    }
    let mut cursor: *mut libc::c_char = string;
    let mut writer: *mut *mut libc::c_char = if limit > 1 as libc::c_int as libc::c_ulong {
        &mut cursor
    } else {
        &mut string
    };
    let mut is_root: bool = field_name == ROOT_FIELD;
    let mut is_visible: bool = include_all as libc::c_int != 0
        || ts_subtree_missing(self_0) as libc::c_int != 0
        || (if alias_symbol as libc::c_int != 0 {
            alias_is_named as libc::c_int
        } else {
            (ts_subtree_visible(self_0) as libc::c_int != 0
                && ts_subtree_named(self_0) as libc::c_int != 0) as libc::c_int
        }) != 0;
    if is_visible {
        if !is_root {
            cursor = cursor.offset(
                snwrite!(*writer, limit as usize, " ",).unwrap_or(usize::MAX) as os::raw::c_int
                    as isize,
            );
            if !field_name.is_null() {
                cursor = cursor.offset(
                    snwrite!(
                        *writer,
                        limit as usize,
                        "{}: ",
                        std::ffi::CStr::from_ptr(field_name).to_string_lossy()
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int as isize,
                );
            }
        }
        if ts_subtree_is_error(self_0) as libc::c_int != 0
            && ts_subtree_child_count(self_0) == 0 as libc::c_int as libc::c_uint
            && (*self_0.ptr).size.bytes > 0 as libc::c_int as libc::c_uint
        {
            cursor = cursor.offset(
                snwrite!(*writer, limit as usize, "(UNEXPECTED ",).unwrap_or(usize::MAX)
                    as os::raw::c_int as isize,
            );
            cursor = cursor.offset(ts_subtree__write_char_to_string(
                *writer,
                limit,
                (*self_0.ptr).c2rust_unnamed.lookahead_char,
            ) as isize);
        } else {
            let mut symbol: TSSymbol = (if alias_symbol as libc::c_int != 0 {
                alias_symbol as libc::c_int
            } else {
                ts_subtree_symbol(self_0) as libc::c_int
            }) as TSSymbol;
            let mut symbol_name: *const libc::c_char = ts_language_symbol_name(language, symbol);
            if ts_subtree_missing(self_0) {
                cursor = cursor.offset(
                    snwrite!(*writer, limit as usize, "(MISSING ",).unwrap_or(usize::MAX)
                        as os::raw::c_int as isize,
                );
                if alias_is_named as libc::c_int != 0
                    || ts_subtree_named(self_0) as libc::c_int != 0
                {
                    cursor = cursor.offset(
                        snwrite!(
                            *writer,
                            limit as usize,
                            "{}",
                            std::ffi::CStr::from_ptr(symbol_name).to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int as isize,
                    );
                } else {
                    cursor = cursor.offset(
                        snwrite!(
                            *writer,
                            limit as usize,
                            "\"{}\"",
                            std::ffi::CStr::from_ptr(symbol_name).to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int as isize,
                    );
                }
            } else {
                cursor = cursor.offset(
                    snwrite!(
                        *writer,
                        limit as usize,
                        "({}",
                        std::ffi::CStr::from_ptr(symbol_name).to_string_lossy()
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int as isize,
                );
            }
        }
    } else if is_root {
        let mut symbol_0: TSSymbol = ts_subtree_symbol(self_0);
        let mut symbol_name_0: *const libc::c_char = ts_language_symbol_name(language, symbol_0);
        cursor = cursor.offset(
            snwrite!(
                *writer,
                limit as usize,
                "(\"{}\")",
                std::ffi::CStr::from_ptr(symbol_name_0).to_string_lossy()
            )
            .unwrap_or(usize::MAX) as os::raw::c_int as isize,
        );
    }
    if ts_subtree_child_count(self_0) != 0 {
        let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
            language,
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id as uint32_t,
        );
        let mut field_map: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
        let mut field_map_end: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
        ts_language_field_map(
            language,
            (*self_0.ptr).c2rust_unnamed.c2rust_unnamed.production_id as uint32_t,
            &mut field_map,
            &mut field_map_end,
        );
        let mut structural_child_index: uint32_t = 0 as libc::c_int as uint32_t;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < (*self_0.ptr).child_count {
            let mut child: Subtree = *if (self_0.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (self_0.ptr as *mut Subtree).offset(-((*self_0.ptr).child_count as isize))
            }
            .offset(i as isize);
            if ts_subtree_extra(child) {
                cursor = cursor.offset(ts_subtree__write_to_string(
                    child,
                    *writer,
                    limit,
                    language,
                    include_all,
                    0 as libc::c_int as TSSymbol,
                    0 as libc::c_int != 0,
                    0 as *const libc::c_char,
                ) as isize);
            } else {
                let mut subtree_alias_symbol: TSSymbol = (if !alias_sequence.is_null() {
                    *alias_sequence.offset(structural_child_index as isize) as libc::c_int
                } else {
                    0 as libc::c_int
                }) as TSSymbol;
                let mut subtree_alias_is_named: bool = if subtree_alias_symbol as libc::c_int != 0 {
                    (ts_language_symbol_metadata(language, subtree_alias_symbol)).named
                        as libc::c_int
                } else {
                    0 as libc::c_int
                } != 0;
                let mut child_field_name: *const libc::c_char = if is_visible as libc::c_int != 0 {
                    0 as *const libc::c_char
                } else {
                    field_name
                };
                let mut map: *const TSFieldMapEntry = field_map;
                while map < field_map_end {
                    if !(*map).inherited
                        && (*map).child_index as libc::c_uint == structural_child_index
                    {
                        child_field_name =
                            *((*language).field_names).offset((*map).field_id as isize);
                        break;
                    } else {
                        map = map.offset(1);
                    }
                }
                cursor = cursor.offset(ts_subtree__write_to_string(
                    child,
                    *writer,
                    limit,
                    language,
                    include_all,
                    subtree_alias_symbol,
                    subtree_alias_is_named,
                    child_field_name,
                ) as isize);
                structural_child_index = structural_child_index.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    if is_visible {
        cursor = cursor.offset(
            snwrite!(*writer, limit as usize, ")",).unwrap_or(usize::MAX) as os::raw::c_int
                as isize,
        );
    }
    return cursor.offset_from(string) as libc::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_string(
    mut self_0: Subtree,
    mut language: *const TSLanguage,
    mut include_all: bool,
) -> *mut libc::c_char {
    let mut scratch_string: [libc::c_char; 1] = [0; 1];
    let mut size: size_t = (ts_subtree__write_to_string(
        self_0,
        scratch_string.as_mut_ptr(),
        1 as libc::c_int as size_t,
        language,
        include_all,
        0 as libc::c_int as TSSymbol,
        0 as libc::c_int != 0,
        ROOT_FIELD,
    ))
    .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut result: *mut libc::c_char = crate::core::alloc::ts_malloc(
        size.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    ts_subtree__write_to_string(
        self_0,
        result,
        size,
        language,
        include_all,
        0 as libc::c_int as TSSymbol,
        0 as libc::c_int != 0,
        ROOT_FIELD,
    );
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree__print_dot_graph(
    mut self_0: *const Subtree,
    mut start_offset: uint32_t,
    mut language: *const TSLanguage,
    mut alias_symbol: TSSymbol,
    mut f: *mut FILE,
) {
    let mut subtree_symbol: TSSymbol = ts_subtree_symbol(*self_0);
    let mut symbol: TSSymbol = (if alias_symbol as libc::c_int != 0 {
        alias_symbol as libc::c_int
    } else {
        subtree_symbol as libc::c_int
    }) as TSSymbol;
    let mut end_offset: uint32_t = start_offset.wrapping_add(ts_subtree_total_bytes(*self_0));
    fwrite!(
        f,
        "tree_{:p} [label=\"",
        self_0 as *mut libc::c_void as *const os::raw::c_int
    )
    .unwrap_or(usize::MAX) as os::raw::c_int;
    ts_language_write_symbol_as_dot_string(language, f, symbol);
    fwrite!(f, "\"",).unwrap_or(usize::MAX) as os::raw::c_int;
    if ts_subtree_child_count(*self_0) == 0 as libc::c_int as libc::c_uint {
        fwrite!(f, ", shape=plaintext",).unwrap_or(usize::MAX) as os::raw::c_int;
    }
    if ts_subtree_extra(*self_0) {
        fwrite!(f, ", fontcolor=gray",).unwrap_or(usize::MAX) as os::raw::c_int;
    }
    fwrite ! (f , ", tooltip=\"range: {} - {}\nstate: {}\nerror-cost: {}\nhas-changes: {}\ndepends-on-column: {}\ndescendant-count: {}\nrepeat-depth: {}\nlookahead-bytes: {}" , start_offset , end_offset , ts_subtree_parse_state (* self_0) as libc :: c_int , ts_subtree_error_cost (* self_0) , ts_subtree_has_changes (* self_0) as libc :: c_int , ts_subtree_depends_on_column (* self_0) as libc :: c_int , ts_subtree_visible_descendant_count (* self_0) , ts_subtree_repeat_depth (* self_0) , ts_subtree_lookahead_bytes (* self_0)) . unwrap_or (usize :: MAX) as os :: raw :: c_int ;
    if ts_subtree_is_error(*self_0) as libc::c_int != 0
        && ts_subtree_child_count(*self_0) == 0 as libc::c_int as libc::c_uint
    {
        fwrite!(
            f,
            "\ncharacter: '{}'",
            (*(*self_0).ptr).c2rust_unnamed.lookahead_char as u8 as char
        )
        .unwrap_or(usize::MAX) as os::raw::c_int;
    }
    fwrite!(f, "\"]\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    let mut child_start_offset: uint32_t = start_offset;
    let mut child_info_offset: uint32_t = ((*language).max_alias_sequence_length as libc::c_int
        * ts_subtree_production_id(*self_0) as libc::c_int)
        as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut n: uint32_t = ts_subtree_child_count(*self_0);
    while i < n {
        let mut child: *const Subtree = &mut *if ((*self_0).data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            ((*self_0).ptr as *mut Subtree).offset(-((*(*self_0).ptr).child_count as isize))
        }
        .offset(i as isize) as *mut Subtree;
        let mut subtree_alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
        if !ts_subtree_extra(*child) && child_info_offset != 0 {
            subtree_alias_symbol =
                *((*language).alias_sequences).offset(child_info_offset as isize);
            child_info_offset = child_info_offset.wrapping_add(1);
        }
        ts_subtree__print_dot_graph(child, child_start_offset, language, subtree_alias_symbol, f);
        fwrite!(
            f,
            "tree_{:p} -> tree_{:p} [tooltip={}]\n",
            self_0 as *mut libc::c_void as *const os::raw::c_int,
            child as *mut libc::c_void as *const os::raw::c_int,
            i
        )
        .unwrap_or(usize::MAX) as os::raw::c_int;
        child_start_offset = (child_start_offset as libc::c_uint)
            .wrapping_add(ts_subtree_total_bytes(*child)) as uint32_t
            as uint32_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_print_dot_graph(
    mut self_0: Subtree,
    mut language: *const TSLanguage,
    mut f: *mut FILE,
) {
    fwrite!(f, "digraph tree {{\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    fwrite!(f, "edge [arrowhead=none]\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    ts_subtree__print_dot_graph(
        &mut self_0,
        0 as libc::c_int as uint32_t,
        language,
        0 as libc::c_int as TSSymbol,
        f,
    );
    fwrite!(f, "}}\n",).unwrap_or(usize::MAX) as os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_external_scanner_state(
    mut self_0: Subtree,
) -> *const ExternalScannerState {
    static mut empty_state: ExternalScannerState = {
        let mut init = ExternalScannerState {
            c2rust_unnamed: C2RustUnnamed_4 {
                short_data: [
                    0 as libc::c_int as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
            },
            length: 0 as libc::c_int as uint32_t,
        };
        init
    };
    if !(self_0.ptr).is_null()
        && !(self_0.data).is_inline()
        && (*self_0.ptr).has_external_tokens() as libc::c_int != 0
        && (*self_0.ptr).child_count == 0 as libc::c_int as libc::c_uint
    {
        return &(*self_0.ptr).c2rust_unnamed.external_scanner_state;
    } else {
        return &empty_state;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_subtree_external_scanner_state_eq(
    mut self_0: Subtree,
    mut other: Subtree,
) -> bool {
    let mut state_self: *const ExternalScannerState = ts_subtree_external_scanner_state(self_0);
    let mut state_other: *const ExternalScannerState = ts_subtree_external_scanner_state(other);
    return ts_external_scanner_state_eq(
        state_self,
        ts_external_scanner_state_data(state_other),
        (*state_other).length,
    );
}
