use crate::core::util::*;
use crate::core::*;
use :: c2rust_bitfields;
use std::os;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
pub struct C2RustUnnamed {
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
pub struct TSLexer {
    pub lookahead: int32_t,
    pub result_symbol: TSSymbol,
    pub advance: Option<unsafe extern "C" fn(*mut TSLexer, bool) -> ()>,
    pub mark_end: Option<unsafe extern "C" fn(*mut TSLexer) -> ()>,
    pub get_column: Option<unsafe extern "C" fn(*mut TSLexer) -> uint32_t>,
    pub is_at_included_range_start: Option<unsafe extern "C" fn(*const TSLexer) -> bool>,
    pub eof: Option<unsafe extern "C" fn(*const TSLexer) -> bool>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLexMode {
    pub lex_state: uint16_t,
    pub external_lex_state: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSSymbolMetadata {
    pub visible: bool,
    pub named: bool,
    pub supertype: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSFieldMapEntry {
    pub field_id: TSFieldId,
    pub child_index: uint8_t,
    pub inherited: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSFieldMapSlice {
    pub index: uint16_t,
    pub length: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union TSParseActionEntry {
    pub action: TSParseAction,
    pub entry: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub count: uint8_t,
    pub reusable: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union TSParseAction {
    pub shift: C2RustUnnamed_2,
    pub reduce: C2RustUnnamed_1,
    pub type_: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub type_: uint8_t,
    pub child_count: uint8_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: int16_t,
    pub production_id: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub type_: uint8_t,
    pub state: TSStateId,
    pub extra: bool,
    pub repetition: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSParser {
    pub lexer: Lexer,
    pub stack: *mut Stack,
    pub tree_pool: SubtreePool,
    pub language: *const TSLanguage,
    pub reduce_actions: ReduceActionSet,
    pub finished_tree: Subtree,
    pub trailing_extras: SubtreeArray,
    pub trailing_extras2: SubtreeArray,
    pub scratch_trees: SubtreeArray,
    pub token_cache: TokenCache,
    pub reusable_node: ReusableNode,
    pub external_scanner_payload: *mut libc::c_void,
    pub dot_graph_file: *mut FILE,
    pub end_clock: TSClock,
    pub timeout_duration: TSDuration,
    pub accept_count: libc::c_uint,
    pub operation_count: libc::c_uint,
    pub cancellation_flag: *const size_t,
    pub old_tree: Subtree,
    pub included_range_differences: TSRangeArray,
    pub included_range_difference_index: libc::c_uint,
}
type C2RustUnnamed_3 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_4 = crate::core::util::LongShortData;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
pub type TSDuration = uint64_t;
pub type TSClock = timespec;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReusableNode {
    pub stack: C2RustUnnamed_7,
    pub last_external_token: Subtree,
}
type C2RustUnnamed_7 = crate::core::util::StackElement<*mut StackEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry {
    pub tree: Subtree,
    pub child_index: uint32_t,
    pub byte_offset: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TokenCache {
    pub token: Subtree,
    pub last_external_token: Subtree,
    pub byte_index: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReduceActionSet {
    pub contents: *mut ReduceAction,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReduceAction {
    pub count: uint32_t,
    pub symbol: TSSymbol,
    pub dynamic_precedence: libc::c_int,
    pub production_id: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
pub type StackVersion = libc::c_uint;
pub const TSParseActionTypeShift: C2RustUnnamed_8 = 0;
pub const ErrorComparisonPreferRight: ErrorComparison = 3;
pub const ErrorComparisonTakeRight: ErrorComparison = 4;
pub type ErrorComparison = libc::c_uint;
pub const ErrorComparisonNone: ErrorComparison = 2;
pub const ErrorComparisonPreferLeft: ErrorComparison = 1;
pub const ErrorComparisonTakeLeft: ErrorComparison = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ErrorStatus {
    pub cost: libc::c_uint,
    pub node_count: libc::c_uint,
    pub dynamic_precedence: libc::c_int,
    pub is_in_error: bool,
}
pub const TSParseActionTypeReduce: C2RustUnnamed_8 = 1;
pub const TSParseActionTypeRecover: C2RustUnnamed_8 = 3;
pub const TSParseActionTypeAccept: C2RustUnnamed_8 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSStringInput {
    pub string: *const libc::c_char,
    pub length: uint32_t,
}
pub type C2RustUnnamed_8 = libc::c_uint;
#[inline]
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    crate::core::alloc::ts_free((*self_0).contents);
    let ref mut fresh0 = (*self_0).contents;
    *fresh0 = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn array__erase(
    mut self_0: *mut VoidArray,
    mut element_size: size_t,
    mut index: uint32_t,
) {
    if index < (*self_0).size {
    } else {
        panic!();
    }
    let mut contents: *mut libc::c_char = (*self_0).contents as *mut libc::c_char;
    std::ptr::copy(
        contents.offset(
            (index.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                .wrapping_mul(element_size) as isize,
        ) as *const libc::c_void,
        contents.offset((index as libc::c_ulong).wrapping_mul(element_size) as isize)
            as *mut libc::c_void,
        ((((*self_0).size)
            .wrapping_sub(index)
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(element_size)) as usize,
    );
    let ref mut fresh1 = (*self_0).size;
    *fresh1 = (*fresh1).wrapping_sub(1);
}
#[inline]
unsafe extern "C" fn array__reserve(
    mut self_0: *mut VoidArray,
    mut element_size: size_t,
    mut new_capacity: uint32_t,
) {
    if new_capacity > (*self_0).capacity {
        if !((*self_0).contents).is_null() {
            let ref mut fresh2 = (*self_0).contents;
            *fresh2 = crate::core::alloc::ts_realloc(
                (*self_0).contents,
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            let ref mut fresh3 = (*self_0).contents;
            *fresh3 = crate::core::alloc::ts_malloc(
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        }
        (*self_0).capacity = new_capacity;
    }
}
#[inline]
unsafe extern "C" fn array__assign(
    mut self_0: *mut VoidArray,
    mut other: *const VoidArray,
    mut element_size: size_t,
) {
    array__reserve(self_0, element_size, (*other).size);
    (*self_0).size = (*other).size;
    std::ptr::copy_nonoverlapping(
        (*other).contents,
        (*self_0).contents,
        ((*self_0).size as libc::c_ulong).wrapping_mul(element_size),
    );
}
#[inline]
unsafe extern "C" fn array__swap(mut self_0: *mut VoidArray, mut other: *mut VoidArray) {
    let mut swap: VoidArray = *other;
    *other = *self_0;
    *self_0 = swap;
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
unsafe extern "C" fn array__splice(
    mut self_0: *mut VoidArray,
    mut element_size: size_t,
    mut index: uint32_t,
    mut old_count: uint32_t,
    mut new_count: uint32_t,
    mut elements: *const libc::c_void,
) {
    let mut new_size: uint32_t = ((*self_0).size)
        .wrapping_add(new_count)
        .wrapping_sub(old_count);
    let mut old_end: uint32_t = index.wrapping_add(old_count);
    let mut new_end: uint32_t = index.wrapping_add(new_count);
    if old_end <= (*self_0).size {
    } else {
        panic!();
    }
    array__reserve(self_0, element_size, new_size);
    let mut contents: *mut libc::c_char = (*self_0).contents as *mut libc::c_char;
    if (*self_0).size > old_end {
        std::ptr::copy(
            contents.offset((old_end as libc::c_ulong).wrapping_mul(element_size) as isize)
                as *const libc::c_void,
            contents.offset((new_end as libc::c_ulong).wrapping_mul(element_size) as isize)
                as *mut libc::c_void,
            ((((*self_0).size).wrapping_sub(old_end) as libc::c_ulong).wrapping_mul(element_size))
                as usize,
        );
    }
    if new_count > 0 as libc::c_int as libc::c_uint {
        if !elements.is_null() {
            std::ptr::copy_nonoverlapping(
                elements,
                contents.offset((index as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                (new_count as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            std::ptr::write_bytes(
                contents.offset((index as libc::c_ulong).wrapping_mul(element_size) as isize)
                    as *mut libc::c_void,
                (0 as libc::c_int) as u8,
                ((new_count as libc::c_ulong).wrapping_mul(element_size)) as usize,
            );
        }
    }
    let ref mut fresh4 = (*self_0).size;
    *fresh4 = (*fresh4 as libc::c_uint).wrapping_add(new_count.wrapping_sub(old_count)) as uint32_t
        as uint32_t;
}
#[inline]
unsafe extern "C" fn duration_from_micros(mut micros: uint64_t) -> TSDuration {
    return micros;
}
#[inline]
unsafe extern "C" fn duration_to_micros(mut self_0: TSDuration) -> uint64_t {
    return self_0;
}
#[inline]
unsafe extern "C" fn clock_null() -> TSClock {
    return {
        let mut init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn clock_after(mut base: TSClock, mut duration: TSDuration) -> TSClock {
    let mut result: TSClock = base;
    result.tv_sec = (result.tv_sec as libc::c_ulong)
        .wrapping_add((duration as usize).wrapping_div(1000000 as libc::c_int as libc::c_ulong))
        as __time_t as __time_t;
    result.tv_nsec = (result.tv_nsec as libc::c_ulong).wrapping_add(
        (duration as usize)
            .wrapping_rem(1000000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(1000 as libc::c_int as libc::c_ulong),
    ) as __syscall_slong_t as __syscall_slong_t;
    if result.tv_nsec >= 1000000000 as libc::c_int as libc::c_long {
        result.tv_nsec -= 1000000000 as libc::c_int as libc::c_long;
        result.tv_sec += 1;
    }
    return result;
}
#[inline]
unsafe extern "C" fn clock_is_null(mut self_0: TSClock) -> bool {
    return self_0.tv_sec == 0;
}
#[inline]
unsafe extern "C" fn clock_is_gt(mut self_0: TSClock, mut other: TSClock) -> bool {
    if self_0.tv_sec > other.tv_sec {
        return 1 as libc::c_int != 0;
    }
    if self_0.tv_sec < other.tv_sec {
        return 0 as libc::c_int != 0;
    }
    return self_0.tv_nsec > other.tv_nsec;
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
unsafe extern "C" fn ts_subtree_has_external_tokens(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (*self_0.ptr).has_external_tokens() as libc::c_int
    } != 0;
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
unsafe extern "C" fn ts_subtree_missing(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).is_missing() as libc::c_int
    } else {
        (*self_0.ptr).is_missing() as libc::c_int
    } != 0;
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
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr).child_count
    };
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
unsafe extern "C" fn ts_subtree_is_eof(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int == 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_subtree_set_extra(mut self_0: *mut MutableSubtree, mut is_extra: bool) {
    if ((*self_0).data).is_inline() {
        let ref mut fresh5 = (*self_0).data;
        (*fresh5).set_extra(is_extra);
    } else {
        let ref mut fresh6 = *(*self_0).ptr;
        (*fresh6).set_extra(is_extra);
    };
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
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.symbol as libc::c_int
    } else {
        (*self_0.ptr).symbol as libc::c_int
    }) as TSSymbol;
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
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new((a.row).wrapping_add(b.row), b.column);
    } else {
        return point__new(a.row, (a.column).wrapping_add(b.column));
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_is_error(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int;
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
unsafe extern "C" fn ts_subtree_parse_state(mut self_0: Subtree) -> TSStateId {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.parse_state as libc::c_int
    } else {
        (*self_0.ptr).parse_state as libc::c_int
    }) as TSStateId;
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
unsafe extern "C" fn ts_subtree_lookahead_bytes(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).lookahead_bytes() as libc::c_uint
    } else {
        (*self_0.ptr).lookahead_bytes
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
unsafe extern "C" fn ts_subtree_is_keyword(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).is_keyword() as libc::c_int
    } else {
        (*self_0.ptr).is_keyword() as libc::c_int
    } != 0;
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
unsafe extern "C" fn ts_subtree_is_fragile(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        ((*self_0.ptr).fragile_left() as libc::c_int != 0
            || (*self_0.ptr).fragile_right() as libc::c_int != 0) as libc::c_int
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
unsafe extern "C" fn ts_language_actions(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
    mut count: *mut uint32_t,
) -> *const TSParseAction {
    let mut entry: TableEntry = TableEntry {
        actions: 0 as *const TSParseAction,
        action_count: 0,
        is_reusable: false,
    };
    ts_language_table_entry(self_0, state, symbol, &mut entry);
    *count = entry.action_count;
    return entry.actions;
}
#[inline]
unsafe extern "C" fn ts_language_has_reduce_action(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
) -> bool {
    let mut entry: TableEntry = TableEntry {
        actions: 0 as *const TSParseAction,
        action_count: 0,
        is_reusable: false,
    };
    ts_language_table_entry(self_0, state, symbol, &mut entry);
    return entry.action_count > 0 as libc::c_int as libc::c_uint
        && (*(entry.actions).offset(0 as libc::c_int as isize)).type_ as libc::c_int
            == TSParseActionTypeReduce as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_language_lookup(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
) -> uint16_t {
    if state as libc::c_uint >= (*self_0).large_state_count {
        let mut index: uint32_t = *((*self_0).small_parse_table_map)
            .offset((state as libc::c_uint).wrapping_sub((*self_0).large_state_count) as isize);
        let mut data: *const uint16_t =
            &*((*self_0).small_parse_table).offset(index as isize) as *const uint16_t;
        let fresh7 = data;
        data = data.offset(1);
        let mut group_count: uint16_t = *fresh7;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < group_count as libc::c_uint {
            let fresh8 = data;
            data = data.offset(1);
            let mut section_value: uint16_t = *fresh8;
            let fresh9 = data;
            data = data.offset(1);
            let mut symbol_count: uint16_t = *fresh9;
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j < symbol_count as libc::c_uint {
                let fresh10 = data;
                data = data.offset(1);
                if *fresh10 as libc::c_int == symbol as libc::c_int {
                    return section_value;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        return 0 as libc::c_int as uint16_t;
    } else {
        return *((*self_0).parse_table).offset(
            (state as libc::c_uint)
                .wrapping_mul((*self_0).symbol_count)
                .wrapping_add(symbol as libc::c_uint) as isize,
        );
    };
}
#[inline]
unsafe extern "C" fn ts_language_has_actions(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
) -> bool {
    return ts_language_lookup(self_0, state, symbol) as libc::c_int != 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_language_enabled_external_tokens(
    mut self_0: *const TSLanguage,
    mut external_scanner_state: libc::c_uint,
) -> *const bool {
    if external_scanner_state == 0 as libc::c_int as libc::c_uint {
        return 0 as *const bool;
    } else {
        return ((*self_0).external_scanner.states).offset(
            ((*self_0).external_token_count).wrapping_mul(external_scanner_state) as isize,
        );
    };
}
#[inline]
unsafe extern "C" fn ts_reduce_action_set_add(
    mut self_0: *mut ReduceActionSet,
    mut new_action: ReduceAction,
) {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).size {
        let mut action: ReduceAction = *((*self_0).contents).offset(i as isize);
        if action.symbol as libc::c_int == new_action.symbol as libc::c_int
            && action.count == new_action.count
        {
            return;
        }
        i = i.wrapping_add(1);
    }
    array__grow(
        self_0 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<ReduceAction>() as libc::c_ulong,
    );
    let ref mut fresh11 = (*self_0).size;
    let fresh12 = *fresh11;
    *fresh11 = (*fresh11).wrapping_add(1);
    *((*self_0).contents).offset(fresh12 as isize) = new_action;
}
#[inline]
unsafe extern "C" fn reusable_node_new() -> ReusableNode {
    return {
        let mut init = ReusableNode {
            stack: {
                let mut init = C2RustUnnamed_7 {
                    contents: 0 as *mut StackEntry,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            last_external_token: Subtree {
                ptr: 0 as *const SubtreeHeapData,
            },
        };
        init
    };
}
#[inline]
unsafe extern "C" fn reusable_node_clear(mut self_0: *mut ReusableNode) {
    (*self_0).stack.size = 0 as libc::c_int as uint32_t;
    (*self_0).last_external_token = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
}
#[inline]
unsafe extern "C" fn reusable_node_tree(mut self_0: *mut ReusableNode) -> Subtree {
    return if (*self_0).stack.size > 0 as libc::c_int as libc::c_uint {
        (*((*self_0).stack.contents)
            .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
        .tree
    } else {
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        }
    };
}
#[inline]
unsafe extern "C" fn reusable_node_byte_offset(mut self_0: *mut ReusableNode) -> uint32_t {
    return if (*self_0).stack.size > 0 as libc::c_int as libc::c_uint {
        (*((*self_0).stack.contents)
            .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
        .byte_offset
    } else {
        4294967295 as libc::c_uint
    };
}
#[inline]
unsafe extern "C" fn reusable_node_delete(mut self_0: *mut ReusableNode) {
    array__delete(&mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut VoidArray);
}
#[inline]
unsafe extern "C" fn reusable_node_advance(mut self_0: *mut ReusableNode) {
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: StackEntry = *(&mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut StackEntry);
    let mut byte_offset: uint32_t =
        (last_entry.byte_offset).wrapping_add(ts_subtree_total_bytes(last_entry.tree));
    if ts_subtree_has_external_tokens(last_entry.tree) {
        (*self_0).last_external_token = ts_subtree_last_external_token(last_entry.tree);
    }
    let mut tree: Subtree = Subtree {
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
    let mut next_index: uint32_t = 0;
    loop {
        let ref mut fresh13 = (*self_0).stack.size;
        *fresh13 = (*fresh13).wrapping_sub(1);
        let mut popped_entry: StackEntry = *((*self_0).stack.contents).offset(*fresh13 as isize);
        next_index = (popped_entry.child_index).wrapping_add(1 as libc::c_int as libc::c_uint);
        if (*self_0).stack.size == 0 as libc::c_int as libc::c_uint {
            return;
        }
        if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            < (*self_0).stack.size
        {
        } else {
            panic!();
        }
        tree = (*(&mut *((*self_0).stack.contents)
            .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut StackEntry))
            .tree;
        if !(ts_subtree_child_count(tree) <= next_index) {
            break;
        }
    }
    array__grow(
        &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackEntry>() as libc::c_ulong,
    );
    let ref mut fresh14 = (*self_0).stack.size;
    let fresh15 = *fresh14;
    *fresh14 = (*fresh14).wrapping_add(1);
    *((*self_0).stack.contents).offset(fresh15 as isize) = {
        let mut init = StackEntry {
            tree: *(if (tree.data).is_inline() as libc::c_int != 0 {
                0 as *mut Subtree
            } else {
                (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
            })
            .offset(next_index as isize),
            child_index: next_index,
            byte_offset: byte_offset,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn reusable_node_descend(mut self_0: *mut ReusableNode) -> bool {
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: StackEntry = *(&mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut StackEntry);
    if ts_subtree_child_count(last_entry.tree) > 0 as libc::c_int as libc::c_uint {
        array__grow(
            &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<StackEntry>() as libc::c_ulong,
        );
        let ref mut fresh16 = (*self_0).stack.size;
        let fresh17 = *fresh16;
        *fresh16 = (*fresh16).wrapping_add(1);
        *((*self_0).stack.contents).offset(fresh17 as isize) = {
            let mut init = StackEntry {
                tree: *(if (last_entry.tree.data).is_inline() as libc::c_int != 0 {
                    0 as *mut Subtree
                } else {
                    (last_entry.tree.ptr as *mut Subtree)
                        .offset(-((*last_entry.tree.ptr).child_count as isize))
                })
                .offset(0 as libc::c_int as isize),
                child_index: 0 as libc::c_int as uint32_t,
                byte_offset: last_entry.byte_offset,
            };
            init
        };
        return 1 as libc::c_int != 0;
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[inline]
unsafe extern "C" fn reusable_node_advance_past_leaf(mut self_0: *mut ReusableNode) {
    while reusable_node_descend(self_0) {}
    reusable_node_advance(self_0);
}
#[inline]
unsafe extern "C" fn reusable_node_reset(mut self_0: *mut ReusableNode, mut tree: Subtree) {
    reusable_node_clear(self_0);
    array__grow(
        &mut (*self_0).stack as *mut C2RustUnnamed_7 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackEntry>() as libc::c_ulong,
    );
    let ref mut fresh18 = (*self_0).stack.size;
    let fresh19 = *fresh18;
    *fresh18 = (*fresh18).wrapping_add(1);
    *((*self_0).stack.contents).offset(fresh19 as isize) = {
        let mut init = StackEntry {
            tree: tree,
            child_index: 0 as libc::c_int as uint32_t,
            byte_offset: 0 as libc::c_int as uint32_t,
        };
        init
    };
    if !reusable_node_descend(self_0) {
        reusable_node_clear(self_0);
    }
}
static mut MAX_VERSION_COUNT: libc::c_uint = 6 as libc::c_int as libc::c_uint;
static mut MAX_VERSION_COUNT_OVERFLOW: libc::c_uint = 4 as libc::c_int as libc::c_uint;
static mut MAX_SUMMARY_DEPTH: libc::c_uint = 16 as libc::c_int as libc::c_uint;
static mut MAX_COST_DIFFERENCE: libc::c_uint =
    (16 as libc::c_int * 100 as libc::c_int) as libc::c_uint;
static mut OP_COUNT_PER_TIMEOUT_CHECK: libc::c_uint = 100 as libc::c_int as libc::c_uint;
unsafe extern "C" fn ts_string_input_read(
    mut _self: *mut libc::c_void,
    mut byte: uint32_t,
    mut point: TSPoint,
    mut length: *mut uint32_t,
) -> *const libc::c_char {
    let mut self_0: *mut TSStringInput = _self as *mut TSStringInput;
    if byte >= (*self_0).length {
        *length = 0 as libc::c_int as uint32_t;
        return b"\0" as *const u8 as *const libc::c_char;
    } else {
        *length = ((*self_0).length).wrapping_sub(byte);
        return ((*self_0).string).offset(byte as isize);
    };
}
unsafe extern "C" fn ts_parser__log(mut self_0: *mut TSParser) {
    if ((*self_0).lexer.logger.log).is_some() {
        ((*self_0).lexer.logger.log).expect("non-null function pointer")(
            (*self_0).lexer.logger.payload,
            TSLogTypeParse,
            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
        );
    }
    if !((*self_0).dot_graph_file).is_null() {
        fwrite!((*self_0).dot_graph_file, "graph {{\nlabel=\"",).unwrap_or(usize::MAX)
            as os::raw::c_int;
        let mut chr: *mut libc::c_char = &mut *((*self_0).lexer.debug_buffer)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize)
            as *mut libc::c_char;
        while *chr as libc::c_int != 0 as libc::c_int {
            if *chr as libc::c_int == '"' as i32 || *chr as libc::c_int == '\\' as i32 {
                fputc('\\' as i32, (*self_0).dot_graph_file);
            }
            fputc(*chr as libc::c_int, (*self_0).dot_graph_file);
            chr = chr.offset(1);
        }
        fwrite!((*self_0).dot_graph_file, "\"\n}}\n\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    }
}
unsafe extern "C" fn ts_parser__breakdown_top_of_stack(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
) -> bool {
    let mut did_break_down: bool = 0 as libc::c_int != 0;
    let mut pending: bool = 0 as libc::c_int != 0;
    loop {
        let mut pop: StackSliceArray = ts_stack_pop_pending((*self_0).stack, version);
        if pop.size == 0 {
            break;
        }
        did_break_down = 1 as libc::c_int != 0;
        pending = 0 as libc::c_int != 0;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < pop.size {
            let mut slice: StackSlice = *(pop.contents).offset(i as isize);
            let mut state: TSStateId = ts_stack_state((*self_0).stack, slice.version);
            if (0 as libc::c_int as uint32_t) < slice.subtrees.size {
            } else {
                panic!();
            }
            let mut parent: Subtree = *(&mut *(slice.subtrees.contents)
                .offset(0 as libc::c_int as isize)
                as *mut Subtree);
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            let mut n: uint32_t = ts_subtree_child_count(parent);
            while j < n {
                let mut child: Subtree = *if (parent.data).is_inline() as libc::c_int != 0 {
                    0 as *mut Subtree
                } else {
                    (parent.ptr as *mut Subtree).offset(-((*parent.ptr).child_count as isize))
                }
                .offset(j as isize);
                pending = ts_subtree_child_count(child) > 0 as libc::c_int as libc::c_uint;
                if ts_subtree_is_error(child) {
                    state = 0 as libc::c_int as TSStateId;
                } else if !ts_subtree_extra(child) {
                    state =
                        ts_language_next_state((*self_0).language, state, ts_subtree_symbol(child));
                }
                ts_subtree_retain(child);
                ts_stack_push((*self_0).stack, slice.version, child, pending, state);
                j = j.wrapping_add(1);
            }
            let mut j_0: uint32_t = 1 as libc::c_int as uint32_t;
            while j_0 < slice.subtrees.size {
                let mut tree: Subtree = *(slice.subtrees.contents).offset(j_0 as isize);
                ts_stack_push(
                    (*self_0).stack,
                    slice.version,
                    tree,
                    0 as libc::c_int != 0,
                    state,
                );
                j_0 = j_0.wrapping_add(1);
            }
            ts_subtree_release(&mut (*self_0).tree_pool, parent);
            array__delete(&mut slice.subtrees as *mut SubtreeArray as *mut VoidArray);
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "breakdown_top_of_stack tree:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(parent),
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            if !((*self_0).dot_graph_file).is_null() {
                ts_stack_print_dot_graph(
                    (*self_0).stack,
                    (*self_0).language,
                    (*self_0).dot_graph_file,
                );
                fputs(
                    b"\n\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).dot_graph_file,
                );
            }
            i = i.wrapping_add(1);
        }
        if !pending {
            break;
        }
    }
    return did_break_down;
}
unsafe extern "C" fn ts_parser__breakdown_lookahead(
    mut self_0: *mut TSParser,
    mut lookahead: *mut Subtree,
    mut state: TSStateId,
    mut reusable_node: *mut ReusableNode,
) {
    let mut did_descend: bool = 0 as libc::c_int != 0;
    let mut tree: Subtree = reusable_node_tree(reusable_node);
    while ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint
        && ts_subtree_parse_state(tree) as libc::c_int != state as libc::c_int
    {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "state_mismatch sym:{}",
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(tree)
                ))
                .to_string_lossy()
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        reusable_node_descend(reusable_node);
        tree = reusable_node_tree(reusable_node);
        did_descend = 1 as libc::c_int != 0;
    }
    if did_descend {
        ts_subtree_release(&mut (*self_0).tree_pool, *lookahead);
        *lookahead = tree;
        ts_subtree_retain(*lookahead);
    }
}
unsafe extern "C" fn ts_parser__compare_versions(
    mut self_0: *mut TSParser,
    mut a: ErrorStatus,
    mut b: ErrorStatus,
) -> ErrorComparison {
    if !a.is_in_error && b.is_in_error as libc::c_int != 0 {
        if a.cost < b.cost {
            return ErrorComparisonTakeLeft;
        } else {
            return ErrorComparisonPreferLeft;
        }
    }
    if a.is_in_error as libc::c_int != 0 && !b.is_in_error {
        if b.cost < a.cost {
            return ErrorComparisonTakeRight;
        } else {
            return ErrorComparisonPreferRight;
        }
    }
    if a.cost < b.cost {
        if (b.cost)
            .wrapping_sub(a.cost)
            .wrapping_mul((1 as libc::c_int as libc::c_uint).wrapping_add(a.node_count))
            > MAX_COST_DIFFERENCE
        {
            return ErrorComparisonTakeLeft;
        } else {
            return ErrorComparisonPreferLeft;
        }
    }
    if b.cost < a.cost {
        if (a.cost)
            .wrapping_sub(b.cost)
            .wrapping_mul((1 as libc::c_int as libc::c_uint).wrapping_add(b.node_count))
            > MAX_COST_DIFFERENCE
        {
            return ErrorComparisonTakeRight;
        } else {
            return ErrorComparisonPreferRight;
        }
    }
    if a.dynamic_precedence > b.dynamic_precedence {
        return ErrorComparisonPreferLeft;
    }
    if b.dynamic_precedence > a.dynamic_precedence {
        return ErrorComparisonPreferRight;
    }
    return ErrorComparisonNone;
}
unsafe extern "C" fn ts_parser__version_status(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
) -> ErrorStatus {
    let mut cost: libc::c_uint = ts_stack_error_cost((*self_0).stack, version);
    let mut is_paused: bool = ts_stack_is_paused((*self_0).stack, version);
    if is_paused {
        cost = cost.wrapping_add(100 as libc::c_int as libc::c_uint);
    }
    return {
        let mut init = ErrorStatus {
            cost: cost,
            node_count: ts_stack_node_count_since_error((*self_0).stack, version),
            dynamic_precedence: ts_stack_dynamic_precedence((*self_0).stack, version),
            is_in_error: is_paused as libc::c_int != 0
                || ts_stack_state((*self_0).stack, version) as libc::c_int == 0 as libc::c_int,
        };
        init
    };
}
unsafe extern "C" fn ts_parser__better_version_exists(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut is_in_error: bool,
    mut cost: libc::c_uint,
) -> bool {
    if !((*self_0).finished_tree.ptr).is_null()
        && ts_subtree_error_cost((*self_0).finished_tree) <= cost
    {
        return 1 as libc::c_int != 0;
    }
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut status: ErrorStatus = {
        let mut init = ErrorStatus {
            cost: cost,
            node_count: ts_stack_node_count_since_error((*self_0).stack, version),
            dynamic_precedence: ts_stack_dynamic_precedence((*self_0).stack, version),
            is_in_error: is_in_error,
        };
        init
    };
    let mut i: StackVersion = 0 as libc::c_int as StackVersion;
    let mut n: StackVersion = ts_stack_version_count((*self_0).stack);
    while i < n {
        if !(i == version
            || !ts_stack_is_active((*self_0).stack, i)
            || (ts_stack_position((*self_0).stack, i)).bytes < position.bytes)
        {
            let mut status_i: ErrorStatus = ts_parser__version_status(self_0, i);
            match ts_parser__compare_versions(self_0, status, status_i) as libc::c_uint {
                4 => return 1 as libc::c_int != 0,
                3 => {
                    if ts_stack_can_merge((*self_0).stack, i, version) {
                        return 1 as libc::c_int != 0;
                    }
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn ts_parser__restore_external_scanner(
    mut self_0: *mut TSParser,
    mut external_token: Subtree,
) {
    if !(external_token.ptr).is_null() {
        ((*(*self_0).language).external_scanner.deserialize).expect("non-null function pointer")(
            (*self_0).external_scanner_payload,
            ts_external_scanner_state_data(
                &(*external_token.ptr).c2rust_unnamed.external_scanner_state,
            ),
            (*external_token.ptr)
                .c2rust_unnamed
                .external_scanner_state
                .length,
        );
    } else {
        ((*(*self_0).language).external_scanner.deserialize).expect("non-null function pointer")(
            (*self_0).external_scanner_payload,
            0 as *const libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    };
}
unsafe extern "C" fn ts_parser__can_reuse_first_leaf(
    mut self_0: *mut TSParser,
    mut state: TSStateId,
    mut tree: Subtree,
    mut table_entry: *mut TableEntry,
) -> bool {
    let mut current_lex_mode: TSLexMode = *((*(*self_0).language).lex_modes).offset(state as isize);
    let mut leaf_symbol: TSSymbol = ts_subtree_leaf_symbol(tree);
    let mut leaf_state: TSStateId = ts_subtree_leaf_parse_state(tree);
    let mut leaf_lex_mode: TSLexMode =
        *((*(*self_0).language).lex_modes).offset(leaf_state as isize);
    if current_lex_mode.lex_state as libc::c_int == -(1 as libc::c_int) as uint16_t as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if (*table_entry).action_count > 0 as libc::c_int as libc::c_uint
        && if std::slice::from_raw_parts(
            &mut leaf_lex_mode as *mut TSLexMode as *const u8,
            ::std::mem::size_of::<TSLexMode>() as libc::c_ulong,
        ) == std::slice::from_raw_parts(
            &mut current_lex_mode as *mut TSLexMode as *const u8,
            ::std::mem::size_of::<TSLexMode>() as libc::c_ulong,
        ) {
            0
        } else {
            1
        } == 0 as libc::c_int
        && (leaf_symbol as libc::c_int
            != (*(*self_0).language).keyword_capture_token as libc::c_int
            || !ts_subtree_is_keyword(tree)
                && ts_subtree_parse_state(tree) as libc::c_int == state as libc::c_int)
    {
        return 1 as libc::c_int != 0;
    }
    if (ts_subtree_size(tree)).bytes == 0 as libc::c_int as libc::c_uint
        && leaf_symbol as libc::c_int != 0 as libc::c_int
    {
        return 0 as libc::c_int != 0;
    }
    return current_lex_mode.external_lex_state as libc::c_int == 0 as libc::c_int
        && (*table_entry).is_reusable as libc::c_int != 0;
}
unsafe extern "C" fn ts_parser__lex(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut parse_state: TSStateId,
) -> Subtree {
    let mut lex_mode: TSLexMode = *((*(*self_0).language).lex_modes).offset(parse_state as isize);
    if lex_mode.lex_state as libc::c_int == -(1 as libc::c_int) as uint16_t as libc::c_int {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "no_lookahead_after_non_terminal_extra",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        return Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
    }
    let start_position: Length = ts_stack_position((*self_0).stack, version);
    let external_token: Subtree = ts_stack_last_external_token((*self_0).stack, version);
    let mut valid_external_tokens: *const bool = ts_language_enabled_external_tokens(
        (*self_0).language,
        lex_mode.external_lex_state as libc::c_uint,
    );
    let mut found_external_token: bool = 0 as libc::c_int != 0;
    let mut error_mode: bool = parse_state as libc::c_int == 0 as libc::c_int;
    let mut skipped_error: bool = 0 as libc::c_int != 0;
    let mut called_get_column: bool = 0 as libc::c_int != 0;
    let mut first_error_character: int32_t = 0 as libc::c_int;
    let mut error_start_position: Length = length_zero();
    let mut error_end_position: Length = length_zero();
    let mut lookahead_end_byte: uint32_t = 0 as libc::c_int as uint32_t;
    let mut external_scanner_state_len: uint32_t = 0 as libc::c_int as uint32_t;
    let mut external_scanner_state_changed: bool = 0 as libc::c_int != 0;
    ts_lexer_reset(&mut (*self_0).lexer, start_position);
    loop {
        let mut current_position: Length = (*self_0).lexer.current_position;
        if !valid_external_tokens.is_null() {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "lex_external state:{}, row:{}, column:{}",
                    lex_mode.external_lex_state as libc::c_int,
                    current_position.extent.row,
                    current_position.extent.column
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            ts_lexer_start(&mut (*self_0).lexer);
            ts_parser__restore_external_scanner(self_0, external_token);
            let mut found_token: bool = ((*(*self_0).language).external_scanner.scan)
                .expect("non-null function pointer")(
                (*self_0).external_scanner_payload,
                &mut (*self_0).lexer.data,
                valid_external_tokens,
            );
            ts_lexer_finish(&mut (*self_0).lexer, &mut lookahead_end_byte);
            if found_token {
                external_scanner_state_len = ((*(*self_0).language).external_scanner.serialize)
                    .expect("non-null function pointer")(
                    (*self_0).external_scanner_payload,
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                );
                external_scanner_state_changed = !ts_external_scanner_state_eq(
                    ts_subtree_external_scanner_state(external_token),
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    external_scanner_state_len,
                );
                if (*self_0).lexer.token_end_position.bytes <= current_position.bytes
                    && (error_mode as libc::c_int != 0
                        || !ts_stack_has_advanced_since_error((*self_0).stack, version))
                    && !external_scanner_state_changed
                {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "ignore_empty_external_token symbol:{}",
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                *((*(*self_0).language).external_scanner.symbol_map)
                                    .offset((*self_0).lexer.data.result_symbol as isize),
                            ))
                            .to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    found_token = 0 as libc::c_int != 0;
                }
            }
            if found_token {
                found_external_token = 1 as libc::c_int != 0;
                called_get_column = (*self_0).lexer.did_get_column;
                break;
            } else {
                ts_lexer_reset(&mut (*self_0).lexer, current_position);
            }
        }
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "lex_internal state:{}, row:{}, column:{}",
                lex_mode.lex_state as libc::c_int,
                current_position.extent.row,
                current_position.extent.column
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        ts_lexer_start(&mut (*self_0).lexer);
        let mut found_token_0: bool = ((*(*self_0).language).lex_fn)
            .expect("non-null function pointer")(
            &mut (*self_0).lexer.data, lex_mode.lex_state
        );
        ts_lexer_finish(&mut (*self_0).lexer, &mut lookahead_end_byte);
        if found_token_0 {
            break;
        }
        if !error_mode {
            error_mode = 1 as libc::c_int != 0;
            lex_mode = *((*(*self_0).language).lex_modes).offset(0 as libc::c_int as isize);
            valid_external_tokens = ts_language_enabled_external_tokens(
                (*self_0).language,
                lex_mode.external_lex_state as libc::c_uint,
            );
            ts_lexer_reset(&mut (*self_0).lexer, start_position);
        } else {
            if !skipped_error {
                if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                    snwrite!(
                        ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "skip_unrecognized_character",
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    ts_parser__log(self_0);
                }
                skipped_error = 1 as libc::c_int != 0;
                error_start_position = (*self_0).lexer.token_start_position;
                error_end_position = (*self_0).lexer.token_start_position;
                first_error_character = (*self_0).lexer.data.lookahead;
            }
            if (*self_0).lexer.current_position.bytes == error_end_position.bytes {
                if ((*self_0).lexer.data.eof).expect("non-null function pointer")(
                    &mut (*self_0).lexer.data,
                ) {
                    (*self_0).lexer.data.result_symbol = -(1 as libc::c_int) as TSSymbol;
                    break;
                } else {
                    ((*self_0).lexer.data.advance).expect("non-null function pointer")(
                        &mut (*self_0).lexer.data,
                        0 as libc::c_int != 0,
                    );
                }
            }
            error_end_position = (*self_0).lexer.current_position;
        }
    }
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
    if skipped_error {
        let mut padding: Length = length_sub(error_start_position, start_position);
        let mut size: Length = length_sub(error_end_position, error_start_position);
        let mut lookahead_bytes: uint32_t =
            lookahead_end_byte.wrapping_sub(error_end_position.bytes);
        result = ts_subtree_new_error(
            &mut (*self_0).tree_pool,
            first_error_character,
            padding,
            size,
            lookahead_bytes,
            parse_state,
            (*self_0).language,
        );
    } else {
        let mut is_keyword: bool = 0 as libc::c_int != 0;
        let mut symbol: TSSymbol = (*self_0).lexer.data.result_symbol;
        let mut padding_0: Length =
            length_sub((*self_0).lexer.token_start_position, start_position);
        let mut size_0: Length = length_sub(
            (*self_0).lexer.token_end_position,
            (*self_0).lexer.token_start_position,
        );
        let mut lookahead_bytes_0: uint32_t =
            lookahead_end_byte.wrapping_sub((*self_0).lexer.token_end_position.bytes);
        if found_external_token {
            symbol = *((*(*self_0).language).external_scanner.symbol_map).offset(symbol as isize);
        } else if symbol as libc::c_int
            == (*(*self_0).language).keyword_capture_token as libc::c_int
            && symbol as libc::c_int != 0 as libc::c_int
        {
            let mut end_byte: uint32_t = (*self_0).lexer.token_end_position.bytes;
            ts_lexer_reset(&mut (*self_0).lexer, (*self_0).lexer.token_start_position);
            ts_lexer_start(&mut (*self_0).lexer);
            if ((*(*self_0).language).keyword_lex_fn).expect("non-null function pointer")(
                &mut (*self_0).lexer.data,
                0 as libc::c_int as TSStateId,
            ) as libc::c_int
                != 0
                && (*self_0).lexer.token_end_position.bytes == end_byte
                && ts_language_has_actions(
                    (*self_0).language,
                    parse_state,
                    (*self_0).lexer.data.result_symbol,
                ) as libc::c_int
                    != 0
            {
                is_keyword = 1 as libc::c_int != 0;
                symbol = (*self_0).lexer.data.result_symbol;
            }
        }
        result = ts_subtree_new_leaf(
            &mut (*self_0).tree_pool,
            symbol,
            padding_0,
            size_0,
            lookahead_bytes_0,
            parse_state,
            found_external_token,
            called_get_column,
            is_keyword,
            (*self_0).language,
        );
        if found_external_token {
            let mut mut_result: MutableSubtree = ts_subtree_to_mut_unsafe(result);
            ts_external_scanner_state_init(
                &mut (*mut_result.ptr).c2rust_unnamed.external_scanner_state,
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                external_scanner_state_len,
            );
            (*mut_result.ptr).set_has_external_scanner_state_change(external_scanner_state_changed);
        }
    }
    if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
        let mut buf: *mut libc::c_char = ((*self_0).lexer.debug_buffer).as_mut_ptr();
        let mut symbol_0: *const libc::c_char =
            ts_language_symbol_name((*self_0).language, ts_subtree_symbol(result));
        let mut off: libc::c_int = snwrite!(buf, 1024 as usize, "lexed_lookahead sym:",)
            .unwrap_or(usize::MAX) as os::raw::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while *symbol_0.offset(i as isize) as libc::c_int != '\0' as i32
            && off < 1024 as libc::c_int
        {
            match *symbol_0.offset(i as isize) as libc::c_int {
                9 => {
                    let fresh20 = off;
                    off = off + 1;
                    *buf.offset(fresh20 as isize) = '\\' as i32 as libc::c_char;
                    let fresh21 = off;
                    off = off + 1;
                    *buf.offset(fresh21 as isize) = 't' as i32 as libc::c_char;
                }
                10 => {
                    let fresh22 = off;
                    off = off + 1;
                    *buf.offset(fresh22 as isize) = '\\' as i32 as libc::c_char;
                    let fresh23 = off;
                    off = off + 1;
                    *buf.offset(fresh23 as isize) = 'n' as i32 as libc::c_char;
                }
                11 => {
                    let fresh24 = off;
                    off = off + 1;
                    *buf.offset(fresh24 as isize) = '\\' as i32 as libc::c_char;
                    let fresh25 = off;
                    off = off + 1;
                    *buf.offset(fresh25 as isize) = 'v' as i32 as libc::c_char;
                }
                12 => {
                    let fresh26 = off;
                    off = off + 1;
                    *buf.offset(fresh26 as isize) = '\\' as i32 as libc::c_char;
                    let fresh27 = off;
                    off = off + 1;
                    *buf.offset(fresh27 as isize) = 'f' as i32 as libc::c_char;
                }
                13 => {
                    let fresh28 = off;
                    off = off + 1;
                    *buf.offset(fresh28 as isize) = '\\' as i32 as libc::c_char;
                    let fresh29 = off;
                    off = off + 1;
                    *buf.offset(fresh29 as isize) = 'r' as i32 as libc::c_char;
                }
                92 => {
                    let fresh30 = off;
                    off = off + 1;
                    *buf.offset(fresh30 as isize) = '\\' as i32 as libc::c_char;
                    let fresh31 = off;
                    off = off + 1;
                    *buf.offset(fresh31 as isize) = '\\' as i32 as libc::c_char;
                }
                _ => {
                    let fresh32 = off;
                    off = off + 1;
                    *buf.offset(fresh32 as isize) = *symbol_0.offset(i as isize);
                }
            }
            i += 1;
        }
        snwrite!(
            buf.offset(off as isize),
            (1024 as libc::c_int - off) as libc::c_ulong as usize,
            ", size:{}",
            (ts_subtree_total_size(result)).bytes
        )
        .unwrap_or(usize::MAX) as os::raw::c_int;
        ts_parser__log(self_0);
    }
    return result;
}
unsafe extern "C" fn ts_parser__get_cached_token(
    mut self_0: *mut TSParser,
    mut state: TSStateId,
    mut position: size_t,
    mut last_external_token: Subtree,
    mut table_entry: *mut TableEntry,
) -> Subtree {
    let mut cache: *mut TokenCache = &mut (*self_0).token_cache;
    if !((*cache).token.ptr).is_null()
        && (*cache).byte_index as libc::c_ulong == position
        && ts_subtree_external_scanner_state_eq((*cache).last_external_token, last_external_token)
            as libc::c_int
            != 0
    {
        ts_language_table_entry(
            (*self_0).language,
            state,
            ts_subtree_symbol((*cache).token),
            table_entry,
        );
        if ts_parser__can_reuse_first_leaf(self_0, state, (*cache).token, table_entry) {
            ts_subtree_retain((*cache).token);
            return (*cache).token;
        }
    }
    return Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
}
unsafe extern "C" fn ts_parser__set_cached_token(
    mut self_0: *mut TSParser,
    mut byte_index: uint32_t,
    mut last_external_token: Subtree,
    mut token: Subtree,
) {
    let mut cache: *mut TokenCache = &mut (*self_0).token_cache;
    if !(token.ptr).is_null() {
        ts_subtree_retain(token);
    }
    if !(last_external_token.ptr).is_null() {
        ts_subtree_retain(last_external_token);
    }
    if !((*cache).token.ptr).is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*cache).token);
    }
    if !((*cache).last_external_token.ptr).is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*cache).last_external_token);
    }
    (*cache).token = token;
    (*cache).byte_index = byte_index;
    (*cache).last_external_token = last_external_token;
}
unsafe extern "C" fn ts_parser__has_included_range_difference(
    mut self_0: *const TSParser,
    mut start_position: uint32_t,
    mut end_position: uint32_t,
) -> bool {
    return ts_range_array_intersects(
        &(*self_0).included_range_differences,
        (*self_0).included_range_difference_index,
        start_position,
        end_position,
    );
}
unsafe extern "C" fn ts_parser__reuse_node(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut state: *mut TSStateId,
    mut position: uint32_t,
    mut last_external_token: Subtree,
    mut table_entry: *mut TableEntry,
) -> Subtree {
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
    loop {
        result = reusable_node_tree(&mut (*self_0).reusable_node);
        if (result.ptr).is_null() {
            break;
        }
        let mut byte_offset: uint32_t = reusable_node_byte_offset(&mut (*self_0).reusable_node);
        let mut end_byte_offset: uint32_t =
            byte_offset.wrapping_add(ts_subtree_total_bytes(result));
        if ts_subtree_is_eof(result) {
            end_byte_offset = 4294967295 as libc::c_uint;
        }
        if byte_offset > position {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "before_reusable_node symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(result),
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            break;
        } else if byte_offset < position {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "past_reusable_node symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(result),
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            if end_byte_offset <= position || !reusable_node_descend(&mut (*self_0).reusable_node) {
                reusable_node_advance(&mut (*self_0).reusable_node);
            }
        } else if !ts_subtree_external_scanner_state_eq(
            (*self_0).reusable_node.last_external_token,
            last_external_token,
        ) {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "reusable_node_has_different_external_scanner_state symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(result),
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            reusable_node_advance(&mut (*self_0).reusable_node);
        } else {
            let mut reason: *const libc::c_char = 0 as *const libc::c_char;
            if ts_subtree_has_changes(result) {
                reason = b"has_changes\0" as *const u8 as *const libc::c_char;
            } else if ts_subtree_is_error(result) {
                reason = b"is_error\0" as *const u8 as *const libc::c_char;
            } else if ts_subtree_missing(result) {
                reason = b"is_missing\0" as *const u8 as *const libc::c_char;
            } else if ts_subtree_is_fragile(result) {
                reason = b"is_fragile\0" as *const u8 as *const libc::c_char;
            } else if ts_parser__has_included_range_difference(self_0, byte_offset, end_byte_offset)
            {
                reason = b"contains_different_included_range\0" as *const u8 as *const libc::c_char;
            }
            if !reason.is_null() {
                if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                    snwrite!(
                        ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "cant_reuse_node_{} tree:{}",
                        std::ffi::CStr::from_ptr(reason).to_string_lossy(),
                        std::ffi::CStr::from_ptr(ts_language_symbol_name(
                            (*self_0).language,
                            ts_subtree_symbol(result),
                        ))
                        .to_string_lossy()
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    ts_parser__log(self_0);
                }
                if !reusable_node_descend(&mut (*self_0).reusable_node) {
                    reusable_node_advance(&mut (*self_0).reusable_node);
                    ts_parser__breakdown_top_of_stack(self_0, version);
                    *state = ts_stack_state((*self_0).stack, version);
                }
            } else {
                let mut leaf_symbol: TSSymbol = ts_subtree_leaf_symbol(result);
                ts_language_table_entry((*self_0).language, *state, leaf_symbol, table_entry);
                if !ts_parser__can_reuse_first_leaf(self_0, *state, result, table_entry) {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "cant_reuse_node symbol:{}, first_leaf_symbol:{}",
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                ts_subtree_symbol(result),
                            ))
                            .to_string_lossy(),
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                leaf_symbol
                            ))
                            .to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    reusable_node_advance_past_leaf(&mut (*self_0).reusable_node);
                    break;
                } else {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "reuse_node symbol:{}",
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                ts_subtree_symbol(result),
                            ))
                            .to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    ts_subtree_retain(result);
                    return result;
                }
            }
        }
    }
    return Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
}
unsafe extern "C" fn ts_parser__select_tree(
    mut self_0: *mut TSParser,
    mut left: Subtree,
    mut right: Subtree,
) -> bool {
    if (left.ptr).is_null() {
        return 1 as libc::c_int != 0;
    }
    if (right.ptr).is_null() {
        return 0 as libc::c_int != 0;
    }
    if ts_subtree_error_cost(right) < ts_subtree_error_cost(left) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "select_smaller_error symbol:{}, over_symbol:{}",
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(right)
                ))
                .to_string_lossy(),
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(left)
                ))
                .to_string_lossy()
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        return 1 as libc::c_int != 0;
    }
    if ts_subtree_error_cost(left) < ts_subtree_error_cost(right) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "select_smaller_error symbol:{}, over_symbol:{}",
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(left)
                ))
                .to_string_lossy(),
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(right)
                ))
                .to_string_lossy()
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        return 0 as libc::c_int != 0;
    }
    if ts_subtree_dynamic_precedence(right) > ts_subtree_dynamic_precedence(left) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "select_higher_precedence symbol:{}, prec:{}, over_symbol:{}, other_prec:{}",
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(right)
                ))
                .to_string_lossy(),
                ts_subtree_dynamic_precedence(right),
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(left)
                ))
                .to_string_lossy(),
                ts_subtree_dynamic_precedence(left)
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        return 1 as libc::c_int != 0;
    }
    if ts_subtree_dynamic_precedence(left) > ts_subtree_dynamic_precedence(right) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "select_higher_precedence symbol:{}, prec:{}, over_symbol:{}, other_prec:{}",
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(left)
                ))
                .to_string_lossy(),
                ts_subtree_dynamic_precedence(left),
                std::ffi::CStr::from_ptr(ts_language_symbol_name(
                    (*self_0).language,
                    ts_subtree_symbol(right)
                ))
                .to_string_lossy(),
                ts_subtree_dynamic_precedence(right)
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        return 0 as libc::c_int != 0;
    }
    if ts_subtree_error_cost(left) > 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    let mut comparison: libc::c_int = ts_subtree_compare(left, right);
    match comparison {
        -1 => {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "select_earlier symbol:{}, over_symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(left)
                    ))
                    .to_string_lossy(),
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(right)
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            return 0 as libc::c_int != 0;
        }
        1 => {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "select_earlier symbol:{}, over_symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(right),
                    ))
                    .to_string_lossy(),
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(left)
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            return 1 as libc::c_int != 0;
        }
        _ => {
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "select_existing symbol:{}, over_symbol:{}",
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(left)
                    ))
                    .to_string_lossy(),
                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                        (*self_0).language,
                        ts_subtree_symbol(right)
                    ))
                    .to_string_lossy()
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            return 0 as libc::c_int != 0;
        }
    };
}
unsafe extern "C" fn ts_parser__select_children(
    mut self_0: *mut TSParser,
    mut left: Subtree,
    mut children: *const SubtreeArray,
) -> bool {
    array__assign(
        &mut (*self_0).scratch_trees as *mut SubtreeArray as *mut VoidArray,
        children as *const VoidArray,
        ::std::mem::size_of::<Subtree>() as libc::c_ulong,
    );
    let mut scratch_tree: MutableSubtree = ts_subtree_new_node(
        ts_subtree_symbol(left),
        &mut (*self_0).scratch_trees,
        0 as libc::c_int as libc::c_uint,
        (*self_0).language,
    );
    return ts_parser__select_tree(self_0, left, ts_subtree_from_mut(scratch_tree));
}
unsafe extern "C" fn ts_parser__shift(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut state: TSStateId,
    mut lookahead: Subtree,
    mut extra: bool,
) {
    let mut is_leaf: bool = ts_subtree_child_count(lookahead) == 0 as libc::c_int as libc::c_uint;
    let mut subtree_to_push: Subtree = lookahead;
    if extra as libc::c_int != ts_subtree_extra(lookahead) as libc::c_int
        && is_leaf as libc::c_int != 0
    {
        let mut result: MutableSubtree = ts_subtree_make_mut(&mut (*self_0).tree_pool, lookahead);
        ts_subtree_set_extra(&mut result, extra);
        subtree_to_push = ts_subtree_from_mut(result);
    }
    ts_stack_push((*self_0).stack, version, subtree_to_push, !is_leaf, state);
    if ts_subtree_has_external_tokens(subtree_to_push) {
        ts_stack_set_last_external_token(
            (*self_0).stack,
            version,
            ts_subtree_last_external_token(subtree_to_push),
        );
    }
}
unsafe extern "C" fn ts_parser__reduce(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut symbol: TSSymbol,
    mut count: uint32_t,
    mut dynamic_precedence: libc::c_int,
    mut production_id: uint16_t,
    mut is_fragile: bool,
    mut end_of_non_terminal_extra: bool,
) -> StackVersion {
    let mut initial_version_count: uint32_t = ts_stack_version_count((*self_0).stack);
    let mut pop: StackSliceArray = ts_stack_pop_count((*self_0).stack, version, count);
    let mut removed_version_count: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < pop.size {
        let mut slice: StackSlice = *(pop.contents).offset(i as isize);
        let mut slice_version: StackVersion = (slice.version).wrapping_sub(removed_version_count);
        if slice_version > MAX_VERSION_COUNT.wrapping_add(MAX_VERSION_COUNT_OVERFLOW) {
            ts_stack_remove_version((*self_0).stack, slice_version);
            ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut slice.subtrees);
            removed_version_count = removed_version_count.wrapping_add(1);
            while i.wrapping_add(1 as libc::c_int as libc::c_uint) < pop.size {
                let mut next_slice: StackSlice = *(pop.contents)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                if next_slice.version != slice.version {
                    break;
                }
                ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut next_slice.subtrees);
                i = i.wrapping_add(1);
            }
        } else {
            let mut children: SubtreeArray = slice.subtrees;
            ts_subtree_array_remove_trailing_extras(&mut children, &mut (*self_0).trailing_extras);
            let mut parent: MutableSubtree = ts_subtree_new_node(
                symbol,
                &mut children,
                production_id as libc::c_uint,
                (*self_0).language,
            );
            while i.wrapping_add(1 as libc::c_int as libc::c_uint) < pop.size {
                let mut next_slice_0: StackSlice = *(pop.contents)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
                if next_slice_0.version != slice.version {
                    break;
                }
                i = i.wrapping_add(1);
                let mut next_slice_children: SubtreeArray = next_slice_0.subtrees;
                ts_subtree_array_remove_trailing_extras(
                    &mut next_slice_children,
                    &mut (*self_0).trailing_extras2,
                );
                if ts_parser__select_children(
                    self_0,
                    ts_subtree_from_mut(parent),
                    &mut next_slice_children,
                ) {
                    ts_subtree_array_clear(
                        &mut (*self_0).tree_pool,
                        &mut (*self_0).trailing_extras,
                    );
                    ts_subtree_release(&mut (*self_0).tree_pool, ts_subtree_from_mut(parent));
                    array__swap(
                        &mut (*self_0).trailing_extras as *mut SubtreeArray as *mut VoidArray,
                        &mut (*self_0).trailing_extras2 as *mut SubtreeArray as *mut VoidArray,
                    );
                    parent = ts_subtree_new_node(
                        symbol,
                        &mut next_slice_children,
                        production_id as libc::c_uint,
                        (*self_0).language,
                    );
                } else {
                    (*self_0).trailing_extras2.size = 0 as libc::c_int as uint32_t;
                    ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut next_slice_0.subtrees);
                }
            }
            let mut state: TSStateId = ts_stack_state((*self_0).stack, slice_version);
            let mut next_state: TSStateId =
                ts_language_next_state((*self_0).language, state, symbol);
            if end_of_non_terminal_extra as libc::c_int != 0
                && next_state as libc::c_int == state as libc::c_int
            {
                (*parent.ptr).set_extra(1 as libc::c_int != 0);
            }
            if is_fragile as libc::c_int != 0
                || pop.size > 1 as libc::c_int as libc::c_uint
                || initial_version_count > 1 as libc::c_int as libc::c_uint
            {
                (*parent.ptr).set_fragile_left(1 as libc::c_int != 0);
                (*parent.ptr).set_fragile_right(1 as libc::c_int != 0);
                (*parent.ptr).parse_state =
                    (32767 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as TSStateId;
            } else {
                (*parent.ptr).parse_state = state;
            }
            let ref mut fresh33 = (*parent.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .dynamic_precedence;
            *fresh33 += dynamic_precedence;
            ts_stack_push(
                (*self_0).stack,
                slice_version,
                ts_subtree_from_mut(parent),
                0 as libc::c_int != 0,
                next_state,
            );
            let mut j: uint32_t = 0 as libc::c_int as uint32_t;
            while j < (*self_0).trailing_extras.size {
                ts_stack_push(
                    (*self_0).stack,
                    slice_version,
                    *((*self_0).trailing_extras.contents).offset(j as isize),
                    0 as libc::c_int != 0,
                    next_state,
                );
                j = j.wrapping_add(1);
            }
            let mut j_0: StackVersion = 0 as libc::c_int as StackVersion;
            while j_0 < slice_version {
                if !(j_0 == version) {
                    if ts_stack_merge((*self_0).stack, j_0, slice_version) {
                        removed_version_count = removed_version_count.wrapping_add(1);
                        break;
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return if ts_stack_version_count((*self_0).stack) > initial_version_count {
        initial_version_count
    } else {
        -(1 as libc::c_int) as StackVersion
    };
}
unsafe extern "C" fn ts_parser__accept(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut lookahead: Subtree,
) {
    if ts_subtree_is_eof(lookahead) {
    } else {
        panic!();
    }
    ts_stack_push(
        (*self_0).stack,
        version,
        lookahead,
        0 as libc::c_int != 0,
        1 as libc::c_int as TSStateId,
    );
    let mut pop: StackSliceArray = ts_stack_pop_all((*self_0).stack, version);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < pop.size {
        let mut trees: SubtreeArray = (*(pop.contents).offset(i as isize)).subtrees;
        let mut root: Subtree = Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
        let mut j: uint32_t = (trees.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        while j.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
            let mut tree: Subtree = *(trees.contents).offset(j as isize);
            if !ts_subtree_extra(tree) {
                if !(tree.data).is_inline() {
                } else {
                    panic!();
                }
                let mut child_count: uint32_t = ts_subtree_child_count(tree);
                let mut children: *const Subtree = if (tree.data).is_inline() as libc::c_int != 0 {
                    0 as *mut Subtree
                } else {
                    (tree.ptr as *mut Subtree).offset(-((*tree.ptr).child_count as isize))
                };
                let mut k: uint32_t = 0 as libc::c_int as uint32_t;
                while k < child_count {
                    ts_subtree_retain(*children.offset(k as isize));
                    k = k.wrapping_add(1);
                }
                array__splice(
                    &mut trees as *mut SubtreeArray as *mut VoidArray,
                    ::std::mem::size_of::<Subtree>() as libc::c_ulong,
                    j,
                    1 as libc::c_int as uint32_t,
                    child_count,
                    children as *const libc::c_void,
                );
                root = ts_subtree_from_mut(ts_subtree_new_node(
                    ts_subtree_symbol(tree),
                    &mut trees,
                    (*tree.ptr).c2rust_unnamed.c2rust_unnamed.production_id as libc::c_uint,
                    (*self_0).language,
                ));
                ts_subtree_release(&mut (*self_0).tree_pool, tree);
                break;
            } else {
                j = j.wrapping_sub(1);
            }
        }
        if !(root.ptr).is_null() {
        } else {
            panic!();
        }
        let ref mut fresh34 = (*self_0).accept_count;
        *fresh34 = (*fresh34).wrapping_add(1);
        if !((*self_0).finished_tree.ptr).is_null() {
            if ts_parser__select_tree(self_0, (*self_0).finished_tree, root) {
                ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).finished_tree);
                (*self_0).finished_tree = root;
            } else {
                ts_subtree_release(&mut (*self_0).tree_pool, root);
            }
        } else {
            (*self_0).finished_tree = root;
        }
        i = i.wrapping_add(1);
    }
    ts_stack_remove_version(
        (*self_0).stack,
        (*(pop.contents).offset(0 as libc::c_int as isize)).version,
    );
    ts_stack_halt((*self_0).stack, version);
}
unsafe extern "C" fn ts_parser__do_all_potential_reductions(
    mut self_0: *mut TSParser,
    mut starting_version: StackVersion,
    mut lookahead_symbol: TSSymbol,
) -> bool {
    let mut initial_version_count: uint32_t = ts_stack_version_count((*self_0).stack);
    let mut can_shift_lookahead_symbol: bool = 0 as libc::c_int != 0;
    let mut version: StackVersion = starting_version;
    let mut current_block_33: u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        let mut version_count: uint32_t = ts_stack_version_count((*self_0).stack);
        if version >= version_count {
            break;
        }
        let mut merged: bool = 0 as libc::c_int != 0;
        let mut j: StackVersion = initial_version_count;
        while j < version {
            if ts_stack_merge((*self_0).stack, j, version) {
                merged = 1 as libc::c_int != 0;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if !merged {
            let mut state: TSStateId = ts_stack_state((*self_0).stack, version);
            let mut has_shift_action: bool = 0 as libc::c_int != 0;
            (*self_0).reduce_actions.size = 0 as libc::c_int as uint32_t;
            let mut first_symbol: TSSymbol = 0;
            let mut end_symbol: TSSymbol = 0;
            if lookahead_symbol as libc::c_int != 0 as libc::c_int {
                first_symbol = lookahead_symbol;
                end_symbol = (lookahead_symbol as libc::c_int + 1 as libc::c_int) as TSSymbol;
            } else {
                first_symbol = 1 as libc::c_int as TSSymbol;
                end_symbol = (*(*self_0).language).token_count as TSSymbol;
            }
            let mut symbol: TSSymbol = first_symbol;
            while (symbol as libc::c_int) < end_symbol as libc::c_int {
                let mut entry: TableEntry = TableEntry {
                    actions: 0 as *const TSParseAction,
                    action_count: 0,
                    is_reusable: false,
                };
                ts_language_table_entry((*self_0).language, state, symbol, &mut entry);
                let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
                while j_0 < entry.action_count {
                    let mut action: TSParseAction = *(entry.actions).offset(j_0 as isize);
                    match action.type_ as libc::c_int {
                        0 | 3 => {
                            if !action.shift.extra && !action.shift.repetition {
                                has_shift_action = 1 as libc::c_int != 0;
                            }
                        }
                        1 => {
                            if action.reduce.child_count as libc::c_int > 0 as libc::c_int {
                                ts_reduce_action_set_add(&mut (*self_0).reduce_actions, {
                                    let mut init = ReduceAction {
                                        count: action.reduce.child_count as uint32_t,
                                        symbol: action.reduce.symbol,
                                        dynamic_precedence: action.reduce.dynamic_precedence
                                            as libc::c_int,
                                        production_id: action.reduce.production_id,
                                    };
                                    init
                                });
                            }
                        }
                        _ => {}
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                symbol = symbol.wrapping_add(1);
            }
            let mut reduction_version: StackVersion = -(1 as libc::c_int) as StackVersion;
            let mut j_1: uint32_t = 0 as libc::c_int as uint32_t;
            while j_1 < (*self_0).reduce_actions.size {
                let mut action_0: ReduceAction =
                    *((*self_0).reduce_actions.contents).offset(j_1 as isize);
                reduction_version = ts_parser__reduce(
                    self_0,
                    version,
                    action_0.symbol,
                    action_0.count,
                    action_0.dynamic_precedence,
                    action_0.production_id,
                    1 as libc::c_int != 0,
                    0 as libc::c_int != 0,
                );
                j_1 = j_1.wrapping_add(1);
            }
            if has_shift_action {
                can_shift_lookahead_symbol = 1 as libc::c_int != 0;
                current_block_33 = 13678349939556791712;
            } else if reduction_version != -(1 as libc::c_int) as StackVersion
                && i < MAX_VERSION_COUNT
            {
                ts_stack_renumber_version((*self_0).stack, reduction_version, version);
                current_block_33 = 14916268686031723178;
            } else {
                if lookahead_symbol as libc::c_int != 0 as libc::c_int {
                    ts_stack_remove_version((*self_0).stack, version);
                }
                current_block_33 = 13678349939556791712;
            }
            match current_block_33 {
                14916268686031723178 => {}
                _ => {
                    if version == starting_version {
                        version = version_count;
                    } else {
                        version = version.wrapping_add(1);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return can_shift_lookahead_symbol;
}
unsafe extern "C" fn ts_parser__recover_to_state(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut depth: libc::c_uint,
    mut goal_state: TSStateId,
) -> bool {
    let mut pop: StackSliceArray = ts_stack_pop_count((*self_0).stack, version, depth);
    let mut previous_version: StackVersion = -(1 as libc::c_int) as StackVersion;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < pop.size {
        let mut slice: StackSlice = *(pop.contents).offset(i as isize);
        if slice.version == previous_version {
            ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut slice.subtrees);
            let fresh35 = i;
            i = i.wrapping_sub(1);
            array__erase(
                &mut pop as *mut StackSliceArray as *mut VoidArray,
                ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                fresh35,
            );
        } else if ts_stack_state((*self_0).stack, slice.version) as libc::c_int
            != goal_state as libc::c_int
        {
            ts_stack_halt((*self_0).stack, slice.version);
            ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut slice.subtrees);
            let fresh36 = i;
            i = i.wrapping_sub(1);
            array__erase(
                &mut pop as *mut StackSliceArray as *mut VoidArray,
                ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                fresh36,
            );
        } else {
            let mut error_trees: SubtreeArray = ts_stack_pop_error((*self_0).stack, slice.version);
            if error_trees.size > 0 as libc::c_int as libc::c_uint {
                if error_trees.size == 1 as libc::c_int as libc::c_uint {
                } else {
                    panic!();
                }
                let mut error_tree: Subtree =
                    *(error_trees.contents).offset(0 as libc::c_int as isize);
                let mut error_child_count: uint32_t = ts_subtree_child_count(error_tree);
                if error_child_count > 0 as libc::c_int as libc::c_uint {
                    array__splice(
                        &mut slice.subtrees as *mut SubtreeArray as *mut VoidArray,
                        ::std::mem::size_of::<Subtree>() as libc::c_ulong,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        error_child_count,
                        (if (error_tree.data).is_inline() as libc::c_int != 0 {
                            0 as *mut Subtree
                        } else {
                            (error_tree.ptr as *mut Subtree)
                                .offset(-((*error_tree.ptr).child_count as isize))
                        }) as *const libc::c_void,
                    );
                    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    while j < error_child_count {
                        ts_subtree_retain(*(slice.subtrees.contents).offset(j as isize));
                        j = j.wrapping_add(1);
                    }
                }
                ts_subtree_array_delete(&mut (*self_0).tree_pool, &mut error_trees);
            }
            ts_subtree_array_remove_trailing_extras(
                &mut slice.subtrees,
                &mut (*self_0).trailing_extras,
            );
            if slice.subtrees.size > 0 as libc::c_int as libc::c_uint {
                let mut error: Subtree = ts_subtree_new_error_node(
                    &mut slice.subtrees,
                    1 as libc::c_int != 0,
                    (*self_0).language,
                );
                ts_stack_push(
                    (*self_0).stack,
                    slice.version,
                    error,
                    0 as libc::c_int != 0,
                    goal_state,
                );
            } else {
                array__delete(&mut slice.subtrees as *mut SubtreeArray as *mut VoidArray);
            }
            let mut j_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j_0 < (*self_0).trailing_extras.size {
                let mut tree: Subtree = *((*self_0).trailing_extras.contents).offset(j_0 as isize);
                ts_stack_push(
                    (*self_0).stack,
                    slice.version,
                    tree,
                    0 as libc::c_int != 0,
                    goal_state,
                );
                j_0 = j_0.wrapping_add(1);
            }
            previous_version = slice.version;
        }
        i = i.wrapping_add(1);
    }
    return previous_version != -(1 as libc::c_int) as StackVersion;
}
unsafe extern "C" fn ts_parser__recover(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut lookahead: Subtree,
) {
    let mut did_recover: bool = 0 as libc::c_int != 0;
    let mut previous_version_count: libc::c_uint = ts_stack_version_count((*self_0).stack);
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut summary: *mut StackSummary = ts_stack_get_summary((*self_0).stack, version);
    let mut node_count_since_error: libc::c_uint =
        ts_stack_node_count_since_error((*self_0).stack, version);
    let mut current_error_cost: libc::c_uint = ts_stack_error_cost((*self_0).stack, version);
    if !summary.is_null() && !ts_subtree_is_error(lookahead) {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*summary).size {
            let mut entry: StackSummaryEntry = *((*summary).contents).offset(i as isize);
            if !(entry.state as libc::c_int == 0 as libc::c_int) {
                if !(entry.position.bytes == position.bytes) {
                    let mut depth: libc::c_uint = entry.depth;
                    if node_count_since_error > 0 as libc::c_int as libc::c_uint {
                        depth = depth.wrapping_add(1);
                    }
                    let mut would_merge: bool = 0 as libc::c_int != 0;
                    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    while j < previous_version_count {
                        if ts_stack_state((*self_0).stack, j) as libc::c_int
                            == entry.state as libc::c_int
                            && (ts_stack_position((*self_0).stack, j)).bytes == position.bytes
                        {
                            would_merge = 1 as libc::c_int != 0;
                            break;
                        } else {
                            j = j.wrapping_add(1);
                        }
                    }
                    if !would_merge {
                        let mut new_cost: libc::c_uint = current_error_cost
                            .wrapping_add(
                                (entry.depth).wrapping_mul(100 as libc::c_int as libc::c_uint),
                            )
                            .wrapping_add(
                                (position.bytes)
                                    .wrapping_sub(entry.position.bytes)
                                    .wrapping_mul(1 as libc::c_int as libc::c_uint),
                            )
                            .wrapping_add(
                                (position.extent.row)
                                    .wrapping_sub(entry.position.extent.row)
                                    .wrapping_mul(30 as libc::c_int as libc::c_uint),
                            );
                        if ts_parser__better_version_exists(
                            self_0,
                            version,
                            0 as libc::c_int != 0,
                            new_cost,
                        ) {
                            break;
                        }
                        if ts_language_has_actions(
                            (*self_0).language,
                            entry.state,
                            ts_subtree_symbol(lookahead),
                        ) {
                            if ts_parser__recover_to_state(self_0, version, depth, entry.state) {
                                did_recover = 1 as libc::c_int != 0;
                                if ((*self_0).lexer.logger.log).is_some()
                                    || !((*self_0).dot_graph_file).is_null()
                                {
                                    snwrite!(
                                        ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                                        1024 as libc::c_int as libc::c_ulong as usize,
                                        "recover_to_previous state:{}, depth:{}",
                                        entry.state as libc::c_int,
                                        depth
                                    )
                                    .unwrap_or(usize::MAX)
                                        as os::raw::c_int;
                                    ts_parser__log(self_0);
                                }
                                if !((*self_0).dot_graph_file).is_null() {
                                    ts_stack_print_dot_graph(
                                        (*self_0).stack,
                                        (*self_0).language,
                                        (*self_0).dot_graph_file,
                                    );
                                    fputs(
                                        b"\n\n\0" as *const u8 as *const libc::c_char,
                                        (*self_0).dot_graph_file,
                                    );
                                }
                                break;
                            }
                        }
                    }
                }
            }
            i = i.wrapping_add(1);
        }
    }
    let mut i_0: libc::c_uint = previous_version_count;
    while i_0 < ts_stack_version_count((*self_0).stack) {
        if !ts_stack_is_active((*self_0).stack, i_0) {
            let fresh37 = i_0;
            i_0 = i_0.wrapping_sub(1);
            ts_stack_remove_version((*self_0).stack, fresh37);
        }
        i_0 = i_0.wrapping_add(1);
    }
    if did_recover as libc::c_int != 0
        && ts_stack_version_count((*self_0).stack) > MAX_VERSION_COUNT
    {
        ts_stack_halt((*self_0).stack, version);
        ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
        return;
    }
    if did_recover as libc::c_int != 0
        && ts_subtree_has_external_scanner_state_change(lookahead) as libc::c_int != 0
    {
        ts_stack_halt((*self_0).stack, version);
        ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
        return;
    }
    if ts_subtree_is_eof(lookahead) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "recover_eof",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        let mut children: SubtreeArray = {
            let mut init = SubtreeArray {
                contents: 0 as *mut Subtree,
                size: 0 as libc::c_int as uint32_t,
                capacity: 0 as libc::c_int as uint32_t,
            };
            init
        };
        let mut parent: Subtree =
            ts_subtree_new_error_node(&mut children, 0 as libc::c_int != 0, (*self_0).language);
        ts_stack_push(
            (*self_0).stack,
            version,
            parent,
            0 as libc::c_int != 0,
            1 as libc::c_int as TSStateId,
        );
        ts_parser__accept(self_0, version, lookahead);
        return;
    }
    let mut new_cost_0: libc::c_uint = current_error_cost
        .wrapping_add(100 as libc::c_int as libc::c_uint)
        .wrapping_add(
            (ts_subtree_total_bytes(lookahead)).wrapping_mul(1 as libc::c_int as libc::c_uint),
        )
        .wrapping_add(
            ((ts_subtree_total_size(lookahead)).extent.row)
                .wrapping_mul(30 as libc::c_int as libc::c_uint),
        );
    if ts_parser__better_version_exists(self_0, version, 0 as libc::c_int != 0, new_cost_0) {
        ts_stack_halt((*self_0).stack, version);
        ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
        return;
    }
    let mut n: libc::c_uint = 0;
    let mut actions: *const TSParseAction = ts_language_actions(
        (*self_0).language,
        1 as libc::c_int as TSStateId,
        ts_subtree_symbol(lookahead),
        &mut n,
    );
    if n > 0 as libc::c_int as libc::c_uint
        && (*actions.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)).type_
            as libc::c_int
            == TSParseActionTypeShift as libc::c_int
        && (*actions.offset(n.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize))
            .shift
            .extra as libc::c_int
            != 0
    {
        let mut mutable_lookahead: MutableSubtree =
            ts_subtree_make_mut(&mut (*self_0).tree_pool, lookahead);
        ts_subtree_set_extra(&mut mutable_lookahead, 1 as libc::c_int != 0);
        lookahead = ts_subtree_from_mut(mutable_lookahead);
    }
    if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
        snwrite!(
            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong as usize,
            "skip_token symbol:{}",
            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                (*self_0).language,
                ts_subtree_symbol(lookahead)
            ))
            .to_string_lossy()
        )
        .unwrap_or(usize::MAX) as os::raw::c_int;
        ts_parser__log(self_0);
    }
    let mut children_0: SubtreeArray = {
        let mut init = SubtreeArray {
            contents: 0 as *mut Subtree,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    array__reserve(
        &mut children_0 as *mut SubtreeArray as *mut VoidArray,
        ::std::mem::size_of::<Subtree>() as libc::c_ulong,
        1 as libc::c_int as uint32_t,
    );
    array__grow(
        &mut children_0 as *mut SubtreeArray as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<Subtree>() as libc::c_ulong,
    );
    let fresh38 = children_0.size;
    children_0.size = (children_0.size).wrapping_add(1);
    *(children_0.contents).offset(fresh38 as isize) = lookahead;
    let mut error_repeat: MutableSubtree = ts_subtree_new_node(
        (-(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int) as TSSymbol,
        &mut children_0,
        0 as libc::c_int as libc::c_uint,
        (*self_0).language,
    );
    if node_count_since_error > 0 as libc::c_int as libc::c_uint {
        let mut pop: StackSliceArray =
            ts_stack_pop_count((*self_0).stack, version, 1 as libc::c_int as uint32_t);
        if pop.size > 1 as libc::c_int as libc::c_uint {
            let mut i_1: libc::c_uint = 1 as libc::c_int as libc::c_uint;
            while i_1 < pop.size {
                ts_subtree_array_delete(
                    &mut (*self_0).tree_pool,
                    &mut (*(pop.contents).offset(i_1 as isize)).subtrees,
                );
                i_1 = i_1.wrapping_add(1);
            }
            while ts_stack_version_count((*self_0).stack)
                > ((*(pop.contents).offset(0 as libc::c_int as isize)).version)
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
            {
                ts_stack_remove_version(
                    (*self_0).stack,
                    ((*(pop.contents).offset(0 as libc::c_int as isize)).version)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                );
            }
        }
        ts_stack_renumber_version(
            (*self_0).stack,
            (*(pop.contents).offset(0 as libc::c_int as isize)).version,
            version,
        );
        array__grow(
            &mut (*(pop.contents).offset(0 as libc::c_int as isize)).subtrees as *mut SubtreeArray
                as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<Subtree>() as libc::c_ulong,
        );
        let ref mut fresh39 = (*(pop.contents).offset(0 as libc::c_int as isize))
            .subtrees
            .size;
        let fresh40 = *fresh39;
        *fresh39 = (*fresh39).wrapping_add(1);
        *((*(pop.contents).offset(0 as libc::c_int as isize))
            .subtrees
            .contents)
            .offset(fresh40 as isize) = ts_subtree_from_mut(error_repeat);
        error_repeat = ts_subtree_new_node(
            (-(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int) as TSSymbol,
            &mut (*(pop.contents).offset(0 as libc::c_int as isize)).subtrees,
            0 as libc::c_int as libc::c_uint,
            (*self_0).language,
        );
    }
    ts_stack_push(
        (*self_0).stack,
        version,
        ts_subtree_from_mut(error_repeat),
        0 as libc::c_int != 0,
        0 as libc::c_int as TSStateId,
    );
    if ts_subtree_has_external_tokens(lookahead) {
        ts_stack_set_last_external_token(
            (*self_0).stack,
            version,
            ts_subtree_last_external_token(lookahead),
        );
    }
}
unsafe extern "C" fn ts_parser__handle_error(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut lookahead: Subtree,
) {
    let mut previous_version_count: uint32_t = ts_stack_version_count((*self_0).stack);
    ts_parser__do_all_potential_reductions(self_0, version, 0 as libc::c_int as TSSymbol);
    let mut version_count: uint32_t = ts_stack_version_count((*self_0).stack);
    let mut position: Length = ts_stack_position((*self_0).stack, version);
    let mut did_insert_missing_token: bool = 0 as libc::c_int != 0;
    let mut v: StackVersion = version;
    while v < version_count {
        if !did_insert_missing_token {
            let mut state: TSStateId = ts_stack_state((*self_0).stack, v);
            let mut missing_symbol: TSSymbol = 1 as libc::c_int as TSSymbol;
            while (missing_symbol as libc::c_int)
                < (*(*self_0).language).token_count as uint16_t as libc::c_int
            {
                let mut state_after_missing_symbol: TSStateId =
                    ts_language_next_state((*self_0).language, state, missing_symbol);
                if !(state_after_missing_symbol as libc::c_int == 0 as libc::c_int
                    || state_after_missing_symbol as libc::c_int == state as libc::c_int)
                {
                    if ts_language_has_reduce_action(
                        (*self_0).language,
                        state_after_missing_symbol,
                        ts_subtree_leaf_symbol(lookahead),
                    ) {
                        ts_lexer_reset(&mut (*self_0).lexer, position);
                        ts_lexer_mark_end(&mut (*self_0).lexer);
                        let mut padding: Length =
                            length_sub((*self_0).lexer.token_end_position, position);
                        let mut lookahead_bytes: uint32_t = (ts_subtree_total_bytes(lookahead))
                            .wrapping_add(ts_subtree_lookahead_bytes(lookahead));
                        let mut version_with_missing_tree: StackVersion =
                            ts_stack_copy_version((*self_0).stack, v);
                        let mut missing_tree: Subtree = ts_subtree_new_missing_leaf(
                            &mut (*self_0).tree_pool,
                            missing_symbol,
                            padding,
                            lookahead_bytes,
                            (*self_0).language,
                        );
                        ts_stack_push(
                            (*self_0).stack,
                            version_with_missing_tree,
                            missing_tree,
                            0 as libc::c_int != 0,
                            state_after_missing_symbol,
                        );
                        if ts_parser__do_all_potential_reductions(
                            self_0,
                            version_with_missing_tree,
                            ts_subtree_leaf_symbol(lookahead),
                        ) {
                            if ((*self_0).lexer.logger.log).is_some()
                                || !((*self_0).dot_graph_file).is_null()
                            {
                                snwrite!(
                                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                                    1024 as libc::c_int as libc::c_ulong as usize,
                                    "recover_with_missing symbol:{}, state:{}",
                                    std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                        (*self_0).language,
                                        missing_symbol
                                    ))
                                    .to_string_lossy(),
                                    ts_stack_state((*self_0).stack, version_with_missing_tree)
                                        as libc::c_int
                                )
                                .unwrap_or(usize::MAX)
                                    as os::raw::c_int;
                                ts_parser__log(self_0);
                            }
                            did_insert_missing_token = 1 as libc::c_int != 0;
                            break;
                        }
                    }
                }
                missing_symbol = missing_symbol.wrapping_add(1);
            }
        }
        ts_stack_push(
            (*self_0).stack,
            v,
            Subtree {
                ptr: 0 as *const SubtreeHeapData,
            },
            0 as libc::c_int != 0,
            0 as libc::c_int as TSStateId,
        );
        v = if v == version {
            previous_version_count
        } else {
            v.wrapping_add(1 as libc::c_int as libc::c_uint)
        };
    }
    let mut i: libc::c_uint = previous_version_count;
    while i < version_count {
        let mut did_merge: bool = ts_stack_merge((*self_0).stack, version, previous_version_count);
        if did_merge {
        } else {
            panic!();
        }
        i = i.wrapping_add(1);
    }
    ts_stack_record_summary((*self_0).stack, version, MAX_SUMMARY_DEPTH);
    if ts_subtree_child_count(lookahead) > 0 as libc::c_int as libc::c_uint {
        ts_parser__breakdown_lookahead(
            self_0,
            &mut lookahead,
            0 as libc::c_int as TSStateId,
            &mut (*self_0).reusable_node,
        );
    }
    ts_parser__recover(self_0, version, lookahead);
    if !((*self_0).dot_graph_file).is_null() {
        ts_stack_print_dot_graph(
            (*self_0).stack,
            (*self_0).language,
            (*self_0).dot_graph_file,
        );
        fputs(
            b"\n\n\0" as *const u8 as *const libc::c_char,
            (*self_0).dot_graph_file,
        );
    }
}
unsafe extern "C" fn ts_parser__advance(
    mut self_0: *mut TSParser,
    mut version: StackVersion,
    mut allow_node_reuse: bool,
) -> bool {
    let mut state: TSStateId = ts_stack_state((*self_0).stack, version);
    let mut position: uint32_t = (ts_stack_position((*self_0).stack, version)).bytes;
    let mut last_external_token: Subtree = ts_stack_last_external_token((*self_0).stack, version);
    let mut did_reuse: bool = 1 as libc::c_int != 0;
    let mut lookahead: Subtree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    let mut table_entry: TableEntry = {
        let mut init = TableEntry {
            actions: 0 as *const TSParseAction,
            action_count: 0 as libc::c_int as uint32_t,
            is_reusable: false,
        };
        init
    };
    if allow_node_reuse {
        lookahead = ts_parser__reuse_node(
            self_0,
            version,
            &mut state,
            position,
            last_external_token,
            &mut table_entry,
        );
    }
    if (lookahead.ptr).is_null() {
        did_reuse = 0 as libc::c_int != 0;
        lookahead = ts_parser__get_cached_token(
            self_0,
            state,
            position as size_t,
            last_external_token,
            &mut table_entry,
        );
    }
    let mut needs_lex: bool = (lookahead.ptr).is_null();
    loop {
        if needs_lex {
            needs_lex = 0 as libc::c_int != 0;
            lookahead = ts_parser__lex(self_0, version, state);
            if !(lookahead.ptr).is_null() {
                ts_parser__set_cached_token(self_0, position, last_external_token, lookahead);
                ts_language_table_entry(
                    (*self_0).language,
                    state,
                    ts_subtree_symbol(lookahead),
                    &mut table_entry,
                );
            } else {
                ts_language_table_entry(
                    (*self_0).language,
                    state,
                    0 as libc::c_int as TSSymbol,
                    &mut table_entry,
                );
            }
        }
        let ref mut fresh41 = (*self_0).operation_count;
        *fresh41 = (*fresh41).wrapping_add(1);
        if *fresh41 == OP_COUNT_PER_TIMEOUT_CHECK {
            (*self_0).operation_count = 0 as libc::c_int as libc::c_uint;
        }
        if (*self_0).operation_count == 0 as libc::c_int as libc::c_uint
            && (!((*self_0).cancellation_flag).is_null()
                && atomic_load((*self_0).cancellation_flag) != 0
                || !clock_is_null((*self_0).end_clock)
                    && clock_is_gt(clock_now(), (*self_0).end_clock) as libc::c_int != 0)
        {
            if !(lookahead.ptr).is_null() {
                ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
            }
            return 0 as libc::c_int != 0;
        }
        let mut last_reduction_version: StackVersion = -(1 as libc::c_int) as StackVersion;
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        while i < table_entry.action_count {
            let mut action: TSParseAction = *(table_entry.actions).offset(i as isize);
            match action.type_ as libc::c_int {
                0 => {
                    if !action.shift.repetition {
                        let mut next_state: TSStateId = 0;
                        if action.shift.extra {
                            next_state = state;
                            if ((*self_0).lexer.logger.log).is_some()
                                || !((*self_0).dot_graph_file).is_null()
                            {
                                snwrite!(
                                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                                    1024 as libc::c_int as libc::c_ulong as usize,
                                    "shift_extra",
                                )
                                .unwrap_or(usize::MAX)
                                    as os::raw::c_int;
                                ts_parser__log(self_0);
                            }
                        } else {
                            next_state = action.shift.state;
                            if ((*self_0).lexer.logger.log).is_some()
                                || !((*self_0).dot_graph_file).is_null()
                            {
                                snwrite!(
                                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                                    1024 as libc::c_int as libc::c_ulong as usize,
                                    "shift state:{}",
                                    next_state as libc::c_int
                                )
                                .unwrap_or(usize::MAX)
                                    as os::raw::c_int;
                                ts_parser__log(self_0);
                            }
                        }
                        if ts_subtree_child_count(lookahead) > 0 as libc::c_int as libc::c_uint {
                            ts_parser__breakdown_lookahead(
                                self_0,
                                &mut lookahead,
                                state,
                                &mut (*self_0).reusable_node,
                            );
                            next_state = ts_language_next_state(
                                (*self_0).language,
                                state,
                                ts_subtree_symbol(lookahead),
                            );
                        }
                        ts_parser__shift(
                            self_0,
                            version,
                            next_state,
                            lookahead,
                            action.shift.extra,
                        );
                        if did_reuse {
                            reusable_node_advance(&mut (*self_0).reusable_node);
                        }
                        return 1 as libc::c_int != 0;
                    }
                }
                1 => {
                    let mut is_fragile: bool =
                        table_entry.action_count > 1 as libc::c_int as libc::c_uint;
                    let mut end_of_non_terminal_extra: bool = (lookahead.ptr).is_null();
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "reduce sym:{}, child_count:{}",
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                action.reduce.symbol,
                            ))
                            .to_string_lossy(),
                            action.reduce.child_count as libc::c_int
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    let mut reduction_version: StackVersion = ts_parser__reduce(
                        self_0,
                        version,
                        action.reduce.symbol,
                        action.reduce.child_count as uint32_t,
                        action.reduce.dynamic_precedence as libc::c_int,
                        action.reduce.production_id,
                        is_fragile,
                        end_of_non_terminal_extra,
                    );
                    if reduction_version != -(1 as libc::c_int) as StackVersion {
                        last_reduction_version = reduction_version;
                    }
                }
                2 => {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "accept",
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    ts_parser__accept(self_0, version, lookahead);
                    return 1 as libc::c_int != 0;
                }
                3 => {
                    if ts_subtree_child_count(lookahead) > 0 as libc::c_int as libc::c_uint {
                        ts_parser__breakdown_lookahead(
                            self_0,
                            &mut lookahead,
                            0 as libc::c_int as TSStateId,
                            &mut (*self_0).reusable_node,
                        );
                    }
                    ts_parser__recover(self_0, version, lookahead);
                    if did_reuse {
                        reusable_node_advance(&mut (*self_0).reusable_node);
                    }
                    return 1 as libc::c_int != 0;
                }
                _ => {}
            }
            i = i.wrapping_add(1);
        }
        if last_reduction_version != -(1 as libc::c_int) as StackVersion {
            ts_stack_renumber_version((*self_0).stack, last_reduction_version, version);
            if !((*self_0).dot_graph_file).is_null() {
                ts_stack_print_dot_graph(
                    (*self_0).stack,
                    (*self_0).language,
                    (*self_0).dot_graph_file,
                );
                fputs(
                    b"\n\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).dot_graph_file,
                );
            }
            state = ts_stack_state((*self_0).stack, version);
            if (lookahead.ptr).is_null() {
                needs_lex = 1 as libc::c_int != 0;
            } else {
                ts_language_table_entry(
                    (*self_0).language,
                    state,
                    ts_subtree_leaf_symbol(lookahead),
                    &mut table_entry,
                );
            }
        } else {
            if (lookahead.ptr).is_null() {
                ts_stack_halt((*self_0).stack, version);
                return 1 as libc::c_int != 0;
            }
            if ts_subtree_is_keyword(lookahead) as libc::c_int != 0
                && ts_subtree_symbol(lookahead) as libc::c_int
                    != (*(*self_0).language).keyword_capture_token as libc::c_int
            {
                ts_language_table_entry(
                    (*self_0).language,
                    state,
                    (*(*self_0).language).keyword_capture_token,
                    &mut table_entry,
                );
                if table_entry.action_count > 0 as libc::c_int as libc::c_uint {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "switch from_keyword:{}, to_word_token:{}",
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                ts_subtree_symbol(lookahead),
                            ))
                            .to_string_lossy(),
                            std::ffi::CStr::from_ptr(ts_language_symbol_name(
                                (*self_0).language,
                                (*(*self_0).language).keyword_capture_token,
                            ))
                            .to_string_lossy()
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    let mut mutable_lookahead: MutableSubtree =
                        ts_subtree_make_mut(&mut (*self_0).tree_pool, lookahead);
                    ts_subtree_set_symbol(
                        &mut mutable_lookahead,
                        (*(*self_0).language).keyword_capture_token,
                        (*self_0).language,
                    );
                    lookahead = ts_subtree_from_mut(mutable_lookahead);
                    continue;
                }
            }
            if state as libc::c_int == 0 as libc::c_int {
                ts_parser__recover(self_0, version, lookahead);
                return 1 as libc::c_int != 0;
            }
            if ts_parser__breakdown_top_of_stack(self_0, version) {
                state = ts_stack_state((*self_0).stack, version);
                ts_subtree_release(&mut (*self_0).tree_pool, lookahead);
                needs_lex = 1 as libc::c_int != 0;
            } else {
                if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                    snwrite!(
                        ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "detect_error",
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    ts_parser__log(self_0);
                }
                ts_stack_pause((*self_0).stack, version, lookahead);
                return 1 as libc::c_int != 0;
            }
        }
    }
}
unsafe extern "C" fn ts_parser__condense_stack(mut self_0: *mut TSParser) -> libc::c_uint {
    let mut made_changes: bool = 0 as libc::c_int != 0;
    let mut min_error_cost: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
        .wrapping_mul(2 as libc::c_uint)
        .wrapping_add(1 as libc::c_uint);
    let mut i: StackVersion = 0 as libc::c_int as StackVersion;
    while i < ts_stack_version_count((*self_0).stack) {
        if ts_stack_is_halted((*self_0).stack, i) {
            ts_stack_remove_version((*self_0).stack, i);
            i = i.wrapping_sub(1);
        } else {
            let mut status_i: ErrorStatus = ts_parser__version_status(self_0, i);
            if !status_i.is_in_error && status_i.cost < min_error_cost {
                min_error_cost = status_i.cost;
            }
            let mut j: StackVersion = 0 as libc::c_int as StackVersion;
            while j < i {
                let mut status_j: ErrorStatus = ts_parser__version_status(self_0, j);
                match ts_parser__compare_versions(self_0, status_j, status_i) as libc::c_uint {
                    0 => {
                        made_changes = 1 as libc::c_int != 0;
                        ts_stack_remove_version((*self_0).stack, i);
                        i = i.wrapping_sub(1);
                        j = i;
                    }
                    1 | 2 => {
                        if ts_stack_merge((*self_0).stack, j, i) {
                            made_changes = 1 as libc::c_int != 0;
                            i = i.wrapping_sub(1);
                            j = i;
                        }
                    }
                    3 => {
                        made_changes = 1 as libc::c_int != 0;
                        if ts_stack_merge((*self_0).stack, j, i) {
                            i = i.wrapping_sub(1);
                            j = i;
                        } else {
                            ts_stack_swap_versions((*self_0).stack, i, j);
                        }
                    }
                    4 => {
                        made_changes = 1 as libc::c_int != 0;
                        ts_stack_remove_version((*self_0).stack, j);
                        i = i.wrapping_sub(1);
                        j = j.wrapping_sub(1);
                    }
                    _ => {}
                }
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    while ts_stack_version_count((*self_0).stack) > MAX_VERSION_COUNT {
        ts_stack_remove_version((*self_0).stack, MAX_VERSION_COUNT);
        made_changes = 1 as libc::c_int != 0;
    }
    if ts_stack_version_count((*self_0).stack) > 0 as libc::c_int as libc::c_uint {
        let mut has_unpaused_version: bool = 0 as libc::c_int != 0;
        let mut i_0: StackVersion = 0 as libc::c_int as StackVersion;
        let mut n: StackVersion = ts_stack_version_count((*self_0).stack);
        while i_0 < n {
            if ts_stack_is_paused((*self_0).stack, i_0) {
                if !has_unpaused_version && (*self_0).accept_count < MAX_VERSION_COUNT {
                    if ((*self_0).lexer.logger.log).is_some()
                        || !((*self_0).dot_graph_file).is_null()
                    {
                        snwrite!(
                            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                            1024 as libc::c_int as libc::c_ulong as usize,
                            "resume version:{}",
                            i_0
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                        ts_parser__log(self_0);
                    }
                    min_error_cost = ts_stack_error_cost((*self_0).stack, i_0);
                    let mut lookahead: Subtree = ts_stack_resume((*self_0).stack, i_0);
                    ts_parser__handle_error(self_0, i_0, lookahead);
                    has_unpaused_version = 1 as libc::c_int != 0;
                } else {
                    ts_stack_remove_version((*self_0).stack, i_0);
                    i_0 = i_0.wrapping_sub(1);
                    n = n.wrapping_sub(1);
                }
            } else {
                has_unpaused_version = 1 as libc::c_int != 0;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    if made_changes {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "condense",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        if !((*self_0).dot_graph_file).is_null() {
            ts_stack_print_dot_graph(
                (*self_0).stack,
                (*self_0).language,
                (*self_0).dot_graph_file,
            );
            fputs(
                b"\n\n\0" as *const u8 as *const libc::c_char,
                (*self_0).dot_graph_file,
            );
        }
    }
    return min_error_cost;
}
unsafe extern "C" fn ts_parser_has_outstanding_parse(mut self_0: *mut TSParser) -> bool {
    return ts_stack_state((*self_0).stack, 0 as libc::c_int as StackVersion) as libc::c_int
        != 1 as libc::c_int
        || ts_stack_node_count_since_error((*self_0).stack, 0 as libc::c_int as StackVersion)
            != 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_new() -> *mut TSParser {
    let mut self_0: *mut TSParser = crate::core::alloc::ts_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<TSParser>() as libc::c_ulong,
    ) as *mut TSParser;
    ts_lexer_init(&mut (*self_0).lexer);
    (*self_0).reduce_actions.size = 0 as libc::c_int as uint32_t;
    (*self_0).reduce_actions.capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh42 = (*self_0).reduce_actions.contents;
    *fresh42 = 0 as *mut ReduceAction;
    array__reserve(
        &mut (*self_0).reduce_actions as *mut ReduceActionSet as *mut VoidArray,
        ::std::mem::size_of::<ReduceAction>() as libc::c_ulong,
        4 as libc::c_int as uint32_t,
    );
    (*self_0).tree_pool = ts_subtree_pool_new(32 as libc::c_int as uint32_t);
    let ref mut fresh43 = (*self_0).stack;
    *fresh43 = ts_stack_new(&mut (*self_0).tree_pool);
    (*self_0).finished_tree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    (*self_0).reusable_node = reusable_node_new();
    let ref mut fresh44 = (*self_0).dot_graph_file;
    *fresh44 = 0 as *mut FILE;
    let ref mut fresh45 = (*self_0).cancellation_flag;
    *fresh45 = 0 as *const size_t;
    (*self_0).timeout_duration = 0 as libc::c_int as TSDuration;
    (*self_0).end_clock = clock_null();
    (*self_0).operation_count = 0 as libc::c_int as libc::c_uint;
    (*self_0).old_tree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    (*self_0).included_range_differences = {
        let mut init = TSRangeArray {
            contents: 0 as *mut TSRange,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    (*self_0).included_range_difference_index = 0 as libc::c_int as libc::c_uint;
    ts_parser__set_cached_token(
        self_0,
        0 as libc::c_int as uint32_t,
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_delete(mut self_0: *mut TSParser) {
    if self_0.is_null() {
        return;
    }
    ts_parser_set_language(self_0, 0 as *const TSLanguage);
    ts_stack_delete((*self_0).stack);
    if !((*self_0).reduce_actions.contents).is_null() {
        array__delete(&mut (*self_0).reduce_actions as *mut ReduceActionSet as *mut VoidArray);
    }
    if !((*self_0).included_range_differences.contents).is_null() {
        array__delete(
            &mut (*self_0).included_range_differences as *mut TSRangeArray as *mut VoidArray,
        );
    }
    if !((*self_0).old_tree.ptr).is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).old_tree);
        (*self_0).old_tree = Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
    }
    ts_lexer_delete(&mut (*self_0).lexer);
    ts_parser__set_cached_token(
        self_0,
        0 as libc::c_int as uint32_t,
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
    );
    ts_subtree_pool_delete(&mut (*self_0).tree_pool);
    reusable_node_delete(&mut (*self_0).reusable_node);
    array__delete(&mut (*self_0).trailing_extras as *mut SubtreeArray as *mut VoidArray);
    array__delete(&mut (*self_0).trailing_extras2 as *mut SubtreeArray as *mut VoidArray);
    array__delete(&mut (*self_0).scratch_trees as *mut SubtreeArray as *mut VoidArray);
    crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_language(mut self_0: *const TSParser) -> *const TSLanguage {
    return (*self_0).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_language(
    mut self_0: *mut TSParser,
    mut language: *const TSLanguage,
) -> bool {
    if !language.is_null() {
        if (*language).version > 14 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int != 0;
        }
        if (*language).version < 13 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int != 0;
        }
    }
    if !((*self_0).external_scanner_payload).is_null()
        && ((*(*self_0).language).external_scanner.destroy).is_some()
    {
        ((*(*self_0).language).external_scanner.destroy).expect("non-null function pointer")(
            (*self_0).external_scanner_payload,
        );
    }
    if !language.is_null() && ((*language).external_scanner.create).is_some() {
        let ref mut fresh46 = (*self_0).external_scanner_payload;
        *fresh46 = ((*language).external_scanner.create).expect("non-null function pointer")();
    } else {
        let ref mut fresh47 = (*self_0).external_scanner_payload;
        *fresh47 = 0 as *mut libc::c_void;
    }
    let ref mut fresh48 = (*self_0).language;
    *fresh48 = language;
    ts_parser_reset(self_0);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_logger(mut self_0: *const TSParser) -> TSLogger {
    return (*self_0).lexer.logger;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_logger(mut self_0: *mut TSParser, mut logger: TSLogger) {
    (*self_0).lexer.logger = logger;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_print_dot_graphs(
    mut self_0: *mut TSParser,
    mut fd: libc::c_int,
) {
    if !((*self_0).dot_graph_file).is_null() {
        fclose((*self_0).dot_graph_file);
    }
    if fd >= 0 as libc::c_int {
        let ref mut fresh49 = (*self_0).dot_graph_file;
        *fresh49 = fdopen(fd, b"a\0" as *const u8 as *const libc::c_char);
    } else {
        let ref mut fresh50 = (*self_0).dot_graph_file;
        *fresh50 = 0 as *mut FILE;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_cancellation_flag(mut self_0: *const TSParser) -> *const size_t {
    return (*self_0).cancellation_flag as *const size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_cancellation_flag(
    mut self_0: *mut TSParser,
    mut flag: *const size_t,
) {
    let ref mut fresh51 = (*self_0).cancellation_flag;
    *fresh51 = flag as *const size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_timeout_micros(mut self_0: *const TSParser) -> uint64_t {
    return duration_to_micros((*self_0).timeout_duration);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_timeout_micros(
    mut self_0: *mut TSParser,
    mut timeout_micros: uint64_t,
) {
    (*self_0).timeout_duration = duration_from_micros(timeout_micros);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_set_included_ranges(
    mut self_0: *mut TSParser,
    mut ranges: *const TSRange,
    mut count: uint32_t,
) -> bool {
    return ts_lexer_set_included_ranges(&mut (*self_0).lexer, ranges, count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_included_ranges(
    mut self_0: *const TSParser,
    mut count: *mut uint32_t,
) -> *const TSRange {
    return ts_lexer_included_ranges(&(*self_0).lexer, count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_reset(mut self_0: *mut TSParser) {
    if !((*self_0).language).is_null()
        && ((*(*self_0).language).external_scanner.deserialize).is_some()
    {
        ((*(*self_0).language).external_scanner.deserialize).expect("non-null function pointer")(
            (*self_0).external_scanner_payload,
            0 as *const libc::c_char,
            0 as libc::c_int as libc::c_uint,
        );
    }
    if !((*self_0).old_tree.ptr).is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).old_tree);
        (*self_0).old_tree = Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
    }
    reusable_node_clear(&mut (*self_0).reusable_node);
    ts_lexer_reset(&mut (*self_0).lexer, length_zero());
    ts_stack_clear((*self_0).stack);
    ts_parser__set_cached_token(
        self_0,
        0 as libc::c_int as uint32_t,
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
    );
    if !((*self_0).finished_tree.ptr).is_null() {
        ts_subtree_release(&mut (*self_0).tree_pool, (*self_0).finished_tree);
        (*self_0).finished_tree = Subtree {
            ptr: 0 as *const SubtreeHeapData,
        };
    }
    (*self_0).accept_count = 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse(
    mut self_0: *mut TSParser,
    mut old_tree: *const TSTree,
    mut input: TSInput,
) -> *mut TSTree {
    if ((*self_0).language).is_null() || (input.read).is_none() {
        return 0 as *mut TSTree;
    }
    ts_lexer_set_input(&mut (*self_0).lexer, input);
    (*self_0).included_range_differences.size = 0 as libc::c_int as uint32_t;
    (*self_0).included_range_difference_index = 0 as libc::c_int as libc::c_uint;
    if ts_parser_has_outstanding_parse(self_0) {
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "resume_parsing",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
    } else if !old_tree.is_null() {
        ts_subtree_retain((*old_tree).root);
        (*self_0).old_tree = (*old_tree).root;
        ts_range_array_get_changed_ranges(
            (*old_tree).included_ranges,
            (*old_tree).included_range_count,
            (*self_0).lexer.included_ranges,
            (*self_0).lexer.included_range_count,
            &mut (*self_0).included_range_differences,
        );
        reusable_node_reset(&mut (*self_0).reusable_node, (*old_tree).root);
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "parse_after_edit",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
        if !((*self_0).dot_graph_file).is_null() {
            ts_subtree_print_dot_graph(
                (*self_0).old_tree,
                (*self_0).language,
                (*self_0).dot_graph_file,
            );
            fputs(
                b"\n\0" as *const u8 as *const libc::c_char,
                (*self_0).dot_graph_file,
            );
        }
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).included_range_differences.size {
            let mut range: *mut TSRange = &mut *((*self_0).included_range_differences.contents)
                .offset(i as isize) as *mut TSRange;
            if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                snwrite!(
                    ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "different_included_range {} - {}",
                    (*range).start_byte,
                    (*range).end_byte
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                ts_parser__log(self_0);
            }
            i = i.wrapping_add(1);
        }
    } else {
        reusable_node_clear(&mut (*self_0).reusable_node);
        if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
            snwrite!(
                ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                1024 as libc::c_int as libc::c_ulong as usize,
                "new_parse",
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            ts_parser__log(self_0);
        }
    }
    (*self_0).operation_count = 0 as libc::c_int as libc::c_uint;
    if (*self_0).timeout_duration != 0 {
        (*self_0).end_clock = clock_after(clock_now(), (*self_0).timeout_duration);
    } else {
        (*self_0).end_clock = clock_null();
    }
    let mut position: uint32_t = 0 as libc::c_int as uint32_t;
    let mut last_position: uint32_t = 0 as libc::c_int as uint32_t;
    let mut version_count: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        let mut version: StackVersion = 0 as libc::c_int as StackVersion;
        loop {
            version_count = ts_stack_version_count((*self_0).stack);
            if !(version < version_count) {
                break;
            }
            let mut allow_node_reuse: bool = version_count == 1 as libc::c_int as libc::c_uint;
            while ts_stack_is_active((*self_0).stack, version) {
                if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
                    snwrite!(
                        ((*self_0).lexer.debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "process version:{}, version_count:{}, state:{}, row:{}, col:{}",
                        version,
                        ts_stack_version_count((*self_0).stack),
                        ts_stack_state((*self_0).stack, version) as libc::c_int,
                        (ts_stack_position((*self_0).stack, version)).extent.row,
                        (ts_stack_position((*self_0).stack, version)).extent.column
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    ts_parser__log(self_0);
                }
                if !ts_parser__advance(self_0, version, allow_node_reuse) {
                    return 0 as *mut TSTree;
                }
                if !((*self_0).dot_graph_file).is_null() {
                    ts_stack_print_dot_graph(
                        (*self_0).stack,
                        (*self_0).language,
                        (*self_0).dot_graph_file,
                    );
                    fputs(
                        b"\n\n\0" as *const u8 as *const libc::c_char,
                        (*self_0).dot_graph_file,
                    );
                }
                position = (ts_stack_position((*self_0).stack, version)).bytes;
                if !(position > last_position
                    || version > 0 as libc::c_int as libc::c_uint && position == last_position)
                {
                    continue;
                }
                last_position = position;
                break;
            }
            version = version.wrapping_add(1);
        }
        let mut min_error_cost: libc::c_uint = ts_parser__condense_stack(self_0);
        if !((*self_0).finished_tree.ptr).is_null()
            && ts_subtree_error_cost((*self_0).finished_tree) < min_error_cost
        {
            ts_stack_clear((*self_0).stack);
            break;
        } else {
            while (*self_0).included_range_difference_index
                < (*self_0).included_range_differences.size
            {
                let mut range_0: *mut TSRange =
                    &mut *((*self_0).included_range_differences.contents)
                        .offset((*self_0).included_range_difference_index as isize)
                        as *mut TSRange;
                if !((*range_0).end_byte <= position) {
                    break;
                }
                let ref mut fresh52 = (*self_0).included_range_difference_index;
                *fresh52 = (*fresh52).wrapping_add(1);
            }
            if !(version_count != 0 as libc::c_int as libc::c_uint) {
                break;
            }
        }
    }
    if !((*self_0).finished_tree.ptr).is_null() {
    } else {
        panic!();
    }
    ts_subtree_balance(
        (*self_0).finished_tree,
        &mut (*self_0).tree_pool,
        (*self_0).language,
    );
    if ((*self_0).lexer.logger.log).is_some() || !((*self_0).dot_graph_file).is_null() {
        snwrite!(
            ((*self_0).lexer.debug_buffer).as_mut_ptr(),
            1024 as libc::c_int as libc::c_ulong as usize,
            "done",
        )
        .unwrap_or(usize::MAX) as os::raw::c_int;
        ts_parser__log(self_0);
    }
    if !((*self_0).dot_graph_file).is_null() {
        ts_subtree_print_dot_graph(
            (*self_0).finished_tree,
            (*self_0).language,
            (*self_0).dot_graph_file,
        );
        fputs(
            b"\n\0" as *const u8 as *const libc::c_char,
            (*self_0).dot_graph_file,
        );
    }
    let mut result: *mut TSTree = ts_tree_new(
        (*self_0).finished_tree,
        (*self_0).language,
        (*self_0).lexer.included_ranges,
        (*self_0).lexer.included_range_count,
    );
    (*self_0).finished_tree = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    ts_parser_reset(self_0);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse_string(
    mut self_0: *mut TSParser,
    mut old_tree: *const TSTree,
    mut string: *const libc::c_char,
    mut length: uint32_t,
) -> *mut TSTree {
    return ts_parser_parse_string_encoding(self_0, old_tree, string, length, TSInputEncodingUTF8);
}
#[no_mangle]
pub unsafe extern "C" fn ts_parser_parse_string_encoding(
    mut self_0: *mut TSParser,
    mut old_tree: *const TSTree,
    mut string: *const libc::c_char,
    mut length: uint32_t,
    mut encoding: TSInputEncoding,
) -> *mut TSTree {
    let mut input: TSStringInput = {
        let mut init = TSStringInput {
            string: string,
            length: length,
        };
        init
    };
    return ts_parser_parse(self_0, old_tree, {
        let mut init = TSInput {
            payload: &mut input as *mut TSStringInput as *mut libc::c_void,
            read: Some(
                ts_string_input_read
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        uint32_t,
                        TSPoint,
                        *mut uint32_t,
                    ) -> *const libc::c_char,
            ),
            encoding: encoding,
        };
        init
    });
}
pub const ErrorComparison_ErrorComparisonPreferRight: ErrorComparison = ErrorComparisonPreferRight;
pub const ErrorComparison_ErrorComparisonTakeRight: ErrorComparison = ErrorComparisonTakeRight;
pub const ErrorComparison_ErrorComparisonNone: ErrorComparison = ErrorComparisonNone;
pub const ErrorComparison_ErrorComparisonPreferLeft: ErrorComparison = ErrorComparisonPreferLeft;
pub const ErrorComparison_ErrorComparisonTakeLeft: ErrorComparison = ErrorComparisonTakeLeft;
