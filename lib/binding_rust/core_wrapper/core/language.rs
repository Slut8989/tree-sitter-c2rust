use crate::core::util::*;
use crate::core::*;
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
pub struct TSLanguage {
    pub version: uint32_t,
    pub symbol_count: uint32_t,
    pub alias_count: uint32_t,
    pub token_count: uint32_t,
    pub external_token_count: uint32_t,
    pub state_count: uint32_t,
    pub large_state_count: uint32_t,
    pub production_id_count: uint32_t,
    pub field_count: uint32_t,
    pub max_alias_sequence_length: uint16_t,
    pub parse_table: *const uint16_t,
    pub small_parse_table: *const uint16_t,
    pub small_parse_table_map: *const uint32_t,
    pub parse_actions: *const TSParseActionEntry,
    pub symbol_names: *const *const libc::c_char,
    pub field_names: *const *const libc::c_char,
    pub field_map_slices: *const TSFieldMapSlice,
    pub field_map_entries: *const TSFieldMapEntry,
    pub symbol_metadata: *const TSSymbolMetadata,
    pub public_symbol_map: *const TSSymbol,
    pub alias_map: *const uint16_t,
    pub alias_sequences: *const TSSymbol,
    pub lex_modes: *const TSLexMode,
    pub lex_fn: Option<unsafe extern "C" fn(*mut TSLexer, TSStateId) -> bool>,
    pub keyword_lex_fn: Option<unsafe extern "C" fn(*mut TSLexer, TSStateId) -> bool>,
    pub keyword_capture_token: TSSymbol,
    pub external_scanner: C2RustUnnamed,
    pub primary_state_ids: *const TSStateId,
}
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
pub struct TableEntry {
    pub actions: *const TSParseAction,
    pub action_count: uint32_t,
    pub is_reusable: bool,
}
pub const TSParseActionTypeShift: C2RustUnnamed_3 = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const TSParseActionTypeRecover: C2RustUnnamed_3 = 3;
pub const TSParseActionTypeAccept: C2RustUnnamed_3 = 2;
pub const TSParseActionTypeReduce: C2RustUnnamed_3 = 1;
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
        let fresh0 = data;
        data = data.offset(1);
        let mut group_count: uint16_t = *fresh0;
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < group_count as libc::c_uint {
            let fresh1 = data;
            data = data.offset(1);
            let mut section_value: uint16_t = *fresh1;
            let fresh2 = data;
            data = data.offset(1);
            let mut symbol_count: uint16_t = *fresh2;
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while j < symbol_count as libc::c_uint {
                let fresh3 = data;
                data = data.offset(1);
                if *fresh3 as libc::c_int == symbol as libc::c_int {
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
unsafe extern "C" fn ts_lookahead_iterator__next(mut self_0: *mut LookaheadIterator) -> bool {
    if (*self_0).is_small_state {
        let ref mut fresh4 = (*self_0).data;
        *fresh4 = (*fresh4).offset(1);
        if (*self_0).data == (*self_0).group_end {
            if (*self_0).group_count as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int != 0;
            }
            let ref mut fresh5 = (*self_0).group_count;
            *fresh5 = (*fresh5).wrapping_sub(1);
            let ref mut fresh6 = (*self_0).data;
            let fresh7 = *fresh6;
            *fresh6 = (*fresh6).offset(1);
            (*self_0).table_value = *fresh7;
            let ref mut fresh8 = (*self_0).data;
            let fresh9 = *fresh8;
            *fresh8 = (*fresh8).offset(1);
            let mut symbol_count: libc::c_uint = *fresh9 as libc::c_uint;
            let ref mut fresh10 = (*self_0).group_end;
            *fresh10 = ((*self_0).data).offset(symbol_count as isize);
            (*self_0).symbol = *(*self_0).data;
        } else {
            (*self_0).symbol = *(*self_0).data;
            return 1 as libc::c_int != 0;
        }
    } else {
        loop {
            let ref mut fresh11 = (*self_0).data;
            *fresh11 = (*fresh11).offset(1);
            let ref mut fresh12 = (*self_0).symbol;
            *fresh12 = (*fresh12).wrapping_add(1);
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
        let ref mut fresh13 = (*self_0).actions;
        *fresh13 = entry.offset(1 as libc::c_int as isize) as *const TSParseAction;
        (*self_0).next_state = 0 as libc::c_int as TSStateId;
    } else {
        (*self_0).action_count = 0 as libc::c_int as uint16_t;
        (*self_0).next_state = (*self_0).table_value;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_count(mut self_0: *const TSLanguage) -> uint32_t {
    return ((*self_0).symbol_count).wrapping_add((*self_0).alias_count);
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_state_count(mut self_0: *const TSLanguage) -> uint32_t {
    return (*self_0).state_count;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_version(mut self_0: *const TSLanguage) -> uint32_t {
    return (*self_0).version;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_count(mut self_0: *const TSLanguage) -> uint32_t {
    return (*self_0).field_count;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_table_entry(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
    mut result: *mut TableEntry,
) {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
        || symbol as libc::c_int
            == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        (*result).action_count = 0 as libc::c_int as uint32_t;
        (*result).is_reusable = 0 as libc::c_int != 0;
        let ref mut fresh14 = (*result).actions;
        *fresh14 = 0 as *const TSParseAction;
    } else {
        if (symbol as libc::c_uint) < (*self_0).token_count {
        } else {
            panic!();
        }
        let mut action_index: uint32_t = ts_language_lookup(self_0, state, symbol) as uint32_t;
        let mut entry: *const TSParseActionEntry =
            &*((*self_0).parse_actions).offset(action_index as isize) as *const TSParseActionEntry;
        (*result).action_count = (*entry).entry.count as uint32_t;
        (*result).is_reusable = (*entry).entry.reusable;
        let ref mut fresh15 = (*result).actions;
        *fresh15 = entry.offset(1 as libc::c_int as isize) as *const TSParseAction;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_metadata(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbolMetadata {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return {
            let mut init = TSSymbolMetadata {
                visible: 1 as libc::c_int != 0,
                named: 1 as libc::c_int != 0,
                supertype: false,
            };
            init
        };
    } else if symbol as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        return {
            let mut init = TSSymbolMetadata {
                visible: 0 as libc::c_int != 0,
                named: 0 as libc::c_int != 0,
                supertype: false,
            };
            init
        };
    } else {
        return *((*self_0).symbol_metadata).offset(symbol as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_public_symbol(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbol {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return symbol;
    }
    return *((*self_0).public_symbol_map).offset(symbol as isize);
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_next_state(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
    mut symbol: TSSymbol,
) -> TSStateId {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int
        || symbol as libc::c_int
            == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        return 0 as libc::c_int as TSStateId;
    } else if (symbol as libc::c_uint) < (*self_0).token_count {
        let mut count: uint32_t = 0;
        let mut actions: *const TSParseAction =
            ts_language_actions(self_0, state, symbol, &mut count);
        if count > 0 as libc::c_int as libc::c_uint {
            let mut action: TSParseAction =
                *actions.offset(count.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
            if action.type_ as libc::c_int == TSParseActionTypeShift as libc::c_int {
                return (if action.shift.extra as libc::c_int != 0 {
                    state as libc::c_int
                } else {
                    action.shift.state as libc::c_int
                }) as TSStateId;
            }
        }
        return 0 as libc::c_int as TSStateId;
    } else {
        return ts_language_lookup(self_0, state, symbol);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_name(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> *const libc::c_char {
    if symbol as libc::c_int == -(1 as libc::c_int) as TSSymbol as libc::c_int {
        return b"ERROR\0" as *const u8 as *const libc::c_char;
    } else if symbol as libc::c_int
        == -(1 as libc::c_int) as TSSymbol as libc::c_int - 1 as libc::c_int
    {
        return b"_ERROR\0" as *const u8 as *const libc::c_char;
    } else if (symbol as libc::c_uint) < ts_language_symbol_count(self_0) {
        return *((*self_0).symbol_names).offset(symbol as isize);
    } else {
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_for_name(
    mut self_0: *const TSLanguage,
    mut string: *const libc::c_char,
    mut length: uint32_t,
    mut is_named: bool,
) -> TSSymbol {
    if strncmp(
        string,
        b"ERROR\0" as *const u8 as *const libc::c_char,
        length as libc::c_ulong,
    ) == 0
    {
        return -(1 as libc::c_int) as TSSymbol;
    }
    let mut count: uint16_t = ts_language_symbol_count(self_0) as uint16_t;
    let mut i: TSSymbol = 0 as libc::c_int as TSSymbol;
    while (i as libc::c_int) < count as libc::c_int {
        let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(self_0, i);
        if !(!metadata.visible && !metadata.supertype
            || metadata.named as libc::c_int != is_named as libc::c_int)
        {
            let mut symbol_name: *const libc::c_char = *((*self_0).symbol_names).offset(i as isize);
            if strncmp(symbol_name, string, length as libc::c_ulong) == 0
                && *symbol_name.offset(length as isize) == 0
            {
                return *((*self_0).public_symbol_map).offset(i as isize);
            }
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int as TSSymbol;
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_symbol_type(
    mut self_0: *const TSLanguage,
    mut symbol: TSSymbol,
) -> TSSymbolType {
    let mut metadata: TSSymbolMetadata = ts_language_symbol_metadata(self_0, symbol);
    if metadata.named as libc::c_int != 0 && metadata.visible as libc::c_int != 0 {
        return TSSymbolTypeRegular;
    } else if metadata.visible {
        return TSSymbolTypeAnonymous;
    } else {
        return TSSymbolTypeAuxiliary;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_name_for_id(
    mut self_0: *const TSLanguage,
    mut id: TSFieldId,
) -> *const libc::c_char {
    let mut count: uint32_t = ts_language_field_count(self_0);
    if count != 0 && id as libc::c_uint <= count {
        return *((*self_0).field_names).offset(id as isize);
    } else {
        return 0 as *const libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ts_language_field_id_for_name(
    mut self_0: *const TSLanguage,
    mut name: *const libc::c_char,
    mut name_length: uint32_t,
) -> TSFieldId {
    let mut count: uint16_t = ts_language_field_count(self_0) as uint16_t;
    let mut i: TSSymbol = 1 as libc::c_int as TSSymbol;
    while (i as libc::c_int) < count as libc::c_int + 1 as libc::c_int {
        match strncmp(
            name,
            *((*self_0).field_names).offset(i as isize),
            name_length as libc::c_ulong,
        ) {
            0 => {
                if *(*((*self_0).field_names).offset(i as isize)).offset(name_length as isize)
                    as libc::c_int
                    == 0 as libc::c_int
                {
                    return i;
                }
            }
            -1 => return 0 as libc::c_int as TSFieldId,
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int as TSFieldId;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_new(
    mut self_0: *const TSLanguage,
    mut state: TSStateId,
) -> *mut TSLookaheadIterator {
    if state as libc::c_uint >= (*self_0).state_count {
        return 0 as *mut TSLookaheadIterator;
    }
    let mut iterator: *mut LookaheadIterator =
        crate::core::alloc::ts_malloc(::std::mem::size_of::<LookaheadIterator>() as libc::c_ulong)
            as *mut LookaheadIterator;
    *iterator = ts_language_lookaheads(self_0, state);
    return iterator as *mut TSLookaheadIterator;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_delete(mut self_0: *mut TSLookaheadIterator) {
    crate::core::alloc::ts_free(self_0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_reset_state(
    mut self_0: *mut TSLookaheadIterator,
    mut state: TSStateId,
) -> bool {
    let mut iterator: *mut LookaheadIterator = self_0 as *mut LookaheadIterator;
    if state as libc::c_uint >= (*(*iterator).language).state_count {
        return 0 as libc::c_int != 0;
    }
    *iterator = ts_language_lookaheads((*iterator).language, state);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_language(
    mut self_0: *const TSLookaheadIterator,
) -> *const TSLanguage {
    let mut iterator: *const LookaheadIterator = self_0 as *const LookaheadIterator;
    return (*iterator).language;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_reset(
    mut self_0: *mut TSLookaheadIterator,
    mut language: *const TSLanguage,
    mut state: TSStateId,
) -> bool {
    if state as libc::c_uint >= (*language).state_count {
        return 0 as libc::c_int != 0;
    }
    let mut iterator: *mut LookaheadIterator = self_0 as *mut LookaheadIterator;
    *iterator = ts_language_lookaheads(language, state);
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_next(mut self_0: *mut TSLookaheadIterator) -> bool {
    let mut iterator: *mut LookaheadIterator = self_0 as *mut LookaheadIterator;
    return ts_lookahead_iterator__next(iterator);
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_current_symbol(
    mut self_0: *const TSLookaheadIterator,
) -> TSSymbol {
    let mut iterator: *const LookaheadIterator = self_0 as *const LookaheadIterator;
    return (*iterator).symbol;
}
#[no_mangle]
pub unsafe extern "C" fn ts_lookahead_iterator_current_symbol_name(
    mut self_0: *const TSLookaheadIterator,
) -> *const libc::c_char {
    let mut iterator: *const LookaheadIterator = self_0 as *const LookaheadIterator;
    return ts_language_symbol_name((*iterator).language, (*iterator).symbol);
}
