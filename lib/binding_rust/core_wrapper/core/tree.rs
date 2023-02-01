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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSTree {
    pub root: Subtree,
    pub language: *const TSLanguage,
    pub included_ranges: *mut TSRange,
    pub included_range_count: libc::c_uint,
}
type C2RustUnnamed_3 = crate::core::util::ScannerStateWithLookahead;
type C2RustUnnamed_4 = crate::core::util::LongShortData;
type C2RustUnnamed_5 = crate::core::util::ScannerStateLookaheadMeta;
type C2RustUnnamed_6 = crate::core::util::ScannerStateLookaheadFirstLeaf;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VoidArray {
    pub contents: *mut libc::c_void,
    pub size: uint32_t,
    pub capacity: uint32_t,
}
type C2RustUnnamed_7 = crate::core::util::StackElement<*mut TreeCursorEntry>;
use crate::core::util::libc::{dup, fclose, fdopen, fputc, fputs, FILE};
pub type _IO_lock_t = ();
#[inline]
unsafe extern "C" fn array__delete(mut self_0: *mut VoidArray) {
    crate::core::alloc::ts_free((*self_0).contents);
    let ref mut fresh0 = (*self_0).contents;
    *fresh0 = 0 as *mut libc::c_void;
    (*self_0).size = 0 as libc::c_int as uint32_t;
    (*self_0).capacity = 0 as libc::c_int as uint32_t;
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
#[no_mangle]
pub unsafe extern "C" fn ts_tree_new(
    mut root: Subtree,
    mut language: *const TSLanguage,
    mut included_ranges: *const TSRange,
    mut included_range_count: libc::c_uint,
) -> *mut TSTree {
    let mut result: *mut TSTree =
        crate::core::alloc::ts_malloc(::std::mem::size_of::<TSTree>() as libc::c_ulong)
            as *mut TSTree;
    (*result).root = root;
    let ref mut fresh1 = (*result).language;
    *fresh1 = language;
    let ref mut fresh2 = (*result).included_ranges;
    *fresh2 = crate::core::alloc::ts_calloc(
        included_range_count as size_t,
        ::std::mem::size_of::<TSRange>() as libc::c_ulong,
    ) as *mut TSRange;
    std::ptr::copy_nonoverlapping(
        included_ranges as *const libc::c_void,
        (*result).included_ranges as *mut libc::c_void,
        (included_range_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TSRange>() as libc::c_ulong),
    );
    (*result).included_range_count = included_range_count;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_copy(mut self_0: *const TSTree) -> *mut TSTree {
    ts_subtree_retain((*self_0).root);
    return ts_tree_new(
        (*self_0).root,
        (*self_0).language,
        (*self_0).included_ranges,
        (*self_0).included_range_count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_delete(mut self_0: *mut TSTree) {
    if self_0.is_null() {
        return;
    }
    let mut pool: SubtreePool = ts_subtree_pool_new(0 as libc::c_int as uint32_t);
    ts_subtree_release(&mut pool, (*self_0).root);
    ts_subtree_pool_delete(&mut pool);
    crate::core::alloc::ts_free((*self_0).included_ranges as *mut libc::c_void);
    crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_root_node(mut self_0: *const TSTree) -> TSNode {
    return ts_node_new(
        self_0,
        &(*self_0).root,
        ts_subtree_padding((*self_0).root),
        0 as libc::c_int as TSSymbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_root_node_with_offset(
    mut self_0: *const TSTree,
    mut offset_bytes: uint32_t,
    mut offset_extent: TSPoint,
) -> TSNode {
    let mut offset: Length = {
        let mut init = Length {
            bytes: offset_bytes,
            extent: offset_extent,
        };
        init
    };
    return ts_node_new(
        self_0,
        &(*self_0).root,
        length_add(offset, ts_subtree_padding((*self_0).root)),
        0 as libc::c_int as TSSymbol,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_language(mut self_0: *const TSTree) -> *const TSLanguage {
    return (*self_0).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_edit(mut self_0: *mut TSTree, mut edit: *const TSInputEdit) {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).included_range_count {
        let mut range: *mut TSRange =
            &mut *((*self_0).included_ranges).offset(i as isize) as *mut TSRange;
        if (*range).end_byte >= (*edit).old_end_byte {
            if (*range).end_byte != 4294967295 as libc::c_uint {
                (*range).end_byte = ((*edit).new_end_byte)
                    .wrapping_add(((*range).end_byte).wrapping_sub((*edit).old_end_byte));
                (*range).end_point = point_add(
                    (*edit).new_end_point,
                    point_sub((*range).end_point, (*edit).old_end_point),
                );
                if (*range).end_byte < (*edit).new_end_byte {
                    (*range).end_byte = 4294967295 as libc::c_uint;
                    (*range).end_point = {
                        let mut init = TSPoint {
                            row: 4294967295 as libc::c_uint,
                            column: 4294967295 as libc::c_uint,
                        };
                        init
                    };
                }
            }
        } else if (*range).end_byte > (*edit).start_byte {
            (*range).end_byte = (*edit).start_byte;
            (*range).end_point = (*edit).start_point;
        }
        if (*range).start_byte >= (*edit).old_end_byte {
            (*range).start_byte = ((*edit).new_end_byte)
                .wrapping_add(((*range).start_byte).wrapping_sub((*edit).old_end_byte));
            (*range).start_point = point_add(
                (*edit).new_end_point,
                point_sub((*range).start_point, (*edit).old_end_point),
            );
            if (*range).start_byte < (*edit).new_end_byte {
                (*range).start_byte = 4294967295 as libc::c_uint;
                (*range).start_point = {
                    let mut init = TSPoint {
                        row: 4294967295 as libc::c_uint,
                        column: 4294967295 as libc::c_uint,
                    };
                    init
                };
            }
        } else if (*range).start_byte > (*edit).start_byte {
            (*range).start_byte = (*edit).start_byte;
            (*range).start_point = (*edit).start_point;
        }
        i = i.wrapping_add(1);
    }
    let mut pool: SubtreePool = ts_subtree_pool_new(0 as libc::c_int as uint32_t);
    (*self_0).root = ts_subtree_edit((*self_0).root, edit, &mut pool);
    ts_subtree_pool_delete(&mut pool);
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_included_ranges(
    mut self_0: *const TSTree,
    mut length: *mut uint32_t,
) -> *mut TSRange {
    *length = (*self_0).included_range_count;
    let mut ranges: *mut TSRange = crate::core::alloc::ts_calloc(
        (*self_0).included_range_count as size_t,
        ::std::mem::size_of::<TSRange>() as libc::c_ulong,
    ) as *mut TSRange;
    std::ptr::copy_nonoverlapping(
        (*self_0).included_ranges as *const libc::c_void,
        ranges as *mut libc::c_void,
        ((*self_0).included_range_count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TSRange>() as libc::c_ulong),
    );
    return ranges;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_get_changed_ranges(
    mut old_tree: *const TSTree,
    mut new_tree: *const TSTree,
    mut length: *mut uint32_t,
) -> *mut TSRange {
    let mut cursor1: TreeCursor = {
        let mut init = TreeCursor {
            tree: 0 as *const TSTree,
            stack: {
                let mut init = C2RustUnnamed_7 {
                    contents: 0 as *mut TreeCursorEntry,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
    let mut cursor2: TreeCursor = {
        let mut init = TreeCursor {
            tree: 0 as *const TSTree,
            stack: {
                let mut init = C2RustUnnamed_7 {
                    contents: 0 as *mut TreeCursorEntry,
                    size: 0 as libc::c_int as uint32_t,
                    capacity: 0 as libc::c_int as uint32_t,
                };
                init
            },
        };
        init
    };
    ts_tree_cursor_init(&mut cursor1, ts_tree_root_node(old_tree));
    ts_tree_cursor_init(&mut cursor2, ts_tree_root_node(new_tree));
    let mut included_range_differences: TSRangeArray = {
        let mut init = TSRangeArray {
            contents: 0 as *mut TSRange,
            size: 0 as libc::c_int as uint32_t,
            capacity: 0 as libc::c_int as uint32_t,
        };
        init
    };
    ts_range_array_get_changed_ranges(
        (*old_tree).included_ranges,
        (*old_tree).included_range_count,
        (*new_tree).included_ranges,
        (*new_tree).included_range_count,
        &mut included_range_differences,
    );
    let mut result: *mut TSRange = 0 as *mut TSRange;
    *length = ts_subtree_get_changed_ranges(
        &(*old_tree).root,
        &(*new_tree).root,
        &mut cursor1,
        &mut cursor2,
        (*old_tree).language,
        &mut included_range_differences,
        &mut result,
    );
    array__delete(&mut included_range_differences as *mut TSRangeArray as *mut VoidArray);
    array__delete(&mut cursor1.stack as *mut C2RustUnnamed_7 as *mut VoidArray);
    array__delete(&mut cursor2.stack as *mut C2RustUnnamed_7 as *mut VoidArray);
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn ts_tree_print_dot_graph(
    mut self_0: *const TSTree,
    mut file_descriptor: libc::c_int,
) {
    let mut file: *mut FILE = fdopen(
        dup(file_descriptor),
        b"a\0" as *const u8 as *const libc::c_char,
    );
    ts_subtree_print_dot_graph((*self_0).root, (*self_0).language, file);
    fclose(file);
}
