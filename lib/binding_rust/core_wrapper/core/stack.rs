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
pub type _IO_lock_t = ();
use crate::core::util::libc::{dup, fclose, fdopen, fputc, fputs, FILE};
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
type C2RustUnnamed_3 = crate::core::util::LongShortData;
type C2RustUnnamed_4 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stack {
    pub heads: C2RustUnnamed_8,
    pub slices: StackSliceArray,
    pub iterators: C2RustUnnamed_7,
    pub node_pool: StackNodeArray,
    pub base_node: *mut StackNode,
    pub subtree_pool: *mut SubtreePool,
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
type C2RustUnnamed_7 = crate::core::util::StackElement<*mut StackIterator>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackIterator {
    pub node: *mut StackNode,
    pub subtrees: SubtreeArray,
    pub subtree_count: uint32_t,
    pub is_pending: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSliceArray {
    pub contents: *mut StackSlice,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSlice {
    pub subtrees: SubtreeArray,
    pub version: StackVersion,
}
pub type StackVersion = libc::c_uint;
type C2RustUnnamed_8 = crate::core::util::StackElement<*mut StackHead>;
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
pub struct StackSummary {
    pub contents: *mut StackSummaryEntry,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackSummaryEntry {
    pub position: Length,
    pub depth: libc::c_uint,
    pub state: TSStateId,
}
pub type StackAction = libc::c_uint;
pub const StackActionNone: C2RustUnnamed_10 = 0;
pub const StackActionStop: C2RustUnnamed_10 = 1;
pub const StackActionPop: C2RustUnnamed_10 = 2;
pub type StackCallback =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SummarizeStackSession {
    pub summary: *mut StackSummary,
    pub max_depth: libc::c_uint,
}
type C2RustUnnamed_9 = crate::core::util::StackElement<*mut *mut StackNode>;
pub type C2RustUnnamed_10 = libc::c_uint;
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
unsafe extern "C" fn ts_subtree_missing(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).is_missing() as libc::c_int
    } else {
        (*self_0.ptr).is_missing() as libc::c_int
    } != 0;
}
#[inline]
unsafe extern "C" fn ts_subtree_alloc_size(mut child_count: uint32_t) -> size_t {
    return (child_count as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<Subtree>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<SubtreeHeapData>() as libc::c_ulong);
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
unsafe extern "C" fn ts_subtree_node_count(mut self_0: Subtree) -> uint32_t {
    return (ts_subtree_visible_descendant_count(self_0)).wrapping_add(
        (if ts_subtree_visible(self_0) as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint,
    );
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
unsafe extern "C" fn ts_subtree_is_error(mut self_0: Subtree) -> bool {
    return ts_subtree_symbol(self_0) as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int;
}
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
unsafe extern "C" fn point_add(mut a: TSPoint, mut b: TSPoint) -> TSPoint {
    if b.row > 0 as libc::c_int as libc::c_uint {
        return point__new((a.row).wrapping_add(b.row), b.column);
    } else {
        return point__new(a.row, (a.column).wrapping_add(b.column));
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
unsafe extern "C" fn stack_node_retain(mut self_0: *mut StackNode) {
    if self_0.is_null() {
        return;
    }
    if (*self_0).ref_count > 0 as libc::c_int as libc::c_uint {
    } else {
        panic!();
    }
    let ref mut fresh5 = (*self_0).ref_count;
    *fresh5 = (*fresh5).wrapping_add(1);
    if (*self_0).ref_count != 0 as libc::c_int as libc::c_uint {
    } else {
        panic!();
    };
}
unsafe extern "C" fn stack_node_release(
    mut self_0: *mut StackNode,
    mut pool: *mut StackNodeArray,
    mut subtree_pool: *mut SubtreePool,
) {
    loop {
        if (*self_0).ref_count != 0 as libc::c_int as libc::c_uint {
        } else {
            panic!();
        }
        let ref mut fresh6 = (*self_0).ref_count;
        *fresh6 = (*fresh6).wrapping_sub(1);
        if (*self_0).ref_count > 0 as libc::c_int as libc::c_uint {
            return;
        }
        let mut first_predecessor: *mut StackNode = 0 as *mut StackNode;
        if (*self_0).link_count as libc::c_int > 0 as libc::c_int {
            let mut i: libc::c_uint =
                ((*self_0).link_count as libc::c_int - 1 as libc::c_int) as libc::c_uint;
            while i > 0 as libc::c_int as libc::c_uint {
                let mut link: StackLink = (*self_0).links[i as usize];
                if !(link.subtree.ptr).is_null() {
                    ts_subtree_release(subtree_pool, link.subtree);
                }
                stack_node_release(link.node, pool, subtree_pool);
                i = i.wrapping_sub(1);
            }
            let mut link_0: StackLink = (*self_0).links[0 as libc::c_int as usize];
            if !(link_0.subtree.ptr).is_null() {
                ts_subtree_release(subtree_pool, link_0.subtree);
            }
            first_predecessor = (*self_0).links[0 as libc::c_int as usize].node;
        }
        if (*pool).size < 50 as libc::c_int as libc::c_uint {
            array__grow(
                pool as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<*mut StackNode>() as libc::c_ulong,
            );
            let ref mut fresh7 = (*pool).size;
            let fresh8 = *fresh7;
            *fresh7 = (*fresh7).wrapping_add(1);
            let ref mut fresh9 = *((*pool).contents).offset(fresh8 as isize);
            *fresh9 = self_0;
        } else {
            crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
        }
        if first_predecessor.is_null() {
            break;
        }
        self_0 = first_predecessor;
    }
}
unsafe extern "C" fn stack_node_new(
    mut previous_node: *mut StackNode,
    mut subtree: Subtree,
    mut is_pending: bool,
    mut state: TSStateId,
    mut pool: *mut StackNodeArray,
) -> *mut StackNode {
    let mut node: *mut StackNode = (if (*pool).size > 0 as libc::c_int as libc::c_uint {
        let ref mut fresh10 = (*pool).size;
        *fresh10 = (*fresh10).wrapping_sub(1);
        *((*pool).contents).offset(*fresh10 as isize) as *mut libc::c_void
    } else {
        crate::core::alloc::ts_malloc(::std::mem::size_of::<StackNode>() as libc::c_ulong)
    }) as *mut StackNode;
    *node = {
        let mut init = StackNode {
            state: state,
            position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            links: [StackLink {
                node: 0 as *mut StackNode,
                subtree: Subtree {
                    data: SubtreeInlineData {
                        is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [0; 1],
                        symbol: 0,
                        parse_state: 0,
                        padding_columns: 0,
                        padding_rows_lookahead_bytes: [0; 1],
                        padding_bytes: 0,
                        size_bytes: 0,
                    },
                },
                is_pending: false,
            }; 8],
            link_count: 0 as libc::c_int as libc::c_ushort,
            ref_count: 1 as libc::c_int as uint32_t,
            error_cost: 0,
            node_count: 0,
            dynamic_precedence: 0,
        };
        init
    };
    if !previous_node.is_null() {
        (*node).link_count = 1 as libc::c_int as libc::c_ushort;
        (*node).links[0 as libc::c_int as usize] = {
            let mut init = StackLink {
                node: previous_node,
                subtree: subtree,
                is_pending: is_pending,
            };
            init
        };
        (*node).position = (*previous_node).position;
        (*node).error_cost = (*previous_node).error_cost;
        (*node).dynamic_precedence = (*previous_node).dynamic_precedence;
        (*node).node_count = (*previous_node).node_count;
        if !(subtree.ptr).is_null() {
            let ref mut fresh11 = (*node).error_cost;
            *fresh11 = (*fresh11).wrapping_add(ts_subtree_error_cost(subtree));
            (*node).position = length_add((*node).position, ts_subtree_total_size(subtree));
            let ref mut fresh12 = (*node).node_count;
            *fresh12 = (*fresh12).wrapping_add(ts_subtree_node_count(subtree));
            (*node).dynamic_precedence += ts_subtree_dynamic_precedence(subtree);
        }
    } else {
        (*node).position = length_zero();
        (*node).error_cost = 0 as libc::c_int as libc::c_uint;
    }
    return node;
}
unsafe extern "C" fn stack__subtree_is_equivalent(mut left: Subtree, mut right: Subtree) -> bool {
    if left.ptr == right.ptr {
        return 1 as libc::c_int != 0;
    }
    if (left.ptr).is_null() || (right.ptr).is_null() {
        return 0 as libc::c_int != 0;
    }
    if ts_subtree_symbol(left) as libc::c_int != ts_subtree_symbol(right) as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    if ts_subtree_error_cost(left) > 0 as libc::c_int as libc::c_uint
        && ts_subtree_error_cost(right) > 0 as libc::c_int as libc::c_uint
    {
        return 1 as libc::c_int != 0;
    }
    return (ts_subtree_padding(left)).bytes == (ts_subtree_padding(right)).bytes
        && (ts_subtree_size(left)).bytes == (ts_subtree_size(right)).bytes
        && ts_subtree_child_count(left) == ts_subtree_child_count(right)
        && ts_subtree_extra(left) as libc::c_int == ts_subtree_extra(right) as libc::c_int
        && ts_subtree_external_scanner_state_eq(left, right) as libc::c_int != 0;
}
unsafe extern "C" fn stack_node_add_link(
    mut self_0: *mut StackNode,
    mut link: StackLink,
    mut subtree_pool: *mut SubtreePool,
) {
    if link.node == self_0 {
        return;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*self_0).link_count as libc::c_int {
        let mut existing_link: *mut StackLink =
            &mut *((*self_0).links).as_mut_ptr().offset(i as isize) as *mut StackLink;
        if stack__subtree_is_equivalent((*existing_link).subtree, link.subtree) {
            if (*existing_link).node == link.node {
                if ts_subtree_dynamic_precedence(link.subtree)
                    > ts_subtree_dynamic_precedence((*existing_link).subtree)
                {
                    ts_subtree_retain(link.subtree);
                    ts_subtree_release(subtree_pool, (*existing_link).subtree);
                    (*existing_link).subtree = link.subtree;
                    (*self_0).dynamic_precedence = (*link.node).dynamic_precedence
                        + ts_subtree_dynamic_precedence(link.subtree);
                }
                return;
            }
            if (*(*existing_link).node).state as libc::c_int == (*link.node).state as libc::c_int
                && (*(*existing_link).node).position.bytes == (*link.node).position.bytes
            {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < (*link.node).link_count as libc::c_int {
                    stack_node_add_link(
                        (*existing_link).node,
                        (*link.node).links[j as usize],
                        subtree_pool,
                    );
                    j += 1;
                }
                let mut dynamic_precedence: int32_t = (*link.node).dynamic_precedence;
                if !(link.subtree.ptr).is_null() {
                    dynamic_precedence += ts_subtree_dynamic_precedence(link.subtree);
                }
                if dynamic_precedence > (*self_0).dynamic_precedence {
                    (*self_0).dynamic_precedence = dynamic_precedence;
                }
                return;
            }
        }
        i += 1;
    }
    if (*self_0).link_count as libc::c_int == 8 as libc::c_int {
        return;
    }
    stack_node_retain(link.node);
    let mut node_count: libc::c_uint = (*link.node).node_count;
    let mut dynamic_precedence_0: libc::c_int = (*link.node).dynamic_precedence;
    let ref mut fresh13 = (*self_0).link_count;
    let fresh14 = *fresh13;
    *fresh13 = (*fresh13).wrapping_add(1);
    (*self_0).links[fresh14 as usize] = link;
    if !(link.subtree.ptr).is_null() {
        ts_subtree_retain(link.subtree);
        node_count = node_count.wrapping_add(ts_subtree_node_count(link.subtree));
        dynamic_precedence_0 += ts_subtree_dynamic_precedence(link.subtree);
    }
    if node_count > (*self_0).node_count {
        (*self_0).node_count = node_count;
    }
    if dynamic_precedence_0 > (*self_0).dynamic_precedence {
        (*self_0).dynamic_precedence = dynamic_precedence_0;
    }
}
unsafe extern "C" fn stack_head_delete(
    mut self_0: *mut StackHead,
    mut pool: *mut StackNodeArray,
    mut subtree_pool: *mut SubtreePool,
) {
    if !((*self_0).node).is_null() {
        if !((*self_0).last_external_token.ptr).is_null() {
            ts_subtree_release(subtree_pool, (*self_0).last_external_token);
        }
        if !((*self_0).lookahead_when_paused.ptr).is_null() {
            ts_subtree_release(subtree_pool, (*self_0).lookahead_when_paused);
        }
        if !((*self_0).summary).is_null() {
            array__delete((*self_0).summary as *mut VoidArray);
            crate::core::alloc::ts_free((*self_0).summary as *mut libc::c_void);
        }
        stack_node_release((*self_0).node, pool, subtree_pool);
    }
}
unsafe extern "C" fn ts_stack__add_version(
    mut self_0: *mut Stack,
    mut original_version: StackVersion,
    mut node: *mut StackNode,
) -> StackVersion {
    let mut head: StackHead = {
        let mut init = StackHead {
            node: node,
            summary: 0 as *mut StackSummary,
            node_count_at_last_error: (*((*self_0).heads.contents)
                .offset(original_version as isize))
            .node_count_at_last_error,
            last_external_token: (*((*self_0).heads.contents).offset(original_version as isize))
                .last_external_token,
            lookahead_when_paused: Subtree {
                ptr: 0 as *const SubtreeHeapData,
            },
            status: StackStatusActive,
        };
        init
    };
    array__grow(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
    );
    let ref mut fresh15 = (*self_0).heads.size;
    let fresh16 = *fresh15;
    *fresh15 = (*fresh15).wrapping_add(1);
    *((*self_0).heads.contents).offset(fresh16 as isize) = head;
    stack_node_retain(node);
    if !(head.last_external_token.ptr).is_null() {
        ts_subtree_retain(head.last_external_token);
    }
    return ((*self_0).heads.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn ts_stack__add_slice(
    mut self_0: *mut Stack,
    mut original_version: StackVersion,
    mut node: *mut StackNode,
    mut subtrees: *mut SubtreeArray,
) {
    let mut i: uint32_t = ((*self_0).slices.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
        let mut version: StackVersion = (*((*self_0).slices.contents).offset(i as isize)).version;
        if (*((*self_0).heads.contents).offset(version as isize)).node == node {
            let mut slice: StackSlice = {
                let mut init = StackSlice {
                    subtrees: *subtrees,
                    version: version,
                };
                init
            };
            array__splice(
                &mut (*self_0).slices as *mut StackSliceArray as *mut VoidArray,
                ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
                i.wrapping_add(1 as libc::c_int as libc::c_uint),
                0 as libc::c_int as uint32_t,
                1 as libc::c_int as uint32_t,
                &mut slice as *mut StackSlice as *const libc::c_void,
            );
            return;
        }
        i = i.wrapping_sub(1);
    }
    let mut version_0: StackVersion = ts_stack__add_version(self_0, original_version, node);
    let mut slice_0: StackSlice = {
        let mut init = StackSlice {
            subtrees: *subtrees,
            version: version_0,
        };
        init
    };
    array__grow(
        &mut (*self_0).slices as *mut StackSliceArray as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
    );
    let ref mut fresh17 = (*self_0).slices.size;
    let fresh18 = *fresh17;
    *fresh17 = (*fresh17).wrapping_add(1);
    *((*self_0).slices.contents).offset(fresh18 as isize) = slice_0;
}
unsafe extern "C" fn stack__iter(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut callback: StackCallback,
    mut payload: *mut libc::c_void,
    mut goal_subtree_count: libc::c_int,
) -> StackSliceArray {
    (*self_0).slices.size = 0 as libc::c_int as uint32_t;
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    let mut new_iterator: StackIterator = {
        let mut init = StackIterator {
            node: (*head).node,
            subtrees: {
                let mut init = SubtreeArray {
                    contents: 0 as *mut Subtree,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
            subtree_count: 0 as libc::c_int as uint32_t,
            is_pending: 1 as libc::c_int != 0,
        };
        init
    };
    let mut include_subtrees: bool = 0 as libc::c_int != 0;
    if goal_subtree_count >= 0 as libc::c_int {
        include_subtrees = 1 as libc::c_int != 0;
        array__reserve(
            &mut new_iterator.subtrees as *mut SubtreeArray as *mut VoidArray,
            ::std::mem::size_of::<Subtree>() as libc::c_ulong,
            (ts_subtree_alloc_size(goal_subtree_count as uint32_t) as uint32_t as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<Subtree>() as libc::c_ulong)
                as uint32_t,
        );
    }
    array__grow(
        &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
    );
    let ref mut fresh19 = (*self_0).iterators.size;
    let fresh20 = *fresh19;
    *fresh19 = (*fresh19).wrapping_add(1);
    *((*self_0).iterators.contents).offset(fresh20 as isize) = new_iterator;
    while (*self_0).iterators.size > 0 as libc::c_int as libc::c_uint {
        let mut i: uint32_t = 0 as libc::c_int as uint32_t;
        let mut size: uint32_t = (*self_0).iterators.size;
        while i < size {
            let mut iterator: *mut StackIterator =
                &mut *((*self_0).iterators.contents).offset(i as isize) as *mut StackIterator;
            let mut node: *mut StackNode = (*iterator).node;
            let mut action: StackAction =
                callback.expect("non-null function pointer")(payload, iterator);
            let mut should_pop: bool = action & StackActionPop as libc::c_int as libc::c_uint != 0;
            let mut should_stop: bool = action & StackActionStop as libc::c_int as libc::c_uint
                != 0
                || (*node).link_count as libc::c_int == 0 as libc::c_int;
            if should_pop {
                let mut subtrees: SubtreeArray = (*iterator).subtrees;
                if !should_stop {
                    ts_subtree_array_copy(subtrees, &mut subtrees);
                }
                ts_subtree_array_reverse(&mut subtrees);
                ts_stack__add_slice(self_0, version, node, &mut subtrees);
            }
            if should_stop {
                if !should_pop {
                    ts_subtree_array_delete((*self_0).subtree_pool, &mut (*iterator).subtrees);
                }
                array__erase(
                    &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
                    ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
                    i,
                );
                i = i.wrapping_sub(1);
                size = size.wrapping_sub(1);
            } else {
                let mut current_block_41: u64;
                let mut j: uint32_t = 1 as libc::c_int as uint32_t;
                while j <= (*node).link_count as libc::c_uint {
                    let mut next_iterator: *mut StackIterator = 0 as *mut StackIterator;
                    let mut link: StackLink = StackLink {
                        node: 0 as *mut StackNode,
                        subtree: Subtree {
                            data: SubtreeInlineData {
                                is_inline_visible_named_extra_has_changes_is_missing_is_keyword: [0;
                                    1],
                                symbol: 0,
                                parse_state: 0,
                                padding_columns: 0,
                                padding_rows_lookahead_bytes: [0; 1],
                                padding_bytes: 0,
                                size_bytes: 0,
                            },
                        },
                        is_pending: false,
                    };
                    if j == (*node).link_count as libc::c_uint {
                        link = (*node).links[0 as libc::c_int as usize];
                        next_iterator = &mut *((*self_0).iterators.contents).offset(i as isize)
                            as *mut StackIterator;
                        current_block_41 = 7746103178988627676;
                    } else if (*self_0).iterators.size >= 64 as libc::c_int as libc::c_uint {
                        current_block_41 = 15925075030174552612;
                    } else {
                        link = (*node).links[j as usize];
                        let mut current_iterator: StackIterator =
                            *((*self_0).iterators.contents).offset(i as isize);
                        array__grow(
                            &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
                            1 as libc::c_int as uint32_t,
                            ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
                        );
                        let ref mut fresh21 = (*self_0).iterators.size;
                        let fresh22 = *fresh21;
                        *fresh21 = (*fresh21).wrapping_add(1);
                        *((*self_0).iterators.contents).offset(fresh22 as isize) = current_iterator;
                        if ((*self_0).iterators.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            < (*self_0).iterators.size
                        {
                        } else {
                            panic!();
                        }
                        next_iterator = &mut *((*self_0).iterators.contents).offset(
                            ((*self_0).iterators.size)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as *mut StackIterator;
                        ts_subtree_array_copy(
                            (*next_iterator).subtrees,
                            &mut (*next_iterator).subtrees,
                        );
                        current_block_41 = 7746103178988627676;
                    }
                    match current_block_41 {
                        7746103178988627676 => {
                            let ref mut fresh23 = (*next_iterator).node;
                            *fresh23 = link.node;
                            if !(link.subtree.ptr).is_null() {
                                if include_subtrees {
                                    array__grow(
                                        &mut (*next_iterator).subtrees as *mut SubtreeArray
                                            as *mut VoidArray,
                                        1 as libc::c_int as uint32_t,
                                        ::std::mem::size_of::<Subtree>() as libc::c_ulong,
                                    );
                                    let ref mut fresh24 = (*next_iterator).subtrees.size;
                                    let fresh25 = *fresh24;
                                    *fresh24 = (*fresh24).wrapping_add(1);
                                    *((*next_iterator).subtrees.contents)
                                        .offset(fresh25 as isize) = link.subtree;
                                    ts_subtree_retain(link.subtree);
                                }
                                if !ts_subtree_extra(link.subtree) {
                                    let ref mut fresh26 = (*next_iterator).subtree_count;
                                    *fresh26 = (*fresh26).wrapping_add(1);
                                    if !link.is_pending {
                                        (*next_iterator).is_pending = 0 as libc::c_int != 0;
                                    }
                                }
                            } else {
                                let ref mut fresh27 = (*next_iterator).subtree_count;
                                *fresh27 = (*fresh27).wrapping_add(1);
                                (*next_iterator).is_pending = 0 as libc::c_int != 0;
                            }
                        }
                        _ => {}
                    }
                    j = j.wrapping_add(1);
                }
            }
            i = i.wrapping_add(1);
        }
    }
    return (*self_0).slices;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_new(mut subtree_pool: *mut SubtreePool) -> *mut Stack {
    let mut self_0: *mut Stack = crate::core::alloc::ts_calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<Stack>() as libc::c_ulong,
    ) as *mut Stack;
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    (*self_0).heads.capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh28 = (*self_0).heads.contents;
    *fresh28 = 0 as *mut StackHead;
    (*self_0).slices.size = 0 as libc::c_int as uint32_t;
    (*self_0).slices.capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh29 = (*self_0).slices.contents;
    *fresh29 = 0 as *mut StackSlice;
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    (*self_0).iterators.capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh30 = (*self_0).iterators.contents;
    *fresh30 = 0 as *mut StackIterator;
    (*self_0).node_pool.size = 0 as libc::c_int as uint32_t;
    (*self_0).node_pool.capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh31 = (*self_0).node_pool.contents;
    *fresh31 = 0 as *mut *mut StackNode;
    array__reserve(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
        4 as libc::c_int as uint32_t,
    );
    array__reserve(
        &mut (*self_0).slices as *mut StackSliceArray as *mut VoidArray,
        ::std::mem::size_of::<StackSlice>() as libc::c_ulong,
        4 as libc::c_int as uint32_t,
    );
    array__reserve(
        &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
        ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
        4 as libc::c_int as uint32_t,
    );
    array__reserve(
        &mut (*self_0).node_pool as *mut StackNodeArray as *mut VoidArray,
        ::std::mem::size_of::<*mut StackNode>() as libc::c_ulong,
        50 as libc::c_int as uint32_t,
    );
    let ref mut fresh32 = (*self_0).subtree_pool;
    *fresh32 = subtree_pool;
    let ref mut fresh33 = (*self_0).base_node;
    *fresh33 = stack_node_new(
        0 as *mut StackNode,
        Subtree {
            ptr: 0 as *const SubtreeHeapData,
        },
        0 as libc::c_int != 0,
        1 as libc::c_int as TSStateId,
        &mut (*self_0).node_pool,
    );
    ts_stack_clear(self_0);
    return self_0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_delete(mut self_0: *mut Stack) {
    if !((*self_0).slices.contents).is_null() {
        array__delete(&mut (*self_0).slices as *mut StackSliceArray as *mut VoidArray);
    }
    if !((*self_0).iterators.contents).is_null() {
        array__delete(&mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray);
    }
    stack_node_release(
        (*self_0).base_node,
        &mut (*self_0).node_pool,
        (*self_0).subtree_pool,
    );
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        stack_head_delete(
            &mut *((*self_0).heads.contents).offset(i as isize),
            &mut (*self_0).node_pool,
            (*self_0).subtree_pool,
        );
        i = i.wrapping_add(1);
    }
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    if !((*self_0).node_pool.contents).is_null() {
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*self_0).node_pool.size {
            crate::core::alloc::ts_free(
                *((*self_0).node_pool.contents).offset(i_0 as isize) as *mut libc::c_void
            );
            i_0 = i_0.wrapping_add(1);
        }
        array__delete(&mut (*self_0).node_pool as *mut StackNodeArray as *mut VoidArray);
    }
    array__delete(&mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray);
    crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_version_count(mut self_0: *const Stack) -> uint32_t {
    return (*self_0).heads.size;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_state(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> TSStateId {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead))
        .node)
        .state;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_position(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> Length {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead))
        .node)
        .position;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_last_external_token(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> Subtree {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead))
        .last_external_token;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_set_last_external_token(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut token: Subtree,
) {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    if !(token.ptr).is_null() {
        ts_subtree_retain(token);
    }
    if !((*head).last_external_token.ptr).is_null() {
        ts_subtree_release((*self_0).subtree_pool, (*head).last_external_token);
    }
    (*head).last_external_token = token;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_error_cost(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> libc::c_uint {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    let mut result: libc::c_uint = (*(*head).node).error_cost;
    if (*head).status as libc::c_uint == StackStatusPaused as libc::c_int as libc::c_uint
        || (*(*head).node).state as libc::c_int == 0 as libc::c_int
            && ((*(*head).node).links[0 as libc::c_int as usize].subtree.ptr).is_null()
    {
        result = result.wrapping_add(500 as libc::c_int as libc::c_uint);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_node_count_since_error(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> libc::c_uint {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    if (*(*head).node).node_count < (*head).node_count_at_last_error {
        (*head).node_count_at_last_error = (*(*head).node).node_count;
    }
    return ((*(*head).node).node_count).wrapping_sub((*head).node_count_at_last_error);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_push(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut subtree: Subtree,
    mut pending: bool,
    mut state: TSStateId,
) {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    let mut new_node: *mut StackNode = stack_node_new(
        (*head).node,
        subtree,
        pending,
        state,
        &mut (*self_0).node_pool,
    );
    if (subtree.ptr).is_null() {
        (*head).node_count_at_last_error = (*new_node).node_count;
    }
    let ref mut fresh34 = (*head).node;
    *fresh34 = new_node;
}
#[inline(always)]
unsafe extern "C" fn pop_count_callback(
    mut payload: *mut libc::c_void,
    mut iterator: *const StackIterator,
) -> StackAction {
    let mut goal_subtree_count: *mut libc::c_uint = payload as *mut libc::c_uint;
    if (*iterator).subtree_count == *goal_subtree_count {
        return (StackActionPop as libc::c_int | StackActionStop as libc::c_int) as StackAction;
    } else {
        return StackActionNone as libc::c_int as StackAction;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_count(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut count: uint32_t,
) -> StackSliceArray {
    return stack__iter(
        self_0,
        version,
        Some(
            pop_count_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction,
        ),
        &mut count as *mut uint32_t as *mut libc::c_void,
        count as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn pop_pending_callback(
    mut payload: *mut libc::c_void,
    mut iterator: *const StackIterator,
) -> StackAction {
    if (*iterator).subtree_count >= 1 as libc::c_int as libc::c_uint {
        if (*iterator).is_pending {
            return (StackActionPop as libc::c_int | StackActionStop as libc::c_int) as StackAction;
        } else {
            return StackActionStop as libc::c_int as StackAction;
        }
    } else {
        return StackActionNone as libc::c_int as StackAction;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_pending(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> StackSliceArray {
    let mut pop: StackSliceArray = stack__iter(
        self_0,
        version,
        Some(
            pop_pending_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction,
        ),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    );
    if pop.size > 0 as libc::c_int as libc::c_uint {
        ts_stack_renumber_version(
            self_0,
            (*(pop.contents).offset(0 as libc::c_int as isize)).version,
            version,
        );
        (*(pop.contents).offset(0 as libc::c_int as isize)).version = version;
    }
    return pop;
}
#[inline(always)]
unsafe extern "C" fn pop_error_callback(
    mut payload: *mut libc::c_void,
    mut iterator: *const StackIterator,
) -> StackAction {
    if (*iterator).subtrees.size > 0 as libc::c_int as libc::c_uint {
        let mut found_error: *mut bool = payload as *mut bool;
        if !*found_error
            && ts_subtree_is_error(
                *((*iterator).subtrees.contents).offset(0 as libc::c_int as isize),
            ) as libc::c_int
                != 0
        {
            *found_error = 1 as libc::c_int != 0;
            return (StackActionPop as libc::c_int | StackActionStop as libc::c_int) as StackAction;
        } else {
            return StackActionStop as libc::c_int as StackAction;
        }
    } else {
        return StackActionNone as libc::c_int as StackAction;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_error(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> SubtreeArray {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut node: *mut StackNode =
        (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead)).node;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*node).link_count as libc::c_uint {
        if !((*node).links[i as usize].subtree.ptr).is_null()
            && ts_subtree_is_error((*node).links[i as usize].subtree) as libc::c_int != 0
        {
            let mut found_error: bool = 0 as libc::c_int != 0;
            let mut pop: StackSliceArray = stack__iter(
                self_0,
                version,
                Some(
                    pop_error_callback
                        as unsafe extern "C" fn(
                            *mut libc::c_void,
                            *const StackIterator,
                        ) -> StackAction,
                ),
                &mut found_error as *mut bool as *mut libc::c_void,
                1 as libc::c_int,
            );
            if pop.size > 0 as libc::c_int as libc::c_uint {
                if pop.size == 1 as libc::c_int as libc::c_uint {
                } else {
                    panic!();
                }
                ts_stack_renumber_version(
                    self_0,
                    (*(pop.contents).offset(0 as libc::c_int as isize)).version,
                    version,
                );
                return (*(pop.contents).offset(0 as libc::c_int as isize)).subtrees;
            }
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return {
        let mut init = SubtreeArray {
            contents: 0 as *mut Subtree,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0,
        };
        init
    };
}
#[inline(always)]
unsafe extern "C" fn pop_all_callback(
    mut payload: *mut libc::c_void,
    mut iterator: *const StackIterator,
) -> StackAction {
    return (if (*(*iterator).node).link_count as libc::c_int == 0 as libc::c_int {
        StackActionPop as libc::c_int
    } else {
        StackActionNone as libc::c_int
    }) as StackAction;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pop_all(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> StackSliceArray {
    return stack__iter(
        self_0,
        version,
        Some(
            pop_all_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction,
        ),
        0 as *mut libc::c_void,
        0 as libc::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn summarize_stack_callback(
    mut payload: *mut libc::c_void,
    mut iterator: *const StackIterator,
) -> StackAction {
    let mut session: *mut SummarizeStackSession = payload as *mut SummarizeStackSession;
    let mut state: TSStateId = (*(*iterator).node).state;
    let mut depth: libc::c_uint = (*iterator).subtree_count;
    if depth > (*session).max_depth {
        return StackActionStop as libc::c_int as StackAction;
    }
    let mut i: libc::c_uint =
        ((*(*session).summary).size).wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
        let mut entry: StackSummaryEntry = *((*(*session).summary).contents).offset(i as isize);
        if entry.depth < depth {
            break;
        }
        if entry.depth == depth && entry.state as libc::c_int == state as libc::c_int {
            return StackActionNone as libc::c_int as StackAction;
        }
        i = i.wrapping_sub(1);
    }
    array__grow(
        (*session).summary as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackSummaryEntry>() as libc::c_ulong,
    );
    let ref mut fresh35 = (*(*session).summary).size;
    let fresh36 = *fresh35;
    *fresh35 = (*fresh35).wrapping_add(1);
    *((*(*session).summary).contents).offset(fresh36 as isize) = {
        let mut init = StackSummaryEntry {
            position: (*(*iterator).node).position,
            depth: depth,
            state: state,
        };
        init
    };
    return StackActionNone as libc::c_int as StackAction;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_record_summary(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut max_depth: libc::c_uint,
) {
    let mut session: SummarizeStackSession = {
        let mut init = SummarizeStackSession {
            summary: crate::core::alloc::ts_malloc(
                ::std::mem::size_of::<StackSummary>() as libc::c_ulong
            ) as *mut StackSummary,
            max_depth: max_depth,
        };
        init
    };
    (*session.summary).size = 0 as libc::c_int as uint32_t;
    (*session.summary).capacity = 0 as libc::c_int as uint32_t;
    let ref mut fresh37 = (*session.summary).contents;
    *fresh37 = 0 as *mut StackSummaryEntry;
    stack__iter(
        self_0,
        version,
        Some(
            summarize_stack_callback
                as unsafe extern "C" fn(*mut libc::c_void, *const StackIterator) -> StackAction,
        ),
        &mut session as *mut SummarizeStackSession as *mut libc::c_void,
        -(1 as libc::c_int),
    );
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    if !((*head).summary).is_null() {
        array__delete((*head).summary as *mut VoidArray);
        crate::core::alloc::ts_free((*head).summary as *mut libc::c_void);
    }
    let ref mut fresh38 = (*head).summary;
    *fresh38 = session.summary;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_get_summary(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> *mut StackSummary {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead))
        .summary;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_dynamic_precedence(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> libc::c_int {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead))
        .node)
        .dynamic_precedence;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_has_advanced_since_error(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> bool {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *const StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    let mut node: *const StackNode = (*head).node;
    if (*node).error_cost == 0 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int != 0;
    }
    while !node.is_null() {
        if !((*node).link_count as libc::c_int > 0 as libc::c_int) {
            break;
        }
        let mut subtree: Subtree = (*node).links[0 as libc::c_int as usize].subtree;
        if (subtree.ptr).is_null() {
            break;
        }
        if ts_subtree_total_bytes(subtree) > 0 as libc::c_int as libc::c_uint {
            return 1 as libc::c_int != 0;
        } else {
            if !((*node).node_count > (*head).node_count_at_last_error
                && ts_subtree_error_cost(subtree) == 0 as libc::c_int as libc::c_uint)
            {
                break;
            }
            node = (*node).links[0 as libc::c_int as usize].node;
        }
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_remove_version(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    stack_head_delete(
        &mut *((*self_0).heads.contents).offset(version as isize),
        &mut (*self_0).node_pool,
        (*self_0).subtree_pool,
    );
    array__erase(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
        version,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_renumber_version(
    mut self_0: *mut Stack,
    mut v1: StackVersion,
    mut v2: StackVersion,
) {
    if v1 == v2 {
        return;
    }
    if v2 < v1 {
    } else {
        panic!();
    }
    if v1 < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut source_head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(v1 as isize) as *mut StackHead;
    let mut target_head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(v2 as isize) as *mut StackHead;
    if !((*target_head).summary).is_null() && ((*source_head).summary).is_null() {
        let ref mut fresh39 = (*source_head).summary;
        *fresh39 = (*target_head).summary;
        let ref mut fresh40 = (*target_head).summary;
        *fresh40 = 0 as *mut StackSummary;
    }
    stack_head_delete(
        target_head,
        &mut (*self_0).node_pool,
        (*self_0).subtree_pool,
    );
    *target_head = *source_head;
    array__erase(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
        v1,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_swap_versions(
    mut self_0: *mut Stack,
    mut v1: StackVersion,
    mut v2: StackVersion,
) {
    let mut temporary_head: StackHead = *((*self_0).heads.contents).offset(v1 as isize);
    *((*self_0).heads.contents).offset(v1 as isize) =
        *((*self_0).heads.contents).offset(v2 as isize);
    *((*self_0).heads.contents).offset(v2 as isize) = temporary_head;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_copy_version(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> StackVersion {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    array__grow(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
    );
    let ref mut fresh41 = (*self_0).heads.size;
    let fresh42 = *fresh41;
    *fresh41 = (*fresh41).wrapping_add(1);
    *((*self_0).heads.contents).offset(fresh42 as isize) =
        *((*self_0).heads.contents).offset(version as isize);
    if ((*self_0).heads.size).wrapping_sub(1 as libc::c_int as libc::c_uint) < (*self_0).heads.size
    {
    } else {
        panic!();
    }
    let mut head: *mut StackHead = &mut *((*self_0).heads.contents)
        .offset(((*self_0).heads.size).wrapping_sub(1 as libc::c_int as libc::c_uint) as isize)
        as *mut StackHead;
    stack_node_retain((*head).node);
    if !((*head).last_external_token.ptr).is_null() {
        ts_subtree_retain((*head).last_external_token);
    }
    let ref mut fresh43 = (*head).summary;
    *fresh43 = 0 as *mut StackSummary;
    return ((*self_0).heads.size).wrapping_sub(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_merge(
    mut self_0: *mut Stack,
    mut version1: StackVersion,
    mut version2: StackVersion,
) -> bool {
    if !ts_stack_can_merge(self_0, version1, version2) {
        return 0 as libc::c_int != 0;
    }
    let mut head1: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version1 as isize) as *mut StackHead;
    let mut head2: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version2 as isize) as *mut StackHead;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*(*head2).node).link_count as libc::c_uint {
        stack_node_add_link(
            (*head1).node,
            (*(*head2).node).links[i as usize],
            (*self_0).subtree_pool,
        );
        i = i.wrapping_add(1);
    }
    if (*(*head1).node).state as libc::c_int == 0 as libc::c_int {
        (*head1).node_count_at_last_error = (*(*head1).node).node_count;
    }
    ts_stack_remove_version(self_0, version2);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_can_merge(
    mut self_0: *mut Stack,
    mut version1: StackVersion,
    mut version2: StackVersion,
) -> bool {
    let mut head1: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version1 as isize) as *mut StackHead;
    let mut head2: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version2 as isize) as *mut StackHead;
    return (*head1).status as libc::c_uint == StackStatusActive as libc::c_int as libc::c_uint
        && (*head2).status as libc::c_uint == StackStatusActive as libc::c_int as libc::c_uint
        && (*(*head1).node).state as libc::c_int == (*(*head2).node).state as libc::c_int
        && (*(*head1).node).position.bytes == (*(*head2).node).position.bytes
        && (*(*head1).node).error_cost == (*(*head2).node).error_cost
        && ts_subtree_external_scanner_state_eq(
            (*head1).last_external_token,
            (*head2).last_external_token,
        ) as libc::c_int
            != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_halt(mut self_0: *mut Stack, mut version: StackVersion) {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead)).status =
        StackStatusHalted;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_pause(
    mut self_0: *mut Stack,
    mut version: StackVersion,
    mut lookahead: Subtree,
) {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    (*head).status = StackStatusPaused;
    (*head).lookahead_when_paused = lookahead;
    (*head).node_count_at_last_error = (*(*head).node).node_count;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_active(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> bool {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead)).status
        as libc::c_uint
        == StackStatusActive as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_halted(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> bool {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead)).status
        as libc::c_uint
        == StackStatusHalted as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_is_paused(
    mut self_0: *const Stack,
    mut version: StackVersion,
) -> bool {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    return (*(&mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead)).status
        as libc::c_uint
        == StackStatusPaused as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_resume(
    mut self_0: *mut Stack,
    mut version: StackVersion,
) -> Subtree {
    if version < (*self_0).heads.size {
    } else {
        panic!();
    }
    let mut head: *mut StackHead =
        &mut *((*self_0).heads.contents).offset(version as isize) as *mut StackHead;
    if (*head).status as libc::c_uint == StackStatusPaused as libc::c_int as libc::c_uint {
    } else {
        panic!();
    }
    let mut result: Subtree = (*head).lookahead_when_paused;
    (*head).status = StackStatusActive;
    (*head).lookahead_when_paused = Subtree {
        ptr: 0 as *const SubtreeHeapData,
    };
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_clear(mut self_0: *mut Stack) {
    stack_node_retain((*self_0).base_node);
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        stack_head_delete(
            &mut *((*self_0).heads.contents).offset(i as isize),
            &mut (*self_0).node_pool,
            (*self_0).subtree_pool,
        );
        i = i.wrapping_add(1);
    }
    (*self_0).heads.size = 0 as libc::c_int as uint32_t;
    array__grow(
        &mut (*self_0).heads as *mut C2RustUnnamed_8 as *mut VoidArray,
        1 as libc::c_int as uint32_t,
        ::std::mem::size_of::<StackHead>() as libc::c_ulong,
    );
    let ref mut fresh44 = (*self_0).heads.size;
    let fresh45 = *fresh44;
    *fresh44 = (*fresh44).wrapping_add(1);
    *((*self_0).heads.contents).offset(fresh45 as isize) = {
        let mut init = StackHead {
            node: (*self_0).base_node,
            summary: 0 as *mut StackSummary,
            node_count_at_last_error: 0,
            last_external_token: Subtree {
                ptr: 0 as *const SubtreeHeapData,
            },
            lookahead_when_paused: Subtree {
                ptr: 0 as *const SubtreeHeapData,
            },
            status: StackStatusActive,
        };
        init
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_stack_print_dot_graph(
    mut self_0: *mut Stack,
    mut language: *const TSLanguage,
    mut f: *mut FILE,
) -> bool {
    array__reserve(
        &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
        ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
        32 as libc::c_int as uint32_t,
    );
    if f.is_null() {
        f = core::ptr::null_mut();
    }
    fwrite!(f, "digraph stack {{\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    fwrite!(f, "rankdir=\"RL\";\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    fwrite!(f, "edge [arrowhead=none]\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    let mut visited_nodes: C2RustUnnamed_9 = {
        let mut init = C2RustUnnamed_9 {
            contents: 0 as *mut *mut StackNode,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    (*self_0).iterators.size = 0 as libc::c_int as uint32_t;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    while i < (*self_0).heads.size {
        let mut head: *mut StackHead =
            &mut *((*self_0).heads.contents).offset(i as isize) as *mut StackHead;
        if !((*head).status as libc::c_uint == StackStatusHalted as libc::c_int as libc::c_uint) {
            fwrite!(f, "node_head_{} [shape=none, label=\"\"]\n", i).unwrap_or(usize::MAX)
                as os::raw::c_int;
            fwrite!(
                f,
                "node_head_{} -> node_{:p} [",
                i,
                (*head).node as *mut libc::c_void as *const os::raw::c_int
            )
            .unwrap_or(usize::MAX) as os::raw::c_int;
            if (*head).status as libc::c_uint == StackStatusPaused as libc::c_int as libc::c_uint {
                fwrite!(f, "color=red ",).unwrap_or(usize::MAX) as os::raw::c_int;
            }
            fwrite ! (f , "label={}, fontcolor=blue, weight=10000, labeltooltip=\"node_count: {}\nerror_cost: {}" , i , ts_stack_node_count_since_error (self_0 , i) , ts_stack_error_cost (self_0 , i)) . unwrap_or (usize :: MAX) as os :: raw :: c_int ;
            if !((*head).summary).is_null() {
                fwrite!(f, "\nsummary:",).unwrap_or(usize::MAX) as os::raw::c_int;
                let mut j: uint32_t = 0 as libc::c_int as uint32_t;
                while j < (*(*head).summary).size {
                    fwrite!(
                        f,
                        " {}",
                        (*((*(*head).summary).contents).offset(j as isize)).state as libc::c_int
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    j = j.wrapping_add(1);
                }
            }
            if !((*head).last_external_token.ptr).is_null() {
                let mut state: *const ExternalScannerState = &(*(*head).last_external_token.ptr)
                    .c2rust_unnamed
                    .external_scanner_state;
                let mut data: *const libc::c_char = ts_external_scanner_state_data(state);
                fwrite!(f, "\nexternal_scanner_state:",).unwrap_or(usize::MAX) as os::raw::c_int;
                let mut j_0: uint32_t = 0 as libc::c_int as uint32_t;
                while j_0 < (*state).length {
                    fwrite!(f, " {:02X}", *data.offset(j_0 as isize) as libc::c_int)
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                    j_0 = j_0.wrapping_add(1);
                }
            }
            fwrite!(f, "\"]\n",).unwrap_or(usize::MAX) as os::raw::c_int;
            array__grow(
                &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
                1 as libc::c_int as uint32_t,
                ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
            );
            let ref mut fresh46 = (*self_0).iterators.size;
            let fresh47 = *fresh46;
            *fresh46 = (*fresh46).wrapping_add(1);
            *((*self_0).iterators.contents).offset(fresh47 as isize) = {
                let mut init = StackIterator {
                    node: (*head).node,
                    subtrees: SubtreeArray {
                        contents: 0 as *mut Subtree,
                        size: 0,
                        capacity: 0,
                    },
                    subtree_count: 0,
                    is_pending: false,
                };
                init
            };
        }
        i = i.wrapping_add(1);
    }
    let mut all_iterators_done: bool = 0 as libc::c_int != 0;
    while !all_iterators_done {
        all_iterators_done = 1 as libc::c_int != 0;
        let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
        while i_0 < (*self_0).iterators.size {
            let mut iterator: StackIterator = *((*self_0).iterators.contents).offset(i_0 as isize);
            let mut node: *mut StackNode = iterator.node;
            let mut j_1: uint32_t = 0 as libc::c_int as uint32_t;
            while j_1 < visited_nodes.size {
                if *(visited_nodes.contents).offset(j_1 as isize) == node {
                    node = 0 as *mut StackNode;
                    break;
                } else {
                    j_1 = j_1.wrapping_add(1);
                }
            }
            if !node.is_null() {
                all_iterators_done = 0 as libc::c_int != 0;
                fwrite!(
                    f,
                    "node_{:p} [",
                    node as *mut libc::c_void as *const os::raw::c_int
                )
                .unwrap_or(usize::MAX) as os::raw::c_int;
                if (*node).state as libc::c_int == 0 as libc::c_int {
                    fwrite!(f, "label=\"?\"",).unwrap_or(usize::MAX) as os::raw::c_int;
                } else if (*node).link_count as libc::c_int == 1 as libc::c_int
                    && !((*node).links[0 as libc::c_int as usize].subtree.ptr).is_null()
                    && ts_subtree_extra((*node).links[0 as libc::c_int as usize].subtree)
                        as libc::c_int
                        != 0
                {
                    fwrite!(f, "shape=point margin=0 label=\"\"",).unwrap_or(usize::MAX)
                        as os::raw::c_int;
                } else {
                    fwrite!(f, "label=\"{}\"", (*node).state as libc::c_int).unwrap_or(usize::MAX)
                        as os::raw::c_int;
                }
                fwrite ! (f , " tooltip=\"position: {},{}\nnode_count:{}\nerror_cost: {}\ndynamic_precedence: {}\"];\n" , ((* node) . position . extent . row) . wrapping_add (1 as libc :: c_int as libc :: c_uint) , (* node) . position . extent . column , (* node) . node_count , (* node) . error_cost , (* node) . dynamic_precedence) . unwrap_or (usize :: MAX) as os :: raw :: c_int ;
                let mut j_2: libc::c_int = 0 as libc::c_int;
                while j_2 < (*node).link_count as libc::c_int {
                    let mut link: StackLink = (*node).links[j_2 as usize];
                    fwrite!(
                        f,
                        "node_{:p} -> node_{:p} [",
                        node as *mut libc::c_void as *const os::raw::c_int,
                        link.node as *mut libc::c_void as *const os::raw::c_int
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int;
                    if link.is_pending {
                        fwrite!(f, "style=dashed ",).unwrap_or(usize::MAX) as os::raw::c_int;
                    }
                    if !(link.subtree.ptr).is_null()
                        && ts_subtree_extra(link.subtree) as libc::c_int != 0
                    {
                        fwrite!(f, "fontcolor=gray ",).unwrap_or(usize::MAX) as os::raw::c_int;
                    }
                    if (link.subtree.ptr).is_null() {
                        fwrite!(f, "color=red",).unwrap_or(usize::MAX) as os::raw::c_int;
                    } else {
                        fwrite!(f, "label=\"",).unwrap_or(usize::MAX) as os::raw::c_int;
                        let mut quoted: bool = ts_subtree_visible(link.subtree) as libc::c_int != 0
                            && !ts_subtree_named(link.subtree);
                        if quoted {
                            fwrite!(f, "'",).unwrap_or(usize::MAX) as os::raw::c_int;
                        }
                        ts_language_write_symbol_as_dot_string(
                            language,
                            f,
                            ts_subtree_symbol(link.subtree),
                        );
                        if quoted {
                            fwrite!(f, "'",).unwrap_or(usize::MAX) as os::raw::c_int;
                        }
                        fwrite!(f, "\"",).unwrap_or(usize::MAX) as os::raw::c_int;
                        fwrite!(
                            f,
                            "labeltooltip=\"error_cost: {}\ndynamic_precedence: {}\"",
                            ts_subtree_error_cost(link.subtree),
                            ts_subtree_dynamic_precedence(link.subtree)
                        )
                        .unwrap_or(usize::MAX) as os::raw::c_int;
                    }
                    fwrite!(f, "];\n",).unwrap_or(usize::MAX) as os::raw::c_int;
                    let mut next_iterator: *mut StackIterator = 0 as *mut StackIterator;
                    if j_2 == 0 as libc::c_int {
                        next_iterator = &mut *((*self_0).iterators.contents).offset(i_0 as isize)
                            as *mut StackIterator;
                    } else {
                        array__grow(
                            &mut (*self_0).iterators as *mut C2RustUnnamed_7 as *mut VoidArray,
                            1 as libc::c_int as uint32_t,
                            ::std::mem::size_of::<StackIterator>() as libc::c_ulong,
                        );
                        let ref mut fresh48 = (*self_0).iterators.size;
                        let fresh49 = *fresh48;
                        *fresh48 = (*fresh48).wrapping_add(1);
                        *((*self_0).iterators.contents).offset(fresh49 as isize) = iterator;
                        if ((*self_0).iterators.size).wrapping_sub(1 as libc::c_int as libc::c_uint)
                            < (*self_0).iterators.size
                        {
                        } else {
                            panic!();
                        }
                        next_iterator = &mut *((*self_0).iterators.contents).offset(
                            ((*self_0).iterators.size)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                as isize,
                        ) as *mut StackIterator;
                    }
                    let ref mut fresh50 = (*next_iterator).node;
                    *fresh50 = link.node;
                    j_2 += 1;
                }
                array__grow(
                    &mut visited_nodes as *mut C2RustUnnamed_9 as *mut VoidArray,
                    1 as libc::c_int as uint32_t,
                    ::std::mem::size_of::<*mut StackNode>() as libc::c_ulong,
                );
                let fresh51 = visited_nodes.size;
                visited_nodes.size = (visited_nodes.size).wrapping_add(1);
                let ref mut fresh52 = *(visited_nodes.contents).offset(fresh51 as isize);
                *fresh52 = node;
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    fwrite!(f, "}}\n",).unwrap_or(usize::MAX) as os::raw::c_int;
    array__delete(&mut visited_nodes as *mut C2RustUnnamed_9 as *mut VoidArray);
    return 1 as libc::c_int != 0;
}
pub const StackStatus_StackStatusHalted: StackStatus = StackStatusHalted;
pub const StackStatus_StackStatusPaused: StackStatus = StackStatusPaused;
pub const StackStatus_StackStatusActive: StackStatus = StackStatusActive;
