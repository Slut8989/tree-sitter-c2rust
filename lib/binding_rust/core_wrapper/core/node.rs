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
type C2RustUnnamed_3 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_4 = crate::core::util::LongShortData;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSNode {
    pub context: [uint32_t; 4],
    pub id: *const libc::c_void,
    pub tree: *const TSTree,
}
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
#[inline]
unsafe extern "C" fn ts_subtree_symbol(mut self_0: Subtree) -> TSSymbol {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.symbol as libc::c_int
    } else {
        (*self_0.ptr).symbol as libc::c_int
    }) as TSSymbol;
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
unsafe extern "C" fn ts_subtree_named(mut self_0: Subtree) -> bool {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        (self_0.data).named() as libc::c_int
    } else {
        (*self_0.ptr).named() as libc::c_int
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
unsafe extern "C" fn ts_subtree_parse_state(mut self_0: Subtree) -> TSStateId {
    return (if (self_0.data).is_inline() as libc::c_int != 0 {
        self_0.data.parse_state as libc::c_int
    } else {
        (*self_0.ptr).parse_state as libc::c_int
    }) as TSStateId;
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
unsafe extern "C" fn ts_subtree_child_count(mut self_0: Subtree) -> uint32_t {
    return if (self_0.data).is_inline() as libc::c_int != 0 {
        0 as libc::c_int as libc::c_uint
    } else {
        (*self_0.ptr).child_count
    };
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
unsafe extern "C" fn ts_subtree_total_bytes(mut self_0: Subtree) -> uint32_t {
    return (ts_subtree_total_size(self_0)).bytes;
}
#[inline]
unsafe extern "C" fn ts_subtree_total_size(mut self_0: Subtree) -> Length {
    return length_add(ts_subtree_padding(self_0), ts_subtree_size(self_0));
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
unsafe extern "C" fn point_lt(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column < b.column;
}
#[inline]
unsafe extern "C" fn point_lte(mut a: TSPoint, mut b: TSPoint) -> bool {
    return a.row < b.row || a.row == b.row && a.column <= b.column;
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
#[no_mangle]
pub unsafe extern "C" fn ts_node_new(
    mut tree: *const TSTree,
    mut subtree: *const Subtree,
    mut position: Length,
    mut alias: TSSymbol,
) -> TSNode {
    return {
        let mut init = TSNode {
            context: [
                position.bytes,
                position.extent.row,
                position.extent.column,
                alias as uint32_t,
            ],
            id: subtree as *const libc::c_void,
            tree: tree,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn ts_node__null() -> TSNode {
    return ts_node_new(
        0 as *const TSTree,
        0 as *const Subtree,
        length_zero(),
        0 as libc::c_int as TSSymbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_start_byte(mut self_0: TSNode) -> uint32_t {
    return self_0.context[0 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_start_point(mut self_0: TSNode) -> TSPoint {
    return {
        let mut init = TSPoint {
            row: self_0.context[1 as libc::c_int as usize],
            column: self_0.context[2 as libc::c_int as usize],
        };
        init
    };
}
#[inline]
unsafe extern "C" fn ts_node__alias(mut self_0: *const TSNode) -> uint32_t {
    return (*self_0).context[3 as libc::c_int as usize];
}
#[inline]
unsafe extern "C" fn ts_node__subtree(mut self_0: TSNode) -> Subtree {
    return *(self_0.id as *const Subtree);
}
#[inline]
unsafe extern "C" fn ts_node_iterate_children(mut node: *const TSNode) -> NodeChildIterator {
    let mut subtree: Subtree = ts_node__subtree(*node);
    if ts_subtree_child_count(subtree) == 0 as libc::c_int as libc::c_uint {
        return {
            let mut init = NodeChildIterator {
                parent: Subtree {
                    ptr: 0 as *const SubtreeHeapData,
                },
                tree: (*node).tree,
                position: length_zero(),
                child_index: 0 as libc::c_int as uint32_t,
                structural_child_index: 0 as libc::c_int as uint32_t,
                alias_sequence: 0 as *const TSSymbol,
            };
            init
        };
    }
    let mut alias_sequence: *const TSSymbol = ts_language_alias_sequence(
        (*(*node).tree).language,
        (*subtree.ptr).c2rust_unnamed.c2rust_unnamed.production_id as uint32_t,
    );
    return {
        let mut init = NodeChildIterator {
            parent: subtree,
            tree: (*node).tree,
            position: {
                let mut init = Length {
                    bytes: ts_node_start_byte(*node),
                    extent: ts_node_start_point(*node),
                };
                init
            },
            child_index: 0 as libc::c_int as uint32_t,
            structural_child_index: 0 as libc::c_int as uint32_t,
            alias_sequence: alias_sequence,
        };
        init
    };
}
#[inline]
unsafe extern "C" fn ts_node_child_iterator_done(mut self_0: *mut NodeChildIterator) -> bool {
    return (*self_0).child_index == (*(*self_0).parent.ptr).child_count;
}
#[inline]
unsafe extern "C" fn ts_node_child_iterator_next(
    mut self_0: *mut NodeChildIterator,
    mut result: *mut TSNode,
) -> bool {
    if ((*self_0).parent.ptr).is_null() || ts_node_child_iterator_done(self_0) as libc::c_int != 0 {
        return 0 as libc::c_int != 0;
    }
    let mut child: *const Subtree = &mut *if ((*self_0).parent.data).is_inline() as libc::c_int != 0
    {
        0 as *mut Subtree
    } else {
        ((*self_0).parent.ptr as *mut Subtree)
            .offset(-((*(*self_0).parent.ptr).child_count as isize))
    }
    .offset((*self_0).child_index as isize) as *mut Subtree;
    let mut alias_symbol: TSSymbol = 0 as libc::c_int as TSSymbol;
    if !ts_subtree_extra(*child) {
        if !((*self_0).alias_sequence).is_null() {
            alias_symbol =
                *((*self_0).alias_sequence).offset((*self_0).structural_child_index as isize);
        }
        let ref mut fresh0 = (*self_0).structural_child_index;
        *fresh0 = (*fresh0).wrapping_add(1);
    }
    if (*self_0).child_index > 0 as libc::c_int as libc::c_uint {
        (*self_0).position = length_add((*self_0).position, ts_subtree_padding(*child));
    }
    *result = ts_node_new((*self_0).tree, child, (*self_0).position, alias_symbol);
    (*self_0).position = length_add((*self_0).position, ts_subtree_size(*child));
    let ref mut fresh1 = (*self_0).child_index;
    *fresh1 = (*fresh1).wrapping_add(1);
    return 1 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_node__is_relevant(mut self_0: TSNode, mut include_anonymous: bool) -> bool {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if include_anonymous {
        return ts_subtree_visible(tree) as libc::c_int != 0 || ts_node__alias(&mut self_0) != 0;
    } else {
        let mut alias: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
        if alias != 0 {
            return (ts_language_symbol_metadata((*self_0.tree).language, alias)).named;
        } else {
            return ts_subtree_visible(tree) as libc::c_int != 0
                && ts_subtree_named(tree) as libc::c_int != 0;
        }
    };
}
#[inline]
unsafe extern "C" fn ts_node__relevant_child_count(
    mut self_0: TSNode,
    mut include_anonymous: bool,
) -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        if include_anonymous {
            return (*tree.ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .visible_child_count;
        } else {
            return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count;
        }
    } else {
        return 0 as libc::c_int as uint32_t;
    };
}
#[inline]
unsafe extern "C" fn ts_node__child(
    mut self_0: TSNode,
    mut child_index: uint32_t,
    mut include_anonymous: bool,
) -> TSNode {
    let mut result: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut index: uint32_t = 0 as libc::c_int as uint32_t;
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut result);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if ts_node__is_relevant(child, include_anonymous) {
                if index == child_index {
                    return child;
                }
                index = index.wrapping_add(1);
            } else {
                let mut grandchild_index: uint32_t = child_index.wrapping_sub(index);
                let mut grandchild_count: uint32_t =
                    ts_node__relevant_child_count(child, include_anonymous);
                if grandchild_index < grandchild_count {
                    did_descend = 1 as libc::c_int != 0;
                    result = child;
                    child_index = grandchild_index;
                    break;
                } else {
                    index = (index as libc::c_uint).wrapping_add(grandchild_count) as uint32_t
                        as uint32_t;
                }
            }
        }
    }
    return ts_node__null();
}
unsafe extern "C" fn ts_subtree_has_trailing_empty_descendant(
    mut self_0: Subtree,
    mut other: Subtree,
) -> bool {
    let mut i: libc::c_uint =
        (ts_subtree_child_count(self_0)).wrapping_sub(1 as libc::c_int as libc::c_uint);
    while i.wrapping_add(1 as libc::c_int as libc::c_uint) > 0 as libc::c_int as libc::c_uint {
        let mut child: Subtree = *if (self_0.data).is_inline() as libc::c_int != 0 {
            0 as *mut Subtree
        } else {
            (self_0.ptr as *mut Subtree).offset(-((*self_0.ptr).child_count as isize))
        }
        .offset(i as isize);
        if ts_subtree_total_bytes(child) > 0 as libc::c_int as libc::c_uint {
            break;
        }
        if child.ptr == other.ptr
            || ts_subtree_has_trailing_empty_descendant(child, other) as libc::c_int != 0
        {
            return 1 as libc::c_int != 0;
        }
        i = i.wrapping_sub(1);
    }
    return 0 as libc::c_int != 0;
}
#[inline]
unsafe extern "C" fn ts_node__prev_sibling(
    mut self_0: TSNode,
    mut include_anonymous: bool,
) -> TSNode {
    let mut self_subtree: Subtree = ts_node__subtree(self_0);
    let mut self_is_empty: bool =
        ts_subtree_total_bytes(self_subtree) == 0 as libc::c_int as libc::c_uint;
    let mut target_end_byte: uint32_t = ts_node_end_byte(self_0);
    let mut node: TSNode = ts_node_parent(self_0);
    let mut earlier_node: TSNode = ts_node__null();
    let mut earlier_node_is_relevant: bool = 0 as libc::c_int != 0;
    while !ts_node_is_null(node) {
        let mut earlier_child: TSNode = ts_node__null();
        let mut earlier_child_is_relevant: bool = 0 as libc::c_int != 0;
        let mut found_child_containing_target: bool = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if child.id == self_0.id {
                break;
            }
            if iterator.position.bytes > target_end_byte {
                found_child_containing_target = 1 as libc::c_int != 0;
                break;
            } else if iterator.position.bytes == target_end_byte
                && (!self_is_empty
                    || ts_subtree_has_trailing_empty_descendant(
                        ts_node__subtree(child),
                        self_subtree,
                    ) as libc::c_int
                        != 0)
            {
                found_child_containing_target = 1 as libc::c_int != 0;
                break;
            } else if ts_node__is_relevant(child, include_anonymous) {
                earlier_child = child;
                earlier_child_is_relevant = 1 as libc::c_int != 0;
            } else if ts_node__relevant_child_count(child, include_anonymous)
                > 0 as libc::c_int as libc::c_uint
            {
                earlier_child = child;
                earlier_child_is_relevant = 0 as libc::c_int != 0;
            }
        }
        if found_child_containing_target {
            if !ts_node_is_null(earlier_child) {
                earlier_node = earlier_child;
                earlier_node_is_relevant = earlier_child_is_relevant;
            }
            node = child;
        } else if earlier_child_is_relevant {
            return earlier_child;
        } else {
            if !ts_node_is_null(earlier_child) {
                node = earlier_child;
            } else if earlier_node_is_relevant {
                return earlier_node;
            } else {
                node = earlier_node;
                earlier_node = ts_node__null();
                earlier_node_is_relevant = 0 as libc::c_int != 0;
            }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__next_sibling(
    mut self_0: TSNode,
    mut include_anonymous: bool,
) -> TSNode {
    let mut target_end_byte: uint32_t = ts_node_end_byte(self_0);
    let mut node: TSNode = ts_node_parent(self_0);
    let mut later_node: TSNode = ts_node__null();
    let mut later_node_is_relevant: bool = 0 as libc::c_int != 0;
    while !ts_node_is_null(node) {
        let mut later_child: TSNode = ts_node__null();
        let mut later_child_is_relevant: bool = 0 as libc::c_int != 0;
        let mut child_containing_target: TSNode = ts_node__null();
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if iterator.position.bytes < target_end_byte {
                continue;
            }
            if ts_node_start_byte(child) <= ts_node_start_byte(self_0) {
                if (ts_node__subtree(child)).ptr != (ts_node__subtree(self_0)).ptr {
                    child_containing_target = child;
                }
            } else if ts_node__is_relevant(child, include_anonymous) {
                later_child = child;
                later_child_is_relevant = 1 as libc::c_int != 0;
                break;
            } else {
                if !(ts_node__relevant_child_count(child, include_anonymous)
                    > 0 as libc::c_int as libc::c_uint)
                {
                    continue;
                }
                later_child = child;
                later_child_is_relevant = 0 as libc::c_int != 0;
                break;
            }
        }
        if !ts_node_is_null(child_containing_target) {
            if !ts_node_is_null(later_child) {
                later_node = later_child;
                later_node_is_relevant = later_child_is_relevant;
            }
            node = child_containing_target;
        } else if later_child_is_relevant {
            return later_child;
        } else {
            if !ts_node_is_null(later_child) {
                node = later_child;
            } else if later_node_is_relevant {
                return later_node;
            } else {
                node = later_node;
            }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__first_child_for_byte(
    mut self_0: TSNode,
    mut goal: uint32_t,
    mut include_anonymous: bool,
) -> TSNode {
    let mut node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if !(ts_node_end_byte(child) > goal) {
                continue;
            }
            if ts_node__is_relevant(child, include_anonymous) {
                return child;
            } else {
                if !(ts_node_child_count(child) > 0 as libc::c_int as libc::c_uint) {
                    continue;
                }
                did_descend = 1 as libc::c_int != 0;
                node = child;
                break;
            }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__descendant_for_byte_range(
    mut self_0: TSNode,
    mut range_start: uint32_t,
    mut range_end: uint32_t,
    mut include_anonymous: bool,
) -> TSNode {
    let mut node: TSNode = self_0;
    let mut last_visible_node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            let mut node_end: uint32_t = iterator.position.bytes;
            if node_end < range_end {
                continue;
            }
            if node_end <= range_start {
                continue;
            }
            if range_start < ts_node_start_byte(child) {
                break;
            }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                last_visible_node = node;
            }
            did_descend = 1 as libc::c_int != 0;
            break;
        }
    }
    return last_visible_node;
}
#[inline]
unsafe extern "C" fn ts_node__descendant_for_point_range(
    mut self_0: TSNode,
    mut range_start: TSPoint,
    mut range_end: TSPoint,
    mut include_anonymous: bool,
) -> TSNode {
    let mut node: TSNode = self_0;
    let mut last_visible_node: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            let mut node_end: TSPoint = iterator.position.extent;
            if point_lt(node_end, range_end) {
                continue;
            }
            if point_lte(node_end, range_start) {
                continue;
            }
            if point_lt(range_start, ts_node_start_point(child)) {
                break;
            }
            node = child;
            if ts_node__is_relevant(node, include_anonymous) {
                last_visible_node = node;
            }
            did_descend = 1 as libc::c_int != 0;
            break;
        }
    }
    return last_visible_node;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_end_byte(mut self_0: TSNode) -> uint32_t {
    return (ts_node_start_byte(self_0))
        .wrapping_add((ts_subtree_size(ts_node__subtree(self_0))).bytes);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_end_point(mut self_0: TSNode) -> TSPoint {
    return point_add(
        ts_node_start_point(self_0),
        (ts_subtree_size(ts_node__subtree(self_0))).extent,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_symbol(mut self_0: TSNode) -> TSSymbol {
    let mut symbol: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    if symbol == 0 {
        symbol = ts_subtree_symbol(ts_node__subtree(self_0));
    }
    return ts_language_public_symbol((*self_0.tree).language, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_type(mut self_0: TSNode) -> *const libc::c_char {
    let mut symbol: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    if symbol == 0 {
        symbol = ts_subtree_symbol(ts_node__subtree(self_0));
    }
    return ts_language_symbol_name((*self_0.tree).language, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_language(mut self_0: TSNode) -> *const TSLanguage {
    return (*self_0.tree).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_grammar_symbol(mut self_0: TSNode) -> TSSymbol {
    return ts_subtree_symbol(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_grammar_type(mut self_0: TSNode) -> *const libc::c_char {
    let mut symbol: TSSymbol = ts_subtree_symbol(ts_node__subtree(self_0));
    return ts_language_symbol_name((*self_0.tree).language, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_string(mut self_0: TSNode) -> *mut libc::c_char {
    return ts_subtree_string(
        ts_node__subtree(self_0),
        (*self_0.tree).language,
        0 as libc::c_int != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_eq(mut self_0: TSNode, mut other: TSNode) -> bool {
    return self_0.tree == other.tree && self_0.id == other.id;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_null(mut self_0: TSNode) -> bool {
    return (self_0.id).is_null();
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_extra(mut self_0: TSNode) -> bool {
    return ts_subtree_extra(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_named(mut self_0: TSNode) -> bool {
    let mut alias: TSSymbol = ts_node__alias(&mut self_0) as TSSymbol;
    return if alias as libc::c_int != 0 {
        (ts_language_symbol_metadata((*self_0.tree).language, alias)).named as libc::c_int
    } else {
        ts_subtree_named(ts_node__subtree(self_0)) as libc::c_int
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_missing(mut self_0: TSNode) -> bool {
    return ts_subtree_missing(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_has_changes(mut self_0: TSNode) -> bool {
    return ts_subtree_has_changes(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_has_error(mut self_0: TSNode) -> bool {
    return ts_subtree_error_cost(ts_node__subtree(self_0)) > 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_is_error(mut self_0: TSNode) -> bool {
    let mut symbol: TSSymbol = ts_node_symbol(self_0);
    return symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_descendant_count(mut self_0: TSNode) -> uint32_t {
    return (ts_subtree_visible_descendant_count(ts_node__subtree(self_0)))
        .wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_parse_state(mut self_0: TSNode) -> TSStateId {
    return ts_subtree_parse_state(ts_node__subtree(self_0));
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_next_parse_state(mut self_0: TSNode) -> TSStateId {
    let mut language: *const TSLanguage = (*self_0.tree).language;
    let mut state: uint16_t = ts_node_parse_state(self_0);
    let mut symbol: uint16_t = ts_node_grammar_symbol(self_0);
    return ts_language_next_state(language, state, symbol);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_parent(mut self_0: TSNode) -> TSNode {
    let mut node: TSNode = ts_tree_root_node(self_0.tree);
    let mut end_byte: uint32_t = ts_node_end_byte(self_0);
    if node.id == self_0.id {
        return ts_node__null();
    }
    let mut last_visible_node: TSNode = node;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut node);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if ts_node_start_byte(child) > ts_node_start_byte(self_0) || child.id == self_0.id {
                break;
            }
            if !(iterator.position.bytes >= end_byte) {
                continue;
            }
            node = child;
            if ts_node__is_relevant(child, 1 as libc::c_int != 0) {
                last_visible_node = node;
            }
            did_descend = 1 as libc::c_int != 0;
            break;
        }
    }
    return last_visible_node;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child(mut self_0: TSNode, mut child_index: uint32_t) -> TSNode {
    return ts_node__child(self_0, child_index, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_child(
    mut self_0: TSNode,
    mut child_index: uint32_t,
) -> TSNode {
    return ts_node__child(self_0, child_index, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_by_field_id(
    mut self_0: TSNode,
    mut field_id: TSFieldId,
) -> TSNode {
    let mut field_map: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
    let mut field_map_end: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
    let mut child: TSNode = TSNode {
        context: [0; 4],
        id: 0 as *const libc::c_void,
        tree: 0 as *const TSTree,
    };
    let mut iterator: NodeChildIterator = NodeChildIterator {
        parent: Subtree {
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
        tree: 0 as *const TSTree,
        position: Length {
            bytes: 0,
            extent: TSPoint { row: 0, column: 0 },
        },
        child_index: 0,
        structural_child_index: 0,
        alias_sequence: 0 as *const TSSymbol,
    };
    'c_4113: loop {
        if field_id == 0 || ts_node_child_count(self_0) == 0 as libc::c_int as libc::c_uint {
            return ts_node__null();
        }
        field_map = 0 as *const TSFieldMapEntry;
        field_map_end = 0 as *const TSFieldMapEntry;
        ts_language_field_map(
            (*self_0.tree).language,
            (*(ts_node__subtree(self_0)).ptr)
                .c2rust_unnamed
                .c2rust_unnamed
                .production_id as uint32_t,
            &mut field_map,
            &mut field_map_end,
        );
        if field_map == field_map_end {
            return ts_node__null();
        }
        while ((*field_map).field_id as libc::c_int) < field_id as libc::c_int {
            field_map = field_map.offset(1);
            if field_map == field_map_end {
                return ts_node__null();
            }
        }
        while (*field_map_end.offset(-(1 as libc::c_int) as isize)).field_id as libc::c_int
            > field_id as libc::c_int
        {
            field_map_end = field_map_end.offset(-1);
            if field_map == field_map_end {
                return ts_node__null();
            }
        }
        child = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        iterator = ts_node_iterate_children(&mut self_0);
        loop {
            if !ts_node_child_iterator_next(&mut iterator, &mut child) {
                break 'c_4113;
            }
            if ts_subtree_extra(ts_node__subtree(child)) {
                continue;
            }
            let mut index: uint32_t =
                (iterator.structural_child_index).wrapping_sub(1 as libc::c_int as libc::c_uint);
            if index < (*field_map).child_index as libc::c_uint {
                continue;
            }
            if (*field_map).inherited {
                if field_map.offset(1 as libc::c_int as isize) == field_map_end {
                    self_0 = child;
                    break;
                } else {
                    let mut result: TSNode = ts_node_child_by_field_id(child, field_id);
                    if !(result.id).is_null() {
                        return result;
                    }
                    field_map = field_map.offset(1);
                    if field_map == field_map_end {
                        return ts_node__null();
                    }
                }
            } else if ts_node__is_relevant(child, 1 as libc::c_int != 0) {
                return child;
            } else {
                if ts_node_child_count(child) > 0 as libc::c_int as libc::c_uint {
                    return ts_node_child(child, 0 as libc::c_int as uint32_t);
                } else {
                    field_map = field_map.offset(1);
                    if field_map == field_map_end {
                        return ts_node__null();
                    }
                }
            }
        }
    }
    return ts_node__null();
}
#[inline]
unsafe extern "C" fn ts_node__field_name_from_language(
    mut self_0: TSNode,
    mut structural_child_index: uint32_t,
) -> *const libc::c_char {
    let mut field_map: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
    let mut field_map_end: *const TSFieldMapEntry = 0 as *const TSFieldMapEntry;
    ts_language_field_map(
        (*self_0.tree).language,
        (*(ts_node__subtree(self_0)).ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .production_id as uint32_t,
        &mut field_map,
        &mut field_map_end,
    );
    while field_map != field_map_end {
        if !(*field_map).inherited
            && (*field_map).child_index as libc::c_uint == structural_child_index
        {
            return *((*(*self_0.tree).language).field_names)
                .offset((*field_map).field_id as isize);
        }
        field_map = field_map.offset(1);
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_field_name_for_child(
    mut self_0: TSNode,
    mut child_index: uint32_t,
) -> *const libc::c_char {
    let mut result: TSNode = self_0;
    let mut did_descend: bool = 1 as libc::c_int != 0;
    let mut inherited_field_name: *const libc::c_char = 0 as *const libc::c_char;
    while did_descend {
        did_descend = 0 as libc::c_int != 0;
        let mut child: TSNode = TSNode {
            context: [0; 4],
            id: 0 as *const libc::c_void,
            tree: 0 as *const TSTree,
        };
        let mut index: uint32_t = 0 as libc::c_int as uint32_t;
        let mut iterator: NodeChildIterator = ts_node_iterate_children(&mut result);
        while ts_node_child_iterator_next(&mut iterator, &mut child) {
            if ts_node__is_relevant(child, 1 as libc::c_int != 0) {
                if index == child_index {
                    let mut field_name: *const libc::c_char = ts_node__field_name_from_language(
                        result,
                        (iterator.structural_child_index)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    if !field_name.is_null() {
                        return field_name;
                    }
                    return inherited_field_name;
                }
                index = index.wrapping_add(1);
            } else {
                let mut grandchild_index: uint32_t = child_index.wrapping_sub(index);
                let mut grandchild_count: uint32_t =
                    ts_node__relevant_child_count(child, 1 as libc::c_int != 0);
                if grandchild_index < grandchild_count {
                    let mut field_name_0: *const libc::c_char = ts_node__field_name_from_language(
                        result,
                        (iterator.structural_child_index)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                    );
                    if !field_name_0.is_null() {
                        inherited_field_name = field_name_0;
                    }
                    did_descend = 1 as libc::c_int != 0;
                    result = child;
                    child_index = grandchild_index;
                    break;
                } else {
                    index = (index as libc::c_uint).wrapping_add(grandchild_count) as uint32_t
                        as uint32_t;
                }
            }
        }
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_by_field_name(
    mut self_0: TSNode,
    mut name: *const libc::c_char,
    mut name_length: uint32_t,
) -> TSNode {
    let mut field_id: TSFieldId =
        ts_language_field_id_for_name((*self_0.tree).language, name, name_length);
    return ts_node_child_by_field_id(self_0, field_id);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_child_count(mut self_0: TSNode) -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        return (*tree.ptr)
            .c2rust_unnamed
            .c2rust_unnamed
            .visible_child_count;
    } else {
        return 0 as libc::c_int as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_child_count(mut self_0: TSNode) -> uint32_t {
    let mut tree: Subtree = ts_node__subtree(self_0);
    if ts_subtree_child_count(tree) > 0 as libc::c_int as libc::c_uint {
        return (*tree.ptr).c2rust_unnamed.c2rust_unnamed.named_child_count;
    } else {
        return 0 as libc::c_int as uint32_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_next_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__next_sibling(self_0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_next_named_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__next_sibling(self_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__prev_sibling(self_0, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_prev_named_sibling(mut self_0: TSNode) -> TSNode {
    return ts_node__prev_sibling(self_0, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_first_child_for_byte(
    mut self_0: TSNode,
    mut byte: uint32_t,
) -> TSNode {
    return ts_node__first_child_for_byte(self_0, byte, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_first_named_child_for_byte(
    mut self_0: TSNode,
    mut byte: uint32_t,
) -> TSNode {
    return ts_node__first_child_for_byte(self_0, byte, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_descendant_for_byte_range(
    mut self_0: TSNode,
    mut start: uint32_t,
    mut end: uint32_t,
) -> TSNode {
    return ts_node__descendant_for_byte_range(self_0, start, end, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_descendant_for_byte_range(
    mut self_0: TSNode,
    mut start: uint32_t,
    mut end: uint32_t,
) -> TSNode {
    return ts_node__descendant_for_byte_range(self_0, start, end, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_descendant_for_point_range(
    mut self_0: TSNode,
    mut start: TSPoint,
    mut end: TSPoint,
) -> TSNode {
    return ts_node__descendant_for_point_range(self_0, start, end, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_named_descendant_for_point_range(
    mut self_0: TSNode,
    mut start: TSPoint,
    mut end: TSPoint,
) -> TSNode {
    return ts_node__descendant_for_point_range(self_0, start, end, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn ts_node_edit(mut self_0: *mut TSNode, mut edit: *const TSInputEdit) {
    let mut start_byte: uint32_t = ts_node_start_byte(*self_0);
    let mut start_point: TSPoint = ts_node_start_point(*self_0);
    if start_byte >= (*edit).old_end_byte {
        start_byte =
            ((*edit).new_end_byte).wrapping_add(start_byte.wrapping_sub((*edit).old_end_byte));
        start_point = point_add(
            (*edit).new_end_point,
            point_sub(start_point, (*edit).old_end_point),
        );
    } else if start_byte > (*edit).start_byte {
        start_byte = (*edit).new_end_byte;
        start_point = (*edit).new_end_point;
    }
    (*self_0).context[0 as libc::c_int as usize] = start_byte;
    (*self_0).context[1 as libc::c_int as usize] = start_point.row;
    (*self_0).context[2 as libc::c_int as usize] = start_point.column;
}
