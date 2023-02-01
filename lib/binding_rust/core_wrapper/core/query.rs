use crate::core::util::*;
use crate::core::*;
use :: c2rust_bitfields;
use std::os;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
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
pub struct C2RustUnnamed_0 {
    pub count: uint8_t,
    pub reusable: bool,
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
pub struct TSQuery {
    pub captures: SymbolTable,
    pub predicate_values: SymbolTable,
    pub capture_quantifiers: C2RustUnnamed_11,
    pub steps: C2RustUnnamed_10,
    pub pattern_map: C2RustUnnamed_9,
    pub predicate_steps: C2RustUnnamed_8,
    pub patterns: C2RustUnnamed_7,
    pub step_offsets: C2RustUnnamed_6,
    pub negated_fields: C2RustUnnamed_5,
    pub string_buffer: C2RustUnnamed_4,
    pub repeat_symbols_with_rootless_patterns: C2RustUnnamed_3,
    pub language: *const TSLanguage,
    pub wildcard_root_pattern_count: uint16_t,
}
type C2RustUnnamed_3 = crate::core::util::StackElement<*mut TSSymbol>;
type C2RustUnnamed_4 = crate::core::util::StackElement<*mut libc::c_char>;
type C2RustUnnamed_5 = crate::core::util::StackElement<*mut TSFieldId>;
type C2RustUnnamed_6 = crate::core::util::StackElement<*mut StepOffset>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StepOffset {
    pub byte_offset: uint32_t,
    pub step_index: uint16_t,
}
type C2RustUnnamed_7 = crate::core::util::StackElement<*mut QueryPattern>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryPattern {
    pub steps: Slice,
    pub predicate_steps: Slice,
    pub start_byte: uint32_t,
    pub is_non_local: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Slice {
    pub offset: uint32_t,
    pub length: uint32_t,
}
type C2RustUnnamed_8 = crate::core::util::StackElement<*mut TSQueryPredicateStep>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryPredicateStep {
    pub type_: TSQueryPredicateStepType,
    pub value_id: uint32_t,
}
type C2RustUnnamed_9 = crate::core::util::StackElement<*mut PatternEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PatternEntry {
    pub step_index: uint16_t,
    pub pattern_index: uint16_t,
    pub is_rooted: bool,
}
type C2RustUnnamed_10 = crate::core::util::StackElement<*mut QueryStep>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct QueryStep {
    pub symbol: TSSymbol,
    pub supertype_symbol: TSSymbol,
    pub field: TSFieldId,
    pub capture_ids: [uint16_t; 3],
    pub depth: uint16_t,
    pub alternative_index: uint16_t,
    pub negated_field_list_id: uint16_t,
    #[bitfield(name = "is_named", ty = "bool", bits = "0..=0")]
    #[bitfield(name = "is_immediate", ty = "bool", bits = "1..=1")]
    #[bitfield(name = "is_last_child", ty = "bool", bits = "2..=2")]
    #[bitfield(name = "is_pass_through", ty = "bool", bits = "3..=3")]
    #[bitfield(name = "is_dead_end", ty = "bool", bits = "4..=4")]
    #[bitfield(name = "alternative_is_immediate", ty = "bool", bits = "5..=5")]
    #[bitfield(name = "contains_captures", ty = "bool", bits = "6..=6")]
    #[bitfield(name = "root_pattern_guaranteed", ty = "bool", bits = "7..=7")]
    #[bitfield(name = "parent_pattern_guaranteed", ty = "bool", bits = "8..=8")]
    pub is_named_is_immediate_is_last_child_is_pass_through_is_dead_end_alternative_is_immediate_contains_captures_root_pattern_guaranteed_parent_pattern_guaranteed:
        [u8; 2],
}
type C2RustUnnamed_11 = crate::core::util::StackElement<*mut CaptureQuantifiers>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureQuantifiers {
    pub contents: *mut uint8_t,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SymbolTable {
    pub characters: C2RustUnnamed_13,
    pub slices: C2RustUnnamed_12,
}
type C2RustUnnamed_12 = crate::core::util::StackElement<*mut Slice>;
type C2RustUnnamed_13 = crate::core::util::StackElement<*mut libc::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryCursor {
    pub query: *const TSQuery,
    pub cursor: TSTreeCursor,
    pub states: C2RustUnnamed_16,
    pub finished_states: C2RustUnnamed_15,
    pub capture_list_pool: CaptureListPool,
    pub depth: uint32_t,
    pub max_start_depth: uint32_t,
    pub start_byte: uint32_t,
    pub end_byte: uint32_t,
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub next_state_id: uint32_t,
    pub on_visible_node: bool,
    pub ascending: bool,
    pub halted: bool,
    pub did_exceed_match_limit: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureListPool {
    pub list: C2RustUnnamed_14,
    pub empty_list: CaptureList,
    pub max_capture_list_count: uint32_t,
    pub free_capture_list_count: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureList {
    pub contents: *mut TSQueryCapture,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryCapture {
    pub node: TSNode,
    pub index: uint32_t,
}
type C2RustUnnamed_14 = crate::core::util::StackElement<*mut CaptureList>;
type C2RustUnnamed_15 = crate::core::util::StackElement<*mut QueryState>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct QueryState {
    pub id: uint32_t,
    pub capture_list_id: uint32_t,
    pub start_depth: uint16_t,
    pub step_index: uint16_t,
    pub pattern_index: uint16_t,
    #[bitfield(name = "consumed_capture_count", ty = "uint16_t", bits = "0..=11")]
    #[bitfield(name = "seeking_immediate_match", ty = "bool", bits = "12..=12")]
    #[bitfield(name = "has_in_progress_alternatives", ty = "bool", bits = "13..=13")]
    #[bitfield(name = "dead", ty = "bool", bits = "14..=14")]
    #[bitfield(name = "needs_parent", ty = "bool", bits = "15..=15")]
    pub consumed_capture_count_seeking_immediate_match_has_in_progress_alternatives_dead_needs_parent:
        [u8; 2],
}
type C2RustUnnamed_16 = crate::core::util::StackElement<*mut QueryState>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSQueryMatch {
    pub id: uint32_t,
    pub pattern_index: uint16_t,
    pub capture_count: uint16_t,
    pub captures: *const TSQueryCapture,
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
pub struct StatePredecessorMap {
    pub contents: *mut TSStateId,
}
type C2RustUnnamed_17 = crate::core::util::StackElement<*mut uint16_t>;
type C2RustUnnamed_18 = crate::core::util::StackElement<*mut uint32_t>;
type C2RustUnnamed_19 = crate::core::util::StackElement<*mut uint16_t>;
type C2RustUnnamed_20 = crate::core::util::StackElement<*mut AnalysisSubgraphNode>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AnalysisSubgraphNode {
    pub state: TSStateId,
    pub production_id: uint16_t,
    #[bitfield(name = "child_index", ty = "uint8_t", bits = "0..=6")]
    #[bitfield(name = "done", ty = "bool", bits = "7..=7")]
    pub child_index_done: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct QueryAnalysis {
    pub states: AnalysisStateSet,
    pub next_states: AnalysisStateSet,
    pub deeper_states: AnalysisStateSet,
    pub state_pool: AnalysisStateSet,
    pub final_step_indices: C2RustUnnamed_22,
    pub finished_parent_symbols: C2RustUnnamed_21,
    pub did_abort: bool,
}
type C2RustUnnamed_21 = crate::core::util::StackElement<*mut TSSymbol>;
type C2RustUnnamed_22 = crate::core::util::StackElement<*mut uint16_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnalysisStateSet {
    pub contents: *mut *mut AnalysisState,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnalysisState {
    pub stack: [AnalysisStateEntry; 8],
    pub depth: uint16_t,
    pub step_index: uint16_t,
    pub root_symbol: TSSymbol,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct AnalysisStateEntry {
    pub parse_state: TSStateId,
    pub parent_symbol: TSSymbol,
    pub child_index: uint16_t,
    #[bitfield(name = "field_id", ty = "TSFieldId", bits = "0..=14")]
    #[bitfield(name = "done", ty = "bool", bits = "15..=15")]
    pub field_id_done: [u8; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnalysisSubgraphArray {
    pub contents: *mut AnalysisSubgraph,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnalysisSubgraph {
    pub symbol: TSSymbol,
    pub start_states: C2RustUnnamed_24,
    pub nodes: C2RustUnnamed_23,
}
type C2RustUnnamed_23 = crate::core::util::StackElement<*mut AnalysisSubgraphNode>;
type C2RustUnnamed_24 = crate::core::util::StackElement<*mut TSStateId>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LookaheadIterator {
    pub language: *const TSLanguage,
    pub data: *const uint16_t,
    pub group_end: *const uint16_t,
    pub state: TSStateId,
    pub table_value: uint16_t,
    pub section_index: uint16_t,
    pub group_count: uint16_t,
    pub is_small_state: bool,
    pub actions: *const TSParseAction,
    pub symbol: TSSymbol,
    pub next_state: TSStateId,
    pub action_count: uint16_t,
}
pub const TSParseActionTypeShift: C2RustUnnamed_31 = 0;
pub const TSParseActionTypeReduce: C2RustUnnamed_31 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stream {
    pub input: *const libc::c_char,
    pub start: *const libc::c_char,
    pub end: *const libc::c_char,
    pub next: int32_t,
    pub next_size: uint8_t,
}
pub type wint_t = libc::c_uint;
type C2RustUnnamed_25 = crate::core::util::StackElement<*mut uint32_t>;
pub const TreeCursorStepHidden: TreeCursorStep = 1;
pub const TreeCursorStepVisible: TreeCursorStep = 2;
pub type TreeCursorStep = libc::c_uint;
pub const TreeCursorStepNone: TreeCursorStep = 0;
type C2RustUnnamed_26 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_27 = crate::core::util::LongShortData;
type C2RustUnnamed_28 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_29 = crate::core::util::ScannerStateLookaheadFirstLeaf;
type C2RustUnnamed_30 = crate::core::util::StackElement<*mut TreeCursorEntry>;
pub type C2RustUnnamed_31 = libc::c_uint;
pub const TSParseActionTypeRecover: C2RustUnnamed_31 = 3;
pub const TSParseActionTypeAccept: C2RustUnnamed_31 = 2;
#[inline]
unsafe extern "C" fn array__reserve(
    mut self_0: *mut VoidArray,
    mut element_size: size_t,
    mut new_capacity: uint32_t,
) {
    if new_capacity > (*self_0).capacity {
        if !((*self_0).contents).is_null() {
            let ref mut fresh0 = (*self_0).contents;
            *fresh0 = crate::core::alloc::ts_realloc(
                (*self_0).contents,
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        } else {
            let ref mut fresh1 = (*self_0).contents;
            *fresh1 = crate::core::alloc::ts_malloc(
                (new_capacity as libc::c_ulong).wrapping_mul(element_size),
            );
        }
        (*self_0).capacity = new_capacity;
    }
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
    let ref mut fresh2 = (*self_0).size;
    *fresh2 = (*fresh2).wrapping_sub(1);
}
#[inline]
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    crate::core::alloc::ts_free((*self_0).contents);
    let ref mut fresh3 = (*self_0).contents;
    *fresh3 = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
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
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.symbol as libc::c_int
    } else {
        (*self_0.ptr).symbol as libc::c_int
    }) as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_language_alias_at(
    mut self_0: *const TSLanguage,
    mut production_id: uint32_t,
    mut child_index: uint32_t,
) -> TSSymbol {
    return (if production_id != 0 {
        *((*self_0).alias_sequences).offset(
            production_id
                .wrapping_mul((*self_0).max_alias_sequence_length as libc::c_uint)
                .wrapping_add(child_index) as isize,
        ) as libc::c_int
    } else {
        0 as libc::c_int
    }) as TSSymbol;
}
#[inline]
unsafe extern "C" fn ts_language_lookaheads(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
) -> LookaheadIterator {
    let mut is_small_state: bool = state as libc::c_uint >= (*self_0).large_state_count;
    let mut data: *const uint16_t = 0 as *const uint16_t;
    let mut group_end: *const uint16_t = 0 as *const uint16_t;
    let mut group_count: uint16_t = 0 as libc::c_int as uint16_t;
    if is_small_state {
        let mut index: uint32_t = *((*self_0).small_parse_table_map)
            .offset((state as libc::c_uint).wrapping_sub((*self_0).large_state_count) as isize);
        data = &*((*self_0).small_parse_table).offset(index as isize) as *const uint16_t;
        group_end = data.offset(1 as libc::c_int as isize);
        group_count = *data;
    } else {
        data = (&*((*self_0).parse_table)
            .offset((state as libc::c_uint).wrapping_mul((*self_0).symbol_count) as isize)
            as *const uint16_t)
            .offset(-(1 as libc::c_int as isize));
    }
    return {
        let mut init = LookaheadIterator {
            language: self_0,
            data: data,
            group_end: group_end,
            state: 0,
            table_value: 0,
            section_index: 0,
            group_count: group_count,
            is_small_state: is_small_state,
            actions: 0 as *const TSParseAction,
            symbol: 65535 as libc::c_int as TSSymbol,
            next_state: 0 as libc::c_int as TSStateId,
            action_count: 0,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn ts_subtree_is_repetition(mut self_0: Subtree) -> uint32_t {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int
    } else {
        (!(*self_0.ptr).named()
            && !(*self_0.ptr).visible()
            && (*self_0.ptr).child_count != 0 as libc::c_int as libc::c_uint) as libc::c_int
    }) as uint32_t;
}
#[inline]
unsafe extern "C" fn ts_lookahead_iterator__next(mut self_0: *mut LookaheadIterator) -> bool {
    if (*self_0).is_small_state {
        let ref mut fresh5 = (*self_0).data;
        *fresh5 = (*fresh5).offset(1);
        if (*self_0).data == (*self_0).group_end {
            if (*self_0).group_count as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            let ref mut fresh6 = (*self_0).group_count;
            *fresh6 = (*fresh6).wrapping_sub(1);
            let ref mut fresh7 = (*self_0).data;
            let fresh8 = *fresh7;
            *fresh7 = (*fresh7).offset(1);
            (*self_0).table_value = *fresh8;
            let ref mut fresh9 = (*self_0).data;
            let fresh10 = *fresh9;
            *fresh9 = (*fresh9).offset(1);
            let mut symbol_count: libc::c_uint = *fresh10 as libc::c_uint;
            let ref mut fresh11 = (*self_0).group_end;
            *fresh11 = ((*self_0).data).offset(symbol_count as isize);
            (*self_0).symbol = *(*self_0).data;
        } else {
            (*self_0).symbol = *(*self_0).data;
            return 1 as libc::c_int != 0;
        }
    } else {
        loop {
            let ref mut fresh12 = (*self_0).data;
            *fresh12 = (*fresh12).offset(1);
            let ref mut fresh13 = (*self_0).symbol;
            *fresh13 = (*fresh13).wrapping_add(1);
            if (*self_0).symbol as libc::c_uint >= (*(*self_0).language).symbol_count {
                return 0 as libc::c_int != 0;
            }
            (*self_0).table_value = *(*self_0).data;
            if !((*self_0).table_value == 0) {
                break;
            }
        }
    }
    if ((*self_0).symbol as libc::c_uint) < (*(*self_0).language).token_count {
        let mut entry: *const TSParseActionEntry = &*((*(*self_0).language).parse_actions)
            .offset((*self_0).table_value as isize)
            as *const TSParseActionEntry;
        (*self_0).action_count = (*entry).entry.count as uint16_t;
        let ref mut fresh14 = (*self_0).actions;
        *fresh14 = entry.offset(1 as libc::c_int as isize) as *const TSParseAction;
        (*self_0).next_state = 0 as libc::c_int as TSStateId;
    } else {
        (*self_0).action_count = 0 as libc::c_int as uint16_t;
        (*self_0).next_state = (*self_0).table_value;
    }
    return 1 as libc::c_int != 0;
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
unsafe extern "C" fn point_lte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column <= b.column;
}
#[inline]
unsafe extern "C" fn ts_language_aliases_for_symbol(
    mut self_0: *const TSLanguage,
    mut original_symbol: TSSymbol,
    mut start: *mut *const TSSymbol,
    mut end: *mut *const TSSymbol,
) {
    *start = &*((*self_0).public_symbol_map).offset(original_symbol as isize) as *const TSSymbol;
    *end = (*start).offset(1 as libc::c_int as isize);
    let mut idx: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        let fresh15 = idx;
        idx = idx.wrapping_add(1);
        let mut symbol: TSSymbol = *((*self_0).alias_map).offset(fresh15 as isize);
        if symbol as libc::c_int == 0 as libc::c_int
            || symbol as libc::c_int > original_symbol as libc::c_int
        {
            break;
        }
        let fresh16 = idx;
        idx = idx.wrapping_add(1);
        let mut count: uint16_t = *((*self_0).alias_map).offset(fresh16 as isize);
        if symbol as libc::c_int == original_symbol as libc::c_int {
            *start = &*((*self_0).alias_map).offset(idx as isize) as *const uint16_t;
            *end = &*((*self_0).alias_map).offset(idx.wrapping_add(count as libc::c_uint) as isize)
                as *const uint16_t;
            break;
        } else {
            idx = idx.wrapping_add(count as libc::c_uint);
        }
    }
}
#[inline]
unsafe extern "C" fn ts_language_state_is_primary(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
) -> bool {
    if (*self_0).version >= 14 as libc::c_int as libc::c_uint {
        return state as libc::c_int
            == *((*self_0).primary_state_ids).offset(state as isize) as libc::c_int;
    } else {
        return 1 as libc::c_int != 0;
    };
}
#[inline]
unsafe extern "C" fn point_gte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row > b.row || a.row == b.row && a.column >= b.column;
}
#[inline]
unsafe extern "C" fn ts_tree_cursor_current_subtree(mut _self: *const TSTreeCursor) -> Subtree {
    let mut self_0: *const TreeCursor = _self as *const TreeCursor;
    if ((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).stack.size
    {
    } else {
        panic!();
    }
    let mut last_entry: *mut TreeCursorEntry = &mut *((*self_0).stack.contents)
        .offset(((*self_0).stack.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut TreeCursorEntry;
    return *(*last_entry).subtree;
}
#[inline]
unsafe extern "C" fn ts_decode_utf8(
    mut string: *const uint8_t,
    mut length: uint32_t,
    mut code_point: *mut int32_t,
) -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh17 = i;
    i = i.wrapping_add(1);
    *code_point = *string.offset(fresh17 as isize) as int32_t;
    if !(*code_point & 0x80 as libc::c_int == 0 as libc::c_int) {
        let mut __t: uint8_t = 0 as libc::c_int as uint8_t;
        if !(i != length
            && (if *code_point >= 0xe0 as libc::c_int {
                ((if *code_point < 0xf0 as libc::c_int {
                    *code_point &= 0xf as libc::c_int;
                    __t = *string.offset(i as isize);
                    ((*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b" 000000000000\x1000\0",
                    ))[*code_point as usize] as libc::c_int
                        & (1 as libc::c_int) << (__t as libc::c_int >> 5 as libc::c_int)
                        != 0
                        && {
                            __t = (__t as libc::c_int & 0x3f as libc::c_int) as uint8_t;
                            1 as libc::c_int != 0
                        }) as libc::c_int
                } else {
                    *code_point -= 0xf0 as libc::c_int;
                    (*code_point <= 4 as libc::c_int
                        && {
                            __t = *string.offset(i as isize);
                            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                                b"\0\0\0\0\0\0\0\0\x1E\x0F\x0F\x0F\0\0\0\0\0",
                            ))[(__t as libc::c_int >> 4 as libc::c_int) as usize]
                                as libc::c_int
                                & (1 as libc::c_int) << *code_point
                                != 0
                        }
                        && {
                            *code_point = *code_point << 6 as libc::c_int
                                | __t as libc::c_int & 0x3f as libc::c_int;
                            i = i.wrapping_add(1);
                            i != length
                        }
                        && {
                            __t = (*string.offset(i as isize) as libc::c_int - 0x80 as libc::c_int)
                                as uint8_t;
                            __t as libc::c_int <= 0x3f as libc::c_int
                        }) as libc::c_int
                }) != 0
                    && {
                        *code_point = *code_point << 6 as libc::c_int | __t as libc::c_int;
                        i = i.wrapping_add(1);
                        i != length
                    }) as libc::c_int
            } else {
                (*code_point >= 0xc2 as libc::c_int && {
                    *code_point &= 0x1f as libc::c_int;
                    1 as libc::c_int != 0
                }) as libc::c_int
            }) != 0
            && {
                __t = (*string.offset(i as isize) as libc::c_int - 0x80 as libc::c_int) as uint8_t;
                __t as libc::c_int <= 0x3f as libc::c_int
            }
            && {
                *code_point = *code_point << 6 as libc::c_int | __t as libc::c_int;
                i = i.wrapping_add(1);
                1 as libc::c_int != 0
            })
        {
            *code_point = -(1 as libc::c_int);
        }
    }
    return i;
}
static mut PARENT_DONE: TSQueryError = 4294967295 as TSQueryError;
static mut PATTERN_DONE_MARKER: uint16_t = 65535 as libc::c_int as uint16_t;
static mut NONE: uint16_t = 65535 as libc::c_int as uint16_t;
static mut WILDCARD_SYMBOL: TSSymbol = 0 as libc::c_int as TSSymbol;
unsafe extern "C" fn stream_advance(mut self_0: *mut Stream) -> bool {
    let ref mut fresh18 = (*self_0).input;
    *fresh18 = (*fresh18).offset((*self_0).next_size as libc::c_int as isize);
    if (*self_0).input < (*self_0).end {
        let mut size: uint32_t = ts_decode_utf8(
            (*self_0).input as *const uint8_t,
            ((*self_0).end).offset_from((*self_0).input) as libc::c_long as uint32_t,
            &mut (*self_0).next,
        );
        if size > 0 as libc::c_int as libc::c_uint {
            (*self_0).next_size = size as uint8_t;
            return 1 as libc::c_int != 0;
        }
    } else {
        (*self_0).next_size = 0 as libc::c_int as uint8_t;
        (*self_0).next = '\0' as i32;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn stream_reset(mut self_0: *mut Stream, mut input: *const libc::c_char) {
    let ref mut fresh19 = (*self_0).input;
    *fresh19 = input;
    (*self_0).next_size = 0 as libc::c_int as uint8_t;
    stream_advance(self_0);
}
unsafe extern "C" fn stream_new(mut string: *const libc::c_char, mut length: uint32_t) -> Stream {
    let mut self_0: Stream = {
        let mut init = Stream {
            input: string,
            start: string,
            end: string.offset(length as isize),
            next: 0 as libc::c_int,
            next_size: 0,
        };
        init
    };
    stream_advance(&mut self_0);
    return self_0;
}
unsafe extern "C" fn stream_skip_whitespace(mut self_0: *mut Stream) {
    loop {
        if iswspace((*self_0).next as wint_t) != 0 {
            stream_advance(self_0);
        } else {
            if !((*self_0).next == ';' as i32) {
                break;
            }
            stream_advance(self_0);
            while (*self_0).next != 0 && (*self_0).next != '\n' as i32 {
                if !stream_advance(self_0) {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn stream_is_ident_start(mut self_0: *mut Stream) -> bool {
    return iswalnum((*self_0).next as wint_t) != 0
        || (*self_0).next == '_' as i32
        || (*self_0).next == '-' as i32;
}
unsafe extern "C" fn stream_scan_identifier(mut stream: *mut Stream) {
    loop {
        stream_advance(stream);
        if !(iswalnum((*stream).next as wint_t) != 0
            || (*stream).next == '_' as i32
            || (*stream).next == '-' as i32
            || (*stream).next == '.' as i32
            || (*stream).next == '?' as i32
            || (*stream).next == '!' as i32)
        {
            break;
        }
    }
}
unsafe extern "C" fn stream_offset(mut self_0: *mut Stream) -> uint32_t {
    return ((*self_0).input).offset_from((*self_0).start) as libc::c_long as uint32_t;
}
unsafe extern "C" fn capture_list_pool_new() -> CaptureListPool {
    return {
        let mut init = CaptureListPool {
            list: {
                let mut init = C2RustUnnamed_14 {
                    contents: 0 as *mut CaptureList,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            empty_list: {
                let mut init = CaptureList {
                    contents: 0 as *mut TSQueryCapture,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            max_capture_list_count: 4294967295 as libc::c_uint,
            free_capture_list_count: 0 as libc::c_int as uint32_t,
        };
        init
    };
}
unsafe extern "C" fn capture_list_pool_reset(mut self_0: *mut CaptureListPool) {
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < (*self_0).list.size as uint16_t as libc::c_int {
        (*((*self_0).list.contents).offset(i as isize)).size = 4294967295 as libc::c_uint;
        i = i.wrapping_add(1);
    }
    (*self_0).free_capture_list_count = (*self_0).list.size;
}
unsafe extern "C" fn capture_list_pool_delete(mut self_0: *mut CaptureListPool) {
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < (*self_0).list.size as uint16_t as libc::c_int {
        array__delete(
            &mut *((*self_0).list.contents).offset(i as isize) as *mut CaptureList
                as *mut VoidArray,
        );
        i = i.wrapping_add(1);
    }
    array__delete(&mut (*self_0).list as *mut C2RustUnnamed_14 as *mut VoidArray);
}
unsafe extern "C" fn capture_list_pool_get(
    mut self_0: *const CaptureListPool,
    mut id: uint16_t,
) -> *const CaptureList {
    if id as libc::c_uint >= (*self_0).list.size {
        return &(*self_0).empty_list;
    }
    return &mut *((*self_0).list.contents).offset(id as isize) as *mut CaptureList;
}
unsafe extern "C" fn capture_list_pool_get_mut(
    mut self_0: *mut CaptureListPool,
    mut id: uint16_t,
) -> *mut CaptureList {
    if (id as libc::c_uint) < (*self_0).list.size {
    } else {
        panic!();
    }
    return &mut *((*self_0).list.contents).offset(id as isize) as *mut CaptureList;
}
unsafe extern "C" fn capture_list_pool_is_empty(mut self_0: *const CaptureListPool) -> bool {
    return (*self_0).free_capture_list_count == 0 as libc::c_int as libc::c_uint
        && (*self_0).list.size >= (*self_0).max_capture_list_count;
}
unsafe extern "C" fn capture_list_pool_acquire(mut self_0: *mut CaptureListPool) -> uint16_t {
    if (*self_0).free_capture_list_count > 0 as libc::c_int as libc::c_uint {
        let mut i: uint16_t = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < (*self_0).list.size as uint16_t as libc::c_int {
            if (*((*self_0).list.contents).offset(i as isize)).size == 4294967295 as libc::c_uint {
                (*((*self_0).list.contents).offset(i as isize)).size = 0 as libc::c_int as uint32_t;
                let ref mut fresh20 = (*self_0).free_capture_list_count;
                *fresh20 = (*fresh20).wrapping_sub(1);
                return i;
            }
            i = i.wrapping_add(1);
        }
    }
    let mut i_0: uint32_t = (*self_0).list.size;
    if i_0 >= (*self_0).max_capture_list_count {
        return NONE;
    }
    let mut list: CaptureList = CaptureList {
        contents: 0 as *mut TSQueryCapture,
        size: 0,
        capacity: 0,
    };
    list.size = 0 as libc::c_int as uint32_t;
    list.capacity = 0 as libc::c_int as uint32_t;
    list.contents = 0 as *mut TSQueryCapture;
    array__grow(
        &mut (*self_0).list as *mut C2RustUnnamed_14 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<CaptureList>() as libc::c_ulong,
    );
    let ref mut fresh21 = (*self_0).list.size;
    let fresh22 = *fresh21;
    *fresh21 = (*fresh21).wrapping_add(1);
    *((*self_0).list.contents).offset(fresh22 as isize) = list;
    return i_0 as uint16_t;
}
unsafe extern "C" fn capture_list_pool_release(mut self_0: *mut CaptureListPool, mut id: uint16_t) {
    if id as libc::c_uint >= (*self_0).list.size {
        return;
    }
    (*((*self_0).list.contents).offset(id as isize)).size = 4294967295 as libc::c_uint;
    let ref mut fresh23 = (*self_0).free_capture_list_count;
    *fresh23 = (*fresh23).wrapping_add(1);
}
unsafe extern "C" fn quantifier_mul(
    mut left: TSQuantifier,
    mut right: TSQuantifier,
) -> TSQuantifier {
    match left as libc::c_uint {
        0 => return TSQuantifierZero,
        1 => match right as libc::c_uint {
            0 => return TSQuantifierZero,
            1 | 3 => return TSQuantifierZeroOrOne,
            2 | 4 => return TSQuantifierZeroOrMore,
            _ => {}
        },
        2 => match right as libc::c_uint {
            0 => return TSQuantifierZero,
            1 | 2 | 3 | 4 => return TSQuantifierZeroOrMore,
            _ => {}
        },
        3 => return right,
        4 => match right as libc::c_uint {
            0 => return TSQuantifierZero,
            1 | 2 => return TSQuantifierZeroOrMore,
            3 | 4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        _ => {}
    }
    return TSQuantifierZero;
}
unsafe extern "C" fn quantifier_join(
    mut left: TSQuantifier,
    mut right: TSQuantifier,
) -> TSQuantifier {
    match left as libc::c_uint {
        0 => match right as libc::c_uint {
            0 => return TSQuantifierZero,
            1 | 3 => return TSQuantifierZeroOrOne,
            2 | 4 => return TSQuantifierZeroOrMore,
            _ => {}
        },
        1 => match right as libc::c_uint {
            0 | 1 | 3 => return TSQuantifierZeroOrOne,
            2 | 4 => return TSQuantifierZeroOrMore,
            _ => {}
        },
        2 => return TSQuantifierZeroOrMore,
        3 => match right as libc::c_uint {
            0 | 1 => return TSQuantifierZeroOrOne,
            2 => return TSQuantifierZeroOrMore,
            3 => return TSQuantifierOne,
            4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        4 => match right as libc::c_uint {
            0 | 1 | 2 => return TSQuantifierZeroOrMore,
            3 | 4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        _ => {}
    }
    return TSQuantifierZero;
}
unsafe extern "C" fn quantifier_add(
    mut left: TSQuantifier,
    mut right: TSQuantifier,
) -> TSQuantifier {
    match left as libc::c_uint {
        0 => return right,
        1 => match right as libc::c_uint {
            0 => return TSQuantifierZeroOrOne,
            1 | 2 => return TSQuantifierZeroOrMore,
            3 | 4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        2 => match right as libc::c_uint {
            0 => return TSQuantifierZeroOrMore,
            1 | 2 => return TSQuantifierZeroOrMore,
            3 | 4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        3 => match right as libc::c_uint {
            0 => return TSQuantifierOne,
            1 | 2 | 3 | 4 => return TSQuantifierOneOrMore,
            _ => {}
        },
        4 => return TSQuantifierOneOrMore,
        _ => {}
    }
    return TSQuantifierZero;
}
unsafe extern "C" fn capture_quantifiers_new() -> CaptureQuantifiers {
    return {
        let mut init = CaptureQuantifiers {
            contents: 0 as *mut uint8_t,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
}
unsafe extern "C" fn capture_quantifiers_delete(mut self_0: *mut CaptureQuantifiers) {
    array__delete(self_0 as *mut VoidArray);
}
unsafe extern "C" fn capture_quantifiers_clear(mut self_0: *mut CaptureQuantifiers) {
    (*self_0).size = 0 as libc::c_int as uint32_t;
}
unsafe extern "C" fn capture_quantifiers_replace(
    mut self_0: *mut CaptureQuantifiers,
    mut quantifiers: *mut CaptureQuantifiers,
) {
    (*self_0).size = 0 as libc::c_int as uint32_t;
    array__splice(
        self_0 as *mut VoidArray,
        ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        (*self_0).size,
        0 as libc::c_int as uint32_t,
        (*quantifiers).size,
        (*quantifiers).contents as *const libc::c_void,
    );
}
unsafe extern "C" fn capture_quantifier_for_id(
    mut self_0: *const CaptureQuantifiers,
    mut id: uint16_t,
) -> TSQuantifier {
    return (if (*self_0).size <= id as libc::c_uint {
        TSQuantifierZero as libc::c_int as libc::c_uint
    } else {
        if (id as uint32_t) < (*self_0).size {
        } else {
            panic!();
        }
        *(&mut *((*self_0).contents).offset(id as isize) as *mut uint8_t) as TSQuantifier
            as libc::c_uint
    }) as TSQuantifier;
}
unsafe extern "C" fn capture_quantifiers_add_for_id(
    mut self_0: *mut CaptureQuantifiers,
    mut id: uint16_t,
    mut quantifier: TSQuantifier,
) {
    if (*self_0).size <= id as libc::c_uint {
        array__grow(
            self_0 as *mut VoidArray,
            ((id as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_sub((*self_0).size),
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        std::ptr::write_bytes(
            ((*self_0).contents).offset((*self_0).size as isize) as *mut libc::c_void,
            (0 as libc::c_int) as u8,
            ((((id as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_sub((*self_0).size)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong))
                as usize,
        );
        let ref mut fresh24 = (*self_0).size;
        *fresh24 = (*fresh24 as libc::c_uint).wrapping_add(
            ((id as libc::c_int + 1 as libc::c_int) as libc::c_uint).wrapping_sub((*self_0).size),
        ) as uint32_t as uint32_t;
    }
    if (id as uint32_t) < (*self_0).size {
    } else {
        panic!();
    }
    let mut own_quantifier: *mut uint8_t =
        &mut *((*self_0).contents).offset(id as isize) as *mut uint8_t;
    *own_quantifier = quantifier_add(*own_quantifier as TSQuantifier, quantifier) as uint8_t;
}
unsafe extern "C" fn capture_quantifiers_add_all(
    mut self_0: *mut CaptureQuantifiers,
    mut quantifiers: *mut CaptureQuantifiers,
) {
    if (*self_0).size < (*quantifiers).size {
        array__grow(
            self_0 as *mut VoidArray,
            ((*quantifiers).size).wrapping_sub((*self_0).size),
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        std::ptr::write_bytes(
            ((*self_0).contents).offset((*self_0).size as isize) as *mut libc::c_void,
            (0 as libc::c_int) as u8,
            ((((*quantifiers).size).wrapping_sub((*self_0).size) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong))
                as usize,
        );
        let ref mut fresh25 = (*self_0).size;
        *fresh25 = (*fresh25 as libc::c_uint)
            .wrapping_add(((*quantifiers).size).wrapping_sub((*self_0).size))
            as uint32_t as uint32_t;
    }
    let mut id: uint16_t = 0 as libc::c_int as uint16_t;
    while (id as libc::c_int) < (*quantifiers).size as uint16_t as libc::c_int {
        if (id as uint32_t) < (*quantifiers).size {
        } else {
            panic!();
        }
        let mut quantifier: *mut uint8_t =
            &mut *((*quantifiers).contents).offset(id as isize) as *mut uint8_t;
        if (id as uint32_t) < (*self_0).size {
        } else {
            panic!();
        }
        let mut own_quantifier: *mut uint8_t =
            &mut *((*self_0).contents).offset(id as isize) as *mut uint8_t;
        *own_quantifier =
            quantifier_add(*own_quantifier as TSQuantifier, *quantifier as TSQuantifier) as uint8_t;
        id = id.wrapping_add(1);
    }
}
unsafe extern "C" fn capture_quantifiers_mul(
    mut self_0: *mut CaptureQuantifiers,
    mut quantifier: TSQuantifier,
) {
    let mut id: uint16_t = 0 as libc::c_int as uint16_t;
    while (id as libc::c_int) < (*self_0).size as uint16_t as libc::c_int {
        if (id as uint32_t) < (*self_0).size {
        } else {
            panic!();
        }
        let mut own_quantifier: *mut uint8_t =
            &mut *((*self_0).contents).offset(id as isize) as *mut uint8_t;
        *own_quantifier = quantifier_mul(*own_quantifier as TSQuantifier, quantifier) as uint8_t;
        id = id.wrapping_add(1);
    }
}
unsafe extern "C" fn capture_quantifiers_join_all(
    mut self_0: *mut CaptureQuantifiers,
    mut quantifiers: *mut CaptureQuantifiers,
) {
    if (*self_0).size < (*quantifiers).size {
        array__grow(
            self_0 as *mut VoidArray,
            ((*quantifiers).size).wrapping_sub((*self_0).size),
            ::std::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        std::ptr::write_bytes(
            ((*self_0).contents).offset((*self_0).size as isize) as *mut libc::c_void,
            (0 as libc::c_int) as u8,
            ((((*quantifiers).size).wrapping_sub((*self_0).size) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint8_t>() as libc::c_ulong))
                as usize,
        );
        let ref mut fresh26 = (*self_0).size;
        *fresh26 = (*fresh26 as libc::c_uint)
            .wrapping_add(((*quantifiers).size).wrapping_sub((*self_0).size))
            as uint32_t as uint32_t;
    }
    let mut id: uint32_t = 0 as libc::c_int as uint32_t;
    while id < (*quantifiers).size {
        if id < (*quantifiers).size {
        } else {
            panic!();
        }
        let mut quantifier: *mut uint8_t =
            &mut *((*quantifiers).contents).offset(id as isize) as *mut uint8_t;
        if id < (*self_0).size {
        } else {
            panic!();
        }
        let mut own_quantifier: *mut uint8_t =
            &mut *((*self_0).contents).offset(id as isize) as *mut uint8_t;
        *own_quantifier =
            quantifier_join(*own_quantifier as TSQuantifier, *quantifier as TSQuantifier)
                as uint8_t;
        id = id.wrapping_add(1);
    }
    let mut id_0: uint32_t = (*quantifiers).size;
    while id_0 < (*self_0).size {
        if id_0 < (*self_0).size {
        } else {
            panic!();
        }
        let mut own_quantifier_0: *mut uint8_t =
            &mut *((*self_0).contents).offset(id_0 as isize) as *mut uint8_t;
        *own_quantifier_0 =
            quantifier_join(*own_quantifier_0 as TSQuantifier, TSQuantifierZero) as uint8_t;
        id_0 = id_0.wrapping_add(1);
    }
}
unsafe extern "C" fn symbol_table_new() -> SymbolTable {
    return {
        let mut init = SymbolTable {
            characters: {
                let mut init = C2RustUnnamed_13 {
                    contents: 0 as *mut libc::c_char,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            slices: {
                let mut init = C2RustUnnamed_12 {
                    contents: 0 as *mut Slice,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
}
unsafe extern "C" fn symbol_table_delete(mut self_0: *mut SymbolTable) {
    array__delete(&mut (*self_0).characters as *mut C2RustUnnamed_13 as *mut VoidArray);
    array__delete(&mut (*self_0).slices as *mut C2RustUnnamed_12 as *mut VoidArray);
}
unsafe extern "C" fn symbol_table_id_for_name(
    mut self_0: *const SymbolTable,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> libc::c_int {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).slices.size {
        let mut slice: Slice = *((*self_0).slices.contents).offset(i as isize);
        if slice.length == length
            && strncmp(
                &mut *((*self_0).characters.contents).offset(slice.offset as isize),
                name,
                length as libc::c_ulong,
            ) == 0
        {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn symbol_table_name_for_id(
    mut self_0: *const SymbolTable,
    mut id: uint16_t,
    mut length: *mut uint32_t,
) -> *const libc::c_char {
    let mut slice: Slice = *((*self_0).slices.contents).offset(id as isize);
    *length = slice.length;
    return &mut *((*self_0).characters.contents).offset(slice.offset as isize)
        as *mut libc::c_char;
}
unsafe extern "C" fn symbol_table_insert_name(
    mut self_0: *mut SymbolTable,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) -> uint16_t {
    let mut id: libc::c_int = symbol_table_id_for_name(self_0, name, length);
    if id >= 0 as libc::c_int {
        return id as uint16_t;
    }
    let mut slice: Slice = {
        let mut init = Slice {
            offset: (*self_0).characters.size,
            length: length,
        };
        init
    };
    array__grow(
        &mut (*self_0).characters as *mut C2RustUnnamed_13 as *mut VoidArray,
        length.wrapping_add(1 as libc::c_int as libc::c_uint),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    );
    std::ptr::write_bytes(
        ((*self_0).characters.contents).offset((*self_0).characters.size as isize)
            as *mut libc::c_void,
        (0 as libc::c_int) as u8,
        ((length.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)) as usize,
    );
    let ref mut fresh27 = (*self_0).characters.size;
    *fresh27 = (*fresh27 as libc::c_uint)
        .wrapping_add(length.wrapping_add(1 as libc::c_int as libc::c_uint))
        as uint32_t as uint32_t;
    std::ptr::copy_nonoverlapping(
        name as *const libc::c_void,
        &mut *((*self_0).characters.contents).offset(slice.offset as isize) as *mut libc::c_char
            as *mut libc::c_void,
        length as libc::c_ulong,
    );
    *((*self_0).characters.contents).offset(
        ((*self_0).characters.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
    ) = 0 as libc::c_int as libc::c_char;
    array__grow(
        &mut (*self_0).slices as *mut C2RustUnnamed_12 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<Slice>() as libc::c_ulong,
    );
    let ref mut fresh28 = (*self_0).slices.size;
    let fresh29 = *fresh28;
    *fresh28 = (*fresh28).wrapping_add(1);
    *((*self_0).slices.contents).offset(fresh29 as isize) = slice;
    return ((*self_0).slices.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as uint16_t;
}
unsafe extern "C" fn query_step__new(
    mut symbol: TSSymbol,
    mut depth: uint16_t,
    mut is_immediate: bool,
) -> QueryStep {
    let mut step: QueryStep = {
        let mut init = QueryStep { is_named_is_immediate_is_last_child_is_pass_through_is_dead_end_alternative_is_immediate_contains_captures_root_pattern_guaranteed_parent_pattern_guaranteed : [0 ; 2] , symbol : symbol , supertype_symbol : 0 , field : 0 as libc :: c_int as TSFieldId , capture_ids : [0 ; 3] , depth : depth , alternative_index : NONE , negated_field_list_id : 0 as libc :: c_int as uint16_t , } ;
        init.set_is_named(0 as libc::c_int != 0);
        init.set_is_immediate(is_immediate);
        init.set_is_last_child(0 as libc::c_int != 0);
        init.set_is_pass_through(0 as libc::c_int != 0);
        init.set_is_dead_end(0 as libc::c_int != 0);
        init.set_alternative_is_immediate(0 as libc::c_int != 0);
        init.set_contains_captures(0 as libc::c_int != 0);
        init.set_root_pattern_guaranteed(0 as libc::c_int != 0);
        init.set_parent_pattern_guaranteed(false);
        init
    };
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        step.capture_ids[i as usize] = NONE;
        i = i.wrapping_add(1);
    }
    return step;
}
unsafe extern "C" fn query_step__add_capture(mut self_0: *mut QueryStep, mut capture_id: uint16_t) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        if (*self_0).capture_ids[i as usize] as libc::c_int == NONE as libc::c_int {
            (*self_0).capture_ids[i as usize] = capture_id;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn query_step__remove_capture(
    mut self_0: *mut QueryStep,
    mut capture_id: uint16_t,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < 3 as libc::c_int as libc::c_uint {
        if (*self_0).capture_ids[i as usize] as libc::c_int == capture_id as libc::c_int {
            (*self_0).capture_ids[i as usize] = NONE;
            while i.wrapping_add(1 as libc::c_int as libc::c_uint)
                < 3 as libc::c_int as libc::c_uint
            {
                if (*self_0).capture_ids[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize]
                    as libc::c_int
                    == NONE as libc::c_int
                {
                    break;
                }
                (*self_0).capture_ids[i as usize] = (*self_0).capture_ids
                    [i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize];
                (*self_0).capture_ids[i.wrapping_add(1 as libc::c_int as libc::c_uint) as usize] =
                    NONE;
                i = i.wrapping_add(1);
            }
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
}
#[inline]
unsafe extern "C" fn state_predecessor_map_new(
    mut language: *const TSLanguage,
) -> StatePredecessorMap {
    return {
        let mut init = StatePredecessorMap {
            contents: crate::core::alloc::ts_calloc(
                ((*language).state_count as size_t)
                    .wrapping_mul((256 as libc::c_int + 1 as libc::c_int) as libc::c_ulong),
                ::std::mem::size_of::<TSStateId>() as libc::c_ulong,
            ) as *mut TSStateId,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn state_predecessor_map_delete(mut self_0: *mut StatePredecessorMap) {
    crate::core::alloc::ts_free((*self_0).contents as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn state_predecessor_map_add(
    mut self_0: *mut StatePredecessorMap,
    mut state: TSStateId,
    mut predecessor: TSStateId,
) {
    let mut index: size_t =
        (state as size_t).wrapping_mul((256 as libc::c_int + 1 as libc::c_int) as libc::c_ulong);
    let mut count: *mut TSStateId =
        &mut *((*self_0).contents).offset(index as isize) as *mut TSStateId;
    if *count as libc::c_int == 0 as libc::c_int
        || (*count as libc::c_int) < 256 as libc::c_int
            && *((*self_0).contents).offset(index.wrapping_add(*count as libc::c_ulong) as isize)
                as libc::c_int
                != predecessor as libc::c_int
    {
        *count = (*count).wrapping_add(1);
        *((*self_0).contents).offset(index.wrapping_add(*count as libc::c_ulong) as isize) =
            predecessor;
    }
}
#[inline]
unsafe extern "C" fn state_predecessor_map_get(
    mut self_0: *const StatePredecessorMap,
    mut state: TSStateId,
    mut count: *mut libc::c_uint,
) -> *const TSStateId {
    let mut index: size_t =
        (state as size_t).wrapping_mul((256 as libc::c_int + 1 as libc::c_int) as libc::c_ulong);
    *count = *((*self_0).contents).offset(index as isize) as libc::c_uint;
    return &mut *((*self_0).contents)
        .offset(index.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
        as *mut TSStateId;
}
unsafe extern "C" fn analysis_state__recursion_depth(
    mut self_0: *const AnalysisState,
) -> libc::c_uint {
    let mut result: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).depth as libc::c_uint {
        let mut symbol: TSSymbol = (*self_0).stack[i as usize].parent_symbol;
        let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while j < i {
            if (*self_0).stack[j as usize].parent_symbol as libc::c_int == symbol as libc::c_int {
                result = result.wrapping_add(1);
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    return result;
}
#[inline]
unsafe extern "C" fn analysis_state__compare_position(
    mut self_0: *const *mut AnalysisState,
    mut other: *const *mut AnalysisState,
) -> libc::c_int {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (**self_0).depth as libc::c_uint {
        if i >= (**other).depth as libc::c_uint {
            return -(1 as libc::c_int);
        }
        if ((**self_0).stack[i as usize].child_index as libc::c_int)
            < (**other).stack[i as usize].child_index as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (**self_0).stack[i as usize].child_index as libc::c_int
            > (**other).stack[i as usize].child_index as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    if ((**self_0).depth as libc::c_int) < (**other).depth as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((**self_0).step_index as libc::c_int) < (**other).step_index as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (**self_0).step_index as libc::c_int > (**other).step_index as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn analysis_state__compare(
    mut self_0: *const *mut AnalysisState,
    mut other: *const *mut AnalysisState,
) -> libc::c_int {
    let mut result: libc::c_int = analysis_state__compare_position(self_0, other);
    if result != 0 as libc::c_int {
        return result;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (**self_0).depth as libc::c_uint {
        if ((**self_0).stack[i as usize].parent_symbol as libc::c_int)
            < (**other).stack[i as usize].parent_symbol as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (**self_0).stack[i as usize].parent_symbol as libc::c_int
            > (**other).stack[i as usize].parent_symbol as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if ((**self_0).stack[i as usize].parse_state as libc::c_int)
            < (**other).stack[i as usize].parse_state as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if (**self_0).stack[i as usize].parse_state as libc::c_int
            > (**other).stack[i as usize].parse_state as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (((**self_0).stack[i as usize]).field_id() as libc::c_int)
            < ((**other).stack[i as usize]).field_id() as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        if ((**self_0).stack[i as usize]).field_id() as libc::c_int
            > ((**other).stack[i as usize]).field_id() as libc::c_int
        {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn analysis_state__top(
    mut self_0: *mut AnalysisState,
) -> *mut AnalysisStateEntry {
    if (*self_0).depth as libc::c_int == 0 as libc::c_int {
        return &mut *((*self_0).stack)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut AnalysisStateEntry;
    }
    return &mut *((*self_0).stack)
        .as_mut_ptr()
        .offset(((*self_0).depth as libc::c_int - 1 as libc::c_int) as isize)
        as *mut AnalysisStateEntry;
}
#[inline]
unsafe extern "C" fn analysis_state__has_supertype(
    mut self_0: *mut AnalysisState,
    mut symbol: TSSymbol,
) -> bool {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).depth as libc::c_uint {
        if (*self_0).stack[i as usize].parent_symbol as libc::c_int == symbol as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn analysis_state_pool__clone_or_reuse(
    mut self_0: *mut AnalysisStateSet,
    mut borrowed_item: *mut AnalysisState,
) -> *mut AnalysisState {
    let mut new_item: *mut AnalysisState = 0 as *mut AnalysisState;
    if (*self_0).size != 0 {
        let ref mut fresh30 = (*self_0).size;
        *fresh30 = (*fresh30).wrapping_sub(1);
        new_item = *((*self_0).contents).offset(*fresh30 as isize);
    } else {
        new_item =
            crate::core::alloc::ts_malloc(::std::mem::size_of::<AnalysisState>() as libc::c_ulong)
                as *mut AnalysisState;
    }
    *new_item = *borrowed_item;
    return new_item;
}
#[inline]
unsafe extern "C" fn analysis_state_set__insert_sorted(
    mut self_0: *mut AnalysisStateSet,
    mut pool: *mut AnalysisStateSet,
    mut borrowed_item: *mut AnalysisState,
) {
    let mut index: libc::c_uint = 0;
    let mut exists: libc::c_uint = 0;
    index = 0 as libc::c_int as libc::c_uint;
    exists = 0 as libc::c_int as libc::c_uint;
    let mut size: uint32_t = ((*self_0).size).wrapping_sub(index);
    if !(size == 0 as libc::c_int as libc::c_uint) {
        let mut comparison: libc::c_int = 0;
        while size > 1 as libc::c_int as libc::c_uint {
            let mut half_size: uint32_t = size.wrapping_div(2 as libc::c_int as libc::c_uint);
            let mut mid_index: uint32_t = index.wrapping_add(half_size);
            comparison = analysis_state__compare(
                &mut *((*self_0).contents).offset(mid_index as isize),
                &mut borrowed_item,
            );
            if comparison <= 0 as libc::c_int {
                index = mid_index;
            }
            size = (size as libc::c_uint).wrapping_sub(half_size) as uint32_t as uint32_t;
        }
        comparison = analysis_state__compare(
            &mut *((*self_0).contents).offset(index as isize),
            &mut borrowed_item,
        );
        if comparison == 0 as libc::c_int {
            exists = 1 as libc::c_int as libc::c_uint;
        } else if comparison < 0 as libc::c_int {
            index = index.wrapping_add(1 as libc::c_int as libc::c_uint);
        }
    }
    if exists == 0 {
        let mut new_item: *mut AnalysisState =
            analysis_state_pool__clone_or_reuse(pool, borrowed_item);
        array__splice(
            self_0 as *mut VoidArray,
            ::std::mem::size_of::<*mut AnalysisState>() as libc::c_ulong,
            index,
            0 as libc::c_int as uint32_t,
            1 as libc::c_int as uint32_t,
            &mut new_item as *mut *mut AnalysisState as *const libc::c_void,
        );
    }
}
#[inline]
unsafe extern "C" fn analysis_state_set__push(
    mut self_0: *mut AnalysisStateSet,
    mut pool: *mut AnalysisStateSet,
    mut borrowed_item: *mut AnalysisState,
) {
    let mut new_item: *mut AnalysisState = analysis_state_pool__clone_or_reuse(pool, borrowed_item);
    array__grow(
        self_0 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<*mut AnalysisState>() as libc::c_ulong,
    );
    let ref mut fresh31 = (*self_0).size;
    let fresh32 = *fresh31;
    *fresh31 = (*fresh31).wrapping_add(1);
    let ref mut fresh33 = *((*self_0).contents).offset(fresh32 as isize);
    *fresh33 = new_item;
}
#[inline]
unsafe extern "C" fn analysis_state_set__clear(
    mut self_0: *mut AnalysisStateSet,
    mut pool: *mut AnalysisStateSet,
) {
    array__splice(
        pool as *mut VoidArray,
        ::std::mem::size_of::<*mut AnalysisState>() as libc::c_ulong,
        (*pool).size,
        0 as libc::c_int as uint32_t,
        (*self_0).size,
        (*self_0).contents as *const libc::c_void,
    );
    (*self_0).size = 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn analysis_state_set__delete(mut self_0: *mut AnalysisStateSet) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).size {
        crate::core::alloc::ts_free(*((*self_0).contents).offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    array__delete(self_0 as *mut VoidArray);
}
#[inline]
unsafe extern "C" fn query_analysis__new() -> QueryAnalysis {
    return {
        let mut init = QueryAnalysis {
            states: {
                let mut init = AnalysisStateSet {
                    contents: 0 as *mut *mut AnalysisState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            next_states: {
                let mut init = AnalysisStateSet {
                    contents: 0 as *mut *mut AnalysisState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            deeper_states: {
                let mut init = AnalysisStateSet {
                    contents: 0 as *mut *mut AnalysisState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            state_pool: {
                let mut init = AnalysisStateSet {
                    contents: 0 as *mut *mut AnalysisState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            final_step_indices: {
                let mut init = C2RustUnnamed_22 {
                    contents: 0 as *mut uint16_t,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            finished_parent_symbols: {
                let mut init = C2RustUnnamed_21 {
                    contents: 0 as *mut TSSymbol,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            did_abort: 0 as libc::c_int != 0,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn query_analysis__delete(mut self_0: *mut QueryAnalysis) {
    analysis_state_set__delete(&mut (*self_0).states);
    analysis_state_set__delete(&mut (*self_0).next_states);
    analysis_state_set__delete(&mut (*self_0).deeper_states);
    analysis_state_set__delete(&mut (*self_0).state_pool);
    array__delete(&mut (*self_0).final_step_indices as *mut C2RustUnnamed_22 as *mut VoidArray);
    array__delete(
        &mut (*self_0).finished_parent_symbols as *mut C2RustUnnamed_21 as *mut VoidArray,
    );
}
#[inline]
unsafe extern "C" fn analysis_subgraph_node__compare(
    mut self_0: *const AnalysisSubgraphNode,
    mut other: *const AnalysisSubgraphNode,
) -> libc::c_int {
    if ((*self_0).state as libc::c_int) < (*other).state as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*self_0).state as libc::c_int > (*other).state as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*self_0).child_index() as libc::c_int) < (*other).child_index() as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*self_0).child_index() as libc::c_int > (*other).child_index() as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*self_0).done() as libc::c_int) < (*other).done() as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*self_0).done() as libc::c_int > (*other).done() as libc::c_int {
        return 1 as libc::c_int;
    }
    if ((*self_0).production_id as libc::c_int) < (*other).production_id as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*self_0).production_id as libc::c_int > (*other).production_id as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_query__pattern_map_search(
    mut self_0: *const TSQuery,
    mut needle: TSSymbol,
    mut result: *mut uint32_t,
) -> bool {
    let mut base_index: uint32_t = (*self_0).wildcard_root_pattern_count as uint32_t;
    let mut size: uint32_t = ((*self_0).pattern_map.size).wrapping_sub(base_index);
    if size == 0 as libc::c_int as libc::c_uint {
        *result = base_index;
        return 0 as libc::c_int != 0;
    }
    while size > 1 as libc::c_int as libc::c_uint {
        let mut half_size: uint32_t = size.wrapping_div(2 as libc::c_int as libc::c_uint);
        let mut mid_index: uint32_t = base_index.wrapping_add(half_size);
        let mut mid_symbol: TSSymbol = (*((*self_0).steps.contents).offset(
            (*((*self_0).pattern_map.contents).offset(mid_index as isize)).step_index as isize,
        ))
        .symbol;
        if needle as libc::c_int > mid_symbol as libc::c_int {
            base_index = mid_index;
        }
        size = (size as libc::c_uint).wrapping_sub(half_size) as uint32_t as uint32_t;
    }
    let mut symbol: TSSymbol = (*((*self_0).steps.contents).offset(
        (*((*self_0).pattern_map.contents).offset(base_index as isize)).step_index as isize,
    ))
    .symbol;
    if needle as libc::c_int > symbol as libc::c_int {
        base_index = base_index.wrapping_add(1);
        if base_index < (*self_0).pattern_map.size {
            symbol = (*((*self_0).steps.contents).offset(
                (*((*self_0).pattern_map.contents).offset(base_index as isize)).step_index as isize,
            ))
            .symbol;
        }
    }
    *result = base_index;
    return needle as libc::c_int == symbol as libc::c_int;
}
#[inline]
unsafe extern "C" fn ts_query__pattern_map_insert(
    mut self_0: *mut TSQuery,
    mut symbol: TSSymbol,
    mut new_entry: PatternEntry,
) {
    let mut index: uint32_t = 0;
    ts_query__pattern_map_search(self_0, symbol, &mut index);
    while index < (*self_0).pattern_map.size {
        let mut entry: *mut PatternEntry =
            &mut *((*self_0).pattern_map.contents).offset(index as isize) as *mut PatternEntry;
        if !((*((*self_0).steps.contents).offset((*entry).step_index as isize)).symbol
            as libc::c_int
            == symbol as libc::c_int
            && ((*entry).pattern_index as libc::c_int) < new_entry.pattern_index as libc::c_int)
        {
            break;
        }
        index = index.wrapping_add(1);
    }
    array__splice(
        &mut (*self_0).pattern_map as *mut C2RustUnnamed_9 as *mut VoidArray,
        ::std::mem::size_of::<PatternEntry>() as libc::c_ulong,
        index,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        &mut new_entry as *mut PatternEntry as *const libc::c_void,
    );
}
unsafe extern "C" fn ts_query__perform_analysis(
    mut self_0: *mut TSQuery,
    mut subgraphs: *const AnalysisSubgraphArray,
    mut analysis: *mut QueryAnalysis,
) {
    let mut recursion_depth_limit: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut prev_final_step_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    (*analysis).final_step_indices.size = 0 as libc::c_int as uint32_t;
    (*analysis).finished_parent_symbols.size = 0 as libc::c_int as uint32_t;
    let mut iteration: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        if iteration == 256 as libc::c_int as libc::c_uint {
            (*analysis).did_abort = 1 as libc::c_int != 0;
            break;
        } else {
            if (*analysis).states.size == 0 as libc::c_int as libc::c_uint {
                if !((*analysis).deeper_states.size > 0 as libc::c_int as libc::c_uint
                    && (*analysis).final_step_indices.size > prev_final_step_count)
                {
                    break;
                }
                prev_final_step_count = (*analysis).final_step_indices.size;
                recursion_depth_limit = recursion_depth_limit.wrapping_add(1);
                let mut _states: AnalysisStateSet = (*analysis).states;
                (*analysis).states = (*analysis).deeper_states;
                (*analysis).deeper_states = _states;
            } else {
                analysis_state_set__clear(
                    &mut (*analysis).next_states,
                    &mut (*analysis).state_pool,
                );
                let mut current_block_136: u64;
                let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while j < (*analysis).states.size {
                    let state: *mut AnalysisState =
                        *((*analysis).states.contents).offset(j as isize);
                    if (*analysis).next_states.size > 0 as libc::c_int as libc::c_uint {
                        if ((*analysis).next_states.size)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            < (*analysis).next_states.size
                        {
                        } else {
                            panic!();
                        }
                        let mut comparison: libc::c_int = analysis_state__compare_position(
                            &state,
                            &mut *((*analysis).next_states.contents).offset(
                                ((*analysis).next_states.size)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                    as isize,
                            ),
                        );
                        if comparison == 0 as libc::c_int {
                            analysis_state_set__insert_sorted(
                                &mut (*analysis).next_states,
                                &mut (*analysis).state_pool,
                                state,
                            );
                            current_block_136 = 8831408221741692167;
                        } else if comparison > 0 as libc::c_int {
                            while j < (*analysis).states.size {
                                analysis_state_set__push(
                                    &mut (*analysis).next_states,
                                    &mut (*analysis).state_pool,
                                    *((*analysis).states.contents).offset(j as isize),
                                );
                                j = j.wrapping_add(1);
                            }
                            break;
                        } else {
                            current_block_136 = 15345278821338558188;
                        }
                    } else {
                        current_block_136 = 15345278821338558188;
                    }
                    match current_block_136 {
                        15345278821338558188 => {
                            let parse_state: TSStateId = (*analysis_state__top(state)).parse_state;
                            let parent_symbol: TSSymbol =
                                (*analysis_state__top(state)).parent_symbol;
                            let parent_field_id: TSFieldId =
                                (*analysis_state__top(state)).field_id();
                            let child_index: libc::c_uint =
                                (*analysis_state__top(state)).child_index as libc::c_uint;
                            let step: *const QueryStep = &mut *((*self_0).steps.contents)
                                .offset((*state).step_index as isize)
                                as *mut QueryStep;
                            let mut subgraph_index: libc::c_uint = 0;
                            let mut exists: libc::c_uint = 0;
                            subgraph_index = 0 as libc::c_int as libc::c_uint;
                            exists = 0 as libc::c_int as libc::c_uint;
                            let mut size: uint32_t =
                                ((*subgraphs).size).wrapping_sub(subgraph_index);
                            if !(size == 0 as libc::c_int as libc::c_uint) {
                                let mut comparison_0: libc::c_int = 0;
                                while size > 1 as libc::c_int as libc::c_uint {
                                    let mut half_size: uint32_t =
                                        size.wrapping_div(2 as libc::c_int as libc::c_uint);
                                    let mut mid_index: uint32_t =
                                        subgraph_index.wrapping_add(half_size);
                                    comparison_0 = (*((*subgraphs).contents)
                                        .offset(mid_index as isize))
                                    .symbol
                                        as libc::c_int
                                        - parent_symbol as libc::c_int;
                                    if comparison_0 <= 0 as libc::c_int {
                                        subgraph_index = mid_index;
                                    }
                                    size = (size as libc::c_uint).wrapping_sub(half_size)
                                        as uint32_t
                                        as uint32_t;
                                }
                                comparison_0 =
                                    (*((*subgraphs).contents).offset(subgraph_index as isize))
                                        .symbol as libc::c_int
                                        - parent_symbol as libc::c_int;
                                if comparison_0 == 0 as libc::c_int {
                                    exists = 1 as libc::c_int as libc::c_uint;
                                } else if comparison_0 < 0 as libc::c_int {
                                    subgraph_index = subgraph_index
                                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                                }
                            }
                            if !(exists == 0) {
                                let mut subgraph: *const AnalysisSubgraph =
                                    &mut *((*subgraphs).contents).offset(subgraph_index as isize)
                                        as *mut AnalysisSubgraph;
                                let mut lookahead_iterator: LookaheadIterator =
                                    ts_language_lookaheads((*self_0).language, parse_state);
                                while ts_lookahead_iterator__next(&mut lookahead_iterator) {
                                    let mut sym: TSSymbol = lookahead_iterator.symbol;
                                    let mut successor: AnalysisSubgraphNode = {
                                        let mut init = AnalysisSubgraphNode {
                                            child_index_done: [0; 1],
                                            c2rust_padding: [0; 1],
                                            state: parse_state,
                                            production_id: 0,
                                        };
                                        init.set_child_index(child_index as uint8_t);
                                        init.set_done(false);
                                        init
                                    };
                                    if lookahead_iterator.action_count != 0 {
                                        let mut action: *const TSParseAction =
                                            &*(lookahead_iterator.actions).offset(
                                                (lookahead_iterator.action_count as libc::c_int
                                                    - 1 as libc::c_int)
                                                    as isize,
                                            )
                                                as *const TSParseAction;
                                        if !((*action).type_ as libc::c_int
                                            == TSParseActionTypeShift as libc::c_int)
                                        {
                                            continue;
                                        }
                                        if !(*action).shift.extra {
                                            successor.state = (*action).shift.state;
                                            successor.set_child_index(successor.child_index() + 1);
                                        }
                                    } else {
                                        if !(lookahead_iterator.next_state as libc::c_int
                                            != 0 as libc::c_int)
                                        {
                                            continue;
                                        }
                                        successor.state = lookahead_iterator.next_state;
                                        successor.set_child_index(successor.child_index() + 1);
                                    }
                                    let mut node_index: libc::c_uint = 0;
                                    node_index = 0 as libc::c_int as libc::c_uint;
                                    exists = 0 as libc::c_int as libc::c_uint;
                                    let mut size_0: uint32_t =
                                        ((*subgraph).nodes.size).wrapping_sub(node_index);
                                    if !(size_0 == 0 as libc::c_int as libc::c_uint) {
                                        let mut comparison_1: libc::c_int = 0;
                                        while size_0 > 1 as libc::c_int as libc::c_uint {
                                            let mut half_size_0: uint32_t = size_0
                                                .wrapping_div(2 as libc::c_int as libc::c_uint);
                                            let mut mid_index_0: uint32_t =
                                                node_index.wrapping_add(half_size_0);
                                            comparison_1 = analysis_subgraph_node__compare(
                                                &mut *((*subgraph).nodes.contents)
                                                    .offset(mid_index_0 as isize),
                                                &mut successor,
                                            );
                                            if comparison_1 <= 0 as libc::c_int {
                                                node_index = mid_index_0;
                                            }
                                            size_0 = (size_0 as libc::c_uint)
                                                .wrapping_sub(half_size_0)
                                                as uint32_t
                                                as uint32_t;
                                        }
                                        comparison_1 = analysis_subgraph_node__compare(
                                            &mut *((*subgraph).nodes.contents)
                                                .offset(node_index as isize),
                                            &mut successor,
                                        );
                                        if comparison_1 == 0 as libc::c_int {
                                            exists = 1 as libc::c_int as libc::c_uint;
                                        } else if comparison_1 < 0 as libc::c_int {
                                            node_index = node_index
                                                .wrapping_add(1 as libc::c_int as libc::c_uint);
                                        }
                                    }
                                    while node_index < (*subgraph).nodes.size {
                                        let fresh34 = node_index;
                                        node_index = node_index.wrapping_add(1);
                                        let mut node: *mut AnalysisSubgraphNode =
                                            &mut *((*subgraph).nodes.contents)
                                                .offset(fresh34 as isize)
                                                as *mut AnalysisSubgraphNode;
                                        if (*node).state as libc::c_int
                                            != successor.state as libc::c_int
                                            || (*node).child_index() as libc::c_int
                                                != successor.child_index() as libc::c_int
                                        {
                                            break;
                                        }
                                        let mut alias: TSSymbol = ts_language_alias_at(
                                            (*self_0).language,
                                            (*node).production_id as uint32_t,
                                            child_index,
                                        );
                                        let mut visible_symbol: TSSymbol =
                                            (if alias as libc::c_int != 0 {
                                                alias as libc::c_int
                                            } else if (*((*(*self_0).language).symbol_metadata)
                                                .offset(sym as isize))
                                            .visible
                                                as libc::c_int
                                                != 0
                                            {
                                                *((*(*self_0).language).public_symbol_map)
                                                    .offset(sym as isize)
                                                    as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            })
                                                as TSSymbol;
                                        let mut field_id: TSFieldId = parent_field_id;
                                        if field_id == 0 {
                                            let mut field_map: *const TSFieldMapEntry =
                                                0 as *const TSFieldMapEntry;
                                            let mut field_map_end: *const TSFieldMapEntry =
                                                0 as *const TSFieldMapEntry;
                                            ts_language_field_map(
                                                (*self_0).language,
                                                (*node).production_id as uint32_t,
                                                &mut field_map,
                                                &mut field_map_end,
                                            );
                                            while field_map != field_map_end {
                                                if !(*field_map).inherited
                                                    && (*field_map).child_index as libc::c_uint
                                                        == child_index
                                                {
                                                    field_id = (*field_map).field_id;
                                                    break;
                                                } else {
                                                    field_map = field_map.offset(1);
                                                }
                                            }
                                        }
                                        let mut next_state: AnalysisState = *state;
                                        let mut next_state_top: *mut AnalysisStateEntry =
                                            analysis_state__top(&mut next_state);
                                        (*next_state_top).child_index =
                                            successor.child_index() as uint16_t;
                                        (*next_state_top).parse_state = successor.state;
                                        if (*node).done() {
                                            (*next_state_top).set_done(1 as libc::c_int != 0);
                                        }
                                        let mut does_match: bool = 0 as libc::c_int != 0;
                                        if visible_symbol != 0 {
                                            does_match = 1 as libc::c_int != 0;
                                            if (*step).symbol as libc::c_int
                                                == WILDCARD_SYMBOL as libc::c_int
                                            {
                                                if (*step).is_named() as libc::c_int != 0
                                                    && !(*((*(*self_0).language).symbol_metadata)
                                                        .offset(visible_symbol as isize))
                                                    .named
                                                {
                                                    does_match = 0 as libc::c_int != 0;
                                                }
                                            } else if (*step).symbol as libc::c_int
                                                != visible_symbol as libc::c_int
                                            {
                                                does_match = 0 as libc::c_int != 0;
                                            }
                                            if (*step).field as libc::c_int != 0
                                                && (*step).field as libc::c_int
                                                    != field_id as libc::c_int
                                            {
                                                does_match = 0 as libc::c_int != 0;
                                            }
                                            if (*step).supertype_symbol as libc::c_int != 0
                                                && !analysis_state__has_supertype(
                                                    state,
                                                    (*step).supertype_symbol,
                                                )
                                            {
                                                does_match = 0 as libc::c_int != 0;
                                            }
                                        } else if sym as libc::c_uint
                                            >= (*(*self_0).language).token_count
                                        {
                                            if !(*next_state_top).done() {
                                                if next_state.depth as libc::c_int
                                                    + 1 as libc::c_int
                                                    >= 8 as libc::c_int
                                                {
                                                    (*analysis).did_abort = 1 as libc::c_int != 0;
                                                    continue;
                                                } else {
                                                    next_state.depth =
                                                        (next_state.depth).wrapping_add(1);
                                                    next_state_top =
                                                        analysis_state__top(&mut next_state);
                                                }
                                            }
                                            *next_state_top = {
                                                let mut init = AnalysisStateEntry {
                                                    field_id_done: [0; 2],
                                                    parse_state: parse_state,
                                                    parent_symbol: sym,
                                                    child_index: 0 as libc::c_int as uint16_t,
                                                };
                                                init.set_field_id(field_id);
                                                init.set_done(0 as libc::c_int != 0);
                                                init
                                            };
                                            if analysis_state__recursion_depth(&mut next_state)
                                                > recursion_depth_limit
                                            {
                                                analysis_state_set__insert_sorted(
                                                    &mut (*analysis).deeper_states,
                                                    &mut (*analysis).state_pool,
                                                    &mut next_state,
                                                );
                                                continue;
                                            }
                                        }
                                        while next_state.depth as libc::c_int > 0 as libc::c_int
                                            && (*next_state_top).done() as libc::c_int != 0
                                        {
                                            next_state.depth = (next_state.depth).wrapping_sub(1);
                                            next_state_top = analysis_state__top(&mut next_state);
                                        }
                                        let mut next_step: *const QueryStep = step;
                                        if does_match {
                                            loop {
                                                next_state.step_index =
                                                    (next_state.step_index).wrapping_add(1);
                                                next_step = &mut *((*self_0).steps.contents)
                                                    .offset(next_state.step_index as isize)
                                                    as *mut QueryStep;
                                                if (*next_step).depth as libc::c_int
                                                    == PATTERN_DONE_MARKER as libc::c_int
                                                    || (*next_step).depth as libc::c_int
                                                        <= (*step).depth as libc::c_int
                                                {
                                                    break;
                                                }
                                            }
                                        } else if successor.state as libc::c_int
                                            == parse_state as libc::c_int
                                        {
                                            continue;
                                        }
                                        loop {
                                            if (*next_step).is_pass_through() {
                                                next_state.step_index =
                                                    (next_state.step_index).wrapping_add(1);
                                                next_step = next_step.offset(1);
                                            } else {
                                                if !(*next_step).is_dead_end() {
                                                    let mut did_finish_pattern: bool =
                                                        (*((*self_0).steps.contents)
                                                            .offset(next_state.step_index as isize))
                                                        .depth
                                                            as libc::c_int
                                                            != (*step).depth as libc::c_int;
                                                    if did_finish_pattern {
                                                        let mut _index: libc::c_uint = 0;
                                                        let mut _exists: libc::c_uint = 0;
                                                        _index = 0 as libc::c_int as libc::c_uint;
                                                        _exists = 0 as libc::c_int as libc::c_uint;
                                                        let mut size_1: uint32_t = ((*analysis)
                                                            .finished_parent_symbols
                                                            .size)
                                                            .wrapping_sub(_index);
                                                        if !(size_1
                                                            == 0 as libc::c_int as libc::c_uint)
                                                        {
                                                            let mut comparison_2: libc::c_int = 0;
                                                            while size_1
                                                                > 1 as libc::c_int as libc::c_uint
                                                            {
                                                                let mut half_size_1: uint32_t =
                                                                    size_1.wrapping_div(
                                                                        2 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                                let mut mid_index_1: uint32_t =
                                                                    _index
                                                                        .wrapping_add(half_size_1);
                                                                comparison_2 = *((*analysis)
                                                                    .finished_parent_symbols
                                                                    .contents)
                                                                    .offset(mid_index_1 as isize)
                                                                    as libc::c_int
                                                                    - (*state).root_symbol
                                                                        as libc::c_int;
                                                                if comparison_2 <= 0 as libc::c_int
                                                                {
                                                                    _index = mid_index_1;
                                                                }
                                                                size_1 = (size_1 as libc::c_uint)
                                                                    .wrapping_sub(half_size_1)
                                                                    as uint32_t
                                                                    as uint32_t;
                                                            }
                                                            comparison_2 = *((*analysis)
                                                                .finished_parent_symbols
                                                                .contents)
                                                                .offset(_index as isize)
                                                                as libc::c_int
                                                                - (*state).root_symbol
                                                                    as libc::c_int;
                                                            if comparison_2 == 0 as libc::c_int {
                                                                _exists = 1 as libc::c_int
                                                                    as libc::c_uint;
                                                            } else if comparison_2
                                                                < 0 as libc::c_int
                                                            {
                                                                _index = _index.wrapping_add(
                                                                    1 as libc::c_int
                                                                        as libc::c_uint,
                                                                );
                                                            }
                                                        }
                                                        if _exists == 0 {
                                                            array__splice(
                                                                &mut (*analysis)
                                                                    .finished_parent_symbols
                                                                    as *mut C2RustUnnamed_21
                                                                    as *mut VoidArray,
                                                                ::std::mem::size_of::<TSSymbol>()
                                                                    as libc::c_ulong,
                                                                _index,
                                                                0 as libc::c_int as uint32_t,
                                                                1 as libc::c_int as uint32_t,
                                                                &mut (*state).root_symbol
                                                                    as *mut TSSymbol
                                                                    as *const libc::c_void,
                                                            );
                                                        }
                                                    } else if next_state.depth as libc::c_int
                                                        == 0 as libc::c_int
                                                    {
                                                        let mut _index_0: libc::c_uint = 0;
                                                        let mut _exists_0: libc::c_uint = 0;
                                                        _index_0 = 0 as libc::c_int as libc::c_uint;
                                                        _exists_0 =
                                                            0 as libc::c_int as libc::c_uint;
                                                        let mut size_2: uint32_t =
                                                            ((*analysis).final_step_indices.size)
                                                                .wrapping_sub(_index_0);
                                                        if !(size_2
                                                            == 0 as libc::c_int as libc::c_uint)
                                                        {
                                                            let mut comparison_3: libc::c_int = 0;
                                                            while size_2
                                                                > 1 as libc::c_int as libc::c_uint
                                                            {
                                                                let mut half_size_2: uint32_t =
                                                                    size_2.wrapping_div(
                                                                        2 as libc::c_int
                                                                            as libc::c_uint,
                                                                    );
                                                                let mut mid_index_2: uint32_t =
                                                                    _index_0
                                                                        .wrapping_add(half_size_2);
                                                                comparison_3 = *((*analysis)
                                                                    .final_step_indices
                                                                    .contents)
                                                                    .offset(mid_index_2 as isize)
                                                                    as libc::c_int
                                                                    - next_state.step_index
                                                                        as libc::c_int;
                                                                if comparison_3 <= 0 as libc::c_int
                                                                {
                                                                    _index_0 = mid_index_2;
                                                                }
                                                                size_2 = (size_2 as libc::c_uint)
                                                                    .wrapping_sub(half_size_2)
                                                                    as uint32_t
                                                                    as uint32_t;
                                                            }
                                                            comparison_3 = *((*analysis)
                                                                .final_step_indices
                                                                .contents)
                                                                .offset(_index_0 as isize)
                                                                as libc::c_int
                                                                - next_state.step_index
                                                                    as libc::c_int;
                                                            if comparison_3 == 0 as libc::c_int {
                                                                _exists_0 = 1 as libc::c_int
                                                                    as libc::c_uint;
                                                            } else if comparison_3
                                                                < 0 as libc::c_int
                                                            {
                                                                _index_0 = _index_0.wrapping_add(
                                                                    1 as libc::c_int
                                                                        as libc::c_uint,
                                                                );
                                                            }
                                                        }
                                                        if _exists_0 == 0 {
                                                            array__splice(
                                                                &mut (*analysis).final_step_indices
                                                                    as *mut C2RustUnnamed_22
                                                                    as *mut VoidArray,
                                                                ::std::mem::size_of::<uint16_t>()
                                                                    as libc::c_ulong,
                                                                _index_0,
                                                                0 as libc::c_int as uint32_t,
                                                                1 as libc::c_int as uint32_t,
                                                                &mut next_state.step_index
                                                                    as *mut uint16_t
                                                                    as *const libc::c_void,
                                                            );
                                                        }
                                                    } else {
                                                        analysis_state_set__insert_sorted(
                                                            &mut (*analysis).next_states,
                                                            &mut (*analysis).state_pool,
                                                            &mut next_state,
                                                        );
                                                    }
                                                }
                                                if !(does_match as libc::c_int != 0
                                                    && (*next_step).alternative_index
                                                        as libc::c_int
                                                        != NONE as libc::c_int
                                                    && (*next_step).alternative_index
                                                        as libc::c_int
                                                        > next_state.step_index as libc::c_int)
                                                {
                                                    break;
                                                }
                                                next_state.step_index =
                                                    (*next_step).alternative_index;
                                                next_step = &mut *((*self_0).steps.contents)
                                                    .offset(next_state.step_index as isize)
                                                    as *mut QueryStep;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    j = j.wrapping_add(1);
                }
                let mut _states_0: AnalysisStateSet = (*analysis).states;
                (*analysis).states = (*analysis).next_states;
                (*analysis).next_states = _states_0;
            }
            iteration = iteration.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn ts_query__analyze_patterns(
    mut self_0: *mut TSQuery,
    mut error_offset: *mut libc::c_uint,
) -> bool {
    let mut non_rooted_pattern_start_steps: C2RustUnnamed_19 = {
        let mut init = C2RustUnnamed_19 {
            contents: 0 as *mut uint16_t,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).pattern_map.size {
        let mut pattern: *mut PatternEntry =
            &mut *((*self_0).pattern_map.contents).offset(i as isize) as *mut PatternEntry;
        if !(*pattern).is_rooted {
            let mut step: *mut QueryStep = &mut *((*self_0).steps.contents)
                .offset((*pattern).step_index as isize)
                as *mut QueryStep;
            if (*step).symbol as libc::c_int != WILDCARD_SYMBOL as libc::c_int {
                array__grow(
                    &mut non_rooted_pattern_start_steps as *mut C2RustUnnamed_19 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                );
                let fresh35 = non_rooted_pattern_start_steps.size;
                non_rooted_pattern_start_steps.size =
                    (non_rooted_pattern_start_steps.size).wrapping_add(1);
                *(non_rooted_pattern_start_steps.contents).offset(fresh35 as isize) = i as uint16_t;
            }
        }
        i = i.wrapping_add(1);
    }
    let mut parent_step_indices: C2RustUnnamed_18 = {
        let mut init = C2RustUnnamed_18 {
            contents: 0 as *mut uint32_t,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_0 < (*self_0).steps.size {
        let mut step_0: *mut QueryStep =
            &mut *((*self_0).steps.contents).offset(i_0 as isize) as *mut QueryStep;
        if (*step_0).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int {
            (*step_0).set_parent_pattern_guaranteed(1 as libc::c_int != 0);
            (*step_0).set_root_pattern_guaranteed(1 as libc::c_int != 0);
        } else {
            let mut has_children: bool = 0 as libc::c_int != 0;
            let mut is_wildcard: bool =
                (*step_0).symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int;
            (*step_0).set_contains_captures(
                (*step_0).capture_ids[0 as libc::c_int as usize] as libc::c_int
                    != NONE as libc::c_int,
            );
            let mut j: libc::c_uint = i_0.wrapping_add(1 as libc::c_int as libc::c_uint);
            while j < (*self_0).steps.size {
                let mut next_step: *mut QueryStep =
                    &mut *((*self_0).steps.contents).offset(j as isize) as *mut QueryStep;
                if (*next_step).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int
                    || (*next_step).depth as libc::c_int <= (*step_0).depth as libc::c_int
                {
                    break;
                }
                if (*next_step).capture_ids[0 as libc::c_int as usize] as libc::c_int
                    != NONE as libc::c_int
                {
                    (*step_0).set_contains_captures(1 as libc::c_int != 0);
                }
                if !is_wildcard {
                    (*next_step).set_root_pattern_guaranteed(1 as libc::c_int != 0);
                    (*next_step).set_parent_pattern_guaranteed(1 as libc::c_int != 0);
                }
                has_children = 1 as libc::c_int != 0;
                j = j.wrapping_add(1);
            }
            if has_children as libc::c_int != 0 && !is_wildcard {
                array__grow(
                    &mut parent_step_indices as *mut C2RustUnnamed_18 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
                );
                let fresh36 = parent_step_indices.size;
                parent_step_indices.size = (parent_step_indices.size).wrapping_add(1);
                *(parent_step_indices.contents).offset(fresh36 as isize) = i_0;
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut subgraphs: AnalysisSubgraphArray = {
        let mut init = AnalysisSubgraphArray {
            contents: 0 as *mut AnalysisSubgraph,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut i_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_1 < parent_step_indices.size {
        let mut parent_step_index: uint32_t = *(parent_step_indices.contents).offset(i_1 as isize);
        let mut parent_symbol: TSSymbol =
            (*((*self_0).steps.contents).offset(parent_step_index as isize)).symbol;
        let mut subgraph: AnalysisSubgraph = {
            let mut init = AnalysisSubgraph {
                symbol: parent_symbol,
                start_states: C2RustUnnamed_24 {
                    contents: 0 as *mut TSStateId,
                    size: 0,
                    capacity: 0,
                },
                nodes: C2RustUnnamed_23 {
                    contents: 0 as *mut AnalysisSubgraphNode,
                    size: 0,
                    capacity: 0,
                },
            };
            init
        };
        let mut _index: libc::c_uint = 0;
        let mut _exists: libc::c_uint = 0;
        _index = 0 as libc::c_int as libc::c_uint;
        _exists = 0 as libc::c_int as libc::c_uint;
        let mut size: uint32_t = (subgraphs.size).wrapping_sub(_index);
        if !(size == 0 as libc::c_int as libc::c_uint) {
            let mut comparison: libc::c_int = 0;
            while size > 1 as libc::c_int as libc::c_uint {
                let mut half_size: uint32_t = size.wrapping_div(2 as libc::c_int as libc::c_uint);
                let mut mid_index: uint32_t = _index.wrapping_add(half_size);
                comparison = (*(subgraphs.contents).offset(mid_index as isize)).symbol
                    as libc::c_int
                    - subgraph.symbol as libc::c_int;
                if comparison <= 0 as libc::c_int {
                    _index = mid_index;
                }
                size = (size as libc::c_uint).wrapping_sub(half_size) as uint32_t as uint32_t;
            }
            comparison = (*(subgraphs.contents).offset(_index as isize)).symbol as libc::c_int
                - subgraph.symbol as libc::c_int;
            if comparison == 0 as libc::c_int {
                _exists = 1 as libc::c_int as libc::c_uint;
            } else if comparison < 0 as libc::c_int {
                _index = _index.wrapping_add(1 as libc::c_int as libc::c_uint);
            }
        }
        if _exists == 0 {
            array__splice(
                &mut subgraphs as *mut AnalysisSubgraphArray as *mut VoidArray,
                ::std::mem::size_of::<AnalysisSubgraph>() as libc::c_ulong,
                _index,
                0 as libc::c_int as uint32_t,
                1 as libc::c_int as uint32_t,
                &mut subgraph as *mut AnalysisSubgraph as *const libc::c_void,
            );
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut sym: TSSymbol = (*(*self_0).language).token_count as uint16_t;
    while (sym as libc::c_int) < (*(*self_0).language).symbol_count as uint16_t as libc::c_int {
        if !(ts_language_symbol_metadata((*self_0).language, sym)).visible {
            let mut subgraph_0: AnalysisSubgraph = {
                let mut init = AnalysisSubgraph {
                    symbol: sym,
                    start_states: C2RustUnnamed_24 {
                        contents: 0 as *mut TSStateId,
                        size: 0,
                        capacity: 0,
                    },
                    nodes: C2RustUnnamed_23 {
                        contents: 0 as *mut AnalysisSubgraphNode,
                        size: 0,
                        capacity: 0,
                    },
                };
                init
            };
            let mut _index_0: libc::c_uint = 0;
            let mut _exists_0: libc::c_uint = 0;
            _index_0 = 0 as libc::c_int as libc::c_uint;
            _exists_0 = 0 as libc::c_int as libc::c_uint;
            let mut size_0: uint32_t = (subgraphs.size).wrapping_sub(_index_0);
            if !(size_0 == 0 as libc::c_int as libc::c_uint) {
                let mut comparison_0: libc::c_int = 0;
                while size_0 > 1 as libc::c_int as libc::c_uint {
                    let mut half_size_0: uint32_t =
                        size_0.wrapping_div(2 as libc::c_int as libc::c_uint);
                    let mut mid_index_0: uint32_t = _index_0.wrapping_add(half_size_0);
                    comparison_0 = (*(subgraphs.contents).offset(mid_index_0 as isize)).symbol
                        as libc::c_int
                        - subgraph_0.symbol as libc::c_int;
                    if comparison_0 <= 0 as libc::c_int {
                        _index_0 = mid_index_0;
                    }
                    size_0 =
                        (size_0 as libc::c_uint).wrapping_sub(half_size_0) as uint32_t as uint32_t;
                }
                comparison_0 = (*(subgraphs.contents).offset(_index_0 as isize)).symbol
                    as libc::c_int
                    - subgraph_0.symbol as libc::c_int;
                if comparison_0 == 0 as libc::c_int {
                    _exists_0 = 1 as libc::c_int as libc::c_uint;
                } else if comparison_0 < 0 as libc::c_int {
                    _index_0 = _index_0.wrapping_add(1 as libc::c_int as libc::c_uint);
                }
            }
            if _exists_0 == 0 {
                array__splice(
                    &mut subgraphs as *mut AnalysisSubgraphArray as *mut VoidArray,
                    ::std::mem::size_of::<AnalysisSubgraph>() as libc::c_ulong,
                    _index_0,
                    0 as libc::c_int as uint32_t,
                    1 as libc::c_int as uint32_t,
                    &mut subgraph_0 as *mut AnalysisSubgraph as *const libc::c_void,
                );
            }
        }
        sym = sym.wrapping_add(1);
    }
    let mut predecessor_map: StatePredecessorMap = state_predecessor_map_new((*self_0).language);
    let mut state: TSStateId = 1 as libc::c_int as TSStateId;
    while (state as libc::c_int) < (*(*self_0).language).state_count as uint16_t as libc::c_int {
        let mut subgraph_index: libc::c_uint = 0;
        let mut exists: libc::c_uint = 0;
        let mut lookahead_iterator: LookaheadIterator =
            ts_language_lookaheads((*self_0).language, state);
        while ts_lookahead_iterator__next(&mut lookahead_iterator) {
            if lookahead_iterator.action_count != 0 {
                let mut i_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while i_2 < lookahead_iterator.action_count as libc::c_uint {
                    let mut action: *const TSParseAction =
                        &*(lookahead_iterator.actions).offset(i_2 as isize) as *const TSParseAction;
                    if (*action).type_ as libc::c_int == TSParseActionTypeReduce as libc::c_int {
                        let mut aliases: *const TSSymbol = 0 as *const TSSymbol;
                        let mut aliases_end: *const TSSymbol = 0 as *const TSSymbol;
                        ts_language_aliases_for_symbol(
                            (*self_0).language,
                            (*action).reduce.symbol,
                            &mut aliases,
                            &mut aliases_end,
                        );
                        let mut symbol: *const TSSymbol = aliases;
                        while symbol < aliases_end {
                            subgraph_index = 0 as libc::c_int as libc::c_uint;
                            exists = 0 as libc::c_int as libc::c_uint;
                            let mut size_1: uint32_t =
                                (subgraphs.size).wrapping_sub(subgraph_index);
                            if !(size_1 == 0 as libc::c_int as libc::c_uint) {
                                let mut comparison_1: libc::c_int = 0;
                                while size_1 > 1 as libc::c_int as libc::c_uint {
                                    let mut half_size_1: uint32_t =
                                        size_1.wrapping_div(2 as libc::c_int as libc::c_uint);
                                    let mut mid_index_1: uint32_t =
                                        subgraph_index.wrapping_add(half_size_1);
                                    comparison_1 =
                                        (*(subgraphs.contents).offset(mid_index_1 as isize)).symbol
                                            as libc::c_int
                                            - *symbol as libc::c_int;
                                    if comparison_1 <= 0 as libc::c_int {
                                        subgraph_index = mid_index_1;
                                    }
                                    size_1 = (size_1 as libc::c_uint).wrapping_sub(half_size_1)
                                        as uint32_t
                                        as uint32_t;
                                }
                                comparison_1 =
                                    (*(subgraphs.contents).offset(subgraph_index as isize)).symbol
                                        as libc::c_int
                                        - *symbol as libc::c_int;
                                if comparison_1 == 0 as libc::c_int {
                                    exists = 1 as libc::c_int as libc::c_uint;
                                } else if comparison_1 < 0 as libc::c_int {
                                    subgraph_index = subgraph_index
                                        .wrapping_add(1 as libc::c_int as libc::c_uint);
                                }
                            }
                            if exists != 0 {
                                let mut subgraph_1: *mut AnalysisSubgraph =
                                    &mut *(subgraphs.contents).offset(subgraph_index as isize)
                                        as *mut AnalysisSubgraph;
                                if (*subgraph_1).nodes.size == 0 as libc::c_int as libc::c_uint || {
                                    if ((*subgraph_1).nodes.size)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        < (*subgraph_1).nodes.size
                                    {
                                    } else {
                                        panic!();
                                    }
                                    (*(&mut *((*subgraph_1).nodes.contents).offset(
                                        ((*subgraph_1).nodes.size)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    )
                                        as *mut AnalysisSubgraphNode))
                                        .state as libc::c_int
                                        != state as libc::c_int
                                } {
                                    array__grow(
                                        &mut (*subgraph_1).nodes as *mut C2RustUnnamed_23
                                            as *mut VoidArray,
                                        1 as libc::c_int as uint32_t,
                                        ::std::mem::size_of::<AnalysisSubgraphNode>()
                                            as libc::c_ulong,
                                    );
                                    let ref mut fresh37 = (*subgraph_1).nodes.size;
                                    let fresh38 = *fresh37;
                                    *fresh37 = (*fresh37).wrapping_add(1);
                                    *((*subgraph_1).nodes.contents).offset(fresh38 as isize) = {
                                        let mut init = AnalysisSubgraphNode {
                                            child_index_done: [0; 1],
                                            c2rust_padding: [0; 1],
                                            state: state,
                                            production_id: (*action).reduce.production_id,
                                        };
                                        init.set_child_index((*action).reduce.child_count);
                                        init.set_done(1 as libc::c_int != 0);
                                        init
                                    };
                                }
                            }
                            symbol = symbol.offset(1);
                        }
                    } else if (*action).type_ as libc::c_int
                        == TSParseActionTypeShift as libc::c_int
                        && !(*action).shift.extra
                    {
                        let mut next_state: TSStateId = (*action).shift.state;
                        state_predecessor_map_add(&mut predecessor_map, next_state, state);
                    }
                    i_2 = i_2.wrapping_add(1);
                }
            } else if lookahead_iterator.next_state as libc::c_int != 0 as libc::c_int {
                if lookahead_iterator.next_state as libc::c_int != state as libc::c_int {
                    state_predecessor_map_add(
                        &mut predecessor_map,
                        lookahead_iterator.next_state,
                        state,
                    );
                }
                if ts_language_state_is_primary((*self_0).language, state) {
                    let mut aliases_0: *const TSSymbol = 0 as *const TSSymbol;
                    let mut aliases_end_0: *const TSSymbol = 0 as *const TSSymbol;
                    ts_language_aliases_for_symbol(
                        (*self_0).language,
                        lookahead_iterator.symbol,
                        &mut aliases_0,
                        &mut aliases_end_0,
                    );
                    let mut symbol_0: *const TSSymbol = aliases_0;
                    while symbol_0 < aliases_end_0 {
                        subgraph_index = 0 as libc::c_int as libc::c_uint;
                        exists = 0 as libc::c_int as libc::c_uint;
                        let mut size_2: uint32_t = (subgraphs.size).wrapping_sub(subgraph_index);
                        if !(size_2 == 0 as libc::c_int as libc::c_uint) {
                            let mut comparison_2: libc::c_int = 0;
                            while size_2 > 1 as libc::c_int as libc::c_uint {
                                let mut half_size_2: uint32_t =
                                    size_2.wrapping_div(2 as libc::c_int as libc::c_uint);
                                let mut mid_index_2: uint32_t =
                                    subgraph_index.wrapping_add(half_size_2);
                                comparison_2 = (*(subgraphs.contents).offset(mid_index_2 as isize))
                                    .symbol
                                    as libc::c_int
                                    - *symbol_0 as libc::c_int;
                                if comparison_2 <= 0 as libc::c_int {
                                    subgraph_index = mid_index_2;
                                }
                                size_2 = (size_2 as libc::c_uint).wrapping_sub(half_size_2)
                                    as uint32_t
                                    as uint32_t;
                            }
                            comparison_2 = (*(subgraphs.contents).offset(subgraph_index as isize))
                                .symbol as libc::c_int
                                - *symbol_0 as libc::c_int;
                            if comparison_2 == 0 as libc::c_int {
                                exists = 1 as libc::c_int as libc::c_uint;
                            } else if comparison_2 < 0 as libc::c_int {
                                subgraph_index =
                                    subgraph_index.wrapping_add(1 as libc::c_int as libc::c_uint);
                            }
                        }
                        if exists != 0 {
                            let mut subgraph_2: *mut AnalysisSubgraph = &mut *(subgraphs.contents)
                                .offset(subgraph_index as isize)
                                as *mut AnalysisSubgraph;
                            if (*subgraph_2).start_states.size == 0 as libc::c_int as libc::c_uint
                                || {
                                    if ((*subgraph_2).start_states.size)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        < (*subgraph_2).start_states.size
                                    {
                                    } else {
                                        panic!();
                                    }
                                    *(&mut *((*subgraph_2).start_states.contents).offset(
                                        ((*subgraph_2).start_states.size)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                            as isize,
                                    ) as *mut TSStateId)
                                        as libc::c_int
                                        != state as libc::c_int
                                }
                            {
                                array__grow(
                                    &mut (*subgraph_2).start_states as *mut C2RustUnnamed_24
                                        as *mut VoidArray,
                                    1 as libc::c_int as uint32_t,
                                    ::std::mem::size_of::<TSStateId>() as libc::c_ulong,
                                );
                                let ref mut fresh39 = (*subgraph_2).start_states.size;
                                let fresh40 = *fresh39;
                                *fresh39 = (*fresh39).wrapping_add(1);
                                *((*subgraph_2).start_states.contents).offset(fresh40 as isize) =
                                    state;
                            }
                        }
                        symbol_0 = symbol_0.offset(1);
                    }
                }
            }
        }
        state = state.wrapping_add(1);
    }
    let mut next_nodes: C2RustUnnamed_20 = {
        let mut init = C2RustUnnamed_20 {
            contents: 0 as *mut AnalysisSubgraphNode,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut i_3: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_3 < subgraphs.size {
        let mut subgraph_3: *mut AnalysisSubgraph =
            &mut *(subgraphs.contents).offset(i_3 as isize) as *mut AnalysisSubgraph;
        if (*subgraph_3).nodes.size == 0 as libc::c_int as libc::c_uint {
            array__delete(
                &mut (*subgraph_3).start_states as *mut C2RustUnnamed_24 as *mut VoidArray,
            );
            array__erase(
                &mut subgraphs as *mut AnalysisSubgraphArray as *mut VoidArray,
                ::std::mem::size_of::<AnalysisSubgraph>() as libc::c_ulong,
                i_3,
            );
            i_3 = i_3.wrapping_sub(1);
        } else {
            array__assign(
                &mut next_nodes as *mut C2RustUnnamed_20 as *mut VoidArray,
                &mut (*subgraph_3).nodes as *mut C2RustUnnamed_23 as *const VoidArray,
                ::std::mem::size_of::<AnalysisSubgraphNode>() as libc::c_ulong,
            );
            while next_nodes.size > 0 as libc::c_int as libc::c_uint {
                next_nodes.size = (next_nodes.size).wrapping_sub(1);
                let mut node: AnalysisSubgraphNode =
                    *(next_nodes.contents).offset(next_nodes.size as isize);
                if node.child_index() as libc::c_int > 1 as libc::c_int {
                    let mut predecessor_count: libc::c_uint = 0;
                    let mut predecessors: *const TSStateId = state_predecessor_map_get(
                        &mut predecessor_map,
                        node.state,
                        &mut predecessor_count,
                    );
                    let mut j_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    while j_0 < predecessor_count {
                        let mut predecessor_node: AnalysisSubgraphNode = {
                            let mut init = AnalysisSubgraphNode {
                                child_index_done: [0; 1],
                                c2rust_padding: [0; 1],
                                state: *predecessors.offset(j_0 as isize),
                                production_id: node.production_id,
                            };
                            init.set_child_index(
                                (node.child_index() as libc::c_int - 1 as libc::c_int) as uint8_t,
                            );
                            init.set_done(0 as libc::c_int != 0);
                            init
                        };
                        let mut index: libc::c_uint = 0;
                        let mut exists_0: libc::c_uint = 0;
                        index = 0 as libc::c_int as libc::c_uint;
                        exists_0 = 0 as libc::c_int as libc::c_uint;
                        let mut size_3: uint32_t = ((*subgraph_3).nodes.size).wrapping_sub(index);
                        if !(size_3 == 0 as libc::c_int as libc::c_uint) {
                            let mut comparison_3: libc::c_int = 0;
                            while size_3 > 1 as libc::c_int as libc::c_uint {
                                let mut half_size_3: uint32_t =
                                    size_3.wrapping_div(2 as libc::c_int as libc::c_uint);
                                let mut mid_index_3: uint32_t = index.wrapping_add(half_size_3);
                                comparison_3 = analysis_subgraph_node__compare(
                                    &mut *((*subgraph_3).nodes.contents)
                                        .offset(mid_index_3 as isize),
                                    &mut predecessor_node,
                                );
                                if comparison_3 <= 0 as libc::c_int {
                                    index = mid_index_3;
                                }
                                size_3 = (size_3 as libc::c_uint).wrapping_sub(half_size_3)
                                    as uint32_t
                                    as uint32_t;
                            }
                            comparison_3 = analysis_subgraph_node__compare(
                                &mut *((*subgraph_3).nodes.contents).offset(index as isize),
                                &mut predecessor_node,
                            );
                            if comparison_3 == 0 as libc::c_int {
                                exists_0 = 1 as libc::c_int as libc::c_uint;
                            } else if comparison_3 < 0 as libc::c_int {
                                index = index.wrapping_add(1 as libc::c_int as libc::c_uint);
                            }
                        }
                        if exists_0 == 0 {
                            array__splice(
                                &mut (*subgraph_3).nodes as *mut C2RustUnnamed_23 as *mut VoidArray,
                                ::std::mem::size_of::<AnalysisSubgraphNode>() as libc::c_ulong,
                                index,
                                0 as libc::c_int as uint32_t,
                                1 as libc::c_int as uint32_t,
                                &mut predecessor_node as *mut AnalysisSubgraphNode
                                    as *const libc::c_void,
                            );
                            array__grow(
                                &mut next_nodes as *mut C2RustUnnamed_20 as *mut VoidArray,
                                1 as libc::c_int as uint32_t,
                                ::std::mem::size_of::<AnalysisSubgraphNode>() as libc::c_ulong,
                            );
                            let fresh41 = next_nodes.size;
                            next_nodes.size = (next_nodes.size).wrapping_add(1);
                            *(next_nodes.contents).offset(fresh41 as isize) = predecessor_node;
                        }
                        j_0 = j_0.wrapping_add(1);
                    }
                }
            }
        }
        i_3 = i_3.wrapping_add(1);
    }
    let mut all_patterns_are_valid: bool = 1 as libc::c_int != 0;
    let mut analysis: QueryAnalysis = query_analysis__new();
    let mut i_4: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_4 < parent_step_indices.size {
        let mut parent_step_index_0: uint16_t =
            *(parent_step_indices.contents).offset(i_4 as isize) as uint16_t;
        let mut parent_depth: uint16_t =
            (*((*self_0).steps.contents).offset(parent_step_index_0 as isize)).depth;
        let mut parent_symbol_0: TSSymbol =
            (*((*self_0).steps.contents).offset(parent_step_index_0 as isize)).symbol;
        if !(parent_symbol_0 as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int) {
            let mut subgraph_index_0: libc::c_uint = 0;
            let mut exists_1: libc::c_uint = 0;
            subgraph_index_0 = 0 as libc::c_int as libc::c_uint;
            exists_1 = 0 as libc::c_int as libc::c_uint;
            let mut size_4: uint32_t = (subgraphs.size).wrapping_sub(subgraph_index_0);
            if !(size_4 == 0 as libc::c_int as libc::c_uint) {
                let mut comparison_4: libc::c_int = 0;
                while size_4 > 1 as libc::c_int as libc::c_uint {
                    let mut half_size_4: uint32_t =
                        size_4.wrapping_div(2 as libc::c_int as libc::c_uint);
                    let mut mid_index_4: uint32_t = subgraph_index_0.wrapping_add(half_size_4);
                    comparison_4 = (*(subgraphs.contents).offset(mid_index_4 as isize)).symbol
                        as libc::c_int
                        - parent_symbol_0 as libc::c_int;
                    if comparison_4 <= 0 as libc::c_int {
                        subgraph_index_0 = mid_index_4;
                    }
                    size_4 =
                        (size_4 as libc::c_uint).wrapping_sub(half_size_4) as uint32_t as uint32_t;
                }
                comparison_4 = (*(subgraphs.contents).offset(subgraph_index_0 as isize)).symbol
                    as libc::c_int
                    - parent_symbol_0 as libc::c_int;
                if comparison_4 == 0 as libc::c_int {
                    exists_1 = 1 as libc::c_int as libc::c_uint;
                } else if comparison_4 < 0 as libc::c_int {
                    subgraph_index_0 =
                        subgraph_index_0.wrapping_add(1 as libc::c_int as libc::c_uint);
                }
            }
            if exists_1 == 0 {
                let mut first_child_step_index: libc::c_uint =
                    (parent_step_index_0 as libc::c_int + 1 as libc::c_int) as libc::c_uint;
                let mut j_1: uint32_t = 0;
                let mut child_exists: uint32_t = 0;
                j_1 = 0 as libc::c_int as uint32_t;
                child_exists = 0 as libc::c_int as uint32_t;
                let mut size_5: uint32_t = ((*self_0).step_offsets.size).wrapping_sub(j_1);
                if !(size_5 == 0 as libc::c_int as libc::c_uint) {
                    let mut comparison_5: libc::c_int = 0;
                    while size_5 > 1 as libc::c_int as libc::c_uint {
                        let mut half_size_5: uint32_t =
                            size_5.wrapping_div(2 as libc::c_int as libc::c_uint);
                        let mut mid_index_5: uint32_t = j_1.wrapping_add(half_size_5);
                        comparison_5 = (*((*self_0).step_offsets.contents)
                            .offset(mid_index_5 as isize))
                        .step_index as libc::c_int
                            - first_child_step_index as libc::c_int;
                        if comparison_5 <= 0 as libc::c_int {
                            j_1 = mid_index_5;
                        }
                        size_5 = (size_5 as libc::c_uint).wrapping_sub(half_size_5) as uint32_t
                            as uint32_t;
                    }
                    comparison_5 = (*((*self_0).step_offsets.contents).offset(j_1 as isize))
                        .step_index as libc::c_int
                        - first_child_step_index as libc::c_int;
                    if comparison_5 == 0 as libc::c_int {
                        child_exists = 1 as libc::c_int as uint32_t;
                    } else if comparison_5 < 0 as libc::c_int {
                        j_1 = (j_1 as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                            as uint32_t as uint32_t;
                    }
                }
                if child_exists != 0 {
                } else {
                    panic!();
                }
                *error_offset =
                    (*((*self_0).step_offsets.contents).offset(j_1 as isize)).byte_offset;
                all_patterns_are_valid = 0 as libc::c_int != 0;
                break;
            } else {
                let mut subgraph_4: *mut AnalysisSubgraph = &mut *(subgraphs.contents)
                    .offset(subgraph_index_0 as isize)
                    as *mut AnalysisSubgraph;
                analysis_state_set__clear(&mut analysis.states, &mut analysis.state_pool);
                analysis_state_set__clear(&mut analysis.deeper_states, &mut analysis.state_pool);
                let mut j_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while j_2 < (*subgraph_4).start_states.size {
                    let mut parse_state: TSStateId =
                        *((*subgraph_4).start_states.contents).offset(j_2 as isize);
                    analysis_state_set__push(
                        &mut analysis.states,
                        &mut analysis.state_pool,
                        &mut {
                            let mut init = AnalysisState {
                                stack: [
                                    {
                                        let mut init = AnalysisStateEntry {
                                            field_id_done: [0; 2],
                                            parse_state: parse_state,
                                            parent_symbol: parent_symbol_0,
                                            child_index: 0 as libc::c_int as uint16_t,
                                        };
                                        init.set_field_id(0 as libc::c_int as TSFieldId);
                                        init.set_done(0 as libc::c_int != 0);
                                        init
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                ],
                                depth: 1 as libc::c_int as uint16_t,
                                step_index: (parent_step_index_0 as libc::c_int + 1 as libc::c_int)
                                    as uint16_t,
                                root_symbol: parent_symbol_0,
                            };
                            init
                        },
                    );
                    j_2 = j_2.wrapping_add(1);
                }
                analysis.did_abort = 0 as libc::c_int != 0;
                ts_query__perform_analysis(self_0, &mut subgraphs, &mut analysis);
                if analysis.did_abort {
                    let mut j_3: libc::c_uint =
                        (parent_step_index_0 as libc::c_int + 1 as libc::c_int) as libc::c_uint;
                    while j_3 < (*self_0).steps.size {
                        let mut step_1: *mut QueryStep =
                            &mut *((*self_0).steps.contents).offset(j_3 as isize) as *mut QueryStep;
                        if (*step_1).depth as libc::c_int <= parent_depth as libc::c_int
                            || (*step_1).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int
                        {
                            break;
                        }
                        if !(*step_1).is_dead_end() {
                            (*step_1).set_parent_pattern_guaranteed(0 as libc::c_int != 0);
                            (*step_1).set_root_pattern_guaranteed(0 as libc::c_int != 0);
                        }
                        j_3 = j_3.wrapping_add(1);
                    }
                } else if analysis.finished_parent_symbols.size == 0 as libc::c_int as libc::c_uint
                {
                    if analysis.final_step_indices.size > 0 as libc::c_int as libc::c_uint {
                    } else {
                        panic!();
                    }
                    if (analysis.final_step_indices.size)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                        < analysis.final_step_indices.size
                    {
                    } else {
                        panic!();
                    }
                    let mut impossible_step_index: uint16_t =
                        *(&mut *(analysis.final_step_indices.contents).offset(
                            (analysis.final_step_indices.size)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as *mut uint16_t);
                    let mut j_4: uint32_t = 0;
                    let mut impossible_exists: uint32_t = 0;
                    j_4 = 0 as libc::c_int as uint32_t;
                    impossible_exists = 0 as libc::c_int as uint32_t;
                    let mut size_6: uint32_t = ((*self_0).step_offsets.size).wrapping_sub(j_4);
                    if !(size_6 == 0 as libc::c_int as libc::c_uint) {
                        let mut comparison_6: libc::c_int = 0;
                        while size_6 > 1 as libc::c_int as libc::c_uint {
                            let mut half_size_6: uint32_t =
                                size_6.wrapping_div(2 as libc::c_int as libc::c_uint);
                            let mut mid_index_6: uint32_t = j_4.wrapping_add(half_size_6);
                            comparison_6 = (*((*self_0).step_offsets.contents)
                                .offset(mid_index_6 as isize))
                            .step_index as libc::c_int
                                - impossible_step_index as libc::c_int;
                            if comparison_6 <= 0 as libc::c_int {
                                j_4 = mid_index_6;
                            }
                            size_6 = (size_6 as libc::c_uint).wrapping_sub(half_size_6) as uint32_t
                                as uint32_t;
                        }
                        comparison_6 = (*((*self_0).step_offsets.contents).offset(j_4 as isize))
                            .step_index as libc::c_int
                            - impossible_step_index as libc::c_int;
                        if comparison_6 == 0 as libc::c_int {
                            impossible_exists = 1 as libc::c_int as uint32_t;
                        } else if comparison_6 < 0 as libc::c_int {
                            j_4 = (j_4 as libc::c_uint)
                                .wrapping_add(1 as libc::c_int as libc::c_uint)
                                as uint32_t as uint32_t;
                        }
                    }
                    if j_4 >= (*self_0).step_offsets.size {
                        j_4 = ((*self_0).step_offsets.size)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint);
                    }
                    *error_offset =
                        (*((*self_0).step_offsets.contents).offset(j_4 as isize)).byte_offset;
                    all_patterns_are_valid = 0 as libc::c_int != 0;
                    break;
                } else {
                    let mut j_5: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    while j_5 < analysis.final_step_indices.size {
                        let mut final_step_index: uint32_t = *(analysis.final_step_indices.contents)
                            .offset(j_5 as isize)
                            as uint32_t;
                        let mut step_2: *mut QueryStep = &mut *((*self_0).steps.contents)
                            .offset(final_step_index as isize)
                            as *mut QueryStep;
                        if (*step_2).depth as libc::c_int != PATTERN_DONE_MARKER as libc::c_int
                            && (*step_2).depth as libc::c_int > parent_depth as libc::c_int
                            && !(*step_2).is_dead_end()
                        {
                            (*step_2).set_parent_pattern_guaranteed(0 as libc::c_int != 0);
                            (*step_2).set_root_pattern_guaranteed(0 as libc::c_int != 0);
                        }
                        j_5 = j_5.wrapping_add(1);
                    }
                }
            }
        }
        i_4 = i_4.wrapping_add(1);
    }
    let mut predicate_capture_ids: C2RustUnnamed_17 = {
        let mut init = C2RustUnnamed_17 {
            contents: 0 as *mut uint16_t,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    let mut i_5: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_5 < (*self_0).patterns.size {
        let mut pattern_0: *mut QueryPattern =
            &mut *((*self_0).patterns.contents).offset(i_5 as isize) as *mut QueryPattern;
        predicate_capture_ids.size = 0 as libc::c_int as uint32_t;
        let mut start: libc::c_uint = (*pattern_0).predicate_steps.offset;
        let mut end: libc::c_uint = start.wrapping_add((*pattern_0).predicate_steps.length);
        let mut j_6: libc::c_uint = start;
        while j_6 < end {
            let mut step_3: *mut TSQueryPredicateStep = &mut *((*self_0).predicate_steps.contents)
                .offset(j_6 as isize)
                as *mut TSQueryPredicateStep;
            if (*step_3).type_ as libc::c_uint
                == TSQueryPredicateStepTypeCapture as libc::c_int as libc::c_uint
            {
                let mut value_id: uint16_t = (*step_3).value_id as uint16_t;
                let mut _index_1: libc::c_uint = 0;
                let mut _exists_1: libc::c_uint = 0;
                _index_1 = 0 as libc::c_int as libc::c_uint;
                _exists_1 = 0 as libc::c_int as libc::c_uint;
                let mut size_7: uint32_t = (predicate_capture_ids.size).wrapping_sub(_index_1);
                if !(size_7 == 0 as libc::c_int as libc::c_uint) {
                    let mut comparison_7: libc::c_int = 0;
                    while size_7 > 1 as libc::c_int as libc::c_uint {
                        let mut half_size_7: uint32_t =
                            size_7.wrapping_div(2 as libc::c_int as libc::c_uint);
                        let mut mid_index_7: uint32_t = _index_1.wrapping_add(half_size_7);
                        comparison_7 = *(predicate_capture_ids.contents)
                            .offset(mid_index_7 as isize)
                            as libc::c_int
                            - value_id as libc::c_int;
                        if comparison_7 <= 0 as libc::c_int {
                            _index_1 = mid_index_7;
                        }
                        size_7 = (size_7 as libc::c_uint).wrapping_sub(half_size_7) as uint32_t
                            as uint32_t;
                    }
                    comparison_7 = *(predicate_capture_ids.contents).offset(_index_1 as isize)
                        as libc::c_int
                        - value_id as libc::c_int;
                    if comparison_7 == 0 as libc::c_int {
                        _exists_1 = 1 as libc::c_int as libc::c_uint;
                    } else if comparison_7 < 0 as libc::c_int {
                        _index_1 = _index_1.wrapping_add(1 as libc::c_int as libc::c_uint);
                    }
                }
                if _exists_1 == 0 {
                    array__splice(
                        &mut predicate_capture_ids as *mut C2RustUnnamed_17 as *mut VoidArray,
                        ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                        _index_1,
                        0 as libc::c_int as uint32_t,
                        1 as libc::c_int as uint32_t,
                        &mut value_id as *mut uint16_t as *const libc::c_void,
                    );
                }
            }
            j_6 = j_6.wrapping_add(1);
        }
        let mut start_0: libc::c_uint = (*pattern_0).steps.offset;
        let mut end_0: libc::c_uint = start_0.wrapping_add((*pattern_0).steps.length);
        let mut j_7: libc::c_uint = start_0;
        while j_7 < end_0 {
            let mut step_4: *mut QueryStep =
                &mut *((*self_0).steps.contents).offset(j_7 as isize) as *mut QueryStep;
            let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while k < 3 as libc::c_int as libc::c_uint {
                let mut capture_id: uint16_t = (*step_4).capture_ids[k as usize];
                if capture_id as libc::c_int == NONE as libc::c_int {
                    break;
                }
                let mut index_0: libc::c_uint = 0;
                let mut exists_2: libc::c_uint = 0;
                index_0 = 0 as libc::c_int as libc::c_uint;
                exists_2 = 0 as libc::c_int as libc::c_uint;
                let mut size_8: uint32_t = (predicate_capture_ids.size).wrapping_sub(index_0);
                if !(size_8 == 0 as libc::c_int as libc::c_uint) {
                    let mut comparison_8: libc::c_int = 0;
                    while size_8 > 1 as libc::c_int as libc::c_uint {
                        let mut half_size_8: uint32_t =
                            size_8.wrapping_div(2 as libc::c_int as libc::c_uint);
                        let mut mid_index_8: uint32_t = index_0.wrapping_add(half_size_8);
                        comparison_8 = *(predicate_capture_ids.contents)
                            .offset(mid_index_8 as isize)
                            as libc::c_int
                            - capture_id as libc::c_int;
                        if comparison_8 <= 0 as libc::c_int {
                            index_0 = mid_index_8;
                        }
                        size_8 = (size_8 as libc::c_uint).wrapping_sub(half_size_8) as uint32_t
                            as uint32_t;
                    }
                    comparison_8 = *(predicate_capture_ids.contents).offset(index_0 as isize)
                        as libc::c_int
                        - capture_id as libc::c_int;
                    if comparison_8 == 0 as libc::c_int {
                        exists_2 = 1 as libc::c_int as libc::c_uint;
                    } else if comparison_8 < 0 as libc::c_int {
                        index_0 = index_0.wrapping_add(1 as libc::c_int as libc::c_uint);
                    }
                }
                if exists_2 != 0 {
                    (*step_4).set_root_pattern_guaranteed(0 as libc::c_int != 0);
                    break;
                } else {
                    k = k.wrapping_add(1);
                }
            }
            j_7 = j_7.wrapping_add(1);
        }
        i_5 = i_5.wrapping_add(1);
    }
    let mut done: bool = (*self_0).steps.size == 0 as libc::c_int as libc::c_uint;
    while !done {
        done = 1 as libc::c_int != 0;
        let mut i_6: libc::c_uint =
            ((*self_0).steps.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
        while i_6 > 0 as libc::c_int as libc::c_uint {
            let mut step_5: *mut QueryStep =
                &mut *((*self_0).steps.contents).offset(i_6 as isize) as *mut QueryStep;
            if !((*step_5).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int) {
                let mut parent_pattern_guaranteed: bool = 0 as libc::c_int != 0;
                loop {
                    if (*step_5).root_pattern_guaranteed() {
                        parent_pattern_guaranteed = 1 as libc::c_int != 0;
                        break;
                    } else {
                        if (*step_5).alternative_index as libc::c_int == NONE as libc::c_int
                            || ((*step_5).alternative_index as libc::c_uint) < i_6
                        {
                            break;
                        }
                        step_5 = &mut *((*self_0).steps.contents)
                            .offset((*step_5).alternative_index as isize)
                            as *mut QueryStep;
                    }
                }
                if !parent_pattern_guaranteed {
                    let mut prev_step: *mut QueryStep = &mut *((*self_0).steps.contents)
                        .offset(i_6.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                        as *mut QueryStep;
                    if !(*prev_step).is_dead_end()
                        && (*prev_step).depth as libc::c_int != PATTERN_DONE_MARKER as libc::c_int
                        && (*prev_step).root_pattern_guaranteed() as libc::c_int != 0
                    {
                        (*prev_step).set_root_pattern_guaranteed(0 as libc::c_int != 0);
                        done = 0 as libc::c_int != 0;
                    }
                }
            }
            i_6 = i_6.wrapping_sub(1);
        }
    }
    analysis.did_abort = 0 as libc::c_int != 0;
    let mut i_7: uint32_t = 0 as libc::c_int as uint32_t;
    while i_7 < non_rooted_pattern_start_steps.size {
        let mut pattern_entry_index: uint16_t =
            *(non_rooted_pattern_start_steps.contents).offset(i_7 as isize);
        let mut pattern_entry: *mut PatternEntry = &mut *((*self_0).pattern_map.contents)
            .offset(pattern_entry_index as isize)
            as *mut PatternEntry;
        analysis_state_set__clear(&mut analysis.states, &mut analysis.state_pool);
        analysis_state_set__clear(&mut analysis.deeper_states, &mut analysis.state_pool);
        let mut j_8: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while j_8 < subgraphs.size {
            let mut subgraph_5: *mut AnalysisSubgraph =
                &mut *(subgraphs.contents).offset(j_8 as isize) as *mut AnalysisSubgraph;
            let mut metadata: TSSymbolMetadata =
                ts_language_symbol_metadata((*self_0).language, (*subgraph_5).symbol);
            if !(metadata.visible as libc::c_int != 0 || metadata.named as libc::c_int != 0) {
                let mut k_0: uint32_t = 0 as libc::c_int as uint32_t;
                while k_0 < (*subgraph_5).start_states.size {
                    let mut parse_state_0: TSStateId =
                        *((*subgraph_5).start_states.contents).offset(k_0 as isize);
                    analysis_state_set__push(
                        &mut analysis.states,
                        &mut analysis.state_pool,
                        &mut {
                            let mut init = AnalysisState {
                                stack: [
                                    {
                                        let mut init = AnalysisStateEntry {
                                            field_id_done: [0; 2],
                                            parse_state: parse_state_0,
                                            parent_symbol: (*subgraph_5).symbol,
                                            child_index: 0 as libc::c_int as uint16_t,
                                        };
                                        init.set_field_id(0 as libc::c_int as TSFieldId);
                                        init.set_done(0 as libc::c_int != 0);
                                        init
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                    AnalysisStateEntry {
                                        parse_state: 0,
                                        parent_symbol: 0,
                                        child_index: 0,
                                        field_id_done: [0; 2],
                                    },
                                ],
                                depth: 1 as libc::c_int as uint16_t,
                                step_index: (*pattern_entry).step_index,
                                root_symbol: (*subgraph_5).symbol,
                            };
                            init
                        },
                    );
                    k_0 = k_0.wrapping_add(1);
                }
            }
            j_8 = j_8.wrapping_add(1);
        }
        ts_query__perform_analysis(self_0, &mut subgraphs, &mut analysis);
        if analysis.finished_parent_symbols.size > 0 as libc::c_int as libc::c_uint {
            (*((*self_0).patterns.contents).offset((*pattern_entry).pattern_index as isize))
                .is_non_local = 1 as libc::c_int != 0;
        }
        let mut k_1: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while k_1 < analysis.finished_parent_symbols.size {
            let mut symbol_1: TSSymbol =
                *(analysis.finished_parent_symbols.contents).offset(k_1 as isize);
            let mut _index_2: libc::c_uint = 0;
            let mut _exists_2: libc::c_uint = 0;
            _index_2 = 0 as libc::c_int as libc::c_uint;
            _exists_2 = 0 as libc::c_int as libc::c_uint;
            let mut size_9: uint32_t =
                ((*self_0).repeat_symbols_with_rootless_patterns.size).wrapping_sub(_index_2);
            if !(size_9 == 0 as libc::c_int as libc::c_uint) {
                let mut comparison_9: libc::c_int = 0;
                while size_9 > 1 as libc::c_int as libc::c_uint {
                    let mut half_size_9: uint32_t =
                        size_9.wrapping_div(2 as libc::c_int as libc::c_uint);
                    let mut mid_index_9: uint32_t = _index_2.wrapping_add(half_size_9);
                    comparison_9 = *((*self_0).repeat_symbols_with_rootless_patterns.contents)
                        .offset(mid_index_9 as isize)
                        as libc::c_int
                        - symbol_1 as libc::c_int;
                    if comparison_9 <= 0 as libc::c_int {
                        _index_2 = mid_index_9;
                    }
                    size_9 =
                        (size_9 as libc::c_uint).wrapping_sub(half_size_9) as uint32_t as uint32_t;
                }
                comparison_9 = *((*self_0).repeat_symbols_with_rootless_patterns.contents)
                    .offset(_index_2 as isize) as libc::c_int
                    - symbol_1 as libc::c_int;
                if comparison_9 == 0 as libc::c_int {
                    _exists_2 = 1 as libc::c_int as libc::c_uint;
                } else if comparison_9 < 0 as libc::c_int {
                    _index_2 = _index_2.wrapping_add(1 as libc::c_int as libc::c_uint);
                }
            }
            if _exists_2 == 0 {
                array__splice(
                    &mut (*self_0).repeat_symbols_with_rootless_patterns as *mut C2RustUnnamed_3
                        as *mut VoidArray,
                    ::std::mem::size_of::<TSSymbol>() as libc::c_ulong,
                    _index_2,
                    0 as libc::c_int as uint32_t,
                    1 as libc::c_int as uint32_t,
                    &mut symbol_1 as *mut TSSymbol as *const libc::c_void,
                );
            }
            k_1 = k_1.wrapping_add(1);
        }
        i_7 = i_7.wrapping_add(1);
    }
    let mut i_8: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_8 < subgraphs.size {
        array__delete(
            &mut (*(subgraphs.contents).offset(i_8 as isize)).start_states as *mut C2RustUnnamed_24
                as *mut VoidArray,
        );
        array__delete(
            &mut (*(subgraphs.contents).offset(i_8 as isize)).nodes as *mut C2RustUnnamed_23
                as *mut VoidArray,
        );
        i_8 = i_8.wrapping_add(1);
    }
    array__delete(&mut subgraphs as *mut AnalysisSubgraphArray as *mut VoidArray);
    query_analysis__delete(&mut analysis);
    array__delete(&mut next_nodes as *mut C2RustUnnamed_20 as *mut VoidArray);
    array__delete(&mut non_rooted_pattern_start_steps as *mut C2RustUnnamed_19 as *mut VoidArray);
    array__delete(&mut parent_step_indices as *mut C2RustUnnamed_18 as *mut VoidArray);
    array__delete(&mut predicate_capture_ids as *mut C2RustUnnamed_17 as *mut VoidArray);
    state_predecessor_map_delete(&mut predecessor_map);
    return all_patterns_are_valid;
}
unsafe extern "C" fn ts_query__add_negated_fields(
    mut self_0: *mut TSQuery,
    mut step_index: uint16_t,
    mut field_ids: *mut TSFieldId,
    mut field_count: uint16_t,
) {
    let mut step: *mut QueryStep =
        &mut *((*self_0).steps.contents).offset(step_index as isize) as *mut QueryStep;
    let mut failed_match: bool = 0 as libc::c_int != 0;
    let mut match_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut start_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).negated_fields.size {
        let mut existing_field_id: TSFieldId =
            *((*self_0).negated_fields.contents).offset(i as isize);
        if existing_field_id as libc::c_int == 0 as libc::c_int {
            if match_count == field_count as libc::c_uint {
                (*step).negated_field_list_id = start_i as uint16_t;
                return;
            } else {
                start_i = i.wrapping_add(1 as libc::c_int as libc::c_uint);
                match_count = 0 as libc::c_int as libc::c_uint;
                failed_match = 0 as libc::c_int != 0;
            }
        } else if match_count < field_count as libc::c_uint
            && existing_field_id as libc::c_int
                == *field_ids.offset(match_count as isize) as libc::c_int
            && !failed_match
        {
            match_count = match_count.wrapping_add(1);
        } else {
            match_count = 0 as libc::c_int as libc::c_uint;
            failed_match = 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    (*step).negated_field_list_id = (*self_0).negated_fields.size as uint16_t;
    array__splice(
        &mut (*self_0).negated_fields as *mut C2RustUnnamed_5 as *mut VoidArray,
        ::std::mem::size_of::<TSFieldId>() as libc::c_ulong,
        (*self_0).negated_fields.size,
        0 as libc::c_int as uint32_t,
        field_count as uint32_t,
        field_ids as *const libc::c_void,
    );
    array__grow(
        &mut (*self_0).negated_fields as *mut C2RustUnnamed_5 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<TSFieldId>() as libc::c_ulong,
    );
    let ref mut fresh42 = (*self_0).negated_fields.size;
    let fresh43 = *fresh42;
    *fresh42 = (*fresh42).wrapping_add(1);
    *((*self_0).negated_fields.contents).offset(fresh43 as isize) = 0 as libc::c_int as TSFieldId;
}
unsafe extern "C" fn ts_query__parse_string_literal(
    mut self_0: *mut TSQuery,
    mut stream: *mut Stream,
) -> TSQueryError {
    let mut string_start: *const libc::c_char = (*stream).input;
    if (*stream).next != '"' as i32 {
        return TSQueryErrorSyntax;
    }
    stream_advance(stream);
    let mut prev_position: *const libc::c_char = (*stream).input;
    let mut is_escaped: bool = 0 as libc::c_int != 0;
    (*self_0).string_buffer.size = 0 as libc::c_int as uint32_t;
    loop {
        if is_escaped {
            is_escaped = 0 as libc::c_int != 0;
            match (*stream).next {
                110 => {
                    array__grow(
                        &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                        1 as libc::c_int as uint32_t,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    let ref mut fresh44 = (*self_0).string_buffer.size;
                    let fresh45 = *fresh44;
                    *fresh44 = (*fresh44).wrapping_add(1);
                    *((*self_0).string_buffer.contents).offset(fresh45 as isize) =
                        '\n' as i32 as libc::c_char;
                }
                114 => {
                    array__grow(
                        &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                        1 as libc::c_int as uint32_t,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    let ref mut fresh46 = (*self_0).string_buffer.size;
                    let fresh47 = *fresh46;
                    *fresh46 = (*fresh46).wrapping_add(1);
                    *((*self_0).string_buffer.contents).offset(fresh47 as isize) =
                        '\r' as i32 as libc::c_char;
                }
                116 => {
                    array__grow(
                        &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                        1 as libc::c_int as uint32_t,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    let ref mut fresh48 = (*self_0).string_buffer.size;
                    let fresh49 = *fresh48;
                    *fresh48 = (*fresh48).wrapping_add(1);
                    *((*self_0).string_buffer.contents).offset(fresh49 as isize) =
                        '\t' as i32 as libc::c_char;
                }
                48 => {
                    array__grow(
                        &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                        1 as libc::c_int as uint32_t,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    let ref mut fresh50 = (*self_0).string_buffer.size;
                    let fresh51 = *fresh50;
                    *fresh50 = (*fresh50).wrapping_add(1);
                    *((*self_0).string_buffer.contents).offset(fresh51 as isize) =
                        '\0' as i32 as libc::c_char;
                }
                _ => {
                    array__splice(
                        &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        (*self_0).string_buffer.size,
                        0 as libc::c_int as uint32_t,
                        (*stream).next_size as uint32_t,
                        (*stream).input as *const libc::c_void,
                    );
                }
            }
            prev_position = ((*stream).input).offset((*stream).next_size as libc::c_int as isize);
        } else if (*stream).next == '\\' as i32 {
            array__splice(
                &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                (*self_0).string_buffer.size,
                0 as libc::c_int as uint32_t,
                ((*stream).input).offset_from(prev_position) as libc::c_long as uint32_t,
                prev_position as *const libc::c_void,
            );
            prev_position = ((*stream).input).offset(1 as libc::c_int as isize);
            is_escaped = 1 as libc::c_int != 0;
        } else if (*stream).next == '"' as i32 {
            array__splice(
                &mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                (*self_0).string_buffer.size,
                0 as libc::c_int as uint32_t,
                ((*stream).input).offset_from(prev_position) as libc::c_long as uint32_t,
                prev_position as *const libc::c_void,
            );
            stream_advance(stream);
            return TSQueryErrorNone;
        } else {
            if (*stream).next == '\n' as i32 {
                stream_reset(stream, string_start);
                return TSQueryErrorSyntax;
            }
        }
        if !stream_advance(stream) {
            stream_reset(stream, string_start);
            return TSQueryErrorSyntax;
        }
    }
}
unsafe extern "C" fn ts_query__parse_predicate(
    mut self_0: *mut TSQuery,
    mut stream: *mut Stream,
) -> TSQueryError {
    if !stream_is_ident_start(stream) {
        return TSQueryErrorSyntax;
    }
    let mut predicate_name: *const libc::c_char = (*stream).input;
    stream_scan_identifier(stream);
    let mut length: uint32_t =
        ((*stream).input).offset_from(predicate_name) as libc::c_long as uint32_t;
    let mut id: uint16_t =
        symbol_table_insert_name(&mut (*self_0).predicate_values, predicate_name, length);
    array__grow(
        &mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<TSQueryPredicateStep>() as libc::c_ulong,
    );
    let ref mut fresh52 = (*self_0).predicate_steps.size;
    let fresh53 = *fresh52;
    *fresh52 = (*fresh52).wrapping_add(1);
    *((*self_0).predicate_steps.contents).offset(fresh53 as isize) = {
        let mut init = TSQueryPredicateStep {
            type_: TSQueryPredicateStepTypeString,
            value_id: id as uint32_t,
        };
        init
    };
    stream_skip_whitespace(stream);
    loop {
        if (*stream).next == ')' as i32 {
            stream_advance(stream);
            stream_skip_whitespace(stream);
            array__grow(
                &mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<TSQueryPredicateStep>() as libc::c_ulong,
            );
            let ref mut fresh54 = (*self_0).predicate_steps.size;
            let fresh55 = *fresh54;
            *fresh54 = (*fresh54).wrapping_add(1);
            *((*self_0).predicate_steps.contents).offset(fresh55 as isize) = {
                let mut init = TSQueryPredicateStep {
                    type_: TSQueryPredicateStepTypeDone,
                    value_id: 0 as libc::c_int as uint32_t,
                };
                init
            };
            break;
        } else {
            if (*stream).next == '@' as i32 {
                stream_advance(stream);
                if !stream_is_ident_start(stream) {
                    return TSQueryErrorSyntax;
                }
                let mut capture_name: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut capture_length: uint32_t =
                    ((*stream).input).offset_from(capture_name) as libc::c_long as uint32_t;
                let mut capture_id: libc::c_int =
                    symbol_table_id_for_name(&mut (*self_0).captures, capture_name, capture_length);
                if capture_id == -(1 as libc::c_int) {
                    stream_reset(stream, capture_name);
                    return TSQueryErrorCapture;
                }
                array__grow(
                    &mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<TSQueryPredicateStep>() as libc::c_ulong,
                );
                let ref mut fresh56 = (*self_0).predicate_steps.size;
                let fresh57 = *fresh56;
                *fresh56 = (*fresh56).wrapping_add(1);
                *((*self_0).predicate_steps.contents).offset(fresh57 as isize) = {
                    let mut init = TSQueryPredicateStep {
                        type_: TSQueryPredicateStepTypeCapture,
                        value_id: capture_id as uint32_t,
                    };
                    init
                };
            } else if (*stream).next == '"' as i32 {
                let mut e: TSQueryError = ts_query__parse_string_literal(self_0, stream);
                if e as u64 != 0 {
                    return e;
                }
                let mut query_id: uint16_t = symbol_table_insert_name(
                    &mut (*self_0).predicate_values,
                    (*self_0).string_buffer.contents,
                    (*self_0).string_buffer.size,
                );
                array__grow(
                    &mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<TSQueryPredicateStep>() as libc::c_ulong,
                );
                let ref mut fresh58 = (*self_0).predicate_steps.size;
                let fresh59 = *fresh58;
                *fresh58 = (*fresh58).wrapping_add(1);
                *((*self_0).predicate_steps.contents).offset(fresh59 as isize) = {
                    let mut init = TSQueryPredicateStep {
                        type_: TSQueryPredicateStepTypeString,
                        value_id: query_id as uint32_t,
                    };
                    init
                };
            } else if stream_is_ident_start(stream) {
                let mut symbol_start: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut symbol_length: uint32_t =
                    ((*stream).input).offset_from(symbol_start) as libc::c_long as uint32_t;
                let mut query_id_0: uint16_t = symbol_table_insert_name(
                    &mut (*self_0).predicate_values,
                    symbol_start,
                    symbol_length,
                );
                array__grow(
                    &mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<TSQueryPredicateStep>() as libc::c_ulong,
                );
                let ref mut fresh60 = (*self_0).predicate_steps.size;
                let fresh61 = *fresh60;
                *fresh60 = (*fresh60).wrapping_add(1);
                *((*self_0).predicate_steps.contents).offset(fresh61 as isize) = {
                    let mut init = TSQueryPredicateStep {
                        type_: TSQueryPredicateStepTypeString,
                        value_id: query_id_0 as uint32_t,
                    };
                    init
                };
            } else {
                return TSQueryErrorSyntax;
            }
            stream_skip_whitespace(stream);
        }
    }
    return TSQueryErrorNone;
}
unsafe extern "C" fn ts_query__parse_pattern(
    mut self_0: *mut TSQuery,
    mut stream: *mut Stream,
    mut depth: uint32_t,
    mut is_immediate: bool,
    mut capture_quantifiers: *mut CaptureQuantifiers,
) -> TSQueryError {
    if (*stream).next == 0 as libc::c_int {
        return TSQueryErrorSyntax;
    }
    if (*stream).next == ')' as i32 || (*stream).next == ']' as i32 {
        return PARENT_DONE;
    }
    let starting_step_index: uint32_t = (*self_0).steps.size;
    if (*self_0).step_offsets.size == 0 as libc::c_int as libc::c_uint || {
        if ((*self_0).step_offsets.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            < (*self_0).step_offsets.size
        {
        } else {
            panic!();
        }
        (*(&mut *((*self_0).step_offsets.contents).offset(
            ((*self_0).step_offsets.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) as *mut StepOffset))
            .step_index as libc::c_uint
            != starting_step_index
    } {
        array__grow(
            &mut (*self_0).step_offsets as *mut C2RustUnnamed_6 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<StepOffset>() as libc::c_ulong,
        );
        let ref mut fresh62 = (*self_0).step_offsets.size;
        let fresh63 = *fresh62;
        *fresh62 = (*fresh62).wrapping_add(1);
        *((*self_0).step_offsets.contents).offset(fresh63 as isize) = {
            let mut init = StepOffset {
                byte_offset: stream_offset(stream),
                step_index: starting_step_index as uint16_t,
            };
            init
        };
    }
    if (*stream).next == '[' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);
        let mut branch_step_indices: C2RustUnnamed_25 = {
            let mut init = C2RustUnnamed_25 {
                contents: 0 as *mut uint32_t,
                size: 0 as libc::c_int as uint32_t,
                capacity: 0 as libc::c_int as uint32_t,
            };
            init
        };
        let mut branch_capture_quantifiers: CaptureQuantifiers = capture_quantifiers_new();
        loop {
            let mut start_index: uint32_t = (*self_0).steps.size;
            let mut e: TSQueryError = ts_query__parse_pattern(
                self_0,
                stream,
                depth,
                is_immediate,
                &mut branch_capture_quantifiers,
            );
            if e as libc::c_uint == PARENT_DONE as libc::c_uint {
                if (*stream).next == ']' as i32
                    && branch_step_indices.size > 0 as libc::c_int as libc::c_uint
                {
                    stream_advance(stream);
                    break;
                } else {
                    e = TSQueryErrorSyntax;
                }
            }
            if e as u64 != 0 {
                capture_quantifiers_delete(&mut branch_capture_quantifiers);
                array__delete(&mut branch_step_indices as *mut C2RustUnnamed_25 as *mut VoidArray);
                return e;
            }
            if start_index == starting_step_index {
                capture_quantifiers_replace(capture_quantifiers, &mut branch_capture_quantifiers);
            } else {
                capture_quantifiers_join_all(capture_quantifiers, &mut branch_capture_quantifiers);
            }
            array__grow(
                &mut branch_step_indices as *mut C2RustUnnamed_25 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
            );
            let fresh64 = branch_step_indices.size;
            branch_step_indices.size = (branch_step_indices.size).wrapping_add(1);
            *(branch_step_indices.contents).offset(fresh64 as isize) = start_index;
            array__grow(
                &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
            );
            let ref mut fresh65 = (*self_0).steps.size;
            let fresh66 = *fresh65;
            *fresh65 = (*fresh65).wrapping_add(1);
            *((*self_0).steps.contents).offset(fresh66 as isize) = query_step__new(
                0 as libc::c_int as TSSymbol,
                depth as uint16_t,
                0 as libc::c_int != 0,
            );
            capture_quantifiers_clear(&mut branch_capture_quantifiers);
        }
        let ref mut fresh67 = (*self_0).steps.size;
        *fresh67 = (*fresh67).wrapping_sub(1);
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (branch_step_indices.size).wrapping_sub(1 as libc::c_int as libc::c_uint) {
            let mut step_index: uint32_t = *(branch_step_indices.contents).offset(i as isize);
            let mut next_step_index: uint32_t = *(branch_step_indices.contents)
                .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            let mut start_step: *mut QueryStep =
                &mut *((*self_0).steps.contents).offset(step_index as isize) as *mut QueryStep;
            let mut end_step: *mut QueryStep = &mut *((*self_0).steps.contents)
                .offset(next_step_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                as *mut QueryStep;
            (*start_step).alternative_index = next_step_index as uint16_t;
            (*end_step).alternative_index = (*self_0).steps.size as uint16_t;
            (*end_step).set_is_dead_end(1 as libc::c_int != 0);
            i = i.wrapping_add(1);
        }
        capture_quantifiers_delete(&mut branch_capture_quantifiers);
        array__delete(&mut branch_step_indices as *mut C2RustUnnamed_25 as *mut VoidArray);
    } else if (*stream).next == '(' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);
        if (*stream).next == '(' as i32
            || (*stream).next == '"' as i32
            || (*stream).next == '[' as i32
        {
            let mut child_is_immediate: bool = is_immediate;
            let mut child_capture_quantifiers: CaptureQuantifiers = capture_quantifiers_new();
            loop {
                if (*stream).next == '.' as i32 {
                    child_is_immediate = 1 as libc::c_int != 0;
                    stream_advance(stream);
                    stream_skip_whitespace(stream);
                }
                let mut e_0: TSQueryError = ts_query__parse_pattern(
                    self_0,
                    stream,
                    depth,
                    child_is_immediate,
                    &mut child_capture_quantifiers,
                );
                if e_0 as libc::c_uint == PARENT_DONE as libc::c_uint {
                    if (*stream).next == ')' as i32 {
                        stream_advance(stream);
                        break;
                    } else {
                        e_0 = TSQueryErrorSyntax;
                    }
                }
                if e_0 as u64 != 0 {
                    capture_quantifiers_delete(&mut child_capture_quantifiers);
                    return e_0;
                }
                capture_quantifiers_add_all(capture_quantifiers, &mut child_capture_quantifiers);
                capture_quantifiers_clear(&mut child_capture_quantifiers);
                child_is_immediate = 0 as libc::c_int != 0;
            }
            capture_quantifiers_delete(&mut child_capture_quantifiers);
        } else if (*stream).next == '.' as i32 || (*stream).next == '#' as i32 {
            stream_advance(stream);
            return ts_query__parse_predicate(self_0, stream);
        } else {
            let mut symbol: TSSymbol = 0;
            if stream_is_ident_start(stream) {
                let mut node_name: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut length: uint32_t =
                    ((*stream).input).offset_from(node_name) as libc::c_long as uint32_t;
                if length > 0 as libc::c_int as libc::c_uint
                    && (*node_name
                        .offset(length.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                        as libc::c_int
                        == '!' as i32
                        || *node_name
                            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
                            as libc::c_int
                            == '?' as i32)
                {
                    stream_reset(stream, node_name);
                    return ts_query__parse_predicate(self_0, stream);
                } else {
                    if length == 1 as libc::c_int as libc::c_uint
                        && *node_name.offset(0 as libc::c_int as isize) as libc::c_int == '_' as i32
                    {
                        symbol = WILDCARD_SYMBOL;
                    } else {
                        symbol = ts_language_symbol_for_name(
                            (*self_0).language,
                            node_name,
                            length,
                            1 as libc::c_int != 0,
                        );
                        if symbol == 0 {
                            stream_reset(stream, node_name);
                            return TSQueryErrorNodeType;
                        }
                    }
                }
            } else {
                return TSQueryErrorSyntax;
            }
            array__grow(
                &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
            );
            let ref mut fresh68 = (*self_0).steps.size;
            let fresh69 = *fresh68;
            *fresh68 = (*fresh68).wrapping_add(1);
            *((*self_0).steps.contents).offset(fresh69 as isize) =
                query_step__new(symbol, depth as uint16_t, is_immediate);
            if ((*self_0).steps.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
                < (*self_0).steps.size
            {
            } else {
                panic!();
            }
            let mut step: *mut QueryStep = &mut *((*self_0).steps.contents).offset(
                ((*self_0).steps.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) as *mut QueryStep;
            if (ts_language_symbol_metadata((*self_0).language, symbol)).supertype {
                (*step).supertype_symbol = (*step).symbol;
                (*step).symbol = WILDCARD_SYMBOL;
            }
            if symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int {
                (*step).set_is_named(1 as libc::c_int != 0);
            }
            stream_skip_whitespace(stream);
            if (*stream).next == '/' as i32 {
                stream_advance(stream);
                if !stream_is_ident_start(stream) {
                    return TSQueryErrorSyntax;
                }
                let mut node_name_0: *const libc::c_char = (*stream).input;
                stream_scan_identifier(stream);
                let mut length_0: uint32_t =
                    ((*stream).input).offset_from(node_name_0) as libc::c_long as uint32_t;
                (*step).symbol = ts_language_symbol_for_name(
                    (*self_0).language,
                    node_name_0,
                    length_0,
                    1 as libc::c_int != 0,
                );
                if (*step).symbol == 0 {
                    stream_reset(stream, node_name_0);
                    return TSQueryErrorNodeType;
                }
                stream_skip_whitespace(stream);
            }
            let mut child_is_immediate_0: bool = 0 as libc::c_int != 0;
            let mut last_child_step_index: uint16_t = 0 as libc::c_int as uint16_t;
            let mut negated_field_count: uint16_t = 0 as libc::c_int as uint16_t;
            let mut negated_field_ids: [TSFieldId; 8] = [0; 8];
            let mut child_capture_quantifiers_0: CaptureQuantifiers = capture_quantifiers_new();
            loop {
                if (*stream).next == '!' as i32 {
                    stream_advance(stream);
                    stream_skip_whitespace(stream);
                    if !stream_is_ident_start(stream) {
                        capture_quantifiers_delete(&mut child_capture_quantifiers_0);
                        return TSQueryErrorSyntax;
                    }
                    let mut field_name: *const libc::c_char = (*stream).input;
                    stream_scan_identifier(stream);
                    let mut length_1: uint32_t =
                        ((*stream).input).offset_from(field_name) as libc::c_long as uint32_t;
                    stream_skip_whitespace(stream);
                    let mut field_id: TSFieldId =
                        ts_language_field_id_for_name((*self_0).language, field_name, length_1);
                    if field_id == 0 {
                        let ref mut fresh70 = (*stream).input;
                        *fresh70 = field_name;
                        capture_quantifiers_delete(&mut child_capture_quantifiers_0);
                        return TSQueryErrorField;
                    }
                    if (negated_field_count as libc::c_int) < 8 as libc::c_int {
                        negated_field_ids[negated_field_count as usize] = field_id;
                        negated_field_count = negated_field_count.wrapping_add(1);
                    }
                } else {
                    if (*stream).next == '.' as i32 {
                        child_is_immediate_0 = 1 as libc::c_int != 0;
                        stream_advance(stream);
                        stream_skip_whitespace(stream);
                    }
                    let mut step_index_0: uint16_t = (*self_0).steps.size as uint16_t;
                    let mut e_1: TSQueryError = ts_query__parse_pattern(
                        self_0,
                        stream,
                        depth.wrapping_add(1 as libc::c_int as libc::c_uint),
                        child_is_immediate_0,
                        &mut child_capture_quantifiers_0,
                    );
                    if e_1 as libc::c_uint == PARENT_DONE as libc::c_uint {
                        if (*stream).next == ')' as i32 {
                            if child_is_immediate_0 {
                                if last_child_step_index as libc::c_int == 0 as libc::c_int {
                                    capture_quantifiers_delete(&mut child_capture_quantifiers_0);
                                    return TSQueryErrorSyntax;
                                }
                                let ref mut fresh71 = *((*self_0).steps.contents)
                                    .offset(last_child_step_index as isize);
                                (*fresh71).set_is_last_child(1 as libc::c_int != 0);
                            }
                            if negated_field_count != 0 {
                                ts_query__add_negated_fields(
                                    self_0,
                                    starting_step_index as uint16_t,
                                    negated_field_ids.as_mut_ptr(),
                                    negated_field_count,
                                );
                            }
                            stream_advance(stream);
                            break;
                        } else {
                            e_1 = TSQueryErrorSyntax;
                        }
                    }
                    if e_1 as u64 != 0 {
                        capture_quantifiers_delete(&mut child_capture_quantifiers_0);
                        return e_1;
                    }
                    capture_quantifiers_add_all(
                        capture_quantifiers,
                        &mut child_capture_quantifiers_0,
                    );
                    last_child_step_index = step_index_0;
                    child_is_immediate_0 = 0 as libc::c_int != 0;
                    capture_quantifiers_clear(&mut child_capture_quantifiers_0);
                }
            }
            capture_quantifiers_delete(&mut child_capture_quantifiers_0);
        }
    } else if (*stream).next == '_' as i32 {
        stream_advance(stream);
        stream_skip_whitespace(stream);
        array__grow(
            &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
        );
        let ref mut fresh72 = (*self_0).steps.size;
        let fresh73 = *fresh72;
        *fresh72 = (*fresh72).wrapping_add(1);
        *((*self_0).steps.contents).offset(fresh73 as isize) =
            query_step__new(WILDCARD_SYMBOL, depth as uint16_t, is_immediate);
    } else if (*stream).next == '"' as i32 {
        let mut string_start: *const libc::c_char = (*stream).input;
        let mut e_2: TSQueryError = ts_query__parse_string_literal(self_0, stream);
        if e_2 as u64 != 0 {
            return e_2;
        }
        let mut symbol_0: TSSymbol = ts_language_symbol_for_name(
            (*self_0).language,
            (*self_0).string_buffer.contents,
            (*self_0).string_buffer.size,
            0 as libc::c_int != 0,
        );
        if symbol_0 == 0 {
            stream_reset(stream, string_start.offset(1 as libc::c_int as isize));
            return TSQueryErrorNodeType;
        }
        array__grow(
            &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
        );
        let ref mut fresh74 = (*self_0).steps.size;
        let fresh75 = *fresh74;
        *fresh74 = (*fresh74).wrapping_add(1);
        *((*self_0).steps.contents).offset(fresh75 as isize) =
            query_step__new(symbol_0, depth as uint16_t, is_immediate);
    } else if stream_is_ident_start(stream) {
        let mut field_name_0: *const libc::c_char = (*stream).input;
        stream_scan_identifier(stream);
        let mut length_2: uint32_t =
            ((*stream).input).offset_from(field_name_0) as libc::c_long as uint32_t;
        stream_skip_whitespace(stream);
        if (*stream).next != ':' as i32 {
            stream_reset(stream, field_name_0);
            return TSQueryErrorSyntax;
        }
        stream_advance(stream);
        stream_skip_whitespace(stream);
        let mut field_capture_quantifiers: CaptureQuantifiers = capture_quantifiers_new();
        let mut e_3: TSQueryError = ts_query__parse_pattern(
            self_0,
            stream,
            depth,
            is_immediate,
            &mut field_capture_quantifiers,
        );
        if e_3 as u64 != 0 {
            capture_quantifiers_delete(&mut field_capture_quantifiers);
            if e_3 as libc::c_uint == PARENT_DONE as libc::c_uint {
                e_3 = TSQueryErrorSyntax;
            }
            return e_3;
        }
        let mut field_id_0: TSFieldId =
            ts_language_field_id_for_name((*self_0).language, field_name_0, length_2);
        if field_id_0 == 0 {
            let ref mut fresh76 = (*stream).input;
            *fresh76 = field_name_0;
            return TSQueryErrorField;
        }
        let mut step_index_1: uint32_t = starting_step_index;
        let mut step_0: *mut QueryStep =
            &mut *((*self_0).steps.contents).offset(step_index_1 as isize) as *mut QueryStep;
        loop {
            (*step_0).field = field_id_0;
            if !((*step_0).alternative_index as libc::c_int != NONE as libc::c_int
                && (*step_0).alternative_index as libc::c_uint > step_index_1
                && ((*step_0).alternative_index as libc::c_uint) < (*self_0).steps.size)
            {
                break;
            }
            step_index_1 = (*step_0).alternative_index as uint32_t;
            step_0 =
                &mut *((*self_0).steps.contents).offset(step_index_1 as isize) as *mut QueryStep;
        }
        capture_quantifiers_add_all(capture_quantifiers, &mut field_capture_quantifiers);
        capture_quantifiers_delete(&mut field_capture_quantifiers);
    } else {
        return TSQueryErrorSyntax;
    }
    stream_skip_whitespace(stream);
    let mut quantifier: TSQuantifier = TSQuantifierOne;
    loop {
        if (*stream).next == '+' as i32 {
            quantifier = quantifier_join(TSQuantifierOneOrMore, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step: QueryStep =
                query_step__new(WILDCARD_SYMBOL, depth as uint16_t, 0 as libc::c_int != 0);
            repeat_step.alternative_index = starting_step_index as uint16_t;
            repeat_step.set_is_pass_through(1 as libc::c_int != 0);
            repeat_step.set_alternative_is_immediate(1 as libc::c_int != 0);
            array__grow(
                &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
            );
            let ref mut fresh77 = (*self_0).steps.size;
            let fresh78 = *fresh77;
            *fresh77 = (*fresh77).wrapping_add(1);
            *((*self_0).steps.contents).offset(fresh78 as isize) = repeat_step;
        } else if (*stream).next == '*' as i32 {
            quantifier = quantifier_join(TSQuantifierZeroOrMore, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut repeat_step_0: QueryStep =
                query_step__new(WILDCARD_SYMBOL, depth as uint16_t, 0 as libc::c_int != 0);
            repeat_step_0.alternative_index = starting_step_index as uint16_t;
            repeat_step_0.set_is_pass_through(1 as libc::c_int != 0);
            repeat_step_0.set_alternative_is_immediate(1 as libc::c_int != 0);
            array__grow(
                &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
            );
            let ref mut fresh79 = (*self_0).steps.size;
            let fresh80 = *fresh79;
            *fresh79 = (*fresh79).wrapping_add(1);
            *((*self_0).steps.contents).offset(fresh80 as isize) = repeat_step_0;
            let mut step_1: *mut QueryStep = &mut *((*self_0).steps.contents)
                .offset(starting_step_index as isize)
                as *mut QueryStep;
            while (*step_1).alternative_index as libc::c_int != NONE as libc::c_int
                && ((*step_1).alternative_index as libc::c_uint)
                    < ((*self_0).steps.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            {
                step_1 = &mut *((*self_0).steps.contents)
                    .offset((*step_1).alternative_index as isize)
                    as *mut QueryStep;
            }
            (*step_1).alternative_index = (*self_0).steps.size as uint16_t;
        } else if (*stream).next == '?' as i32 {
            quantifier = quantifier_join(TSQuantifierZeroOrOne, quantifier);
            stream_advance(stream);
            stream_skip_whitespace(stream);
            let mut step_2: *mut QueryStep = &mut *((*self_0).steps.contents)
                .offset(starting_step_index as isize)
                as *mut QueryStep;
            while (*step_2).alternative_index as libc::c_int != NONE as libc::c_int
                && ((*step_2).alternative_index as libc::c_uint) < (*self_0).steps.size
            {
                step_2 = &mut *((*self_0).steps.contents)
                    .offset((*step_2).alternative_index as isize)
                    as *mut QueryStep;
            }
            (*step_2).alternative_index = (*self_0).steps.size as uint16_t;
        } else {
            if !((*stream).next == '@' as i32) {
                break;
            }
            stream_advance(stream);
            if !stream_is_ident_start(stream) {
                return TSQueryErrorSyntax;
            }
            let mut capture_name: *const libc::c_char = (*stream).input;
            stream_scan_identifier(stream);
            let mut length_3: uint32_t =
                ((*stream).input).offset_from(capture_name) as libc::c_long as uint32_t;
            stream_skip_whitespace(stream);
            let mut capture_id: uint16_t =
                symbol_table_insert_name(&mut (*self_0).captures, capture_name, length_3);
            capture_quantifiers_add_for_id(capture_quantifiers, capture_id, TSQuantifierOne);
            let mut step_index_2: uint32_t = starting_step_index;
            loop {
                let mut step_3: *mut QueryStep = &mut *((*self_0).steps.contents)
                    .offset(step_index_2 as isize)
                    as *mut QueryStep;
                query_step__add_capture(step_3, capture_id);
                if !((*step_3).alternative_index as libc::c_int != NONE as libc::c_int
                    && (*step_3).alternative_index as libc::c_uint > step_index_2
                    && ((*step_3).alternative_index as libc::c_uint) < (*self_0).steps.size)
                {
                    break;
                }
                step_index_2 = (*step_3).alternative_index as uint32_t;
                step_3 = &mut *((*self_0).steps.contents).offset(step_index_2 as isize)
                    as *mut QueryStep;
            }
        }
    }
    capture_quantifiers_mul(capture_quantifiers, quantifier);
    return TSQueryErrorNone;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_new(
    mut language: *const TSLanguage,
    mut source: *const libc::c_char,
    mut source_len: uint32_t,
    mut error_offset: *mut uint32_t,
    mut error_type: *mut TSQueryError,
) -> *mut TSQuery {
    if language.is_null()
        || (*language).version > 14 as libc::c_int as libc::c_uint
        || (*language).version < 13 as libc::c_int as libc::c_uint
    {
        *error_type = TSQueryErrorLanguage;
        return 0 as *mut TSQuery;
    }
    let mut self_0: *mut TSQuery =
        crate::core::alloc::ts_malloc(::std::mem::size_of::<TSQuery>() as libc::c_ulong)
            as *mut TSQuery;
    *self_0 = {
        let mut init = TSQuery {
            captures: symbol_table_new(),
            predicate_values: symbol_table_new(),
            capture_quantifiers: {
                let mut init = C2RustUnnamed_11 {
                    contents: 0 as *mut CaptureQuantifiers,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            steps: {
                let mut init = C2RustUnnamed_10 {
                    contents: 0 as *mut QueryStep,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            pattern_map: {
                let mut init = C2RustUnnamed_9 {
                    contents: 0 as *mut PatternEntry,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            predicate_steps: {
                let mut init = C2RustUnnamed_8 {
                    contents: 0 as *mut TSQueryPredicateStep,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            patterns: {
                let mut init = C2RustUnnamed_7 {
                    contents: 0 as *mut QueryPattern,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            step_offsets: {
                let mut init = C2RustUnnamed_6 {
                    contents: 0 as *mut StepOffset,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            negated_fields: {
                let mut init = C2RustUnnamed_5 {
                    contents: 0 as *mut TSFieldId,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            string_buffer: {
                let mut init = C2RustUnnamed_4 {
                    contents: 0 as *mut libc::c_char,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            repeat_symbols_with_rootless_patterns: {
                let mut init = C2RustUnnamed_3 {
                    contents: 0 as *mut TSSymbol,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            language: language,
            wildcard_root_pattern_count: 0 as libc::c_int as uint16_t,
        };
        init
    };
    array__grow(
        &mut (*self_0).negated_fields as *mut C2RustUnnamed_5 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<TSFieldId>() as libc::c_ulong,
    );
    let ref mut fresh81 = (*self_0).negated_fields.size;
    let fresh82 = *fresh81;
    *fresh81 = (*fresh81).wrapping_add(1);
    *((*self_0).negated_fields.contents).offset(fresh82 as isize) = 0 as libc::c_int as TSFieldId;
    let mut stream: Stream = stream_new(source, source_len);
    stream_skip_whitespace(&mut stream);
    while stream.input < stream.end {
        let mut pattern_index: uint32_t = (*self_0).patterns.size;
        let mut start_step_index: uint32_t = (*self_0).steps.size;
        let mut start_predicate_step_index: uint32_t = (*self_0).predicate_steps.size;
        array__grow(
            &mut (*self_0).patterns as *mut C2RustUnnamed_7 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<QueryPattern>() as libc::c_ulong,
        );
        let ref mut fresh83 = (*self_0).patterns.size;
        let fresh84 = *fresh83;
        *fresh83 = (*fresh83).wrapping_add(1);
        *((*self_0).patterns.contents).offset(fresh84 as isize) = {
            let mut init = QueryPattern {
                steps: {
                    let mut init = Slice {
                        offset: start_step_index,
                        length: 0,
                    };
                    init
                },
                predicate_steps: {
                    let mut init = Slice {
                        offset: start_predicate_step_index,
                        length: 0,
                    };
                    init
                },
                start_byte: stream_offset(&mut stream),
                is_non_local: 0 as libc::c_int != 0,
            };
            init
        };
        let mut capture_quantifiers: CaptureQuantifiers = capture_quantifiers_new();
        *error_type = ts_query__parse_pattern(
            self_0,
            &mut stream,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int != 0,
            &mut capture_quantifiers,
        );
        array__grow(
            &mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<QueryStep>() as libc::c_ulong,
        );
        let ref mut fresh85 = (*self_0).steps.size;
        let fresh86 = *fresh85;
        *fresh85 = (*fresh85).wrapping_add(1);
        *((*self_0).steps.contents).offset(fresh86 as isize) = query_step__new(
            0 as libc::c_int as TSSymbol,
            PATTERN_DONE_MARKER,
            0 as libc::c_int != 0,
        );
        if ((*self_0).patterns.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
            < (*self_0).patterns.size
        {
        } else {
            panic!();
        }
        let mut pattern: *mut QueryPattern = &mut *((*self_0).patterns.contents).offset(
            ((*self_0).patterns.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) as *mut QueryPattern;
        (*pattern).steps.length = ((*self_0).steps.size).wrapping_sub(start_step_index);
        (*pattern).predicate_steps.length =
            ((*self_0).predicate_steps.size).wrapping_sub(start_predicate_step_index);
        if *error_type as u64 != 0 {
            if *error_type as libc::c_uint == PARENT_DONE as libc::c_uint {
                *error_type = TSQueryErrorSyntax;
            }
            *error_offset = stream_offset(&mut stream);
            capture_quantifiers_delete(&mut capture_quantifiers);
            ts_query_delete(self_0);
            return 0 as *mut TSQuery;
        }
        array__grow(
            &mut (*self_0).capture_quantifiers as *mut C2RustUnnamed_11 as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<CaptureQuantifiers>() as libc::c_ulong,
        );
        let ref mut fresh87 = (*self_0).capture_quantifiers.size;
        let fresh88 = *fresh87;
        *fresh87 = (*fresh87).wrapping_add(1);
        *((*self_0).capture_quantifiers.contents).offset(fresh88 as isize) = capture_quantifiers;
        let mut wildcard_root_alternative_index: uint16_t = NONE;
        loop {
            let mut step: *mut QueryStep = &mut *((*self_0).steps.contents)
                .offset(start_step_index as isize)
                as *mut QueryStep;
            if (*step).symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int
                && (*step).depth as libc::c_int == 0 as libc::c_int
                && (*step).field == 0
            {
                let mut second_step: *mut QueryStep = &mut *((*self_0).steps.contents).offset(
                    start_step_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                ) as *mut QueryStep;
                if (*second_step).symbol as libc::c_int != WILDCARD_SYMBOL as libc::c_int
                    && (*second_step).depth as libc::c_int == 1 as libc::c_int
                {
                    wildcard_root_alternative_index = (*step).alternative_index;
                    start_step_index = (start_step_index as libc::c_uint)
                        .wrapping_add(1 as libc::c_int as libc::c_uint)
                        as uint32_t as uint32_t;
                    step = second_step;
                }
            }
            let mut start_depth: uint32_t = (*step).depth as uint32_t;
            let mut is_rooted: bool = start_depth == 0 as libc::c_int as libc::c_uint;
            let mut step_index: uint32_t =
                start_step_index.wrapping_add(1 as libc::c_int as libc::c_uint);
            while step_index < (*self_0).steps.size {
                let mut child_step: *mut QueryStep =
                    &mut *((*self_0).steps.contents).offset(step_index as isize) as *mut QueryStep;
                if (*child_step).is_dead_end() {
                    break;
                }
                if (*child_step).depth as libc::c_uint == start_depth {
                    is_rooted = 0 as libc::c_int != 0;
                    break;
                } else {
                    step_index = step_index.wrapping_add(1);
                }
            }
            ts_query__pattern_map_insert(self_0, (*step).symbol, {
                let mut init = PatternEntry {
                    step_index: start_step_index as uint16_t,
                    pattern_index: pattern_index as uint16_t,
                    is_rooted: is_rooted,
                };
                init
            });
            if (*step).symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int {
                let ref mut fresh89 = (*self_0).wildcard_root_pattern_count;
                *fresh89 = (*fresh89).wrapping_add(1);
            }
            if (*step).alternative_index as libc::c_int != NONE as libc::c_int {
                start_step_index = (*step).alternative_index as uint32_t;
            } else {
                if !(wildcard_root_alternative_index as libc::c_int != NONE as libc::c_int) {
                    break;
                }
                start_step_index = wildcard_root_alternative_index as uint32_t;
                wildcard_root_alternative_index = NONE;
            }
        }
    }
    if !ts_query__analyze_patterns(self_0, error_offset) {
        *error_type = TSQueryErrorStructure;
        ts_query_delete(self_0);
        return 0 as *mut TSQuery;
    }
    array__delete(&mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_delete(mut self_0: *mut TSQuery) {
    if !self_0.is_null() {
        array__delete(&mut (*self_0).steps as *mut C2RustUnnamed_10 as *mut VoidArray);
        array__delete(&mut (*self_0).pattern_map as *mut C2RustUnnamed_9 as *mut VoidArray);
        array__delete(&mut (*self_0).predicate_steps as *mut C2RustUnnamed_8 as *mut VoidArray);
        array__delete(&mut (*self_0).patterns as *mut C2RustUnnamed_7 as *mut VoidArray);
        array__delete(&mut (*self_0).step_offsets as *mut C2RustUnnamed_6 as *mut VoidArray);
        array__delete(&mut (*self_0).string_buffer as *mut C2RustUnnamed_4 as *mut VoidArray);
        array__delete(&mut (*self_0).negated_fields as *mut C2RustUnnamed_5 as *mut VoidArray);
        array__delete(
            &mut (*self_0).repeat_symbols_with_rootless_patterns as *mut C2RustUnnamed_3
                as *mut VoidArray,
        );
        symbol_table_delete(&mut (*self_0).captures);
        symbol_table_delete(&mut (*self_0).predicate_values);
        let mut index: uint32_t = 0 as libc::c_int as uint32_t;
        while index < (*self_0).capture_quantifiers.size {
            if index < (*self_0).capture_quantifiers.size {
            } else {
                panic!();
            }
            let mut capture_quantifiers: *mut CaptureQuantifiers =
                &mut *((*self_0).capture_quantifiers.contents).offset(index as isize)
                    as *mut CaptureQuantifiers;
            capture_quantifiers_delete(capture_quantifiers);
            index = index.wrapping_add(1);
        }
        array__delete(
            &mut (*self_0).capture_quantifiers as *mut C2RustUnnamed_11 as *mut VoidArray,
        );
        crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_pattern_count(mut self_0: *const TSQuery) -> uint32_t {
    return (*self_0).patterns.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_capture_count(mut self_0: *const TSQuery) -> uint32_t {
    return (*self_0).captures.slices.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_string_count(mut self_0: *const TSQuery) -> uint32_t {
    return (*self_0).predicate_values.slices.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_capture_name_for_id(
    mut self_0: *const TSQuery,
    mut index: uint32_t,
    mut length: *mut uint32_t,
) -> *const libc::c_char {
    return symbol_table_name_for_id(&(*self_0).captures, index as uint16_t, length);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_capture_quantifier_for_id(
    mut self_0: *const TSQuery,
    mut pattern_index: uint32_t,
    mut capture_index: uint32_t,
) -> TSQuantifier {
    if pattern_index < (*self_0).capture_quantifiers.size {
    } else {
        panic!();
    }
    let mut capture_quantifiers: *mut CaptureQuantifiers =
        &mut *((*self_0).capture_quantifiers.contents).offset(pattern_index as isize)
            as *mut CaptureQuantifiers;
    return capture_quantifier_for_id(capture_quantifiers, capture_index as uint16_t);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_string_value_for_id(
    mut self_0: *const TSQuery,
    mut index: uint32_t,
    mut length: *mut uint32_t,
) -> *const libc::c_char {
    return symbol_table_name_for_id(&(*self_0).predicate_values, index as uint16_t, length);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_predicates_for_pattern(
    mut self_0: *const TSQuery,
    mut pattern_index: uint32_t,
    mut step_count: *mut uint32_t,
) -> *const TSQueryPredicateStep {
    let mut slice: Slice =
        (*((*self_0).patterns.contents).offset(pattern_index as isize)).predicate_steps;
    *step_count = slice.length;
    if ((*self_0).predicate_steps.contents).is_null() {
        return 0 as *const TSQueryPredicateStep;
    }
    return &mut *((*self_0).predicate_steps.contents).offset(slice.offset as isize)
        as *mut TSQueryPredicateStep;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_start_byte_for_pattern(
    mut self_0: *const TSQuery,
    mut pattern_index: uint32_t,
) -> uint32_t {
    return (*((*self_0).patterns.contents).offset(pattern_index as isize)).start_byte;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_is_pattern_rooted(
    mut self_0: *const TSQuery,
    mut pattern_index: uint32_t,
) -> bool {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).pattern_map.size {
        let mut entry: *mut PatternEntry =
            &mut *((*self_0).pattern_map.contents).offset(i as isize) as *mut PatternEntry;
        if (*entry).pattern_index as libc::c_uint == pattern_index {
            if !(*entry).is_rooted {
                return 0 as libc::c_int != 0;
            }
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_is_pattern_non_local(
    mut self_0: *const TSQuery,
    mut pattern_index: uint32_t,
) -> bool {
    if pattern_index < (*self_0).patterns.size {
        return (*((*self_0).patterns.contents).offset(pattern_index as isize)).is_non_local;
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_is_pattern_guaranteed_at_step(
    mut self_0: *const TSQuery,
    mut byte_offset: uint32_t,
) -> bool {
    let mut step_index: uint32_t = 4294967295 as libc::c_uint;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).step_offsets.size {
        let mut step_offset: *mut StepOffset =
            &mut *((*self_0).step_offsets.contents).offset(i as isize) as *mut StepOffset;
        if (*step_offset).byte_offset > byte_offset {
            break;
        }
        step_index = (*step_offset).step_index as uint32_t;
        i = i.wrapping_add(1);
    }
    if step_index < (*self_0).steps.size {
        return (*((*self_0).steps.contents).offset(step_index as isize)).root_pattern_guaranteed();
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_query__step_is_fallible(
    mut self_0: *const TSQuery,
    mut step_index: uint16_t,
) -> bool {
    if (step_index as uint32_t).wrapping_add(1 as libc::c_int as libc::c_uint)
        < (*self_0).steps.size
    {
    } else {
        panic!();
    }
    let mut step: *mut QueryStep =
        &mut *((*self_0).steps.contents).offset(step_index as isize) as *mut QueryStep;
    let mut next_step: *mut QueryStep = &mut *((*self_0).steps.contents)
        .offset((step_index as libc::c_int + 1 as libc::c_int) as isize)
        as *mut QueryStep;
    return (*next_step).depth as libc::c_int != PATTERN_DONE_MARKER as libc::c_int
        && (*next_step).depth as libc::c_int > (*step).depth as libc::c_int
        && !(*next_step).parent_pattern_guaranteed();
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_disable_capture(
    mut self_0: *mut TSQuery,
    mut name: *const libc::c_char,
    mut length: uint32_t,
) {
    let mut id: libc::c_int = symbol_table_id_for_name(&mut (*self_0).captures, name, length);
    if id != -(1 as libc::c_int) {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).steps.size {
            let mut step: *mut QueryStep =
                &mut *((*self_0).steps.contents).offset(i as isize) as *mut QueryStep;
            query_step__remove_capture(step, id as uint16_t);
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_disable_pattern(
    mut self_0: *mut TSQuery,
    mut pattern_index: uint32_t,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).pattern_map.size {
        let mut pattern: *mut PatternEntry =
            &mut *((*self_0).pattern_map.contents).offset(i as isize) as *mut PatternEntry;
        if (*pattern).pattern_index as libc::c_uint == pattern_index {
            array__erase(
                &mut (*self_0).pattern_map as *mut C2RustUnnamed_9 as *mut VoidArray,
                ::std::mem::size_of::<PatternEntry>() as libc::c_ulong,
                i,
            );
            i = i.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_new() -> *mut TSQueryCursor {
    let mut self_0: *mut TSQueryCursor = crate::core::alloc::ts_malloc(::std::mem::size_of::<
        TSQueryCursor,
    >() as libc::c_ulong) as *mut TSQueryCursor;
    *self_0 = {
        let mut init = TSQueryCursor {
            query: 0 as *const TSQuery,
            cursor: TSTreeCursor {
                tree: 0 as *const libc::c_void,
                id: 0 as *const libc::c_void,
                context: [0; 2],
            },
            states: {
                let mut init = C2RustUnnamed_16 {
                    contents: 0 as *mut QueryState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            finished_states: {
                let mut init = C2RustUnnamed_15 {
                    contents: 0 as *mut QueryState,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            capture_list_pool: capture_list_pool_new(),
            depth: 0,
            max_start_depth: 4294967295 as libc::c_uint,
            start_byte: 0 as libc::c_int as uint32_t,
            end_byte: 4294967295 as libc::c_uint,
            start_point: {
                let mut init = TSPoint {
                    row: 0 as libc::c_int as uint32_t,
                    column: 0 as libc::c_int as uint32_t,
                };
                init
            },
            end_point: {
                let mut init = TSPoint {
                    row: 4294967295 as libc::c_uint,
                    column: 4294967295 as libc::c_uint,
                };
                init
            },
            next_state_id: 0,
            on_visible_node: false,
            ascending: 0 as libc::c_int != 0,
            halted: 0 as libc::c_int != 0,
            did_exceed_match_limit: 0 as libc::c_int != 0,
        };
        init
    };
    array__reserve(
        &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
        8 as libc::c_int as uint32_t,
    );
    array__reserve(
        &mut (*self_0).finished_states as *mut C2RustUnnamed_15 as *mut VoidArray,
        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
        8 as libc::c_int as uint32_t,
    );
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_delete(mut self_0: *mut TSQueryCursor) {
    array__delete(&mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray);
    array__delete(&mut (*self_0).finished_states as *mut C2RustUnnamed_15 as *mut VoidArray);
    ts_tree_cursor_delete(&mut (*self_0).cursor);
    capture_list_pool_delete(&mut (*self_0).capture_list_pool);
    crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_did_exceed_match_limit(
    mut self_0: *const TSQueryCursor,
) -> bool {
    return (*self_0).did_exceed_match_limit;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_match_limit(mut self_0: *const TSQueryCursor) -> uint32_t {
    return (*self_0).capture_list_pool.max_capture_list_count;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_match_limit(
    mut self_0: *mut TSQueryCursor,
    mut limit: uint32_t,
) {
    (*self_0).capture_list_pool.max_capture_list_count = limit;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_exec(
    mut self_0: *mut TSQueryCursor,
    mut query: *const TSQuery,
    mut node: TSNode,
) {
    if !query.is_null() {
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*query).steps.size {
            let mut step: *mut QueryStep =
                &mut *((*query).steps.contents).offset(i as isize) as *mut QueryStep;
            if !((*step).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int) {
                if !(*step).is_dead_end() {
                    if !(*step).is_pass_through() {
                        (*step).symbol as libc::c_int != WILDCARD_SYMBOL as libc::c_int;
                    }
                }
            }
            (*step).field != 0;
            (*step).alternative_index as libc::c_int != NONE as libc::c_int;
            i = i.wrapping_add(1);
        }
    }
    (*self_0).states.size = 0 as libc::c_int as uint32_t;
    (*self_0).finished_states.size = 0 as libc::c_int as uint32_t;
    ts_tree_cursor_reset(&mut (*self_0).cursor, node);
    capture_list_pool_reset(&mut (*self_0).capture_list_pool);
    (*self_0).on_visible_node = 1 as libc::c_int != 0;
    (*self_0).next_state_id = 0 as libc::c_int as uint32_t;
    (*self_0).depth = 0 as libc::c_int as uint32_t;
    (*self_0).ascending = 0 as libc::c_int != 0;
    (*self_0).halted = 0 as libc::c_int != 0;
    let ref mut fresh90 = (*self_0).query;
    *fresh90 = query;
    (*self_0).did_exceed_match_limit = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_byte_range(
    mut self_0: *mut TSQueryCursor,
    mut start_byte: uint32_t,
    mut end_byte: uint32_t,
) {
    if end_byte == 0 as libc::c_int as libc::c_uint {
        end_byte = 4294967295 as libc::c_uint;
    }
    (*self_0).start_byte = start_byte;
    (*self_0).end_byte = end_byte;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_point_range(
    mut self_0: *mut TSQueryCursor,
    mut start_point: TSPoint,
    mut end_point: TSPoint,
) {
    if end_point.row == 0 as libc::c_int as libc::c_uint
        && end_point.column == 0 as libc::c_int as libc::c_uint
    {
        end_point = {
            let mut init = TSPoint {
                row: 4294967295 as libc::c_uint,
                column: 4294967295 as libc::c_uint,
            };
            init
        };
    }
    (*self_0).start_point = start_point;
    (*self_0).end_point = end_point;
}
unsafe extern "C" fn ts_query_cursor__first_in_progress_capture(
    mut self_0: *mut TSQueryCursor,
    mut state_index: *mut uint32_t,
    mut byte_offset: *mut uint32_t,
    mut pattern_index: *mut uint32_t,
    mut root_pattern_guaranteed: *mut bool,
) -> bool {
    let mut result: bool = 0 as libc::c_int != 0;
    *state_index = 4294967295 as libc::c_uint;
    *byte_offset = 4294967295 as libc::c_uint;
    *pattern_index = 4294967295 as libc::c_uint;
    let mut current_block_10: u64;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).states.size {
        let mut state: *mut QueryState =
            &mut *((*self_0).states.contents).offset(i as isize) as *mut QueryState;
        if !(*state).dead() {
            let mut captures: *const CaptureList = capture_list_pool_get(
                &mut (*self_0).capture_list_pool,
                (*state).capture_list_id as uint16_t,
            );
            if !((*state).consumed_capture_count() as libc::c_uint >= (*captures).size) {
                let mut node: TSNode = (*((*captures).contents)
                    .offset((*state).consumed_capture_count() as isize))
                .node;
                if ts_node_end_byte(node) <= (*self_0).start_byte
                    || point_lte(ts_node_end_point(node), (*self_0).start_point) as libc::c_int != 0
                {
                    (*state).set_consumed_capture_count((*state).consumed_capture_count() + 1);
                    i = i.wrapping_sub(1);
                } else {
                    let mut node_start_byte: uint32_t = ts_node_start_byte(node);
                    if !result
                        || node_start_byte < *byte_offset
                        || node_start_byte == *byte_offset
                            && ((*state).pattern_index as libc::c_uint) < *pattern_index
                    {
                        let mut step: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
                            .offset((*state).step_index as isize)
                            as *mut QueryStep;
                        if !root_pattern_guaranteed.is_null() {
                            *root_pattern_guaranteed = (*step).root_pattern_guaranteed();
                            current_block_10 = 13242334135786603907;
                        } else if (*step).root_pattern_guaranteed() {
                            current_block_10 = 4644295000439058019;
                        } else {
                            current_block_10 = 13242334135786603907;
                        }
                        match current_block_10 {
                            4644295000439058019 => {}
                            _ => {
                                result = 1 as libc::c_int != 0;
                                *state_index = i;
                                *byte_offset = node_start_byte;
                                *pattern_index = (*state).pattern_index as uint32_t;
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor__compare_nodes(
    mut left: TSNode,
    mut right: TSNode,
) -> libc::c_int {
    if left.id != right.id {
        let mut left_start: uint32_t = ts_node_start_byte(left);
        let mut right_start: uint32_t = ts_node_start_byte(right);
        if left_start < right_start {
            return -(1 as libc::c_int);
        }
        if left_start > right_start {
            return 1 as libc::c_int;
        }
        let mut left_node_count: uint32_t = ts_node_end_byte(left);
        let mut right_node_count: uint32_t = ts_node_end_byte(right);
        if left_node_count > right_node_count {
            return -(1 as libc::c_int);
        }
        if left_node_count < right_node_count {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor__compare_captures(
    mut self_0: *mut TSQueryCursor,
    mut left_state: *mut QueryState,
    mut right_state: *mut QueryState,
    mut left_contains_right: *mut bool,
    mut right_contains_left: *mut bool,
) {
    let mut left_captures: *const CaptureList = capture_list_pool_get(
        &mut (*self_0).capture_list_pool,
        (*left_state).capture_list_id as uint16_t,
    );
    let mut right_captures: *const CaptureList = capture_list_pool_get(
        &mut (*self_0).capture_list_pool,
        (*right_state).capture_list_id as uint16_t,
    );
    *left_contains_right = 1 as libc::c_int != 0;
    *right_contains_left = 1 as libc::c_int != 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        if i < (*left_captures).size {
            if j < (*right_captures).size {
                let mut left: *mut TSQueryCapture =
                    &mut *((*left_captures).contents).offset(i as isize) as *mut TSQueryCapture;
                let mut right: *mut TSQueryCapture =
                    &mut *((*right_captures).contents).offset(j as isize) as *mut TSQueryCapture;
                if (*left).node.id == (*right).node.id && (*left).index == (*right).index {
                    i = i.wrapping_add(1);
                    j = j.wrapping_add(1);
                } else {
                    match ts_query_cursor__compare_nodes((*left).node, (*right).node) {
                        -1 => {
                            *right_contains_left = 0 as libc::c_int != 0;
                            i = i.wrapping_add(1);
                        }
                        1 => {
                            *left_contains_right = 0 as libc::c_int != 0;
                            j = j.wrapping_add(1);
                        }
                        _ => {
                            *right_contains_left = 0 as libc::c_int != 0;
                            *left_contains_right = 0 as libc::c_int != 0;
                            i = i.wrapping_add(1);
                            j = j.wrapping_add(1);
                        }
                    }
                }
            } else {
                *right_contains_left = 0 as libc::c_int != 0;
                break;
            }
        } else {
            if j < (*right_captures).size {
                *left_contains_right = 0 as libc::c_int != 0;
            }
            break;
        }
    }
}
unsafe extern "C" fn ts_query_cursor__add_state(
    mut self_0: *mut TSQueryCursor,
    mut pattern: *const PatternEntry,
) {
    let mut step: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
        .offset((*pattern).step_index as isize)
        as *mut QueryStep;
    let mut start_depth: uint32_t = ((*self_0).depth).wrapping_sub((*step).depth as libc::c_uint);
    let mut index: uint32_t = (*self_0).states.size;
    while index > 0 as libc::c_int as libc::c_uint {
        let mut prev_state: *mut QueryState = &mut *((*self_0).states.contents)
            .offset(index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
            as *mut QueryState;
        if ((*prev_state).start_depth as libc::c_uint) < start_depth {
            break;
        }
        if (*prev_state).start_depth as libc::c_uint == start_depth {
            if (*prev_state).pattern_index as libc::c_int == (*pattern).pattern_index as libc::c_int
                && (*prev_state).step_index as libc::c_int == (*pattern).step_index as libc::c_int
            {
                return;
            }
            if (*prev_state).pattern_index as libc::c_int <= (*pattern).pattern_index as libc::c_int
            {
                break;
            }
        }
        index = index.wrapping_sub(1);
    }
    array__splice(
        &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
        index,
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        &mut {
            let mut init = QueryState { consumed_capture_count_seeking_immediate_match_has_in_progress_alternatives_dead_needs_parent : [0 ; 2] , id : 4294967295 as libc :: c_uint , capture_list_id : NONE as uint32_t , start_depth : start_depth as uint16_t , step_index : (* pattern) . step_index , pattern_index : (* pattern) . pattern_index , } ;
            init.set_consumed_capture_count(0 as libc::c_int as uint16_t);
            init.set_seeking_immediate_match(1 as libc::c_int != 0);
            init.set_has_in_progress_alternatives(0 as libc::c_int != 0);
            init.set_dead(0 as libc::c_int != 0);
            init.set_needs_parent((*step).depth as libc::c_int == 1 as libc::c_int);
            init
        } as *mut QueryState as *const libc::c_void,
    );
}
unsafe extern "C" fn ts_query_cursor__prepare_to_capture(
    mut self_0: *mut TSQueryCursor,
    mut state: *mut QueryState,
    mut state_index_to_preserve: libc::c_uint,
) -> *mut CaptureList {
    if (*state).capture_list_id == NONE as libc::c_uint {
        (*state).capture_list_id =
            capture_list_pool_acquire(&mut (*self_0).capture_list_pool) as uint32_t;
        if (*state).capture_list_id == NONE as libc::c_uint {
            (*self_0).did_exceed_match_limit = 1 as libc::c_int != 0;
            let mut state_index: uint32_t = 0;
            let mut byte_offset: uint32_t = 0;
            let mut pattern_index: uint32_t = 0;
            if ts_query_cursor__first_in_progress_capture(
                self_0,
                &mut state_index,
                &mut byte_offset,
                &mut pattern_index,
                0 as *mut bool,
            ) as libc::c_int
                != 0
                && state_index != state_index_to_preserve
            {
                let mut other_state: *mut QueryState = &mut *((*self_0).states.contents)
                    .offset(state_index as isize)
                    as *mut QueryState;
                (*state).capture_list_id = (*other_state).capture_list_id;
                (*other_state).capture_list_id = NONE as uint32_t;
                (*other_state).set_dead(1 as libc::c_int != 0);
                let mut list: *mut CaptureList = capture_list_pool_get_mut(
                    &mut (*self_0).capture_list_pool,
                    (*state).capture_list_id as uint16_t,
                );
                (*list).size = 0 as libc::c_int as uint32_t;
                return list;
            } else {
                return 0 as *mut CaptureList;
            }
        }
    }
    return capture_list_pool_get_mut(
        &mut (*self_0).capture_list_pool,
        (*state).capture_list_id as uint16_t,
    );
}
unsafe extern "C" fn ts_query_cursor__capture(
    mut self_0: *mut TSQueryCursor,
    mut state: *mut QueryState,
    mut step: *mut QueryStep,
    mut node: TSNode,
) {
    if (*state).dead() {
        return;
    }
    let mut capture_list: *mut CaptureList =
        ts_query_cursor__prepare_to_capture(self_0, state, 4294967295 as libc::c_uint);
    if capture_list.is_null() {
        (*state).set_dead(1 as libc::c_int != 0);
        return;
    }
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while j < 3 as libc::c_int as libc::c_uint {
        let mut capture_id: uint16_t = (*step).capture_ids[j as usize];
        if (*step).capture_ids[j as usize] as libc::c_int == NONE as libc::c_int {
            break;
        }
        array__grow(
            capture_list as *mut VoidArray,
            1 as libc::c_int as uint32_t,
            ::std::mem::size_of::<TSQueryCapture>() as libc::c_ulong,
        );
        let ref mut fresh91 = (*capture_list).size;
        let fresh92 = *fresh91;
        *fresh91 = (*fresh91).wrapping_add(1);
        *((*capture_list).contents).offset(fresh92 as isize) = {
            let mut init = TSQueryCapture {
                node: node,
                index: capture_id as uint32_t,
            };
            init
        };
        j = j.wrapping_add(1);
    }
}
unsafe extern "C" fn ts_query_cursor__copy_state(
    mut self_0: *mut TSQueryCursor,
    mut state_ref: *mut *mut QueryState,
) -> *mut QueryState {
    let mut state: *const QueryState = *state_ref;
    let mut state_index: uint32_t =
        state.offset_from((*self_0).states.contents) as libc::c_long as uint32_t;
    let mut copy: QueryState = *state;
    copy.capture_list_id = NONE as uint32_t;
    if (*state).capture_list_id != NONE as libc::c_uint {
        let mut new_captures: *mut CaptureList =
            ts_query_cursor__prepare_to_capture(self_0, &mut copy, state_index);
        if new_captures.is_null() {
            return 0 as *mut QueryState;
        }
        let mut old_captures: *const CaptureList = capture_list_pool_get(
            &mut (*self_0).capture_list_pool,
            (*state).capture_list_id as uint16_t,
        );
        array__splice(
            new_captures as *mut VoidArray,
            ::std::mem::size_of::<TSQueryCapture>() as libc::c_ulong,
            (*new_captures).size,
            0 as libc::c_int as uint32_t,
            (*old_captures).size,
            (*old_captures).contents as *const libc::c_void,
        );
    }
    array__splice(
        &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
        state_index.wrapping_add(1 as libc::c_int as libc::c_uint),
        0 as libc::c_int as uint32_t,
        1 as libc::c_int as uint32_t,
        &mut copy as *mut QueryState as *const libc::c_void,
    );
    *state_ref = &mut *((*self_0).states.contents).offset(state_index as isize) as *mut QueryState;
    return &mut *((*self_0).states.contents)
        .offset(state_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
        as *mut QueryState;
}
#[inline]
unsafe extern "C" fn ts_query_cursor__should_descend(
    mut self_0: *mut TSQueryCursor,
    mut node_intersects_range: bool,
) -> bool {
    if node_intersects_range as libc::c_int != 0 && (*self_0).depth < (*self_0).max_start_depth {
        return 1 as libc::c_int != 0;
    }
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).states.size {
        let mut state: *mut QueryState =
            &mut *((*self_0).states.contents).offset(i as isize) as *mut QueryState;
        let mut next_step: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
            .offset((*state).step_index as isize)
            as *mut QueryStep;
        if (*next_step).depth as libc::c_int != PATTERN_DONE_MARKER as libc::c_int
            && ((*state).start_depth as libc::c_int + (*next_step).depth as libc::c_int)
                as libc::c_uint
                > (*self_0).depth
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_add(1);
    }
    if (*self_0).depth >= (*self_0).max_start_depth {
        return 0 as libc::c_int != 0;
    }
    if !(*self_0).on_visible_node {
        let mut subtree: Subtree = ts_tree_cursor_current_subtree(&mut (*self_0).cursor);
        if ts_subtree_is_repetition(subtree) != 0 {
            let mut exists: bool = false;
            let mut index: uint32_t = 0;
            index = 0 as libc::c_int as uint32_t;
            exists = 0 as libc::c_int != 0;
            let mut size: uint32_t = ((*(*self_0).query)
                .repeat_symbols_with_rootless_patterns
                .size)
                .wrapping_sub(index);
            if !(size == 0 as libc::c_int as libc::c_uint) {
                let mut comparison: libc::c_int = 0;
                while size > 1 as libc::c_int as libc::c_uint {
                    let mut half_size: uint32_t =
                        size.wrapping_div(2 as libc::c_int as libc::c_uint);
                    let mut mid_index: uint32_t = index.wrapping_add(half_size);
                    comparison = *((*(*self_0).query)
                        .repeat_symbols_with_rootless_patterns
                        .contents)
                        .offset(mid_index as isize) as libc::c_int
                        - ts_subtree_symbol(subtree) as libc::c_int;
                    if comparison <= 0 as libc::c_int {
                        index = mid_index;
                    }
                    size = (size as libc::c_uint).wrapping_sub(half_size) as uint32_t as uint32_t;
                }
                comparison = *((*(*self_0).query)
                    .repeat_symbols_with_rootless_patterns
                    .contents)
                    .offset(index as isize) as libc::c_int
                    - ts_subtree_symbol(subtree) as libc::c_int;
                if comparison == 0 as libc::c_int {
                    exists = 1 as libc::c_int != 0;
                } else if comparison < 0 as libc::c_int {
                    index = (index as libc::c_uint).wrapping_add(1 as libc::c_int as libc::c_uint)
                        as uint32_t as uint32_t;
                }
            }
            return exists;
        }
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_query_cursor__advance(
    mut self_0: *mut TSQueryCursor,
    mut stop_on_definite_step: bool,
) -> bool {
    let mut did_match: bool = 0 as libc::c_int != 0;
    let mut current_block_185: u64;
    loop {
        if (*self_0).halted {
            while (*self_0).states.size > 0 as libc::c_int as libc::c_uint {
                let ref mut fresh93 = (*self_0).states.size;
                *fresh93 = (*fresh93).wrapping_sub(1);
                let mut state: QueryState = *((*self_0).states.contents).offset(*fresh93 as isize);
                capture_list_pool_release(
                    &mut (*self_0).capture_list_pool,
                    state.capture_list_id as uint16_t,
                );
            }
        }
        if did_match as libc::c_int != 0 || (*self_0).halted as libc::c_int != 0 {
            return did_match;
        }
        if (*self_0).ascending {
            if (*self_0).on_visible_node {
                let mut deleted_count: uint32_t = 0 as libc::c_int as uint32_t;
                let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut n: libc::c_uint = (*self_0).states.size;
                while i < n {
                    let mut state_0: *mut QueryState =
                        &mut *((*self_0).states.contents).offset(i as isize) as *mut QueryState;
                    let mut step: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
                        .offset((*state_0).step_index as isize)
                        as *mut QueryStep;
                    if (*step).depth as libc::c_int == PATTERN_DONE_MARKER as libc::c_int
                        && ((*state_0).start_depth as libc::c_uint > (*self_0).depth
                            || (*self_0).depth == 0 as libc::c_int as libc::c_uint)
                    {
                        array__grow(
                            &mut (*self_0).finished_states as *mut C2RustUnnamed_15
                                as *mut VoidArray,
                            1 as libc::c_int as uint32_t,
                            ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                        );
                        let ref mut fresh94 = (*self_0).finished_states.size;
                        let fresh95 = *fresh94;
                        *fresh94 = (*fresh94).wrapping_add(1);
                        *((*self_0).finished_states.contents).offset(fresh95 as isize) = *state_0;
                        did_match = 1 as libc::c_int != 0;
                        deleted_count = deleted_count.wrapping_add(1);
                    } else if (*step).depth as libc::c_int != PATTERN_DONE_MARKER as libc::c_int
                        && ((*state_0).start_depth as uint32_t)
                            .wrapping_add((*step).depth as uint32_t)
                            > (*self_0).depth
                    {
                        capture_list_pool_release(
                            &mut (*self_0).capture_list_pool,
                            (*state_0).capture_list_id as uint16_t,
                        );
                        deleted_count = deleted_count.wrapping_add(1);
                    } else if deleted_count > 0 as libc::c_int as libc::c_uint {
                        *((*self_0).states.contents)
                            .offset(i.wrapping_sub(deleted_count) as isize) = *state_0;
                    }
                    i = i.wrapping_add(1);
                }
                let ref mut fresh96 = (*self_0).states.size;
                *fresh96 =
                    (*fresh96 as libc::c_uint).wrapping_sub(deleted_count) as uint32_t as uint32_t;
            }
            match ts_tree_cursor_goto_next_sibling_internal(&mut (*self_0).cursor) as libc::c_uint {
                2 => {
                    if !(*self_0).on_visible_node {
                        let ref mut fresh97 = (*self_0).depth;
                        *fresh97 = (*fresh97).wrapping_add(1);
                        (*self_0).on_visible_node = 1 as libc::c_int != 0;
                    }
                    (*self_0).ascending = 0 as libc::c_int != 0;
                }
                1 => {
                    if (*self_0).on_visible_node {
                        let ref mut fresh98 = (*self_0).depth;
                        *fresh98 = (*fresh98).wrapping_sub(1);
                        (*self_0).on_visible_node = 0 as libc::c_int != 0;
                    }
                    (*self_0).ascending = 0 as libc::c_int != 0;
                }
                _ => {
                    if ts_tree_cursor_goto_parent(&mut (*self_0).cursor) {
                        let ref mut fresh99 = (*self_0).depth;
                        *fresh99 = (*fresh99).wrapping_sub(1);
                    } else {
                        (*self_0).halted = 1 as libc::c_int != 0;
                    }
                }
            }
        } else {
            let mut node: TSNode = ts_tree_cursor_current_node(&mut (*self_0).cursor);
            let mut parent_node: TSNode = ts_tree_cursor_parent_node(&mut (*self_0).cursor);
            let mut parent_precedes_range: bool = !ts_node_is_null(parent_node)
                && (ts_node_end_byte(parent_node) <= (*self_0).start_byte
                    || point_lte(ts_node_end_point(parent_node), (*self_0).start_point)
                        as libc::c_int
                        != 0);
            let mut parent_follows_range: bool = !ts_node_is_null(parent_node)
                && (ts_node_start_byte(parent_node) >= (*self_0).end_byte
                    || point_gte(ts_node_start_point(parent_node), (*self_0).end_point)
                        as libc::c_int
                        != 0);
            let mut node_precedes_range: bool = parent_precedes_range as libc::c_int != 0
                || (ts_node_end_byte(node) <= (*self_0).start_byte
                    || point_lte(ts_node_end_point(node), (*self_0).start_point) as libc::c_int
                        != 0);
            let mut node_follows_range: bool = parent_follows_range as libc::c_int != 0
                || (ts_node_start_byte(node) >= (*self_0).end_byte
                    || point_gte(ts_node_start_point(node), (*self_0).end_point) as libc::c_int
                        != 0);
            let mut parent_intersects_range: bool = !parent_precedes_range && !parent_follows_range;
            let mut node_intersects_range: bool = !node_precedes_range && !node_follows_range;
            if (*self_0).on_visible_node {
                let mut symbol: TSSymbol = ts_node_symbol(node);
                let mut is_named: bool = ts_node_is_named(node);
                let mut has_later_siblings: bool = false;
                let mut has_later_named_siblings: bool = false;
                let mut can_have_later_siblings_with_this_field: bool = false;
                let mut field_id: TSFieldId = 0 as libc::c_int as TSFieldId;
                let mut supertypes: [TSSymbol; 8] =
                    [0 as libc::c_int as TSSymbol, 0, 0, 0, 0, 0, 0, 0];
                let mut supertype_count: libc::c_uint = 8 as libc::c_int as libc::c_uint;
                ts_tree_cursor_current_status(
                    &mut (*self_0).cursor,
                    &mut field_id,
                    &mut has_later_siblings,
                    &mut has_later_named_siblings,
                    &mut can_have_later_siblings_with_this_field,
                    supertypes.as_mut_ptr(),
                    &mut supertype_count,
                );
                let mut node_is_error: bool =
                    symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int;
                let mut parent_is_error: bool = !ts_node_is_null(parent_node)
                    && ts_node_symbol(parent_node) as libc::c_int
                        == -(1 as libc::c_int) as TSSymbol as libc::c_int;
                if !node_is_error {
                    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    while i_0 < (*(*self_0).query).wildcard_root_pattern_count as libc::c_uint {
                        let mut pattern: *mut PatternEntry =
                            &mut *((*(*self_0).query).pattern_map.contents).offset(i_0 as isize)
                                as *mut PatternEntry;
                        let mut step_0: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
                            .offset((*pattern).step_index as isize)
                            as *mut QueryStep;
                        let mut start_depth: uint32_t =
                            ((*self_0).depth).wrapping_sub((*step_0).depth as libc::c_uint);
                        if (if (*pattern).is_rooted as libc::c_int != 0 {
                            node_intersects_range as libc::c_int
                        } else {
                            (parent_intersects_range as libc::c_int != 0 && !parent_is_error)
                                as libc::c_int
                        }) != 0
                            && ((*step_0).field == 0
                                || field_id as libc::c_int == (*step_0).field as libc::c_int)
                            && ((*step_0).supertype_symbol == 0
                                || supertype_count > 0 as libc::c_int as libc::c_uint)
                            && start_depth <= (*self_0).max_start_depth
                        {
                            ts_query_cursor__add_state(self_0, pattern);
                        }
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                let mut i_1: libc::c_uint = 0;
                if ts_query__pattern_map_search((*self_0).query, symbol, &mut i_1) {
                    let mut pattern_0: *mut PatternEntry =
                        &mut *((*(*self_0).query).pattern_map.contents).offset(i_1 as isize)
                            as *mut PatternEntry;
                    let mut step_1: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
                        .offset((*pattern_0).step_index as isize)
                        as *mut QueryStep;
                    let mut start_depth_0: uint32_t =
                        ((*self_0).depth).wrapping_sub((*step_1).depth as libc::c_uint);
                    loop {
                        if (if (*pattern_0).is_rooted as libc::c_int != 0 {
                            node_intersects_range as libc::c_int
                        } else {
                            (parent_intersects_range as libc::c_int != 0 && !parent_is_error)
                                as libc::c_int
                        }) != 0
                            && ((*step_1).field == 0
                                || field_id as libc::c_int == (*step_1).field as libc::c_int)
                            && start_depth_0 <= (*self_0).max_start_depth
                        {
                            ts_query_cursor__add_state(self_0, pattern_0);
                        }
                        i_1 = i_1.wrapping_add(1);
                        if i_1 == (*(*self_0).query).pattern_map.size {
                            break;
                        }
                        pattern_0 = &mut *((*(*self_0).query).pattern_map.contents)
                            .offset(i_1 as isize)
                            as *mut PatternEntry;
                        step_1 = &mut *((*(*self_0).query).steps.contents)
                            .offset((*pattern_0).step_index as isize)
                            as *mut QueryStep;
                        if !((*step_1).symbol as libc::c_int == symbol as libc::c_int) {
                            break;
                        }
                    }
                }
                let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                let mut copy_count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while j < (*self_0).states.size {
                    let mut state_1: *mut QueryState =
                        &mut *((*self_0).states.contents).offset(j as isize) as *mut QueryState;
                    let mut step_2: *mut QueryStep = &mut *((*(*self_0).query).steps.contents)
                        .offset((*state_1).step_index as isize)
                        as *mut QueryStep;
                    (*state_1).set_has_in_progress_alternatives(0 as libc::c_int != 0);
                    copy_count = 0 as libc::c_int as libc::c_uint;
                    if !(((*state_1).start_depth as uint32_t)
                        .wrapping_add((*step_2).depth as uint32_t)
                        != (*self_0).depth)
                    {
                        let mut node_does_match: bool = 0 as libc::c_int != 0;
                        if (*step_2).symbol as libc::c_int == WILDCARD_SYMBOL as libc::c_int {
                            node_does_match = !node_is_error
                                && (is_named as libc::c_int != 0 || !(*step_2).is_named());
                        } else {
                            node_does_match =
                                symbol as libc::c_int == (*step_2).symbol as libc::c_int;
                        }
                        let mut later_sibling_can_match: bool = has_later_siblings;
                        if (*step_2).is_immediate() as libc::c_int != 0
                            && is_named as libc::c_int != 0
                            || (*state_1).seeking_immediate_match() as libc::c_int != 0
                        {
                            later_sibling_can_match = 0 as libc::c_int != 0;
                        }
                        if (*step_2).is_last_child() as libc::c_int != 0
                            && has_later_named_siblings as libc::c_int != 0
                        {
                            node_does_match = 0 as libc::c_int != 0;
                        }
                        if (*step_2).supertype_symbol != 0 {
                            let mut has_supertype: bool = 0 as libc::c_int != 0;
                            let mut k: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                            while k < supertype_count {
                                if supertypes[k as usize] as libc::c_int
                                    == (*step_2).supertype_symbol as libc::c_int
                                {
                                    has_supertype = 1 as libc::c_int != 0;
                                    break;
                                } else {
                                    k = k.wrapping_add(1);
                                }
                            }
                            if !has_supertype {
                                node_does_match = 0 as libc::c_int != 0;
                            }
                        }
                        if (*step_2).field != 0 {
                            if (*step_2).field as libc::c_int == field_id as libc::c_int {
                                if !can_have_later_siblings_with_this_field {
                                    later_sibling_can_match = 0 as libc::c_int != 0;
                                }
                            } else {
                                node_does_match = 0 as libc::c_int != 0;
                            }
                        }
                        if (*step_2).negated_field_list_id != 0 {
                            let mut negated_field_ids: *mut TSFieldId =
                                &mut *((*(*self_0).query).negated_fields.contents)
                                    .offset((*step_2).negated_field_list_id as isize)
                                    as *mut TSFieldId;
                            loop {
                                let mut negated_field_id: TSFieldId = *negated_field_ids;
                                if !(negated_field_id != 0) {
                                    break;
                                }
                                negated_field_ids = negated_field_ids.offset(1);
                                if ((ts_node_child_by_field_id(node, negated_field_id)).id)
                                    .is_null()
                                {
                                    continue;
                                }
                                node_does_match = 0 as libc::c_int != 0;
                                break;
                            }
                        }
                        if !node_does_match {
                            if !later_sibling_can_match {
                                capture_list_pool_release(
                                    &mut (*self_0).capture_list_pool,
                                    (*state_1).capture_list_id as uint16_t,
                                );
                                array__erase(
                                    &mut (*self_0).states as *mut C2RustUnnamed_16
                                        as *mut VoidArray,
                                    ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                                    j,
                                );
                                j = j.wrapping_sub(1);
                            }
                        } else {
                            if later_sibling_can_match as libc::c_int != 0
                                && ((*step_2).contains_captures() as libc::c_int != 0
                                    || ts_query__step_is_fallible(
                                        (*self_0).query,
                                        (*state_1).step_index,
                                    ) as libc::c_int
                                        != 0)
                            {
                                if !(ts_query_cursor__copy_state(self_0, &mut state_1)).is_null() {
                                    copy_count = copy_count.wrapping_add(1);
                                }
                            }
                            if (*state_1).needs_parent() {
                                let mut parent: TSNode =
                                    ts_tree_cursor_parent_node(&mut (*self_0).cursor);
                                if ts_node_is_null(parent) {
                                    (*state_1).set_dead(1 as libc::c_int != 0);
                                } else {
                                    (*state_1).set_needs_parent(0 as libc::c_int != 0);
                                    let mut skipped_wildcard_step: *mut QueryStep = step_2;
                                    loop {
                                        skipped_wildcard_step = skipped_wildcard_step.offset(-1);
                                        if !((*skipped_wildcard_step).is_dead_end() as libc::c_int
                                            != 0
                                            || (*skipped_wildcard_step).is_pass_through()
                                                as libc::c_int
                                                != 0
                                            || (*skipped_wildcard_step).depth as libc::c_int
                                                > 0 as libc::c_int)
                                        {
                                            break;
                                        }
                                    }
                                    if (*skipped_wildcard_step).capture_ids
                                        [0 as libc::c_int as usize]
                                        as libc::c_int
                                        != NONE as libc::c_int
                                    {
                                        ts_query_cursor__capture(
                                            self_0,
                                            state_1,
                                            skipped_wildcard_step,
                                            parent,
                                        );
                                    }
                                }
                            }
                            if (*step_2).capture_ids[0 as libc::c_int as usize] as libc::c_int
                                != NONE as libc::c_int
                            {
                                ts_query_cursor__capture(self_0, state_1, step_2, node);
                            }
                            if (*state_1).dead() {
                                array__erase(
                                    &mut (*self_0).states as *mut C2RustUnnamed_16
                                        as *mut VoidArray,
                                    ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                                    j,
                                );
                                j = j.wrapping_sub(1);
                            } else {
                                let ref mut fresh100 = (*state_1).step_index;
                                *fresh100 = (*fresh100).wrapping_add(1);
                                (*state_1).set_seeking_immediate_match(0 as libc::c_int != 0);
                                let mut next_step: *mut QueryStep =
                                    &mut *((*(*self_0).query).steps.contents)
                                        .offset((*state_1).step_index as isize)
                                        as *mut QueryStep;
                                if stop_on_definite_step as libc::c_int != 0
                                    && (*next_step).root_pattern_guaranteed() as libc::c_int != 0
                                {
                                    did_match = 1 as libc::c_int != 0;
                                }
                                let mut end_index: libc::c_uint =
                                    j.wrapping_add(1 as libc::c_int as libc::c_uint);
                                let mut k_0: libc::c_uint = j;
                                while k_0 < end_index {
                                    let mut child_state: *mut QueryState =
                                        &mut *((*self_0).states.contents).offset(k_0 as isize)
                                            as *mut QueryState;
                                    let mut child_step: *mut QueryStep =
                                        &mut *((*(*self_0).query).steps.contents)
                                            .offset((*child_state).step_index as isize)
                                            as *mut QueryStep;
                                    if (*child_step).alternative_index as libc::c_int
                                        != NONE as libc::c_int
                                    {
                                        if (*child_step).is_dead_end() {
                                            (*child_state).step_index =
                                                (*child_step).alternative_index;
                                            k_0 = k_0.wrapping_sub(1);
                                        } else {
                                            if (*child_step).is_pass_through() {
                                                let ref mut fresh101 = (*child_state).step_index;
                                                *fresh101 = (*fresh101).wrapping_add(1);
                                                k_0 = k_0.wrapping_sub(1);
                                            }
                                            let mut copy: *mut QueryState =
                                                ts_query_cursor__copy_state(
                                                    self_0,
                                                    &mut child_state,
                                                );
                                            if !copy.is_null() {
                                                end_index = end_index.wrapping_add(1);
                                                copy_count = copy_count.wrapping_add(1);
                                                (*copy).step_index =
                                                    (*child_step).alternative_index;
                                                if (*child_step).alternative_is_immediate() {
                                                    (*copy).set_seeking_immediate_match(
                                                        1 as libc::c_int != 0,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    k_0 = k_0.wrapping_add(1);
                                }
                            }
                        }
                    }
                    j = j.wrapping_add((1 as libc::c_int as libc::c_uint).wrapping_add(copy_count));
                }
                let mut j_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                while j_0 < (*self_0).states.size {
                    let mut state_2: *mut QueryState =
                        &mut *((*self_0).states.contents).offset(j_0 as isize) as *mut QueryState;
                    if (*state_2).dead() {
                        array__erase(
                            &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
                            ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                            j_0,
                        );
                        j_0 = j_0.wrapping_sub(1);
                    } else {
                        let mut did_remove: bool = 0 as libc::c_int != 0;
                        let mut current_block_163: u64;
                        let mut k_1: libc::c_uint =
                            j_0.wrapping_add(1 as libc::c_int as libc::c_uint);
                        while k_1 < (*self_0).states.size {
                            let mut other_state: *mut QueryState = &mut *((*self_0).states.contents)
                                .offset(k_1 as isize)
                                as *mut QueryState;
                            if (*other_state).start_depth as libc::c_int
                                != (*state_2).start_depth as libc::c_int
                                || (*other_state).pattern_index as libc::c_int
                                    != (*state_2).pattern_index as libc::c_int
                            {
                                break;
                            }
                            let mut left_contains_right: bool = false;
                            let mut right_contains_left: bool = false;
                            ts_query_cursor__compare_captures(
                                self_0,
                                state_2,
                                other_state,
                                &mut left_contains_right,
                                &mut right_contains_left,
                            );
                            if left_contains_right {
                                if (*state_2).step_index as libc::c_int
                                    == (*other_state).step_index as libc::c_int
                                {
                                    capture_list_pool_release(
                                        &mut (*self_0).capture_list_pool,
                                        (*other_state).capture_list_id as uint16_t,
                                    );
                                    array__erase(
                                        &mut (*self_0).states as *mut C2RustUnnamed_16
                                            as *mut VoidArray,
                                        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                                        k_1,
                                    );
                                    k_1 = k_1.wrapping_sub(1);
                                    current_block_163 = 8190651249203672769;
                                } else {
                                    (*other_state)
                                        .set_has_in_progress_alternatives(1 as libc::c_int != 0);
                                    current_block_163 = 10730478470646396074;
                                }
                            } else {
                                current_block_163 = 10730478470646396074;
                            }
                            match current_block_163 {
                                10730478470646396074 => {
                                    if right_contains_left {
                                        if (*state_2).step_index as libc::c_int
                                            == (*other_state).step_index as libc::c_int
                                        {
                                            capture_list_pool_release(
                                                &mut (*self_0).capture_list_pool,
                                                (*state_2).capture_list_id as uint16_t,
                                            );
                                            array__erase(
                                                &mut (*self_0).states as *mut C2RustUnnamed_16
                                                    as *mut VoidArray,
                                                ::std::mem::size_of::<QueryState>()
                                                    as libc::c_ulong,
                                                j_0,
                                            );
                                            j_0 = j_0.wrapping_sub(1);
                                            did_remove = 1 as libc::c_int != 0;
                                            break;
                                        } else {
                                            (*state_2).set_has_in_progress_alternatives(
                                                1 as libc::c_int != 0,
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                            k_1 = k_1.wrapping_add(1);
                        }
                        if !did_remove {
                            let mut next_step_0: *mut QueryStep =
                                &mut *((*(*self_0).query).steps.contents)
                                    .offset((*state_2).step_index as isize)
                                    as *mut QueryStep;
                            if (*next_step_0).depth as libc::c_int
                                == PATTERN_DONE_MARKER as libc::c_int
                            {
                                if !(*state_2).has_in_progress_alternatives() {
                                    array__grow(
                                        &mut (*self_0).finished_states as *mut C2RustUnnamed_15
                                            as *mut VoidArray,
                                        1 as libc::c_int as uint32_t,
                                        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                                    );
                                    let ref mut fresh102 = (*self_0).finished_states.size;
                                    let fresh103 = *fresh102;
                                    *fresh102 = (*fresh102).wrapping_add(1);
                                    *((*self_0).finished_states.contents)
                                        .offset(fresh103 as isize) = *state_2;
                                    array__erase(
                                        &mut (*self_0).states as *mut C2RustUnnamed_16
                                            as *mut VoidArray,
                                        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                                        state_2.offset_from((*self_0).states.contents)
                                            as libc::c_long
                                            as uint32_t,
                                    );
                                    did_match = 1 as libc::c_int != 0;
                                    j_0 = j_0.wrapping_sub(1);
                                }
                            }
                        }
                    }
                    j_0 = j_0.wrapping_add(1);
                }
            }
            if ts_query_cursor__should_descend(self_0, node_intersects_range) {
                match ts_tree_cursor_goto_first_child_internal(&mut (*self_0).cursor)
                    as libc::c_uint
                {
                    2 => {
                        current_block_185 = 1788859052630180541;
                        match current_block_185 {
                            2368509847378411332 => {
                                (*self_0).on_visible_node = 0 as libc::c_int != 0;
                                continue;
                            }
                            _ => {
                                let ref mut fresh104 = (*self_0).depth;
                                *fresh104 = (*fresh104).wrapping_add(1);
                                (*self_0).on_visible_node = 1 as libc::c_int != 0;
                                continue;
                            }
                        }
                    }
                    1 => {
                        current_block_185 = 2368509847378411332;
                        match current_block_185 {
                            2368509847378411332 => {
                                (*self_0).on_visible_node = 0 as libc::c_int != 0;
                                continue;
                            }
                            _ => {
                                let ref mut fresh104 = (*self_0).depth;
                                *fresh104 = (*fresh104).wrapping_add(1);
                                (*self_0).on_visible_node = 1 as libc::c_int != 0;
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            (*self_0).ascending = 1 as libc::c_int != 0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_next_match(
    mut self_0: *mut TSQueryCursor,
    mut match_0: *mut TSQueryMatch,
) -> bool {
    if (*self_0).finished_states.size == 0 as libc::c_int as libc::c_uint {
        if !ts_query_cursor__advance(self_0, 0 as libc::c_int != 0) {
            return 0 as libc::c_int != 0;
        }
    }
    let mut state: *mut QueryState = &mut *((*self_0).finished_states.contents)
        .offset(0 as libc::c_int as isize) as *mut QueryState;
    if (*state).id == 4294967295 as libc::c_uint {
        let ref mut fresh105 = (*self_0).next_state_id;
        let fresh106 = *fresh105;
        *fresh105 = (*fresh105).wrapping_add(1);
        (*state).id = fresh106;
    }
    (*match_0).id = (*state).id;
    (*match_0).pattern_index = (*state).pattern_index;
    let mut captures: *const CaptureList = capture_list_pool_get(
        &mut (*self_0).capture_list_pool,
        (*state).capture_list_id as uint16_t,
    );
    let ref mut fresh107 = (*match_0).captures;
    *fresh107 = (*captures).contents;
    (*match_0).capture_count = (*captures).size as uint16_t;
    capture_list_pool_release(
        &mut (*self_0).capture_list_pool,
        (*state).capture_list_id as uint16_t,
    );
    array__erase(
        &mut (*self_0).finished_states as *mut C2RustUnnamed_15 as *mut VoidArray,
        ::std::mem::size_of::<QueryState>() as libc::c_ulong,
        0 as libc::c_int as uint32_t,
    );
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_remove_match(
    mut self_0: *mut TSQueryCursor,
    mut match_id: uint32_t,
) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).finished_states.size {
        let mut state: *const QueryState =
            &mut *((*self_0).finished_states.contents).offset(i as isize) as *mut QueryState;
        if (*state).id == match_id {
            capture_list_pool_release(
                &mut (*self_0).capture_list_pool,
                (*state).capture_list_id as uint16_t,
            );
            array__erase(
                &mut (*self_0).finished_states as *mut C2RustUnnamed_15 as *mut VoidArray,
                ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                i,
            );
            return;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i_0 < (*self_0).states.size {
        let mut state_0: *const QueryState =
            &mut *((*self_0).states.contents).offset(i_0 as isize) as *mut QueryState;
        if (*state_0).id == match_id {
            capture_list_pool_release(
                &mut (*self_0).capture_list_pool,
                (*state_0).capture_list_id as uint16_t,
            );
            array__erase(
                &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
                ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                i_0,
            );
            return;
        }
        i_0 = i_0.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_next_capture(
    mut self_0: *mut TSQueryCursor,
    mut match_0: *mut TSQueryMatch,
    mut capture_index: *mut uint32_t,
) -> bool {
    loop {
        let mut first_unfinished_capture_byte: uint32_t = 0;
        let mut first_unfinished_pattern_index: uint32_t = 0;
        let mut first_unfinished_state_index: uint32_t = 0;
        let mut first_unfinished_state_is_definite: bool = 0 as libc::c_int != 0;
        ts_query_cursor__first_in_progress_capture(
            self_0,
            &mut first_unfinished_state_index,
            &mut first_unfinished_capture_byte,
            &mut first_unfinished_pattern_index,
            &mut first_unfinished_state_is_definite,
        );
        let mut first_finished_state: *mut QueryState = 0 as *mut QueryState;
        let mut first_finished_capture_byte: uint32_t = first_unfinished_capture_byte;
        let mut first_finished_pattern_index: uint32_t = first_unfinished_pattern_index;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (*self_0).finished_states.size {
            let mut state: *mut QueryState =
                &mut *((*self_0).finished_states.contents).offset(i as isize) as *mut QueryState;
            let mut captures: *const CaptureList = capture_list_pool_get(
                &mut (*self_0).capture_list_pool,
                (*state).capture_list_id as uint16_t,
            );
            if (*state).consumed_capture_count() as libc::c_uint >= (*captures).size {
                capture_list_pool_release(
                    &mut (*self_0).capture_list_pool,
                    (*state).capture_list_id as uint16_t,
                );
                array__erase(
                    &mut (*self_0).finished_states as *mut C2RustUnnamed_15 as *mut VoidArray,
                    ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                    i,
                );
            } else {
                let mut node: TSNode = (*((*captures).contents)
                    .offset((*state).consumed_capture_count() as isize))
                .node;
                if ts_node_end_byte(node) <= (*self_0).start_byte {
                    (*state).set_consumed_capture_count((*state).consumed_capture_count() + 1);
                } else {
                    let mut node_start_byte: uint32_t = ts_node_start_byte(node);
                    if node_start_byte < first_finished_capture_byte
                        || node_start_byte == first_finished_capture_byte
                            && ((*state).pattern_index as libc::c_uint)
                                < first_finished_pattern_index
                    {
                        first_finished_state = state;
                        first_finished_capture_byte = node_start_byte;
                        first_finished_pattern_index = (*state).pattern_index as uint32_t;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        let mut state_0: *mut QueryState = 0 as *mut QueryState;
        if !first_finished_state.is_null() {
            state_0 = first_finished_state;
        } else if first_unfinished_state_is_definite {
            state_0 = &mut *((*self_0).states.contents)
                .offset(first_unfinished_state_index as isize)
                as *mut QueryState;
        } else {
            state_0 = 0 as *mut QueryState;
        }
        if !state_0.is_null() {
            if (*state_0).id == 4294967295 as libc::c_uint {
                let ref mut fresh108 = (*self_0).next_state_id;
                let fresh109 = *fresh108;
                *fresh108 = (*fresh108).wrapping_add(1);
                (*state_0).id = fresh109;
            }
            (*match_0).id = (*state_0).id;
            (*match_0).pattern_index = (*state_0).pattern_index;
            let mut captures_0: *const CaptureList = capture_list_pool_get(
                &mut (*self_0).capture_list_pool,
                (*state_0).capture_list_id as uint16_t,
            );
            let ref mut fresh110 = (*match_0).captures;
            *fresh110 = (*captures_0).contents;
            (*match_0).capture_count = (*captures_0).size as uint16_t;
            *capture_index = (*state_0).consumed_capture_count() as uint32_t;
            (*state_0).set_consumed_capture_count((*state_0).consumed_capture_count() + 1);
            return 1 as libc::c_int != 0;
        }
        if capture_list_pool_is_empty(&mut (*self_0).capture_list_pool) {
            capture_list_pool_release(
                &mut (*self_0).capture_list_pool,
                (*((*self_0).states.contents).offset(first_unfinished_state_index as isize))
                    .capture_list_id as uint16_t,
            );
            array__erase(
                &mut (*self_0).states as *mut C2RustUnnamed_16 as *mut VoidArray,
                ::std::mem::size_of::<QueryState>() as libc::c_ulong,
                first_unfinished_state_index,
            );
        }
        if !ts_query_cursor__advance(self_0, 1 as libc::c_int != 0)
            && (*self_0).finished_states.size == 0 as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int != 0;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_query_cursor_set_max_start_depth(
    mut self_0: *mut TSQueryCursor,
    mut max_start_depth: uint32_t,
) {
    (*self_0).max_start_depth = max_start_depth;
}
pub const TreeCursorStep_TreeCursorStepHidden: TreeCursorStep = TreeCursorStepHidden;
pub const TreeCursorStep_TreeCursorStepVisible: TreeCursorStep = TreeCursorStepVisible;
pub const TreeCursorStep_TreeCursorStepNone: TreeCursorStep = TreeCursorStepNone;
