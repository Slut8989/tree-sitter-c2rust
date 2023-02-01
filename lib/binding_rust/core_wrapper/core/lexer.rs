use crate::core::util::*;
use crate::core::*;
use std::os;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type TSSymbol = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Lexer {
    pub data: TSLexer,
    pub current_position: Length,
    pub token_start_position: Length,
    pub token_end_position: Length,
    pub included_ranges: *mut TSRange,
    pub chunk: *const libc::c_char,
    pub input: TSInput,
    pub logger: TSLogger,
    pub included_range_count: uint32_t,
    pub current_included_range_index: uint32_t,
    pub chunk_start: uint32_t,
    pub chunk_size: uint32_t,
    pub lookahead_size: uint32_t,
    pub did_get_column: bool,
    pub debug_buffer: [libc::c_char; 1024],
}
pub type UnicodeDecodeFunction =
    Option<unsafe extern "C" fn(*const uint8_t, uint32_t, *mut int32_t) -> uint32_t>;
pub type UChar32 = int32_t;
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
#[inline]
unsafe extern "C" fn length_is_undefined(mut length: Length) -> bool {
    return length.bytes == 0 as libc::c_int as libc::c_uint
        && length.extent.column != 0 as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn ts_decode_utf8(
    mut string: *const uint8_t,
    mut length: uint32_t,
    mut code_point: *mut int32_t,
) -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh0 = i;
    i = i.wrapping_add(1);
    *code_point = *string.offset(fresh0 as isize) as int32_t;
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
#[inline]
unsafe extern "C" fn ts_decode_utf16(
    mut string: *const uint8_t,
    mut length: uint32_t,
    mut code_point: *mut int32_t,
) -> uint32_t {
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let fresh1 = i;
    i = i.wrapping_add(1);
    *code_point = *(string as *mut uint16_t).offset(fresh1 as isize) as int32_t;
    if *code_point as libc::c_uint & 0xfffffc00 as libc::c_uint
        == 0xd800 as libc::c_int as libc::c_uint
    {
        let mut __c2: uint16_t = 0;
        if i != length && {
            __c2 = *(string as *mut uint16_t).offset(i as isize);
            __c2 as libc::c_uint & 0xfffffc00 as libc::c_uint
                == 0xdc00 as libc::c_int as libc::c_uint
        } {
            i = i.wrapping_add(1);
            *code_point = (*code_point << 10 as libc::c_ulong) + __c2 as UChar32
                - (((0xd800 as libc::c_int) << 10 as libc::c_ulong) + 0xdc00 as libc::c_int
                    - 0x10000 as libc::c_int);
        }
    }
    return i.wrapping_mul(2 as libc::c_int as libc::c_uint);
}
static mut TS_DECODE_ERROR: int32_t = -(1 as libc::c_int);
static mut BYTE_ORDER_MARK: int32_t = 0xfeff as libc::c_int;
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
unsafe extern "C" fn ts_lexer__eof(mut _self: *const TSLexer) -> bool {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    return (*self_0).current_included_range_index == (*self_0).included_range_count;
}
unsafe extern "C" fn ts_lexer__clear_chunk(mut self_0: *mut Lexer) {
    let ref mut fresh2 = (*self_0).chunk;
    *fresh2 = 0 as *const libc::c_char;
    (*self_0).chunk_size = 0 as libc::c_int as uint32_t;
    (*self_0).chunk_start = 0 as libc::c_int as uint32_t;
}
unsafe extern "C" fn ts_lexer__get_chunk(mut self_0: *mut Lexer) {
    (*self_0).chunk_start = (*self_0).current_position.bytes;
    let ref mut fresh3 = (*self_0).chunk;
    *fresh3 = ((*self_0).input.read).expect("non-null function pointer")(
        (*self_0).input.payload,
        (*self_0).current_position.bytes,
        (*self_0).current_position.extent,
        &mut (*self_0).chunk_size,
    );
    if (*self_0).chunk_size == 0 {
        (*self_0).current_included_range_index = (*self_0).included_range_count;
        let ref mut fresh4 = (*self_0).chunk;
        *fresh4 = 0 as *const libc::c_char;
    }
}
unsafe extern "C" fn ts_lexer__get_lookahead(mut self_0: *mut Lexer) {
    let mut position_in_chunk: uint32_t =
        ((*self_0).current_position.bytes).wrapping_sub((*self_0).chunk_start);
    let mut size: uint32_t = ((*self_0).chunk_size).wrapping_sub(position_in_chunk);
    if size == 0 as libc::c_int as libc::c_uint {
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\0' as i32;
        return;
    }
    let mut chunk: *const uint8_t =
        ((*self_0).chunk as *const uint8_t).offset(position_in_chunk as isize);
    let mut decode: UnicodeDecodeFunction = if (*self_0).input.encoding as libc::c_uint
        == TSInputEncodingUTF8 as libc::c_int as libc::c_uint
    {
        Some(
            ts_decode_utf8
                as unsafe extern "C" fn(*const uint8_t, uint32_t, *mut int32_t) -> uint32_t,
        )
    } else {
        Some(
            ts_decode_utf16
                as unsafe extern "C" fn(*const uint8_t, uint32_t, *mut int32_t) -> uint32_t,
        )
    };
    (*self_0).lookahead_size =
        decode.expect("non-null function pointer")(chunk, size, &mut (*self_0).data.lookahead);
    if (*self_0).data.lookahead == TS_DECODE_ERROR && size < 4 as libc::c_int as libc::c_uint {
        ts_lexer__get_chunk(self_0);
        chunk = (*self_0).chunk as *const uint8_t;
        size = (*self_0).chunk_size;
        (*self_0).lookahead_size =
            decode.expect("non-null function pointer")(chunk, size, &mut (*self_0).data.lookahead);
    }
    if (*self_0).data.lookahead == TS_DECODE_ERROR {
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
    }
}
unsafe extern "C" fn ts_lexer_goto(mut self_0: *mut Lexer, mut position: Length) {
    (*self_0).current_position = position;
    let mut found_included_range: bool = 0 as libc::c_int != 0;
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < (*self_0).included_range_count {
        let mut included_range: *mut TSRange =
            &mut *((*self_0).included_ranges).offset(i as isize) as *mut TSRange;
        if (*included_range).end_byte > (*self_0).current_position.bytes
            && (*included_range).end_byte > (*included_range).start_byte
        {
            if (*included_range).start_byte >= (*self_0).current_position.bytes {
                (*self_0).current_position = {
                    let mut init = Length {
                        bytes: (*included_range).start_byte,
                        extent: (*included_range).start_point,
                    };
                    init
                };
            }
            (*self_0).current_included_range_index = i;
            found_included_range = 1 as libc::c_int != 0;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if found_included_range {
        if !((*self_0).chunk).is_null()
            && ((*self_0).current_position.bytes < (*self_0).chunk_start
                || (*self_0).current_position.bytes
                    >= ((*self_0).chunk_start).wrapping_add((*self_0).chunk_size))
        {
            ts_lexer__clear_chunk(self_0);
        }
        (*self_0).lookahead_size = 0 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\0' as i32;
    } else {
        (*self_0).current_included_range_index = (*self_0).included_range_count;
        let mut last_included_range: *mut TSRange = &mut *((*self_0).included_ranges).offset(
            ((*self_0).included_range_count).wrapping_sub(1 as libc::c_int as libc::c_uint)
                as isize,
        ) as *mut TSRange;
        (*self_0).current_position = {
            let mut init = Length {
                bytes: (*last_included_range).end_byte,
                extent: (*last_included_range).end_point,
            };
            init
        };
        ts_lexer__clear_chunk(self_0);
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
        (*self_0).data.lookahead = '\0' as i32;
    };
}
unsafe extern "C" fn ts_lexer__do_advance(mut self_0: *mut Lexer, mut skip: bool) {
    if (*self_0).lookahead_size != 0 {
        let ref mut fresh5 = (*self_0).current_position.bytes;
        *fresh5 = (*fresh5 as libc::c_uint).wrapping_add((*self_0).lookahead_size) as uint32_t
            as uint32_t;
        if (*self_0).data.lookahead == '\n' as i32 {
            let ref mut fresh6 = (*self_0).current_position.extent.row;
            *fresh6 = (*fresh6).wrapping_add(1);
            (*self_0).current_position.extent.column = 0 as libc::c_int as uint32_t;
        } else {
            let ref mut fresh7 = (*self_0).current_position.extent.column;
            *fresh7 = (*fresh7 as libc::c_uint).wrapping_add((*self_0).lookahead_size) as uint32_t
                as uint32_t;
        }
    }
    let mut current_range: *const TSRange = &mut *((*self_0).included_ranges)
        .offset((*self_0).current_included_range_index as isize)
        as *mut TSRange;
    while (*self_0).current_position.bytes >= (*current_range).end_byte
        || (*current_range).end_byte == (*current_range).start_byte
    {
        let ref mut fresh8 = (*self_0).current_included_range_index;
        *fresh8 = (*fresh8).wrapping_add(1);
        if (*self_0).current_included_range_index < (*self_0).included_range_count {
            current_range = current_range.offset(1);
            (*self_0).current_position = {
                let mut init = Length {
                    bytes: (*current_range).start_byte,
                    extent: (*current_range).start_point,
                };
                init
            };
        } else {
            current_range = 0 as *const TSRange;
            break;
        }
    }
    if skip {
        (*self_0).token_start_position = (*self_0).current_position;
    }
    if !current_range.is_null() {
        if (*self_0).current_position.bytes < (*self_0).chunk_start
            || (*self_0).current_position.bytes
                >= ((*self_0).chunk_start).wrapping_add((*self_0).chunk_size)
        {
            ts_lexer__get_chunk(self_0);
        }
        ts_lexer__get_lookahead(self_0);
    } else {
        ts_lexer__clear_chunk(self_0);
        (*self_0).data.lookahead = '\0' as i32;
        (*self_0).lookahead_size = 1 as libc::c_int as uint32_t;
    };
}
unsafe extern "C" fn ts_lexer__advance(mut _self: *mut TSLexer, mut skip: bool) {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    if ((*self_0).chunk).is_null() {
        return;
    }
    if skip {
        if ((*self_0).logger.log).is_some() {
            {
                if 32 as libc::c_int <= (*self_0).data.lookahead
                    && (*self_0).data.lookahead < 127 as libc::c_int
                {
                    snwrite!(
                        ((*self_0).debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "skip character:'{}'",
                        (*self_0).data.lookahead as u8 as char
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int
                } else {
                    snwrite!(
                        ((*self_0).debug_buffer).as_mut_ptr(),
                        1024 as libc::c_int as libc::c_ulong as usize,
                        "skip character:{}",
                        (*self_0).data.lookahead
                    )
                    .unwrap_or(usize::MAX) as os::raw::c_int
                }
            };
            ((*self_0).logger.log).expect("non-null function pointer")(
                (*self_0).logger.payload,
                TSLogTypeLex,
                ((*self_0).debug_buffer).as_mut_ptr(),
            );
        }
    } else if ((*self_0).logger.log).is_some() {
        {
            if 32 as libc::c_int <= (*self_0).data.lookahead
                && (*self_0).data.lookahead < 127 as libc::c_int
            {
                snwrite!(
                    ((*self_0).debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "consume character:'{}'",
                    (*self_0).data.lookahead as u8 as char
                )
                .unwrap_or(usize::MAX) as os::raw::c_int
            } else {
                snwrite!(
                    ((*self_0).debug_buffer).as_mut_ptr(),
                    1024 as libc::c_int as libc::c_ulong as usize,
                    "consume character:{}",
                    (*self_0).data.lookahead
                )
                .unwrap_or(usize::MAX) as os::raw::c_int
            }
        };
        ((*self_0).logger.log).expect("non-null function pointer")(
            (*self_0).logger.payload,
            TSLogTypeLex,
            ((*self_0).debug_buffer).as_mut_ptr(),
        );
    }
    ts_lexer__do_advance(self_0, skip);
}
unsafe extern "C" fn ts_lexer__mark_end(mut _self: *mut TSLexer) {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    if !ts_lexer__eof(&mut (*self_0).data) {
        let mut current_included_range: *mut TSRange = &mut *((*self_0).included_ranges)
            .offset((*self_0).current_included_range_index as isize)
            as *mut TSRange;
        if (*self_0).current_included_range_index > 0 as libc::c_int as libc::c_uint
            && (*self_0).current_position.bytes == (*current_included_range).start_byte
        {
            let mut previous_included_range: *mut TSRange =
                current_included_range.offset(-(1 as libc::c_int as isize));
            (*self_0).token_end_position = {
                let mut init = Length {
                    bytes: (*previous_included_range).end_byte,
                    extent: (*previous_included_range).end_point,
                };
                init
            };
            return;
        }
    }
    (*self_0).token_end_position = (*self_0).current_position;
}
unsafe extern "C" fn ts_lexer__get_column(mut _self: *mut TSLexer) -> uint32_t {
    let mut self_0: *mut Lexer = _self as *mut Lexer;
    let mut goal_byte: uint32_t = (*self_0).current_position.bytes;
    (*self_0).did_get_column = 1 as libc::c_int != 0;
    let ref mut fresh9 = (*self_0).current_position.bytes;
    *fresh9 = (*fresh9 as libc::c_uint).wrapping_sub((*self_0).current_position.extent.column)
        as uint32_t as uint32_t;
    (*self_0).current_position.extent.column = 0 as libc::c_int as uint32_t;
    if (*self_0).current_position.bytes < (*self_0).chunk_start {
        ts_lexer__get_chunk(self_0);
    }
    let mut result: uint32_t = 0 as libc::c_int as uint32_t;
    if !ts_lexer__eof(_self) {
        ts_lexer__get_lookahead(self_0);
        while (*self_0).current_position.bytes < goal_byte && !((*self_0).chunk).is_null() {
            result = result.wrapping_add(1);
            ts_lexer__do_advance(self_0, 0 as libc::c_int != 0);
            if ts_lexer__eof(_self) {
                break;
            }
        }
    }
    return result;
}
unsafe extern "C" fn ts_lexer__is_at_included_range_start(mut _self: *const TSLexer) -> bool {
    let mut self_0: *const Lexer = _self as *const Lexer;
    if (*self_0).current_included_range_index < (*self_0).included_range_count {
        let mut current_range: *mut TSRange = &mut *((*self_0).included_ranges)
            .offset((*self_0).current_included_range_index as isize)
            as *mut TSRange;
        return (*self_0).current_position.bytes == (*current_range).start_byte;
    } else {
        return 0 as libc::c_int != 0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_init(mut self_0: *mut Lexer) {
    *self_0 = {
        let mut init = Lexer {
            data: {
                let mut init = TSLexer {
                    lookahead: 0 as libc::c_int,
                    result_symbol: 0 as libc::c_int as TSSymbol,
                    advance: Some(
                        ts_lexer__advance as unsafe extern "C" fn(*mut TSLexer, bool) -> (),
                    ),
                    mark_end: Some(ts_lexer__mark_end as unsafe extern "C" fn(*mut TSLexer) -> ()),
                    get_column: Some(
                        ts_lexer__get_column as unsafe extern "C" fn(*mut TSLexer) -> uint32_t,
                    ),
                    is_at_included_range_start: Some(
                        ts_lexer__is_at_included_range_start
                            as unsafe extern "C" fn(*const TSLexer) -> bool,
                    ),
                    eof: Some(ts_lexer__eof as unsafe extern "C" fn(*const TSLexer) -> bool),
                };
                init
            },
            current_position: {
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
            },
            token_start_position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            token_end_position: Length {
                bytes: 0,
                extent: TSPoint { row: 0, column: 0 },
            },
            included_ranges: 0 as *mut TSRange,
            chunk: 0 as *const libc::c_char,
            input: TSInput {
                payload: 0 as *mut libc::c_void,
                read: None,
                encoding: TSInputEncodingUTF8,
            },
            logger: {
                let mut init = TSLogger {
                    payload: 0 as *mut libc::c_void,
                    log: None,
                };
                init
            },
            included_range_count: 0 as libc::c_int as uint32_t,
            current_included_range_index: 0 as libc::c_int as uint32_t,
            chunk_start: 0 as libc::c_int as uint32_t,
            chunk_size: 0 as libc::c_int as uint32_t,
            lookahead_size: 0,
            did_get_column: false,
            debug_buffer: [0; 1024],
        };
        init
    };
    ts_lexer_set_included_ranges(self_0, 0 as *const TSRange, 0 as libc::c_int as uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_delete(mut self_0: *mut Lexer) {
    crate::core::alloc::ts_free((*self_0).included_ranges as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_set_input(mut self_0: *mut Lexer, mut input: TSInput) {
    (*self_0).input = input;
    ts_lexer__clear_chunk(self_0);
    ts_lexer_goto(self_0, (*self_0).current_position);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_reset(mut self_0: *mut Lexer, mut position: Length) {
    if position.bytes != (*self_0).current_position.bytes {
        ts_lexer_goto(self_0, position);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_start(mut self_0: *mut Lexer) {
    (*self_0).token_start_position = (*self_0).current_position;
    (*self_0).token_end_position = LENGTH_UNDEFINED;
    (*self_0).data.result_symbol = 0 as libc::c_int as TSSymbol;
    (*self_0).did_get_column = 0 as libc::c_int != 0;
    if !ts_lexer__eof(&mut (*self_0).data) {
        if (*self_0).chunk_size == 0 {
            ts_lexer__get_chunk(self_0);
        }
        if (*self_0).lookahead_size == 0 {
            ts_lexer__get_lookahead(self_0);
        }
        if (*self_0).current_position.bytes == 0 as libc::c_int as libc::c_uint
            && (*self_0).data.lookahead == BYTE_ORDER_MARK
        {
            ts_lexer__advance(&mut (*self_0).data, 1 as libc::c_int != 0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_finish(
    mut self_0: *mut Lexer,
    mut lookahead_end_byte: *mut uint32_t,
) {
    if length_is_undefined((*self_0).token_end_position) {
        ts_lexer__mark_end(&mut (*self_0).data);
    }
    if (*self_0).token_end_position.bytes < (*self_0).token_start_position.bytes {
        (*self_0).token_start_position = (*self_0).token_end_position;
    }
    let mut current_lookahead_end_byte: uint32_t =
        ((*self_0).current_position.bytes).wrapping_add(1 as libc::c_int as libc::c_uint);
    if (*self_0).data.lookahead == TS_DECODE_ERROR {
        current_lookahead_end_byte = current_lookahead_end_byte.wrapping_add(1);
    }
    if current_lookahead_end_byte > *lookahead_end_byte {
        *lookahead_end_byte = current_lookahead_end_byte;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_advance_to_end(mut self_0: *mut Lexer) {
    while !((*self_0).chunk).is_null() {
        ts_lexer__advance(&mut (*self_0).data, 0 as libc::c_int != 0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_mark_end(mut self_0: *mut Lexer) {
    ts_lexer__mark_end(&mut (*self_0).data);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_set_included_ranges(
    mut self_0: *mut Lexer,
    mut ranges: *const TSRange,
    mut count: uint32_t,
) -> bool {
    if count == 0 as libc::c_int as libc::c_uint || ranges.is_null() {
        ranges = &DEFAULT_RANGE;
        count = 1 as libc::c_int as uint32_t;
    } else {
        let mut previous_byte: uint32_t = 0 as libc::c_int as uint32_t;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < count {
            let mut range: *const TSRange = &*ranges.offset(i as isize) as *const TSRange;
            if (*range).start_byte < previous_byte || (*range).end_byte < (*range).start_byte {
                return 0 as libc::c_int != 0;
            }
            previous_byte = (*range).end_byte;
            i = i.wrapping_add(1);
        }
    }
    let mut size: size_t =
        (count as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TSRange>() as libc::c_ulong);
    let ref mut fresh10 = (*self_0).included_ranges;
    *fresh10 = crate::core::alloc::ts_realloc((*self_0).included_ranges as *mut libc::c_void, size)
        as *mut TSRange;
    std::ptr::copy_nonoverlapping(
        ranges as *const libc::c_void,
        (*self_0).included_ranges as *mut libc::c_void,
        size,
    );
    (*self_0).included_range_count = count;
    ts_lexer_goto(self_0, (*self_0).current_position);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lexer_included_ranges(
    mut self_0: *const Lexer,
    mut count: *mut uint32_t,
) -> *mut TSRange {
    *count = (*self_0).included_range_count;
    return (*self_0).included_ranges;
}
