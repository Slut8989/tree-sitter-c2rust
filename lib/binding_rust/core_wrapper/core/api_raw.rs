use crate::core::util::*;
use crate::core::*;
use :: c2rust_bitfields;
use std::os;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub struct TSRange {
    pub start_point: TSPoint,
    pub end_point: TSPoint,
    pub start_byte: uint32_t,
    pub end_byte: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSPoint {
    pub row: uint32_t,
    pub column: uint32_t,
}
type C2RustUnnamed_3 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_4 = crate::core::util::LongShortData;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Length {
    pub bytes: uint32_t,
    pub extent: TSPoint,
}
pub type TSDuration = uint64_t;
use crate::core::util::libc::{dup, fclose, fdopen, fputc, fputs, FILE};
pub type _IO_lock_t = ();
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
pub struct StackNode {
    pub state: TSStateId,
    pub position: Length,
    pub links: [StackLink; 8],
    pub link_count: libc::c_ushort,
    pub ref_count: uint32_t,
    pub error_cost: libc::c_uint,
    pub node_count: libc::c_uint,
    pub dynamic_precedence: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackLink {
    pub node: *mut StackNode,
    pub subtree: Subtree,
    pub is_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackNodeArray {
    pub contents: *mut *mut StackNode,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
type C2RustUnnamed_8 = crate::core::util::StackElement<*mut StackIterator>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackIterator {
    pub node: *mut StackNode,
    pub subtrees: SubtreeArray,
    pub subtree_count: uint32_t,
    pub is_pending: bool,
}
pub type StackVersion = libc::c_uint;
type C2RustUnnamed_9 = crate::core::util::StackElement<*mut StackHead>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackHead {
    pub node: *mut StackNode,
    pub summary: *mut StackSummary,
    pub node_count_at_last_error: libc::c_uint,
    pub last_external_token: Subtree,
    pub lookahead_when_paused: Subtree,
    pub status: StackStatus,
}
pub type StackStatus = libc::c_uint;
pub const StackStatusHalted: StackStatus = 2;
pub const StackStatusPaused: StackStatus = 1;
pub const StackStatusActive: StackStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSLogger {
    pub payload: *mut libc::c_void,
    pub log: Option<unsafe extern "C" fn(*mut libc::c_void, TSLogType, *const libc::c_char) -> ()>,
}
pub type TSLogType = libc::c_uint;
pub const TSLogTypeLex: TSLogType = 1;
pub const TSLogTypeParse: TSLogType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSInput {
    pub payload: *mut libc::c_void,
    pub read: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            uint32_t,
            TSPoint,
            *mut uint32_t,
        ) -> *const libc::c_char,
    >,
    pub encoding: TSInputEncoding,
}
pub type TSInputEncoding = libc::c_uint;
pub const TSInputEncodingUTF16: TSInputEncoding = 1;
pub const TSInputEncodingUTF8: TSInputEncoding = 0;
type C2RustUnnamed_10 = crate::core::util::StackElement<*mut TSSymbol>;
type C2RustUnnamed_11 = crate::core::util::StackElement<*mut libc::c_char>;
type C2RustUnnamed_12 = crate::core::util::StackElement<*mut TSFieldId>;
type C2RustUnnamed_13 = crate::core::util::StackElement<*mut StepOffset>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StepOffset {
    pub byte_offset: uint32_t,
    pub step_index: uint16_t,
}
type C2RustUnnamed_14 = crate::core::util::StackElement<*mut QueryPattern>;
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
type C2RustUnnamed_15 = crate::core::util::StackElement<*mut TSQueryPredicateStep>;
pub type TSQueryPredicateStepType = libc::c_uint;
pub const TSQueryPredicateStepTypeString: TSQueryPredicateStepType = 2;
pub const TSQueryPredicateStepTypeCapture: TSQueryPredicateStepType = 1;
pub const TSQueryPredicateStepTypeDone: TSQueryPredicateStepType = 0;
type C2RustUnnamed_16 = crate::core::util::StackElement<*mut PatternEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PatternEntry {
    pub step_index: uint16_t,
    pub pattern_index: uint16_t,
    pub is_rooted: bool,
}
type C2RustUnnamed_17 = crate::core::util::StackElement<*mut QueryStep>;
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
type C2RustUnnamed_18 = crate::core::util::StackElement<*mut CaptureQuantifiers>;
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
    pub characters: C2RustUnnamed_20,
    pub slices: C2RustUnnamed_19,
}
type C2RustUnnamed_19 = crate::core::util::StackElement<*mut Slice>;
type C2RustUnnamed_20 = crate::core::util::StackElement<*mut libc::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CaptureListPool {
    pub list: C2RustUnnamed_21,
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
type C2RustUnnamed_21 = crate::core::util::StackElement<*mut CaptureList>;
type C2RustUnnamed_22 = crate::core::util::StackElement<*mut QueryState>;
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
type C2RustUnnamed_23 = crate::core::util::StackElement<*mut QueryState>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSTreeCursor {
    pub tree: *const libc::c_void,
    pub id: *const libc::c_void,
    pub context: [uint32_t; 2],
}
pub type TSSymbolType = libc::c_uint;
pub const TSSymbolTypeAuxiliary: TSSymbolType = 2;
pub const TSSymbolTypeAnonymous: TSSymbolType = 1;
pub const TSSymbolTypeRegular: TSSymbolType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSInputEdit {
    pub start_byte: uint32_t,
    pub old_end_byte: uint32_t,
    pub new_end_byte: uint32_t,
    pub start_point: TSPoint,
    pub old_end_point: TSPoint,
    pub new_end_point: TSPoint,
}
pub type TSQuantifier = libc::c_uint;
pub const TSQuantifierOneOrMore: TSQuantifier = 4;
pub const TSQuantifierOne: TSQuantifier = 3;
pub const TSQuantifierZeroOrMore: TSQuantifier = 2;
pub const TSQuantifierZeroOrOne: TSQuantifier = 1;
pub const TSQuantifierZero: TSQuantifier = 0;
pub type TSQueryError = libc::c_uint;
pub const TSQueryErrorLanguage: TSQueryError = 6;
pub const TSQueryErrorStructure: TSQueryError = 5;
pub const TSQueryErrorCapture: TSQueryError = 4;
pub const TSQueryErrorField: TSQueryError = 3;
pub const TSQueryErrorNodeType: TSQueryError = 2;
pub const TSQueryErrorSyntax: TSQueryError = 1;
pub const TSQueryErrorNone: TSQueryError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
pub type UnicodeDecodeFunction =
    Option<unsafe extern "C" fn(*const uint8_t, uint32_t, *mut int32_t) -> uint32_t>;
pub type UChar32 = int32_t;
type C2RustUnnamed_24 = crate::core::util::StackElement<*mut *mut StackNode>;
pub type StackAction = libc::c_uint;
pub const StackActionNone: C2RustUnnamed_37 = 0;
pub const StackActionStop: C2RustUnnamed_37 = 1;
pub const StackActionPop: C2RustUnnamed_37 = 2;
pub type StackCallback =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction>;
pub const TSParseActionTypeShift: C2RustUnnamed_36 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SummarizeStackSession {
    pub summary: *mut StackSummary,
    pub max_depth: libc::c_uint,
}
pub const TSParseActionTypeReduce: C2RustUnnamed_36 = 1;
pub const TSParseActionTypeRecover: C2RustUnnamed_36 = 3;
pub const TSParseActionTypeAccept: C2RustUnnamed_36 = 2;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSStringInput {
    pub string: *const libc::c_char,
    pub length: uint32_t,
}
type C2RustUnnamed_25 = crate::core::util::StackElement<*mut StackEntry_0>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackEntry_0 {
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
type C2RustUnnamed_26 = crate::core::util::StackElement<*mut TreeCursorEntry>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Iterator_0 {
    pub cursor: TreeCursor,
    pub language: *const TSLanguage,
    pub visible_depth: libc::c_uint,
    pub in_padding: bool,
}
pub const IteratorDiffers: IteratorComparison = 0;
pub const IteratorMayDiffer: IteratorComparison = 1;
pub const IteratorMatches: IteratorComparison = 2;
pub type IteratorComparison = libc::c_uint;
pub const _ISprint: C2RustUnnamed_38 = 16384;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NodeChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
}
pub const TreeCursorStepVisible: TreeCursorStep = 2;
pub const TreeCursorStepHidden: TreeCursorStep = 1;
pub type TreeCursorStep = libc::c_uint;
pub const TreeCursorStepNone: TreeCursorStep = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CursorChildIterator {
    pub parent: Subtree,
    pub tree: *const TSTree,
    pub position: Length,
    pub child_index: uint32_t,
    pub structural_child_index: uint32_t,
    pub descendant_index: uint32_t,
    pub alias_sequence: *const TSSymbol,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StatePredecessorMap {
    pub contents: *mut TSStateId,
}
type C2RustUnnamed_27 = crate::core::util::StackElement<*mut uint16_t>;
type C2RustUnnamed_28 = crate::core::util::StackElement<*mut uint32_t>;
type C2RustUnnamed_29 = crate::core::util::StackElement<*mut uint16_t>;
type C2RustUnnamed_30 = crate::core::util::StackElement<*mut AnalysisSubgraphNode>;
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
    pub final_step_indices: C2RustUnnamed_32,
    pub finished_parent_symbols: C2RustUnnamed_31,
    pub did_abort: bool,
}
type C2RustUnnamed_31 = crate::core::util::StackElement<*mut TSSymbol>;
type C2RustUnnamed_32 = crate::core::util::StackElement<*mut uint16_t>;
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
    pub start_states: C2RustUnnamed_34,
    pub nodes: C2RustUnnamed_33,
}
type C2RustUnnamed_33 = crate::core::util::StackElement<*mut AnalysisSubgraphNode>;
type C2RustUnnamed_34 = crate::core::util::StackElement<*mut TSStateId>;
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
type C2RustUnnamed_35 = crate::core::util::StackElement<*mut uint32_t>;
pub type C2RustUnnamed_36 = libc::c_uint;
pub type C2RustUnnamed_37 = libc::c_uint;
pub type C2RustUnnamed_38 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_38 = 8;
pub const _ISpunct: C2RustUnnamed_38 = 4;
pub const _IScntrl: C2RustUnnamed_38 = 2;
pub const _ISblank: C2RustUnnamed_38 = 1;
pub const _ISgraph: C2RustUnnamed_38 = 32768;
pub const _ISspace: C2RustUnnamed_38 = 8192;
pub const _ISxdigit: C2RustUnnamed_38 = 4096;
pub const _ISdigit: C2RustUnnamed_38 = 2048;
pub const _ISalpha: C2RustUnnamed_38 = 1024;
pub const _ISlower: C2RustUnnamed_38 = 512;
pub const _ISupper: C2RustUnnamed_38 = 256;
static mut LENGTH_UNDEFINED: Length = {
    let mut init = Length {
        bytes: 0 as libc::c_int as uint32_t,
        extent: {
            let mut init = TSPoint {
                row: 0 as libc::c_int as uint32_t,
                column: 1 as libc::c_int as uint32_t,
            };
            init
        },
    };
    init
};
static mut LENGTH_MAX: Length = {
    let mut init = Length {
        bytes: 4294967295 as libc::c_uint,
        extent: {
            let mut init = TSPoint {
                row: 4294967295 as libc::c_uint,
                column: 4294967295 as libc::c_uint,
            };
            init
        },
    };
    init
};
static mut BYTE_ORDER_MARK: int32_t = 0xfeff as libc::c_int;
static mut TS_DECODE_ERROR: int32_t = -(1 as libc::c_int);
static mut DEFAULT_RANGE: TSRange = {
    let mut init = TSRange {
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
        start_byte: 0 as libc::c_int as uint32_t,
        end_byte: 4294967295 as libc::c_uint,
    };
    init
};
static mut MAX_SUMMARY_DEPTH: libc::c_uint = 16 as libc::c_int as libc::c_uint;
static mut MAX_COST_DIFFERENCE: libc::c_uint =
    (16 as libc::c_int * 100 as libc::c_int) as libc::c_uint;
static mut MAX_VERSION_COUNT_OVERFLOW: libc::c_uint = 4 as libc::c_int as libc::c_uint;
static mut MAX_VERSION_COUNT: libc::c_uint = 6 as libc::c_int as libc::c_uint;
static mut OP_COUNT_PER_TIMEOUT_CHECK: libc::c_uint = 100 as libc::c_int as libc::c_uint;
static mut NONE: uint16_t = 65535 as libc::c_int as uint16_t;
static mut PATTERN_DONE_MARKER: uint16_t = 65535 as libc::c_int as uint16_t;
static mut WILDCARD_SYMBOL: TSSymbol = 0 as libc::c_int as TSSymbol;
static mut PARENT_DONE: TSQueryError = 4294967295 as TSQueryError;
static mut ROOT_FIELD: *const libc::c_char = b"__ROOT__\0" as *const u8 as *const libc::c_char;
pub const StackStatus_StackStatusHalted: StackStatus = StackStatusHalted;
pub const StackStatus_StackStatusPaused: StackStatus = StackStatusPaused;
pub const StackStatus_StackStatusActive: StackStatus = StackStatusActive;
pub const TSLogType_TSLogTypeLex: TSLogType = TSLogTypeLex;
pub const TSLogType_TSLogTypeParse: TSLogType = TSLogTypeParse;
pub const TSInputEncoding_TSInputEncodingUTF16: TSInputEncoding = TSInputEncodingUTF16;
pub const TSInputEncoding_TSInputEncodingUTF8: TSInputEncoding = TSInputEncodingUTF8;
pub const TSQueryPredicateStepType_TSQueryPredicateStepTypeString: TSQueryPredicateStepType =
    TSQueryPredicateStepTypeString;
pub const TSQueryPredicateStepType_TSQueryPredicateStepTypeCapture: TSQueryPredicateStepType =
    TSQueryPredicateStepTypeCapture;
pub const TSQueryPredicateStepType_TSQueryPredicateStepTypeDone: TSQueryPredicateStepType =
    TSQueryPredicateStepTypeDone;
pub const TSSymbolType_TSSymbolTypeAuxiliary: TSSymbolType = TSSymbolTypeAuxiliary;
pub const TSSymbolType_TSSymbolTypeAnonymous: TSSymbolType = TSSymbolTypeAnonymous;
pub const TSSymbolType_TSSymbolTypeRegular: TSSymbolType = TSSymbolTypeRegular;
pub const TSQuantifier_TSQuantifierOneOrMore: TSQuantifier = TSQuantifierOneOrMore;
pub const TSQuantifier_TSQuantifierOne: TSQuantifier = TSQuantifierOne;
pub const TSQuantifier_TSQuantifierZeroOrMore: TSQuantifier = TSQuantifierZeroOrMore;
pub const TSQuantifier_TSQuantifierZeroOrOne: TSQuantifier = TSQuantifierZeroOrOne;
pub const TSQuantifier_TSQuantifierZero: TSQuantifier = TSQuantifierZero;
pub const TSQueryError_TSQueryErrorLanguage: TSQueryError = TSQueryErrorLanguage;
pub const TSQueryError_TSQueryErrorStructure: TSQueryError = TSQueryErrorStructure;
pub const TSQueryError_TSQueryErrorCapture: TSQueryError = TSQueryErrorCapture;
pub const TSQueryError_TSQueryErrorField: TSQueryError = TSQueryErrorField;
pub const TSQueryError_TSQueryErrorNodeType: TSQueryError = TSQueryErrorNodeType;
pub const TSQueryError_TSQueryErrorSyntax: TSQueryError = TSQueryErrorSyntax;
pub const TSQueryError_TSQueryErrorNone: TSQueryError = TSQueryErrorNone;
pub const ErrorComparison_ErrorComparisonPreferRight: ErrorComparison = ErrorComparisonPreferRight;
pub const ErrorComparison_ErrorComparisonTakeRight: ErrorComparison = ErrorComparisonTakeRight;
pub const ErrorComparison_ErrorComparisonNone: ErrorComparison = ErrorComparisonNone;
pub const ErrorComparison_ErrorComparisonPreferLeft: ErrorComparison = ErrorComparisonPreferLeft;
pub const ErrorComparison_ErrorComparisonTakeLeft: ErrorComparison = ErrorComparisonTakeLeft;
pub const IteratorComparison_IteratorDiffers: IteratorComparison = IteratorDiffers;
pub const IteratorComparison_IteratorMayDiffer: IteratorComparison = IteratorMayDiffer;
pub const IteratorComparison_IteratorMatches: IteratorComparison = IteratorMatches;
pub const TreeCursorStep_TreeCursorStepVisible: TreeCursorStep = TreeCursorStepVisible;
pub const TreeCursorStep_TreeCursorStepHidden: TreeCursorStep = TreeCursorStepHidden;
pub const TreeCursorStep_TreeCursorStepNone: TreeCursorStep = TreeCursorStepNone;
