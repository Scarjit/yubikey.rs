// Copyright (c) 2014-2016 Yubico AB
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//   * Redistributions of source code must retain the above copyright
//     notice, this list of conditions and the following disclaimer.
//
//   * Redistributions in binary form must reproduce the above
//     copyright notice, this list of conditions and the following
//     disclaimer in the documentation and/or other materials provided
//     with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern {
    static mut _DefaultRuneLocale : Struct1;
    fn __maskrune(arg1 : i32, arg2 : usize) -> i32;
    static mut __stderrp : *mut __sFILE;
    fn __swbuf(arg1 : i32, arg2 : *mut __sFILE) -> i32;
    fn __tolower(arg1 : i32) -> i32;
    fn __toupper(arg1 : i32) -> i32;
    fn _ykpiv_alloc(
        state : *mut ykpiv_state, size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn _ykpiv_begin_transaction(state : *mut ykpiv_state) -> Enum5;
    fn _ykpiv_end_transaction(state : *mut ykpiv_state) -> Enum5;
    fn _ykpiv_ensure_application_selected(
        state : *mut ykpiv_state
    ) -> Enum5;
    fn _ykpiv_fetch_object(
        state : *mut ykpiv_state,
        object_id : i32,
        data : *mut u8,
        len : *mut usize
    ) -> Enum5;
    fn _ykpiv_free(
        state : *mut ykpiv_state, data : *mut ::std::os::raw::c_void
    );
    fn _ykpiv_get_length(buffer : *const u8, len : *mut usize) -> u32;
    fn _ykpiv_has_valid_length(
        buffer : *const u8, len : usize
    ) -> bool;
    fn _ykpiv_prng_generate(buffer : *mut u8, cb_req : usize) -> Enum7;
    fn _ykpiv_realloc(
        state : *mut ykpiv_state,
        address : *mut ::std::os::raw::c_void,
        size : usize
    ) -> *mut ::std::os::raw::c_void;
    fn _ykpiv_save_object(
        state : *mut ykpiv_state,
        object_id : i32,
        indata : *mut u8,
        len : usize
    ) -> Enum5;
    fn _ykpiv_set_length(buffer : *mut u8, length : usize) -> u32;
    fn _ykpiv_transfer_data(
        state : *mut ykpiv_state,
        templ : *const u8,
        in_data : *const u8,
        in_len : isize,
        out_data : *mut u8,
        out_len : *mut usize,
        sw : *mut i32
    ) -> Enum5;
    fn fprintf(arg1 : *mut __sFILE, arg2 : *const u8, ...) -> i32;
    fn memcpy(
        __dst : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memmove(
        __dst : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __len : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset_s(
        __s : *mut ::std::os::raw::c_void,
        __smax : usize,
        __c : i32,
        __n : usize
    ) -> i32;
    fn pkcs5_pbkdf2_sha1(
        password : *const u8,
        cb_password : usize,
        salt : *const u8,
        cb_salt : usize,
        iterations : usize,
        key : *const u8,
        cb_key : usize
    ) -> Enum11;
    fn setting_get_bool(
        sz_setting : *const u8, f_default : bool
    ) -> _setting_bool_t;
    fn time(arg1 : *mut isize) -> isize;
    fn ykpiv_change_puk(
        state : *mut ykpiv_state,
        current_puk : *const u8,
        current_puk_len : usize,
        new_puk : *const u8,
        new_puk_len : usize,
        tries : *mut i32
    ) -> Enum5;
    fn ykpiv_set_mgmkey(
        state : *mut ykpiv_state, new_key : *const u8
    ) -> Enum5;
    fn ykpiv_transfer_data(
        state : *mut ykpiv_state,
        templ : *const u8,
        in_data : *const u8,
        in_len : isize,
        out_data : *mut u8,
        out_len : *mut usize,
        sw : *mut i32
    ) -> Enum5;
}

enum __sFILEX {
}

#[derive(Copy)]
#[repr(C)]
pub struct __sbuf {
    pub _base : *mut u8,
    pub _size : i32,
}

impl Clone for __sbuf {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct __sFILE {
    pub _p : *mut u8,
    pub _r : i32,
    pub _w : i32,
    pub _flags : i16,
    pub _file : i16,
    pub _bf : __sbuf,
    pub _lbfsize : i32,
    pub _cookie : *mut ::std::os::raw::c_void,
    pub _close : unsafe extern fn(*mut ::std::os::raw::c_void) -> i32,
    pub _read : unsafe extern fn(*mut ::std::os::raw::c_void, *mut u8, i32) -> i32,
    pub _seek : unsafe extern fn(*mut ::std::os::raw::c_void, isize, i32) -> isize,
    pub _write : unsafe extern fn(*mut ::std::os::raw::c_void, *const u8, i32) -> i32,
    pub _ub : __sbuf,
    pub _extra : *mut __sFILEX,
    pub _ur : i32,
    pub _ubuf : [u8; 3],
    pub _nbuf : [u8; 1],
    pub _lb : __sbuf,
    pub _blksize : i32,
    pub _offset : isize,
}

impl Clone for __sFILE {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __sputc(
    mut _c : i32, mut _p : *mut __sFILE
) -> i32 {
    if {
           (*_p)._w = (*_p)._w - 1;
           (*_p)._w
       } >= 0i32 || (*_p)._w >= (*_p)._lbfsize && (_c as (u8) as (i32) != b'\n' as (i32)) {
        ({
             let _rhs = _c;
             let _lhs
                 = &mut *{
                             let _old = (*_p)._p;
                             (*_p)._p = (*_p)._p.offset(1isize);
                             _old
                         };
             *_lhs = _rhs as (u8);
             *_lhs
         }) as (i32)
    } else {
        __swbuf(_c,_p)
    }
}

#[no_mangle]
pub unsafe extern fn isascii(mut _c : i32) -> i32 {
    (_c & !0x7fi32 == 0i32) as (i32)
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub __min : i32,
    pub __max : i32,
    pub __map : i32,
    pub __types : *mut u32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub __nranges : i32,
    pub __ranges : *mut Struct3,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub __name : [u8; 14],
    pub __mask : u32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub __magic : [u8; 8],
    pub __encoding : [u8; 32],
    pub __sgetrune : unsafe extern fn(*const u8, usize, *mut *const u8) -> i32,
    pub __sputrune : unsafe extern fn(i32, *mut u8, usize, *mut *mut u8) -> i32,
    pub __invalid_rune : i32,
    pub __runetype : [u32; 256],
    pub __maplower : [i32; 256],
    pub __mapupper : [i32; 256],
    pub __runetype_ext : Struct2,
    pub __maplower_ext : Struct2,
    pub __mapupper_ext : Struct2,
    pub __variable : *mut ::std::os::raw::c_void,
    pub __variable_len : i32,
    pub __ncharclasses : i32,
    pub __charclasses : *mut Struct4,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn __istype(mut _c : i32, mut _f : usize) -> i32 {
    if isascii(_c) != 0 {
        !(_DefaultRuneLocale.__runetype[
              _c as (usize)
          ] as (usize) & _f == 0) as (i32)
    } else {
        !(__maskrune(_c,_f) == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn __isctype(mut _c : i32, mut _f : usize) -> i32 {
    if _c < 0i32 || _c >= 256i32 {
        0i32
    } else {
        !(_DefaultRuneLocale.__runetype[
              _c as (usize)
          ] as (usize) & _f == 0) as (i32)
    }
}

#[no_mangle]
pub unsafe extern fn __wcwidth(mut _c : i32) -> i32 {
    let mut _x : u32;
    if _c == 0i32 {
        0i32
    } else {
        _x = __maskrune(_c,0xe0000000usize | 0x40000usize) as (u32);
        (if _x as (usize) & 0xe0000000usize != 0usize {
             ((_x as (usize) & 0xe0000000usize) >> 30i32) as (i32)
         } else if _x as (usize) & 0x40000usize != 0usize {
             1i32
         } else {
             -1i32
         })
    }
}

#[no_mangle]
pub unsafe extern fn isalnum(mut _c : i32) -> i32 {
    __istype(_c,(0x100isize | 0x400isize) as (usize))
}

#[no_mangle]
pub unsafe extern fn isalpha(mut _c : i32) -> i32 {
    __istype(_c,0x100usize)
}

#[no_mangle]
pub unsafe extern fn isblank(mut _c : i32) -> i32 {
    __istype(_c,0x20000usize)
}

#[no_mangle]
pub unsafe extern fn iscntrl(mut _c : i32) -> i32 {
    __istype(_c,0x200usize)
}

#[no_mangle]
pub unsafe extern fn isdigit(mut _c : i32) -> i32 {
    __isctype(_c,0x400usize)
}

#[no_mangle]
pub unsafe extern fn isgraph(mut _c : i32) -> i32 {
    __istype(_c,0x800usize)
}

#[no_mangle]
pub unsafe extern fn islower(mut _c : i32) -> i32 {
    __istype(_c,0x1000usize)
}

#[no_mangle]
pub unsafe extern fn isprint(mut _c : i32) -> i32 {
    __istype(_c,0x40000usize)
}

#[no_mangle]
pub unsafe extern fn ispunct(mut _c : i32) -> i32 {
    __istype(_c,0x2000usize)
}

#[no_mangle]
pub unsafe extern fn isspace(mut _c : i32) -> i32 {
    __istype(_c,0x4000usize)
}

#[no_mangle]
pub unsafe extern fn isupper(mut _c : i32) -> i32 {
    __istype(_c,0x8000usize)
}

#[no_mangle]
pub unsafe extern fn isxdigit(mut _c : i32) -> i32 {
    __isctype(_c,0x10000usize)
}

#[no_mangle]
pub unsafe extern fn toascii(mut _c : i32) -> i32 { _c & 0x7fi32 }

#[no_mangle]
pub unsafe extern fn tolower(mut _c : i32) -> i32 { __tolower(_c) }

#[no_mangle]
pub unsafe extern fn toupper(mut _c : i32) -> i32 { __toupper(_c) }

#[no_mangle]
pub unsafe extern fn digittoint(mut _c : i32) -> i32 {
    __maskrune(_c,0xfusize)
}

#[no_mangle]
pub unsafe extern fn ishexnumber(mut _c : i32) -> i32 {
    __istype(_c,0x10000usize)
}

#[no_mangle]
pub unsafe extern fn isideogram(mut _c : i32) -> i32 {
    __istype(_c,0x80000usize)
}

#[no_mangle]
pub unsafe extern fn isnumber(mut _c : i32) -> i32 {
    __istype(_c,0x400usize)
}

#[no_mangle]
pub unsafe extern fn isphonogram(mut _c : i32) -> i32 {
    __istype(_c,0x200000usize)
}

#[no_mangle]
pub unsafe extern fn isrune(mut _c : i32) -> i32 {
    __istype(_c,0xfffffff0usize)
}

#[no_mangle]
pub unsafe extern fn isspecial(mut _c : i32) -> i32 {
    __istype(_c,0x100000usize)
}

#[no_mangle]
pub static mut CHUID_TMPL : *const u8 = 0x30i32 as (*const u8);

#[no_mangle]
pub static mut CCC_TMPL : *const u8 = 0xf0i32 as (*const u8);

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum5 {
    YKPIV_OK = 0i32,
    YKPIV_MEMORY_ERROR = -1i32,
    YKPIV_PCSC_ERROR = -2i32,
    YKPIV_SIZE_ERROR = -3i32,
    YKPIV_APPLET_ERROR = -4i32,
    YKPIV_AUTHENTICATION_ERROR = -5i32,
    YKPIV_RANDOMNESS_ERROR = -6i32,
    YKPIV_GENERIC_ERROR = -7i32,
    YKPIV_KEY_ERROR = -8i32,
    YKPIV_PARSE_ERROR = -9i32,
    YKPIV_WRONG_PIN = -10i32,
    YKPIV_INVALID_OBJECT = -11i32,
    YKPIV_ALGORITHM_ERROR = -12i32,
    YKPIV_PIN_LOCKED = -13i32,
    YKPIV_ARGUMENT_ERROR = -14i32,
    YKPIV_RANGE_ERROR = -15i32,
    YKPIV_NOT_SUPPORTED = -16i32,
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub data : [u8; 16],
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_get_cardid(
    mut state : *mut ykpiv_state, mut cardid : *mut Struct6
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3063];
    let mut len : usize = ::std::mem::size_of::<[u8; 3063]>();
    if cardid.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            res = _ykpiv_fetch_object(
                      state,
                      0x5fc102i32,
                      buf.as_mut_ptr(),
                      &mut len as (*mut usize)
                  );
            if Enum5::YKPIV_OK as (i32) == res as (i32) {
                if len != 59usize {
                    res = Enum5::YKPIV_GENERIC_ERROR;
                } else {
                    memcpy(
                        (*cardid).data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                        buf.as_mut_ptr().offset(
                            29isize
                        ) as (*const ::std::os::raw::c_void),
                        16usize
                    );
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum7 {
    PRNG_OK = 0i32,
    PRNG_GENERAL_ERROR = -1i32,
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_set_cardid(
    mut state : *mut ykpiv_state, mut cardid : *const Struct6
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut id : [u8; 16];
    let mut buf : [u8; 59];
    let mut len : usize = 0usize;
    if state.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        if cardid.is_null() {
            if Enum7::PRNG_OK as (i32) != _ykpiv_prng_generate(
                                              id.as_mut_ptr(),
                                              ::std::mem::size_of::<[u8; 16]>()
                                          ) as (i32) {
                return Enum5::YKPIV_RANDOMNESS_ERROR;
            }
        } else {
            memcpy(
                id.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                (*cardid).data.as_mut_ptr() as (*const ::std::os::raw::c_void),
                ::std::mem::size_of::<[u8; 16]>()
            );
        }
        (if Enum5::YKPIV_OK as (i32) != {
                                            res = _ykpiv_begin_transaction(state);
                                            res
                                        } as (i32) {
             Enum5::YKPIV_PCSC_ERROR
         } else {
             if !(Enum5::YKPIV_OK as (i32) != {
                                                  res = _ykpiv_ensure_application_selected(state);
                                                  res
                                              } as (i32)) {
                 memcpy(
                     buf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                     CHUID_TMPL as (*const ::std::os::raw::c_void),
                     59usize
                 );
                 memcpy(
                     buf.as_mut_ptr().offset(29isize) as (*mut ::std::os::raw::c_void),
                     id.as_mut_ptr() as (*const ::std::os::raw::c_void),
                     ::std::mem::size_of::<[u8; 16]>()
                 );
                 len = 59usize;
                 res = _ykpiv_save_object(state,0x5fc102i32,buf.as_mut_ptr(),len);
             }
             _ykpiv_end_transaction(state);
             res
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
    pub data : [u8; 14],
}

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_get_cccid(
    mut state : *mut ykpiv_state, mut ccc : *mut Struct8
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3063];
    let mut len : usize = ::std::mem::size_of::<[u8; 3063]>();
    if ccc.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            res = _ykpiv_fetch_object(
                      state,
                      0x5fc107i32,
                      buf.as_mut_ptr(),
                      &mut len as (*mut usize)
                  );
            if Enum5::YKPIV_OK as (i32) == res as (i32) {
                if len != 51usize {
                    res = Enum5::YKPIV_GENERIC_ERROR;
                } else {
                    memcpy(
                        (*ccc).data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                        buf.as_mut_ptr().offset(9isize) as (*const ::std::os::raw::c_void),
                        14usize
                    );
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_set_cccid(
    mut state : *mut ykpiv_state, mut ccc : *const Struct8
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut id : [u8; 14];
    let mut buf : [u8; 51];
    let mut len : usize = 0usize;
    if state.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        if ccc.is_null() {
            if Enum7::PRNG_OK as (i32) != _ykpiv_prng_generate(
                                              id.as_mut_ptr(),
                                              ::std::mem::size_of::<[u8; 14]>()
                                          ) as (i32) {
                return Enum5::YKPIV_RANDOMNESS_ERROR;
            }
        } else {
            memcpy(
                id.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                (*ccc).data.as_mut_ptr() as (*const ::std::os::raw::c_void),
                ::std::mem::size_of::<[u8; 14]>()
            );
        }
        (if Enum5::YKPIV_OK as (i32) != {
                                            res = _ykpiv_begin_transaction(state);
                                            res
                                        } as (i32) {
             Enum5::YKPIV_PCSC_ERROR
         } else {
             if !(Enum5::YKPIV_OK as (i32) != {
                                                  res = _ykpiv_ensure_application_selected(state);
                                                  res
                                              } as (i32)) {
                 len = 51usize;
                 memcpy(
                     buf.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                     CCC_TMPL as (*const ::std::os::raw::c_void),
                     len
                 );
                 memcpy(
                     buf.as_mut_ptr().offset(9isize) as (*mut ::std::os::raw::c_void),
                     id.as_mut_ptr() as (*const ::std::os::raw::c_void),
                     14usize
                 );
                 res = _ykpiv_save_object(state,0x5fc107i32,buf.as_mut_ptr(),len);
             }
             _ykpiv_end_transaction(state);
             res
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct ykpiv_allocator {
    pub pfn_alloc : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub pfn_realloc : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub pfn_free : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub alloc_data : *mut ::std::os::raw::c_void,
}

impl Clone for ykpiv_allocator {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct _ykpiv_version_t {
    pub major : u8,
    pub minor : u8,
    pub patch : u8,
}

impl Clone for _ykpiv_version_t {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct ykpiv_state {
    pub context : i32,
    pub card : i32,
    pub verbose : i32,
    pub pin : *mut u8,
    pub allocator : ykpiv_allocator,
    pub isNEO : bool,
    pub ver : _ykpiv_version_t,
    pub serial : u32,
}

impl Clone for ykpiv_state {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_devicemodel(
    mut state : *mut ykpiv_state
) -> u32 {
    if state.is_null(
       ) || (*state).context == 0 || (*state).context as (usize) == -1i32 as (usize) {
        0x0u32
    } else {
        (if (*state).isNEO {
             0x4e450000i32 | 0x7233i32
         } else {
             0x594b0000i32 | 0x34i32
         }) as (u32)
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _ykpiv_key {
    pub slot : u8,
    pub cert_len : u16,
    pub cert : [u8; 1],
}

impl Clone for _ykpiv_key {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_list_keys(
    mut state : *mut ykpiv_state,
    mut key_count : *mut u8,
    mut data : *mut *mut _ykpiv_key,
    mut data_len : *mut usize
) -> Enum5 {
    let mut _currentBlock;
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut pKey
        : *mut _ykpiv_key
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_key);
    let mut pData
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut pTemp
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cbData : usize = 0usize;
    let mut offset : usize = 0usize;
    let mut buf : [u8; 3072];
    let mut cbBuf : usize = 0usize;
    let mut i : usize = 0usize;
    let mut cbRealloc : usize = 0usize;
    let CB_PAGE : usize = 4096usize;
    let mut SLOTS : *const u8 = 0x9ai32 as (*const u8);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut _ykpiv_key) == data || 0i32 as (*mut ::std::os::raw::c_void) as (*mut usize) == data_len || 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == key_count {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            *key_count = 0u8;
            *data = 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_key);
            *data_len = 0usize;
            if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == {
                                                                         pData = _ykpiv_alloc(
                                                                                     state,
                                                                                     CB_PAGE
                                                                                 ) as (*mut u8);
                                                                         pData
                                                                     } {
                res = Enum5::YKPIV_MEMORY_ERROR;
            } else {
                cbData = CB_PAGE;
                i = 0usize;
                'loop5: loop {
                    if !(i < ::std::mem::size_of::<*const u8>()) {
                        _currentBlock = 6;
                        break;
                    }
                    cbBuf = ::std::mem::size_of::<[u8; 3072]>();
                    res = _read_certificate(
                              state,
                              *SLOTS.offset(i as (isize)),
                              buf.as_mut_ptr(),
                              &mut cbBuf as (*mut usize)
                          );
                    if res as (i32) == Enum5::YKPIV_OK as (i32) && (cbBuf > 0usize) {
                        cbRealloc = if ::std::mem::size_of::<_ykpiv_key>().wrapping_add(
                                           cbBuf
                                       ).wrapping_sub(
                                           1usize
                                       ) > cbData.wrapping_sub(offset) {
                                        (if ::std::mem::size_of::<_ykpiv_key>().wrapping_add(
                                                cbBuf
                                            ).wrapping_sub(
                                                1usize
                                            ).wrapping_sub(
                                                cbData.wrapping_sub(offset)
                                            ) > CB_PAGE {
                                             ::std::mem::size_of::<_ykpiv_key>().wrapping_add(
                                                 cbBuf
                                             ).wrapping_sub(
                                                 1usize
                                             ).wrapping_sub(
                                                 cbData.wrapping_sub(offset)
                                             )
                                         } else {
                                             CB_PAGE
                                         })
                                    } else {
                                        0usize
                                    };
                        if 0usize != cbRealloc {
                            if {
                                   pTemp = _ykpiv_realloc(
                                               state,
                                               pData as (*mut ::std::os::raw::c_void),
                                               cbData.wrapping_add(cbRealloc)
                                           ) as (*mut u8);
                                   pTemp
                               }.is_null(
                               ) {
                                _currentBlock = 15;
                                break;
                            }
                            pData = pTemp;
                            pTemp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                        }
                        cbData = cbData.wrapping_add(cbRealloc);
                        pKey = pData.offset(offset as (isize)) as (*mut _ykpiv_key);
                        (*pKey).slot = *SLOTS.offset(i as (isize));
                        (*pKey).cert_len = cbBuf as (u16);
                        memcpy(
                            (*pKey).cert.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                            buf.as_mut_ptr() as (*const ::std::os::raw::c_void),
                            cbBuf
                        );
                        offset = offset.wrapping_add(
                                     ::std::mem::size_of::<_ykpiv_key>().wrapping_add(
                                         cbBuf
                                     ).wrapping_sub(
                                         1usize
                                     )
                                 );
                        *key_count = (*key_count as (i32) + 1) as (u8);
                    }
                    i = i.wrapping_add(1usize);
                }
                if _currentBlock == 6 {
                    *data = pData as (*mut _ykpiv_key);
                    pData = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    if !data_len.is_null() {
                        *data_len = offset;
                    }
                    res = Enum5::YKPIV_OK;
                } else {
                    res = Enum5::YKPIV_MEMORY_ERROR;
                }
            }
        }
        if !pData.is_null() {
            _ykpiv_free(state,pData as (*mut ::std::os::raw::c_void));
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_free(
    mut state : *mut ykpiv_state,
    mut data : *mut ::std::os::raw::c_void
) -> Enum5 {
    if data.is_null() {
        Enum5::YKPIV_OK
    } else if state.is_null() || (*state).allocator.pfn_free == 0 {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        _ykpiv_free(state,data);
        Enum5::YKPIV_OK
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_read_cert(
    mut state : *mut ykpiv_state,
    mut slot : u8,
    mut data : *mut *mut u8,
    mut data_len : *mut usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3072];
    let mut cbBuf : usize = ::std::mem::size_of::<[u8; 3072]>();
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut u8) == data || 0i32 as (*mut ::std::os::raw::c_void) as (*mut usize) == data_len {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            *data = 0i32 as (*mut u8);
            *data_len = 0usize;
            if Enum5::YKPIV_OK as (i32) == {
                                               res = _read_certificate(
                                                         state,
                                                         slot,
                                                         buf.as_mut_ptr(),
                                                         &mut cbBuf as (*mut usize)
                                                     );
                                               res
                                           } as (i32) {
                if cbBuf == 0usize {
                    *data = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    *data_len = 0usize;
                } else if {
                              *data = _ykpiv_alloc(state,cbBuf) as (*mut u8);
                              *data
                          }.is_null(
                          ) {
                    res = Enum5::YKPIV_MEMORY_ERROR;
                } else {
                    memcpy(
                        *data as (*mut ::std::os::raw::c_void),
                        buf.as_mut_ptr() as (*const ::std::os::raw::c_void),
                        cbBuf
                    );
                    *data_len = cbBuf;
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_write_cert(
    mut state : *mut ykpiv_state,
    mut slot : u8,
    mut data : *mut u8,
    mut data_len : usize,
    mut certinfo : u8
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    if Enum5::YKPIV_OK as (i32) != {
                                       res = _ykpiv_begin_transaction(state);
                                       res
                                   } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            res = _write_certificate(state,slot,data,data_len,certinfo);
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_delete_cert(
    mut state : *mut ykpiv_state, mut slot : u8
) -> Enum5 {
    ykpiv_util_write_cert(
        state,
        slot,
        0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
        0usize,
        0u8
    )
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_block_puk(
    mut state : *mut ykpiv_state
) -> Enum5 {
    let mut _currentBlock;
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut puk : *mut u8 = 0x30i32 as (*mut u8);
    let mut tries : i32 = -1i32;
    let mut data : [u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut p_item
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_item : usize = 0usize;
    let mut flags : u8 = 0u8;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if Enum5::YKPIV_OK as (i32) != {
                                           res = _ykpiv_ensure_application_selected(state);
                                           res
                                       } as (i32) {
            _currentBlock = 20;
        } else {
            _currentBlock = 3;
        }
        'loop3: loop {
            if _currentBlock == 3 {
                if tries != 0i32 {
                    if Enum5::YKPIV_OK as (i32) == {
                                                       res = ykpiv_change_puk(
                                                                 state,
                                                                 puk as (*const u8),
                                                                 ::std::mem::size_of::<*mut u8>(),
                                                                 puk as (*const u8),
                                                                 ::std::mem::size_of::<*mut u8>(),
                                                                 &mut tries as (*mut i32)
                                                             );
                                                       res
                                                   } as (i32) {
                        let _rhs = 1;
                        let _lhs = &mut *puk.offset(0isize);
                        *_lhs = (*_lhs as (i32) + _rhs) as (u8);
                        _currentBlock = 3;
                    } else {
                        if !(Enum5::YKPIV_PIN_LOCKED as (i32) == res as (i32)) {
                            _currentBlock = 3;
                            continue;
                        }
                        tries = 0i32;
                        res = Enum5::YKPIV_OK;
                        _currentBlock = 3;
                    }
                } else {
                    if Enum5::YKPIV_OK as (i32) == _read_metadata(
                                                       state,
                                                       0x80u8,
                                                       data.as_mut_ptr(),
                                                       &mut cb_data as (*mut usize)
                                                   ) as (i32) {
                        if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                           data.as_mut_ptr(),
                                                           cb_data,
                                                           0x81u8,
                                                           &mut p_item as (*mut *mut u8),
                                                           &mut cb_item as (*mut usize)
                                                       ) as (i32) {
                            if ::std::mem::size_of::<u8>() == cb_item {
                                memcpy(
                                    &mut flags as (*mut u8) as (*mut ::std::os::raw::c_void),
                                    p_item as (*const ::std::os::raw::c_void),
                                    cb_item
                                );
                            } else if (*state).verbose != 0 {
                                fprintf(
                                    __stderrp,
                                    (*b"admin flags exist, but are incorrect size = %lu\0").as_ptr(
                                    ),
                                    cb_item
                                );
                            }
                        }
                    }
                    flags = (flags as (i32) | 0x1i32) as (u8);
                    if Enum5::YKPIV_OK as (i32) != _set_metadata_item(
                                                       data.as_mut_ptr(),
                                                       &mut cb_data as (*mut usize),
                                                       3063usize,
                                                       0x81u8,
                                                       &mut flags as (*mut u8),
                                                       ::std::mem::size_of::<u8>()
                                                   ) as (i32) {
                        if (*state).verbose == 0 {
                            _currentBlock = 20;
                            continue;
                        }
                        fprintf(__stderrp,(*b"could not set admin flags\0").as_ptr());
                        _currentBlock = 20;
                    } else {
                        if !(Enum5::YKPIV_OK as (i32) != _write_metadata(
                                                             state,
                                                             0x80u8,
                                                             data.as_mut_ptr(),
                                                             cb_data
                                                         ) as (i32)) {
                            _currentBlock = 20;
                            continue;
                        }
                        if (*state).verbose == 0 {
                            _currentBlock = 20;
                            continue;
                        }
                        fprintf(__stderrp,(*b"could not write admin metadata\0").as_ptr());
                        _currentBlock = 20;
                    }
                }
            } else {
                _ykpiv_end_transaction(state);
                return res;
            }
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _ykpiv_container {
    pub name : [i32; 40],
    pub slot : u8,
    pub key_spec : u8,
    pub key_size_bits : u16,
    pub flags : u8,
    pub pin_id : u8,
    pub associated_echd_container : u8,
    pub cert_fingerprint : [u8; 20],
}

impl Clone for _ykpiv_container {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_read_mscmap(
    mut state : *mut ykpiv_state,
    mut containers : *mut *mut _ykpiv_container,
    mut n_containers : *mut usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3072];
    let mut cbBuf : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut len : usize = 0usize;
    let mut ptr
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut *mut _ykpiv_container) == containers || 0i32 as (*mut ::std::os::raw::c_void) as (*mut usize) == n_containers {
        res = Enum5::YKPIV_GENERIC_ERROR;
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        return Enum5::YKPIV_PCSC_ERROR;
    } else if !(Enum5::YKPIV_OK as (i32) != {
                                                res = _ykpiv_ensure_application_selected(state);
                                                res
                                            } as (i32)) {
        *containers = 0i32 as (*mut _ykpiv_container);
        *n_containers = 0usize;
        if Enum5::YKPIV_OK as (i32) == {
                                           res = _ykpiv_fetch_object(
                                                     state,
                                                     0x5fff10i32,
                                                     buf.as_mut_ptr(),
                                                     &mut cbBuf as (*mut usize)
                                                 );
                                           res
                                       } as (i32) {
            ptr = buf.as_mut_ptr();
            if cbBuf < 2usize {
                res = Enum5::YKPIV_OK;
            } else if *{
                           let _old = ptr;
                           ptr = ptr.offset(1isize);
                           _old
                       } as (i32) == 0x81i32 {
                ptr = ptr.offset(
                          _ykpiv_get_length(
                              ptr as (*const u8),
                              &mut len as (*mut usize)
                          ) as (usize) as (isize)
                      );
                if len > cbBuf.wrapping_sub(
                             ((ptr as (isize)).wrapping_sub(
                                  buf.as_mut_ptr() as (isize)
                              ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                         ) {
                    res = Enum5::YKPIV_OK;
                } else if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_container) == {
                                                                                                  *containers = _ykpiv_alloc(
                                                                                                                    state,
                                                                                                                    len
                                                                                                                ) as (*mut _ykpiv_container);
                                                                                                  *containers
                                                                                              } {
                    res = Enum5::YKPIV_MEMORY_ERROR;
                } else {
                    memcpy(
                        *containers as (*mut ::std::os::raw::c_void),
                        ptr as (*const ::std::os::raw::c_void),
                        len
                    );
                    *n_containers = len.wrapping_div(
                                        ::std::mem::size_of::<_ykpiv_container>()
                                    );
                }
            }
        }
    }
    _ykpiv_end_transaction(state);
    res
}

unsafe extern fn _obj_size_max(
    mut state : *mut ykpiv_state
) -> usize {
    (if !state.is_null() && (*state).isNEO {
         2048i32 - 9i32
     } else {
         3063i32
     }) as (usize)
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_write_mscmap(
    mut state : *mut ykpiv_state,
    mut containers : *mut _ykpiv_container,
    mut n_containers : usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3063];
    let mut offset : usize = 0usize;
    let mut req_len : usize = 0usize;
    let mut data_len
        : usize
        = n_containers.wrapping_mul(
              ::std::mem::size_of::<_ykpiv_container>()
          );
    if Enum5::YKPIV_OK as (i32) != {
                                       res = _ykpiv_begin_transaction(state);
                                       res
                                   } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_container) == containers || 0usize == n_containers {
                if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_container) != containers || 0usize != n_containers {
                    res = Enum5::YKPIV_GENERIC_ERROR;
                } else {
                    res = _ykpiv_save_object(
                              state,
                              0x5fff10i32,
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                              0usize
                          );
                }
            } else {
                req_len = 1usize.wrapping_add(
                              _ykpiv_set_length(buf.as_mut_ptr(),data_len) as (usize)
                          ).wrapping_add(
                              data_len
                          );
                if req_len > _obj_size_max(state) {
                    res = Enum5::YKPIV_SIZE_ERROR;
                } else {
                    buf[
                        {
                            let _old = offset;
                            offset = offset.wrapping_add(1usize);
                            _old
                        }
                    ] = 0x81u8;
                    offset = offset.wrapping_add(
                                 _ykpiv_set_length(
                                     buf.as_mut_ptr().offset(offset as (isize)),
                                     data_len
                                 ) as (usize)
                             );
                    memcpy(
                        buf.as_mut_ptr().offset(
                            offset as (isize)
                        ) as (*mut ::std::os::raw::c_void),
                        containers as (*mut u8) as (*const ::std::os::raw::c_void),
                        data_len
                    );
                    offset = offset.wrapping_add(data_len);
                    res = _ykpiv_save_object(
                              state,
                              0x5fff10i32,
                              buf.as_mut_ptr(),
                              offset
                          );
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_read_msroots(
    mut state : *mut ykpiv_state,
    mut data : *mut *mut u8,
    mut data_len : *mut usize
) -> Enum5 {
    let mut _currentBlock;
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3072];
    let mut cbBuf : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut len : usize = 0usize;
    let mut ptr
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut object_id : i32 = 0i32;
    let mut tag : u8 = 0u8;
    let mut pData
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut pTemp
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cbData : usize = 0usize;
    let mut cbRealloc : usize = 0usize;
    let mut offset : usize = 0usize;
    if data.is_null() || data_len.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            *data = 0i32 as (*mut u8);
            *data_len = 0usize;
            cbData = _obj_size_max(state);
            if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == {
                                                                         pData = _ykpiv_alloc(
                                                                                     state,
                                                                                     cbData
                                                                                 ) as (*mut u8);
                                                                         pData
                                                                     } {
                res = Enum5::YKPIV_MEMORY_ERROR;
            } else {
                object_id = 0x5fff11i32;
                'loop5: loop {
                    if !(object_id <= 0x5fff15i32) {
                        _currentBlock = 15;
                        break;
                    }
                    cbBuf = ::std::mem::size_of::<[u8; 3072]>();
                    if Enum5::YKPIV_OK as (i32) != {
                                                       res = _ykpiv_fetch_object(
                                                                 state,
                                                                 object_id,
                                                                 buf.as_mut_ptr(),
                                                                 &mut cbBuf as (*mut usize)
                                                             );
                                                       res
                                                   } as (i32) {
                        _currentBlock = 21;
                        break;
                    }
                    ptr = buf.as_mut_ptr();
                    if cbBuf < 2usize {
                        _currentBlock = 19;
                        break;
                    }
                    tag = *{
                               let _old = ptr;
                               ptr = ptr.offset(1isize);
                               _old
                           };
                    if 0x83i32 != tag as (i32) && (0x82i32 != tag as (i32)) || 0x5fff15i32 == object_id && (0x82i32 != tag as (i32)) {
                        _currentBlock = 18;
                        break;
                    }
                    ptr = ptr.offset(
                              _ykpiv_get_length(
                                  ptr as (*const u8),
                                  &mut len as (*mut usize)
                              ) as (isize)
                          );
                    if len > cbBuf.wrapping_sub(
                                 ((ptr as (isize)).wrapping_sub(
                                      buf.as_mut_ptr() as (isize)
                                  ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                             ) {
                        _currentBlock = 17;
                        break;
                    }
                    cbRealloc = if len > cbData.wrapping_sub(offset) {
                                    len.wrapping_sub(cbData.wrapping_sub(offset))
                                } else {
                                    0usize
                                };
                    if 0usize != cbRealloc {
                        if {
                               pTemp = _ykpiv_realloc(
                                           state,
                                           pData as (*mut ::std::os::raw::c_void),
                                           cbData.wrapping_add(cbRealloc)
                                       ) as (*mut u8);
                               pTemp
                           }.is_null(
                           ) {
                            _currentBlock = 16;
                            break;
                        }
                        pData = pTemp;
                        pTemp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    }
                    cbData = cbData.wrapping_add(cbRealloc);
                    memcpy(
                        pData.offset(offset as (isize)) as (*mut ::std::os::raw::c_void),
                        ptr as (*const ::std::os::raw::c_void),
                        len
                    );
                    offset = offset.wrapping_add(len);
                    if 0x82i32 == tag as (i32) {
                        _currentBlock = 15;
                        break;
                    }
                    object_id = object_id + 1;
                }
                if _currentBlock == 21 {
                } else if _currentBlock == 15 {
                    *data = pData;
                    pData = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                    *data_len = offset;
                    res = Enum5::YKPIV_OK;
                } else if _currentBlock == 16 {
                    res = Enum5::YKPIV_MEMORY_ERROR;
                } else if _currentBlock == 17 {
                    res = Enum5::YKPIV_OK;
                } else if _currentBlock == 18 {
                    res = Enum5::YKPIV_OK;
                } else {
                    res = Enum5::YKPIV_OK;
                }
            }
        }
        if !pData.is_null() {
            _ykpiv_free(state,pData as (*mut ::std::os::raw::c_void));
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_write_msroots(
    mut state : *mut ykpiv_state,
    mut data : *mut u8,
    mut data_len : usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf : [u8; 3063];
    let mut offset : usize = 0usize;
    let mut data_offset : usize = 0usize;
    let mut data_chunk : usize = 0usize;
    let mut n_objs : usize = 0usize;
    let mut i : u32 = 0u32;
    let mut cb_obj_max : usize = _obj_size_max(state);
    if Enum5::YKPIV_OK as (i32) != {
                                       res = _ykpiv_begin_transaction(state);
                                       res
                                   } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == data || 0usize == data_len {
                if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) != data || 0usize != data_len {
                    res = Enum5::YKPIV_GENERIC_ERROR;
                } else {
                    res = _ykpiv_save_object(
                              state,
                              0x5fff11i32,
                              0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                              0usize
                          );
                }
            } else {
                n_objs = data_len.wrapping_div(
                             cb_obj_max.wrapping_sub((2i32 + 2i32) as (usize))
                         ).wrapping_add(
                             1usize
                         );
                if n_objs > 5usize {
                    res = Enum5::YKPIV_SIZE_ERROR;
                } else {
                    i = 0u32;
                    'loop5: loop {
                        if !(i as (usize) < n_objs) {
                            break;
                        }
                        offset = 0usize;
                        data_chunk = if cb_obj_max.wrapping_sub(
                                            (2i32 + 2i32) as (usize)
                                        ) < data_len.wrapping_sub(data_offset) {
                                         cb_obj_max.wrapping_sub((2i32 + 2i32) as (usize))
                                     } else {
                                         data_len.wrapping_sub(data_offset)
                                     };
                        buf[
                            {
                                let _old = offset;
                                offset = offset.wrapping_add(1usize);
                                _old
                            }
                        ] = if i as (usize) == n_objs.wrapping_sub(1usize) {
                                0x82i32
                            } else {
                                0x83i32
                            } as (u8);
                        offset = offset.wrapping_add(
                                     _ykpiv_set_length(
                                         buf.as_mut_ptr().offset(offset as (isize)),
                                         data_chunk
                                     ) as (usize)
                                 );
                        memcpy(
                            buf.as_mut_ptr().offset(
                                offset as (isize)
                            ) as (*mut ::std::os::raw::c_void),
                            data.offset(
                                data_offset as (isize)
                            ) as (*const ::std::os::raw::c_void),
                            data_chunk
                        );
                        offset = offset.wrapping_add(data_chunk);
                        res = _ykpiv_save_object(
                                  state,
                                  0x5fff11u32.wrapping_add(i) as (i32),
                                  buf.as_mut_ptr(),
                                  offset
                              );
                        if Enum5::YKPIV_OK as (i32) != res as (i32) {
                            break;
                        }
                        data_offset = data_offset.wrapping_add(data_chunk);
                        i = i.wrapping_add(1u32);
                    }
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum _setting_source_t {
    SETTING_SOURCE_USER,
    SETTING_SOURCE_ADMIN,
    SETTING_SOURCE_DEFAULT,
}

#[derive(Copy)]
#[repr(C)]
pub struct _setting_bool_t {
    pub value : bool,
    pub source : _setting_source_t,
}

impl Clone for _setting_bool_t {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_generate_key(
    mut state : *mut ykpiv_state,
    mut slot : u8,
    mut algorithm : u8,
    mut pin_policy : u8,
    mut touch_policy : u8,
    mut modulus : *mut *mut u8,
    mut modulus_len : *mut usize,
    mut exp : *mut *mut u8,
    mut exp_len : *mut usize,
    mut point : *mut *mut u8,
    mut point_len : *mut usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut in_data : [u8; 11];
    let mut in_ptr : *mut u8 = in_data.as_mut_ptr();
    let mut data : [u8; 1024];
    let mut templ : *mut u8 = 0i32 as (*mut u8);
    let mut recv_len : usize = ::std::mem::size_of::<[u8; 1024]>();
    let mut sw : i32;
    let mut ptr_modulus
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_modulus : usize = 0usize;
    let mut ptr_exp
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_exp : usize = 0usize;
    let mut ptr_point
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_point : usize = 0usize;
    let mut setting_roca = _setting_bool_t { value: false, source: _setting_source_t::SETTING_SOURCE_DEFAULT };
    let mut sz_setting_roca
        : *const u8
        = (*b"Enable_Unsafe_Keygen_ROCA\0").as_ptr();
    let mut sz_roca_format
        : *const u8
        = (*b"YubiKey serial number %u is affected by vulnerability CVE-2017-15361 (ROCA) and should be replaced. On-chip key generation %s  See YSA-2017-01 <https://www.yubico.com/support/security-advisories/ysa-2017-01/> for additional information on device replacement and mitigation assistance.\n\0").as_ptr(
          );
    let mut sz_roca_allow_user
        : *const u8
        = (*b"was permitted by an end-user configuration setting, but is not recommended.\0").as_ptr(
          );
    let mut sz_roca_allow_admin
        : *const u8
        = (*b"was permitted by an administrator configuration setting, but is not recommended.\0").as_ptr(
          );
    let mut sz_roca_block_user
        : *const u8
        = (*b"was blocked due to an end-user configuration setting.\0").as_ptr(
          );
    let mut sz_roca_block_admin
        : *const u8
        = (*b"was blocked due to an administrator configuration setting.\0").as_ptr(
          );
    let mut sz_roca_default
        : *const u8
        = (*b"was permitted by default, but is not recommended.  The default behavior will change in a future Yubico release.\0").as_ptr(
          );
    if state.is_null() {
        Enum5::YKPIV_ARGUMENT_ERROR
    } else {
        if ykpiv_util_devicemodel(
               state
           ) == (0x594b0000i32 | 0x34i32) as (u32) && (algorithm as (i32) == 0x6i32 || algorithm as (i32) == 0x7i32) {
            if (*state).ver.major as (i32) == 4i32 && ((*state).ver.minor as (i32) < 3i32 || (*state).ver.minor as (i32) == 3i32 && ((*state).ver.patch as (i32) < 5i32)) {
                let mut psz_msg
                    : *const u8
                    = 0i32 as (*mut ::std::os::raw::c_void) as (*const u8);
                setting_roca = setting_get_bool(sz_setting_roca,true);
                let switch9 = setting_roca.source;
                if switch9 as (i32) == _setting_source_t::SETTING_SOURCE_USER as (i32) {
                    psz_msg = if setting_roca.value {
                                  sz_roca_allow_user
                              } else {
                                  sz_roca_block_user
                              };
                } else if switch9 as (i32) == _setting_source_t::SETTING_SOURCE_ADMIN as (i32) {
                    psz_msg = if setting_roca.value {
                                  sz_roca_allow_admin
                              } else {
                                  sz_roca_block_admin
                              };
                } else {
                    psz_msg = sz_roca_default;
                }
                fprintf(__stderrp,sz_roca_format,(*state).serial,psz_msg);
                if !setting_roca.value {
                    return Enum5::YKPIV_NOT_SUPPORTED;
                }
            }
        }
        if algorithm as (i32) == 0x14i32 || algorithm as (i32) == 0x11i32 {
            if point.is_null() || point_len.is_null() {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"Invalid output parameter for ECC algorithm\0").as_ptr()
                    );
                }
                return Enum5::YKPIV_GENERIC_ERROR;
            } else {
                *point = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                *point_len = 0usize;
            }
        } else if algorithm as (i32) == 0x7i32 || algorithm as (i32) == 0x6i32 {
            if modulus.is_null() || modulus_len.is_null() || exp.is_null(
                                                             ) || exp_len.is_null() {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"Invalid output parameter for RSA algorithm\0").as_ptr()
                    );
                }
                return Enum5::YKPIV_GENERIC_ERROR;
            } else {
                *modulus = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                *modulus_len = 0usize;
                *exp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                *exp_len = 0usize;
            }
        } else {
            if (*state).verbose != 0 {
                fprintf(__stderrp,(*b"Invalid algorithm specified\0").as_ptr());
            }
            return Enum5::YKPIV_GENERIC_ERROR;
        }
        (if Enum5::YKPIV_OK as (i32) != {
                                            res = _ykpiv_begin_transaction(state);
                                            res
                                        } as (i32) {
             Enum5::YKPIV_PCSC_ERROR
         } else {
             if !(Enum5::YKPIV_OK as (i32) != {
                                                  res = _ykpiv_ensure_application_selected(state);
                                                  res
                                              } as (i32)) {
                 *templ.offset(3isize) = slot;
                 *{
                      let _old = in_ptr;
                      in_ptr = in_ptr.offset(1isize);
                      _old
                  } = 0xacu8;
                 *{
                      let _old = in_ptr;
                      in_ptr = in_ptr.offset(1isize);
                      _old
                  } = 3u8;
                 *{
                      let _old = in_ptr;
                      in_ptr = in_ptr.offset(1isize);
                      _old
                  } = 0x80u8;
                 *{
                      let _old = in_ptr;
                      in_ptr = in_ptr.offset(1isize);
                      _old
                  } = 1u8;
                 *{
                      let _old = in_ptr;
                      in_ptr = in_ptr.offset(1isize);
                      _old
                  } = algorithm;
                 if in_data[4usize] as (i32) == 0i32 {
                     res = Enum5::YKPIV_ALGORITHM_ERROR;
                     if (*state).verbose != 0 {
                         fprintf(__stderrp,(*b"Unexpected algorithm.\n\0").as_ptr());
                     }
                 } else {
                     if pin_policy as (i32) != 0i32 {
                         let _rhs = 3i32;
                         let _lhs = &mut in_data[1usize];
                         *_lhs = (*_lhs as (i32) + _rhs) as (u8);
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = 0xaau8;
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = 1u8;
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = pin_policy;
                     }
                     if touch_policy as (i32) != 0i32 {
                         let _rhs = 3i32;
                         let _lhs = &mut in_data[1usize];
                         *_lhs = (*_lhs as (i32) + _rhs) as (u8);
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = 0xabu8;
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = 1u8;
                         *{
                              let _old = in_ptr;
                              in_ptr = in_ptr.offset(1isize);
                              _old
                          } = touch_policy;
                     }
                     if Enum5::YKPIV_OK as (i32) != {
                                                        res = _ykpiv_transfer_data(
                                                                  state,
                                                                  templ as (*const u8),
                                                                  in_data.as_mut_ptr(
                                                                  ) as (*const u8),
                                                                  (in_ptr as (isize)).wrapping_sub(
                                                                      in_data.as_mut_ptr(
                                                                      ) as (isize)
                                                                  ) / ::std::mem::size_of::<u8>(
                                                                      ) as (isize),
                                                                  data.as_mut_ptr(),
                                                                  &mut recv_len as (*mut usize),
                                                                  &mut sw as (*mut i32)
                                                              );
                                                        res
                                                    } as (i32) {
                         if (*state).verbose != 0 {
                             fprintf(__stderrp,(*b"Failed to communicate.\n\0").as_ptr());
                         }
                     } else if sw != 0x9000i32 {
                         if (*state).verbose != 0 {
                             fprintf(__stderrp,(*b"Failed to generate new key (\0").as_ptr());
                         }
                         if sw == 0x6b00i32 {
                             res = Enum5::YKPIV_KEY_ERROR;
                             if (*state).verbose != 0 {
                                 fprintf(__stderrp,(*b"incorrect slot)\n\0").as_ptr());
                             }
                         } else if sw == 0x6a80i32 {
                             res = Enum5::YKPIV_ALGORITHM_ERROR;
                             if (*state).verbose != 0 {
                                 if pin_policy as (i32) != 0i32 {
                                     fprintf(
                                         __stderrp,
                                         (*b"pin policy not supported?)\n\0").as_ptr()
                                     );
                                 } else if touch_policy as (i32) != 0i32 {
                                     fprintf(
                                         __stderrp,
                                         (*b"touch policy not supported?)\n\0").as_ptr()
                                     );
                                 } else {
                                     fprintf(
                                         __stderrp,
                                         (*b"algorithm not supported?)\n\0").as_ptr()
                                     );
                                 }
                             }
                         } else if sw == 0x6982i32 {
                             res = Enum5::YKPIV_AUTHENTICATION_ERROR;
                             if (*state).verbose != 0 {
                                 fprintf(__stderrp,(*b"not authenticated)\n\0").as_ptr());
                             }
                         } else {
                             res = Enum5::YKPIV_GENERIC_ERROR;
                             if (*state).verbose != 0 {
                                 fprintf(__stderrp,(*b"error %x)\n\0").as_ptr(),sw);
                             }
                         }
                     } else if 0x6i32 == algorithm as (i32) || 0x7i32 == algorithm as (i32) {
                         let mut data_ptr : *mut u8 = data.as_mut_ptr().offset(5isize);
                         let mut len : usize = 0usize;
                         if *data_ptr as (i32) != 0x81i32 {
                             if (*state).verbose != 0 {
                                 fprintf(
                                     __stderrp,
                                     (*b"Failed to parse public key structure (modulus).\n\0").as_ptr(
                                     )
                                 );
                             }
                             res = Enum5::YKPIV_PARSE_ERROR;
                         } else {
                             data_ptr = data_ptr.offset(1isize);
                             data_ptr = data_ptr.offset(
                                            _ykpiv_get_length(
                                                data_ptr as (*const u8),
                                                &mut len as (*mut usize)
                                            ) as (isize)
                                        );
                             cb_modulus = len;
                             if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == {
                                                                                          ptr_modulus = _ykpiv_alloc(
                                                                                                            state,
                                                                                                            cb_modulus
                                                                                                        ) as (*mut u8);
                                                                                          ptr_modulus
                                                                                      } {
                                 if (*state).verbose != 0 {
                                     fprintf(
                                         __stderrp,
                                         (*b"Failed to allocate memory for modulus.\n\0").as_ptr()
                                     );
                                 }
                                 res = Enum5::YKPIV_MEMORY_ERROR;
                             } else {
                                 memcpy(
                                     ptr_modulus as (*mut ::std::os::raw::c_void),
                                     data_ptr as (*const ::std::os::raw::c_void),
                                     cb_modulus
                                 );
                                 data_ptr = data_ptr.offset(len as (isize));
                                 if *data_ptr as (i32) != 0x82i32 {
                                     if (*state).verbose != 0 {
                                         fprintf(
                                             __stderrp,
                                             (*b"Failed to parse public key structure (public exponent).\n\0").as_ptr(
                                             )
                                         );
                                     }
                                     res = Enum5::YKPIV_PARSE_ERROR;
                                 } else {
                                     data_ptr = data_ptr.offset(1isize);
                                     data_ptr = data_ptr.offset(
                                                    _ykpiv_get_length(
                                                        data_ptr as (*const u8),
                                                        &mut len as (*mut usize)
                                                    ) as (isize)
                                                );
                                     cb_exp = len;
                                     if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == {
                                                                                                  ptr_exp = _ykpiv_alloc(
                                                                                                                state,
                                                                                                                cb_exp
                                                                                                            ) as (*mut u8);
                                                                                                  ptr_exp
                                                                                              } {
                                         if (*state).verbose != 0 {
                                             fprintf(
                                                 __stderrp,
                                                 (*b"Failed to allocate memory for public exponent.\n\0").as_ptr(
                                                 )
                                             );
                                         }
                                         res = Enum5::YKPIV_MEMORY_ERROR;
                                     } else {
                                         memcpy(
                                             ptr_exp as (*mut ::std::os::raw::c_void),
                                             data_ptr as (*const ::std::os::raw::c_void),
                                             cb_exp
                                         );
                                         *modulus = ptr_modulus;
                                         ptr_modulus = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                                         *modulus_len = cb_modulus;
                                         *exp = ptr_exp;
                                         ptr_exp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                                         *exp_len = cb_exp;
                                     }
                                 }
                             }
                         }
                     } else if 0x11i32 == algorithm as (i32) || 0x14i32 == algorithm as (i32) {
                         let mut data_ptr : *mut u8 = data.as_mut_ptr().offset(3isize);
                         let mut len : usize;
                         if 0x11i32 == algorithm as (i32) {
                             len = 65usize;
                         } else {
                             len = 97usize;
                         }
                         if *{
                                 let _old = data_ptr;
                                 data_ptr = data_ptr.offset(1isize);
                                 _old
                             } as (i32) != 0x86i32 {
                             if (*state).verbose != 0 {
                                 fprintf(
                                     __stderrp,
                                     (*b"Failed to parse public key structure.\n\0").as_ptr()
                                 );
                             }
                             res = Enum5::YKPIV_PARSE_ERROR;
                         } else if *{
                                        let _old = data_ptr;
                                        data_ptr = data_ptr.offset(1isize);
                                        _old
                                    } as (usize) != len {
                             if (*state).verbose != 0 {
                                 fprintf(__stderrp,(*b"Unexpected length.\n\0").as_ptr());
                             }
                             res = Enum5::YKPIV_ALGORITHM_ERROR;
                         } else {
                             cb_point = len;
                             if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == {
                                                                                          ptr_point = _ykpiv_alloc(
                                                                                                          state,
                                                                                                          cb_point
                                                                                                      ) as (*mut u8);
                                                                                          ptr_point
                                                                                      } {
                                 if (*state).verbose != 0 {
                                     fprintf(
                                         __stderrp,
                                         (*b"Failed to allocate memory for public point.\n\0").as_ptr(
                                         )
                                     );
                                 }
                                 res = Enum5::YKPIV_MEMORY_ERROR;
                             } else {
                                 memcpy(
                                     ptr_point as (*mut ::std::os::raw::c_void),
                                     data_ptr as (*const ::std::os::raw::c_void),
                                     cb_point
                                 );
                                 *point = ptr_point;
                                 ptr_point = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
                                 *point_len = cb_point;
                             }
                         }
                     } else {
                         if (*state).verbose != 0 {
                             fprintf(__stderrp,(*b"Wrong algorithm.\n\0").as_ptr());
                         }
                         res = Enum5::YKPIV_ALGORITHM_ERROR;
                     }
                 }
             }
             if !ptr_modulus.is_null() {
                 _ykpiv_free(state,modulus as (*mut ::std::os::raw::c_void));
             }
             if !ptr_exp.is_null() {
                 _ykpiv_free(state,ptr_exp as (*mut ::std::os::raw::c_void));
             }
             if !ptr_point.is_null() {
                 _ykpiv_free(state,ptr_exp as (*mut ::std::os::raw::c_void));
             }
             _ykpiv_end_transaction(state);
             res
         })
    }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum10 {
    YKPIV_CONFIG_MGM_MANUAL = 0i32,
    YKPIV_CONFIG_MGM_DERIVED = 1i32,
    YKPIV_CONFIG_MGM_PROTECTED = 2i32,
}

#[derive(Copy)]
#[repr(C)]
pub struct _ykpiv_config {
    pub protected_data_available : u8,
    pub puk_blocked : u8,
    pub puk_noblock_on_upgrade : u8,
    pub pin_last_changed : u32,
    pub mgm_type : Enum10,
}

impl Clone for _ykpiv_config {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_get_config(
    mut state : *mut ykpiv_state, mut config : *mut _ykpiv_config
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut data = [0u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut p_item
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_item : usize = 0usize;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_config) == config {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        (*config).protected_data_available = 0u8;
        (*config).puk_blocked = 0u8;
        (*config).puk_noblock_on_upgrade = 0u8;
        (*config).pin_last_changed = 0u32;
        (*config).mgm_type = Enum10::YKPIV_CONFIG_MGM_MANUAL;
        (if Enum5::YKPIV_OK as (i32) != {
                                            res = _ykpiv_begin_transaction(state);
                                            res
                                        } as (i32) {
             Enum5::YKPIV_PCSC_ERROR
         } else {
             if !(Enum5::YKPIV_OK as (i32) != {
                                                  res = _ykpiv_ensure_application_selected(state);
                                                  res
                                              } as (i32)) {
                 if Enum5::YKPIV_OK as (i32) == _read_metadata(
                                                    state,
                                                    0x80u8,
                                                    data.as_mut_ptr(),
                                                    &mut cb_data as (*mut usize)
                                                ) as (i32) {
                     if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                        data.as_mut_ptr(),
                                                        cb_data,
                                                        0x81u8,
                                                        &mut p_item as (*mut *mut u8),
                                                        &mut cb_item as (*mut usize)
                                                    ) as (i32) {
                         if *p_item as (i32) & 0x1i32 != 0 {
                             (*config).puk_blocked = 1u8;
                         }
                         if *p_item as (i32) & 0x2i32 != 0 {
                             (*config).mgm_type = Enum10::YKPIV_CONFIG_MGM_PROTECTED;
                         }
                     }
                     if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                        data.as_mut_ptr(),
                                                        cb_data,
                                                        0x82u8,
                                                        &mut p_item as (*mut *mut u8),
                                                        &mut cb_item as (*mut usize)
                                                    ) as (i32) {
                         if (*config).mgm_type as (i32) != Enum10::YKPIV_CONFIG_MGM_MANUAL as (i32) {
                             if (*state).verbose != 0 {
                                 fprintf(
                                     __stderrp,
                                     (*b"conflicting types of mgm key administration configured\n\0").as_ptr(
                                     )
                                 );
                             }
                         } else {
                             (*config).mgm_type = Enum10::YKPIV_CONFIG_MGM_DERIVED;
                         }
                     }
                     if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                        data.as_mut_ptr(),
                                                        cb_data,
                                                        0x83u8,
                                                        &mut p_item as (*mut *mut u8),
                                                        &mut cb_item as (*mut usize)
                                                    ) as (i32) {
                         if 4usize != cb_item {
                             if (*state).verbose != 0 {
                                 fprintf(
                                     __stderrp,
                                     (*b"pin timestamp in admin metadata is an invalid size\0").as_ptr(
                                     )
                                 );
                             }
                         } else {
                             memcpy(
                                 &mut (*config).pin_last_changed as (*mut u32) as (*mut ::std::os::raw::c_void),
                                 p_item as (*const ::std::os::raw::c_void),
                                 cb_item
                             );
                         }
                     }
                 }
                 cb_data = ::std::mem::size_of::<[u8; 3072]>();
                 if Enum5::YKPIV_OK as (i32) == _read_metadata(
                                                    state,
                                                    0x88u8,
                                                    data.as_mut_ptr(),
                                                    &mut cb_data as (*mut usize)
                                                ) as (i32) {
                     (*config).protected_data_available = 1u8;
                     if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                        data.as_mut_ptr(),
                                                        cb_data,
                                                        0x81u8,
                                                        &mut p_item as (*mut *mut u8),
                                                        &mut cb_item as (*mut usize)
                                                    ) as (i32) {
                         if *p_item as (i32) & 0x1i32 != 0 {
                             (*config).puk_noblock_on_upgrade = 1u8;
                         }
                     }
                     if Enum5::YKPIV_OK as (i32) == _get_metadata_item(
                                                        data.as_mut_ptr(),
                                                        cb_data,
                                                        0x89u8,
                                                        &mut p_item as (*mut *mut u8),
                                                        &mut cb_item as (*mut usize)
                                                    ) as (i32) {
                         if (*config).mgm_type as (i32) != Enum10::YKPIV_CONFIG_MGM_PROTECTED as (i32) {
                             if (*state).verbose != 0 {
                                 fprintf(
                                     __stderrp,
                                     (*b"conflicting types of mgm key administration configured - protected mgm exists\n\0").as_ptr(
                                     )
                                 );
                             }
                         }
                         (*config).mgm_type = Enum10::YKPIV_CONFIG_MGM_PROTECTED;
                     }
                 }
             }
             _ykpiv_end_transaction(state);
             res
         })
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_set_pin_last_changed(
    mut state : *mut ykpiv_state
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut ykrc : Enum5 = Enum5::YKPIV_OK;
    let mut data = [0u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut tnow : isize = 0isize;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            if Enum5::YKPIV_OK as (i32) != {
                                               ykrc = _read_metadata(
                                                          state,
                                                          0x80u8,
                                                          data.as_mut_ptr(),
                                                          &mut cb_data as (*mut usize)
                                                      );
                                               ykrc
                                           } as (i32) {
                cb_data = 0usize;
            }
            tnow = time(0i32 as (*mut ::std::os::raw::c_void) as (*mut isize));
            if Enum5::YKPIV_OK as (i32) != {
                                               res = _set_metadata_item(
                                                         data.as_mut_ptr(),
                                                         &mut cb_data as (*mut usize),
                                                         3063usize,
                                                         0x83u8,
                                                         &mut tnow as (*mut isize) as (*mut u8),
                                                         4usize
                                                     );
                                               res
                                           } as (i32) {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not set pin timestamp, err = %d\n\0").as_ptr(),
                        res as (i32)
                    );
                }
            } else if Enum5::YKPIV_OK as (i32) != {
                                                      res = _write_metadata(
                                                                state,
                                                                0x80u8,
                                                                data.as_mut_ptr(),
                                                                cb_data
                                                            );
                                                      res
                                                  } as (i32) {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not write admin data, err = %d\n\0").as_ptr(),
                        res as (i32)
                    );
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct _ykpiv_mgm {
    pub data : [u8; 24],
}

impl Clone for _ykpiv_mgm {
    fn clone(&self) -> Self { *self }
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Enum11 {
    PKCS5_OK = 0i32,
    PKCS5_GENERAL_ERROR = -1i32,
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_get_derived_mgm(
    mut state : *mut ykpiv_state,
    mut pin : *const u8,
    pin_len : usize,
    mut mgm : *mut _ykpiv_mgm
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut p5rc : Enum11 = Enum11::PKCS5_OK;
    let mut data = [0u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut p_item
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_item : usize = 0usize;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else if 0i32 as (*mut ::std::os::raw::c_void) as (*const u8) == pin || 0usize == pin_len || 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_mgm) == mgm {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            if Enum5::YKPIV_OK as (i32) == {
                                               res = _read_metadata(
                                                         state,
                                                         0x80u8,
                                                         data.as_mut_ptr(),
                                                         &mut cb_data as (*mut usize)
                                                     );
                                               res
                                           } as (i32) {
                if Enum5::YKPIV_OK as (i32) == {
                                                   res = _get_metadata_item(
                                                             data.as_mut_ptr(),
                                                             cb_data,
                                                             0x82u8,
                                                             &mut p_item as (*mut *mut u8),
                                                             &mut cb_item as (*mut usize)
                                                         );
                                                   res
                                               } as (i32) {
                    if cb_item != 16usize {
                        if (*state).verbose != 0 {
                            fprintf(
                                __stderrp,
                                (*b"derived mgm salt exists, but is incorrect size = %lu\n\0").as_ptr(
                                ),
                                cb_item
                            );
                        }
                        res = Enum5::YKPIV_GENERIC_ERROR;
                    } else if Enum11::PKCS5_OK as (i32) != {
                                                               p5rc = pkcs5_pbkdf2_sha1(
                                                                          pin,
                                                                          pin_len,
                                                                          p_item as (*const u8),
                                                                          cb_item,
                                                                          10000usize,
                                                                          (*mgm).data.as_mut_ptr(
                                                                          ) as (*const u8),
                                                                          ::std::mem::size_of::<[u8; 24]>(
                                                                          )
                                                                      );
                                                               p5rc
                                                           } as (i32) {
                        if (*state).verbose != 0 {
                            fprintf(
                                __stderrp,
                                (*b"pbkdf2 failure, err = %d\n\0").as_ptr(),
                                p5rc as (i32)
                            );
                        }
                        res = Enum5::YKPIV_GENERIC_ERROR;
                    }
                }
            }
        }
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_get_protected_mgm(
    mut state : *mut ykpiv_state, mut mgm : *mut _ykpiv_mgm
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut data = [0u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut p_item
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_item : usize = 0usize;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else if 0i32 as (*mut ::std::os::raw::c_void) as (*mut _ykpiv_mgm) == mgm {
        Enum5::YKPIV_GENERIC_ERROR
    } else if Enum5::YKPIV_OK as (i32) != {
                                              res = _ykpiv_begin_transaction(state);
                                              res
                                          } as (i32) {
        Enum5::YKPIV_PCSC_ERROR
    } else {
        if !(Enum5::YKPIV_OK as (i32) != {
                                             res = _ykpiv_ensure_application_selected(state);
                                             res
                                         } as (i32)) {
            if Enum5::YKPIV_OK as (i32) != {
                                               res = _read_metadata(
                                                         state,
                                                         0x88u8,
                                                         data.as_mut_ptr(),
                                                         &mut cb_data as (*mut usize)
                                                     );
                                               res
                                           } as (i32) {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not read protected data, err = %d\n\0").as_ptr(),
                        res as (i32)
                    );
                }
            } else if Enum5::YKPIV_OK as (i32) != {
                                                      res = _get_metadata_item(
                                                                data.as_mut_ptr(),
                                                                cb_data,
                                                                0x89u8,
                                                                &mut p_item as (*mut *mut u8),
                                                                &mut cb_item as (*mut usize)
                                                            );
                                                      res
                                                  } as (i32) {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not read protected mgm from metadata, err = %d\n\0").as_ptr(
                        ),
                        res as (i32)
                    );
                }
            } else if cb_item != ::std::mem::size_of::<[u8; 24]>() {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"protected data contains mgm, but is the wrong size = %lu\n\0").as_ptr(
                        ),
                        cb_item
                    );
                }
                res = Enum5::YKPIV_AUTHENTICATION_ERROR;
            } else {
                memcpy(
                    (*mgm).data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                    p_item as (*const ::std::os::raw::c_void),
                    cb_item
                );
            }
        }
        memset_s(
            data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<[u8; 3072]>(),
            0i32,
            ::std::mem::size_of::<[u8; 3072]>()
        );
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_set_protected_mgm(
    mut state : *mut ykpiv_state, mut mgm : *mut _ykpiv_mgm
) -> Enum5 {
    let mut _currentBlock;
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut ykrc : Enum5 = Enum5::YKPIV_OK;
    let mut prngrc : Enum7 = Enum7::PRNG_OK;
    let mut fGenerate : bool = false;
    let mut mgm_key : [u8; 24];
    let mut i : usize = 0usize;
    let mut data = [0u8; 3072];
    let mut cb_data : usize = ::std::mem::size_of::<[u8; 3072]>();
    let mut p_item
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_item : usize = 0usize;
    let mut flags_1 : u8 = 0u8;
    if 0i32 as (*mut ::std::os::raw::c_void) as (*mut ykpiv_state) == state {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        if mgm.is_null() {
            fGenerate = true;
        } else {
            fGenerate = true;
            memcpy(
                mgm_key.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                (*mgm).data.as_mut_ptr() as (*const ::std::os::raw::c_void),
                ::std::mem::size_of::<[u8; 24]>()
            );
            i = 0usize;
            'loop3: loop {
                if !(i < ::std::mem::size_of::<[u8; 24]>()) {
                    _currentBlock = 8;
                    break;
                }
                if mgm_key[i] as (i32) != 0i32 {
                    _currentBlock = 6;
                    break;
                }
                i = i.wrapping_add(1usize);
            }
            if _currentBlock == 8 {
            } else {
                fGenerate = false;
            }
        }
        if Enum5::YKPIV_OK as (i32) != {
                                           res = _ykpiv_begin_transaction(state);
                                           res
                                       } as (i32) {
            res = Enum5::YKPIV_PCSC_ERROR;
        } else if !(Enum5::YKPIV_OK as (i32) != {
                                                    res = _ykpiv_ensure_application_selected(state);
                                                    res
                                                } as (i32)) {
            'loop10: loop {
                if fGenerate {
                    if Enum7::PRNG_OK as (i32) != {
                                                      prngrc = _ykpiv_prng_generate(
                                                                   mgm_key.as_mut_ptr(),
                                                                   ::std::mem::size_of::<[u8; 24]>()
                                                               );
                                                      prngrc
                                                  } as (i32) {
                        _currentBlock = 47;
                        break;
                    }
                }
                if Enum5::YKPIV_OK as (i32) != {
                                                   ykrc = ykpiv_set_mgmkey(
                                                              state,
                                                              mgm_key.as_mut_ptr() as (*const u8)
                                                          );
                                                   ykrc
                                               } as (i32) {
                    if Enum5::YKPIV_KEY_ERROR as (i32) != ykrc as (i32) {
                        _currentBlock = 44;
                        break;
                    }
                } else {
                    fGenerate = false;
                }
                if !fGenerate {
                    _currentBlock = 16;
                    break;
                }
            }
            if _currentBlock == 16 {
                if !mgm.is_null() {
                    memcpy(
                        (*mgm).data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
                        mgm_key.as_mut_ptr() as (*const ::std::os::raw::c_void),
                        ::std::mem::size_of::<[u8; 24]>()
                    );
                }
                if Enum5::YKPIV_OK as (i32) != {
                                                   ykrc = _read_metadata(
                                                              state,
                                                              0x88u8,
                                                              data.as_mut_ptr(),
                                                              &mut cb_data as (*mut usize)
                                                          );
                                                   ykrc
                                               } as (i32) {
                    cb_data = 0usize;
                }
                if Enum5::YKPIV_OK as (i32) != {
                                                   ykrc = _set_metadata_item(
                                                              data.as_mut_ptr(),
                                                              &mut cb_data as (*mut usize),
                                                              3063usize,
                                                              0x89u8,
                                                              mgm_key.as_mut_ptr(),
                                                              ::std::mem::size_of::<[u8; 24]>()
                                                          );
                                                   ykrc
                                               } as (i32) {
                    if (*state).verbose != 0 {
                        fprintf(
                            __stderrp,
                            (*b"could not set protected mgm item, err = %d\n\0").as_ptr(),
                            ykrc as (i32)
                        );
                        _currentBlock = 26;
                    } else {
                        _currentBlock = 26;
                    }
                } else if Enum5::YKPIV_OK as (i32) != {
                                                          ykrc = _write_metadata(
                                                                     state,
                                                                     0x88u8,
                                                                     data.as_mut_ptr(),
                                                                     cb_data
                                                                 );
                                                          ykrc
                                                      } as (i32) {
                    if (*state).verbose != 0 {
                        fprintf(
                            __stderrp,
                            (*b"could not write protected data, err = %d\n\0").as_ptr(),
                            ykrc as (i32)
                        );
                        _currentBlock = 51;
                    } else {
                        _currentBlock = 51;
                    }
                } else {
                    _currentBlock = 26;
                }
                if _currentBlock == 51 {
                } else {
                    cb_data = ::std::mem::size_of::<[u8; 3072]>();
                    if Enum5::YKPIV_OK as (i32) != {
                                                       ykrc = _read_metadata(
                                                                  state,
                                                                  0x80u8,
                                                                  data.as_mut_ptr(),
                                                                  &mut cb_data as (*mut usize)
                                                              );
                                                       ykrc
                                                   } as (i32) {
                        cb_data = 0usize;
                    } else {
                        if Enum5::YKPIV_OK as (i32) != {
                                                           ykrc = _get_metadata_item(
                                                                      data.as_mut_ptr(),
                                                                      cb_data,
                                                                      0x81u8,
                                                                      &mut p_item as (*mut *mut u8),
                                                                      &mut cb_item as (*mut usize)
                                                                  );
                                                           ykrc
                                                       } as (i32) {
                            if (*state).verbose != 0 {
                                fprintf(
                                    __stderrp,
                                    (*b"admin data exists, but flags are not present\n\0").as_ptr()
                                );
                            }
                        }
                        if cb_item == ::std::mem::size_of::<u8>() {
                            memcpy(
                                &mut flags_1 as (*mut u8) as (*mut ::std::os::raw::c_void),
                                p_item as (*const ::std::os::raw::c_void),
                                cb_item
                            );
                        } else if (*state).verbose != 0 {
                            fprintf(
                                __stderrp,
                                (*b"admin data flags are an incorrect size = %lu\n\0").as_ptr(),
                                cb_item
                            );
                        }
                        if Enum5::YKPIV_OK as (i32) != {
                                                           ykrc = _set_metadata_item(
                                                                      data.as_mut_ptr(),
                                                                      &mut cb_data as (*mut usize),
                                                                      3063usize,
                                                                      0x82u8,
                                                                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                                                                      0usize
                                                                  );
                                                           ykrc
                                                       } as (i32) {
                            if (*state).verbose != 0 {
                                fprintf(
                                    __stderrp,
                                    (*b"could not unset derived mgm salt, err = %d\n\0").as_ptr(),
                                    ykrc as (i32)
                                );
                            }
                        }
                    }
                    flags_1 = (flags_1 as (i32) | 0x2i32) as (u8);
                    if Enum5::YKPIV_OK as (i32) != {
                                                       ykrc = _set_metadata_item(
                                                                  data.as_mut_ptr(),
                                                                  &mut cb_data as (*mut usize),
                                                                  3063usize,
                                                                  0x81u8,
                                                                  &mut flags_1 as (*mut u8),
                                                                  ::std::mem::size_of::<u8>()
                                                              );
                                                       ykrc
                                                   } as (i32) {
                        if (*state).verbose != 0 {
                            fprintf(
                                __stderrp,
                                (*b"could not set admin flags item, err = %d\n\0").as_ptr(),
                                ykrc as (i32)
                            );
                        }
                    } else if Enum5::YKPIV_OK as (i32) != {
                                                              ykrc = _write_metadata(
                                                                         state,
                                                                         0x80u8,
                                                                         data.as_mut_ptr(),
                                                                         cb_data
                                                                     );
                                                              ykrc
                                                          } as (i32) {
                        if (*state).verbose != 0 {
                            fprintf(
                                __stderrp,
                                (*b"could not write admin data, err = %d\n\0").as_ptr(),
                                ykrc as (i32)
                            );
                        }
                    }
                }
            } else if _currentBlock == 44 {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not set new derived mgm key, err = %d\n\0").as_ptr(),
                        ykrc as (i32)
                    );
                }
                res = ykrc;
            } else {
                if (*state).verbose != 0 {
                    fprintf(
                        __stderrp,
                        (*b"could not generate new mgm, err = %d\n\0").as_ptr(),
                        prngrc as (i32)
                    );
                }
                res = Enum5::YKPIV_RANDOMNESS_ERROR;
            }
        }
        memset_s(
            data.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<[u8; 3072]>(),
            0i32,
            ::std::mem::size_of::<[u8; 3072]>()
        );
        memset_s(
            mgm_key.as_mut_ptr() as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<[u8; 24]>(),
            0i32,
            ::std::mem::size_of::<[u8; 24]>()
        );
        _ykpiv_end_transaction(state);
        res
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_reset(
    mut state : *mut ykpiv_state
) -> Enum5 {
    let mut templ : *mut u8 = 0i32 as (*mut u8);
    let mut data : [u8; 255];
    let mut recv_len : usize = ::std::mem::size_of::<[u8; 255]>();
    let mut res : Enum5;
    let mut sw : i32;
    res = ykpiv_transfer_data(
              state,
              templ as (*const u8),
              0i32 as (*mut ::std::os::raw::c_void) as (*const u8),
              0isize,
              data.as_mut_ptr(),
              &mut recv_len as (*mut usize),
              &mut sw as (*mut i32)
          );
    if Enum5::YKPIV_OK as (i32) == res as (i32) && (0x9000i32 == sw) {
        Enum5::YKPIV_OK
    } else {
        Enum5::YKPIV_GENERIC_ERROR
    }
}

#[no_mangle]
pub unsafe extern fn ykpiv_util_slot_object(mut slot : u8) -> u32 {
    let mut object_id : i32 = -1i32;
    if slot as (i32) == 0xf9i32 {
        object_id = 0x5fff01i32;
    } else if slot as (i32) == 0x9ei32 {
        object_id = 0x5fc101i32;
    } else if slot as (i32) == 0x9di32 {
        object_id = 0x5fc10bi32;
    } else if slot as (i32) == 0x9ci32 {
        object_id = 0x5fc10ai32;
    } else if slot as (i32) == 0x9ai32 {
        object_id = 0x5fc105i32;
    } else if slot as (i32) >= 0x82i32 && (slot as (i32) <= 0x95i32) {
        object_id = 0x5fc10di32 + (slot as (i32) - 0x82i32);
    }
    object_id as (u32)
}

unsafe extern fn _read_certificate(
    mut state : *mut ykpiv_state,
    mut slot : u8,
    mut buf : *mut u8,
    mut buf_len : *mut usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut ptr
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut object_id : i32 = ykpiv_util_slot_object(slot) as (i32);
    let mut len : usize = 0usize;
    if -1i32 == object_id {
        Enum5::YKPIV_INVALID_OBJECT
    } else {
        if Enum5::YKPIV_OK as (i32) == {
                                           res = _ykpiv_fetch_object(state,object_id,buf,buf_len);
                                           res
                                       } as (i32) {
            ptr = buf;
            if *buf_len < 2usize {
                *buf_len = 0usize;
                return Enum5::YKPIV_OK;
            } else if *{
                           let _old = ptr;
                           ptr = ptr.offset(1isize);
                           _old
                       } as (i32) == 0x70i32 {
                ptr = ptr.offset(
                          _ykpiv_get_length(
                              ptr as (*const u8),
                              &mut len as (*mut usize)
                          ) as (isize)
                      );
                if len > (*buf_len).wrapping_sub(
                             ((ptr as (isize)).wrapping_sub(
                                  buf as (isize)
                              ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                         ) {
                    *buf_len = 0usize;
                    return Enum5::YKPIV_OK;
                } else {
                    memmove(
                        buf as (*mut ::std::os::raw::c_void),
                        ptr as (*const ::std::os::raw::c_void),
                        len
                    );
                    *buf_len = len;
                }
            }
        } else {
            *buf_len = 0usize;
        }
        res
    }
}

unsafe extern fn _write_certificate(
    mut state : *mut ykpiv_state,
    mut slot : u8,
    mut data : *mut u8,
    mut data_len : usize,
    mut certinfo : u8
) -> Enum5 {
    let mut buf : [u8; 3063];
    let mut object_id : i32 = ykpiv_util_slot_object(slot) as (i32);
    let mut offset : usize = 0usize;
    let mut req_len : usize = 0usize;
    if -1i32 == object_id {
        Enum5::YKPIV_INVALID_OBJECT
    } else if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) == data || 0usize == data_len {
        (if 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8) != data || 0usize != data_len {
             Enum5::YKPIV_GENERIC_ERROR
         } else {
             _ykpiv_save_object(
                 state,
                 object_id,
                 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                 0usize
             )
         })
    } else {
        req_len = (1i32 + 3i32 + 2i32) as (usize);
        req_len = req_len.wrapping_add(
                      _ykpiv_set_length(buf.as_mut_ptr(),data_len) as (usize)
                  );
        req_len = req_len.wrapping_add(data_len);
        (if req_len < data_len {
             Enum5::YKPIV_SIZE_ERROR
         } else if req_len > _obj_size_max(state) {
             Enum5::YKPIV_SIZE_ERROR
         } else {
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = 0x70u8;
             offset = offset.wrapping_add(
                          _ykpiv_set_length(
                              buf.as_mut_ptr().offset(offset as (isize)),
                              data_len
                          ) as (usize)
                      );
             memcpy(
                 buf.as_mut_ptr().offset(
                     offset as (isize)
                 ) as (*mut ::std::os::raw::c_void),
                 data as (*const ::std::os::raw::c_void),
                 data_len
             );
             offset = offset.wrapping_add(data_len);
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = 0x71u8;
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = 0x1u8;
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = if certinfo as (i32) == 1i32 {
                     0x1i32
                 } else {
                     0x0i32
                 } as (u8);
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = 0xfeu8;
             buf[
                 {
                     let _old = offset;
                     offset = offset.wrapping_add(1usize);
                     _old
                 }
             ] = 0o0u8;
             _ykpiv_save_object(state,object_id,buf.as_mut_ptr(),offset)
         })
    }
}

unsafe extern fn _get_metadata_item(
    mut data : *mut u8,
    mut cb_data : usize,
    mut tag : u8,
    mut pp_item : *mut *mut u8,
    mut pcb_item : *mut usize
) -> Enum5 {
    let mut _currentBlock;
    let mut p_temp : *mut u8 = data;
    let mut cb_temp : usize = 0usize;
    let mut tag_temp : u8 = 0u8;
    if data.is_null() || pp_item.is_null() || pcb_item.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        *pp_item = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
        *pcb_item = 0usize;
        'loop2: loop {
            if !(p_temp < data.offset(cb_data as (isize))) {
                _currentBlock = 6;
                break;
            }
            tag_temp = *{
                            let _old = p_temp;
                            p_temp = p_temp.offset(1isize);
                            _old
                        };
            if !_ykpiv_has_valid_length(
                    p_temp as (*const u8),
                    ((data.offset(cb_data as (isize)) as (isize)).wrapping_sub(
                         p_temp as (isize)
                     ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                ) {
                _currentBlock = 9;
                break;
            }
            p_temp = p_temp.offset(
                         _ykpiv_get_length(
                             p_temp as (*const u8),
                             &mut cb_temp as (*mut usize)
                         ) as (isize)
                     );
            if tag_temp as (i32) == tag as (i32) {
                _currentBlock = 6;
                break;
            }
            p_temp = p_temp.offset(cb_temp as (isize));
        }
        (if _currentBlock == 6 {
             (if p_temp < data.offset(cb_data as (isize)) {
                  *pp_item = p_temp;
                  *pcb_item = cb_temp;
                  Enum5::YKPIV_OK
              } else {
                  Enum5::YKPIV_GENERIC_ERROR
              })
         } else {
             Enum5::YKPIV_SIZE_ERROR
         })
    }
}

unsafe extern fn _get_length_size(mut length : usize) -> i32 {
    if length < 0x80usize {
        1i32
    } else if length < 0xffusize {
        2i32
    } else {
        3i32
    }
}

unsafe extern fn _set_metadata_item(
    mut data : *mut u8,
    mut pcb_data : *mut usize,
    mut cb_data_max : usize,
    mut tag : u8,
    mut p_item : *mut u8,
    mut cb_item : usize
) -> Enum5 {
    let mut _currentBlock;
    let mut p_temp : *mut u8 = data;
    let mut cb_temp : usize = 0usize;
    let mut tag_temp : u8 = 0u8;
    let mut cb_len : usize = 0usize;
    let mut p_next
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_moved : isize = 0isize;
    if data.is_null() || pcb_data.is_null() {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        'loop1: loop {
            if !(p_temp < data.offset(*pcb_data as (isize))) {
                _currentBlock = 2;
                break;
            }
            tag_temp = *{
                            let _old = p_temp;
                            p_temp = p_temp.offset(1isize);
                            _old
                        };
            cb_len = _ykpiv_get_length(
                         p_temp as (*const u8),
                         &mut cb_temp as (*mut usize)
                     ) as (usize);
            p_temp = p_temp.offset(cb_len as (isize));
            if tag_temp as (i32) == tag as (i32) {
                _currentBlock = 9;
                break;
            }
            p_temp = p_temp.offset(cb_temp as (isize));
        }
        (if _currentBlock == 2 {
             (if cb_item == 0usize {
                  Enum5::YKPIV_OK
              } else {
                  p_temp = data.offset(*pcb_data as (isize));
                  cb_len = _get_length_size(cb_item) as (usize);
                  (if (*pcb_data).wrapping_add(cb_len).wrapping_add(
                          cb_item
                      ) > cb_data_max {
                       Enum5::YKPIV_GENERIC_ERROR
                   } else {
                       *{
                            let _old = p_temp;
                            p_temp = p_temp.offset(1isize);
                            _old
                        } = tag;
                       p_temp = p_temp.offset(
                                    _ykpiv_set_length(p_temp,cb_item) as (isize)
                                );
                       memcpy(
                           p_temp as (*mut ::std::os::raw::c_void),
                           p_item as (*const ::std::os::raw::c_void),
                           cb_item
                       );
                       *pcb_data = (*pcb_data).wrapping_add(
                                       1usize.wrapping_add(cb_len).wrapping_add(cb_item)
                                   );
                       Enum5::YKPIV_OK
                   })
              })
         } else if cb_temp == cb_item {
             memcpy(
                 p_temp as (*mut ::std::os::raw::c_void),
                 p_item as (*const ::std::os::raw::c_void),
                 cb_item
             );
             Enum5::YKPIV_OK
         } else {
             p_next = p_temp.offset(cb_temp as (isize));
             cb_moved = cb_item as (isize) - cb_temp as (isize) + (if cb_item != 0usize {
                                                                       _get_length_size(cb_item)
                                                                   } else {
                                                                       -1i32
                                                                   } as (isize) - cb_len as (isize));
             (if (*pcb_data).wrapping_add(cb_moved as (usize)) > cb_data_max {
                  Enum5::YKPIV_GENERIC_ERROR
              } else {
                  memmove(
                      p_next.offset(cb_moved) as (*mut ::std::os::raw::c_void),
                      p_next as (*const ::std::os::raw::c_void),
                      (*pcb_data).wrapping_sub(
                          ((p_next as (isize)).wrapping_sub(
                               data as (isize)
                           ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                      )
                  );
                  *pcb_data = (*pcb_data).wrapping_add(cb_moved as (usize));
                  if cb_item != 0usize {
                      p_temp = p_temp.offset(-(cb_len as (isize)));
                      p_temp = p_temp.offset(
                                   _ykpiv_set_length(p_temp,cb_item) as (isize)
                               );
                      memcpy(
                          p_temp as (*mut ::std::os::raw::c_void),
                          p_item as (*const ::std::os::raw::c_void),
                          cb_item
                      );
                  }
                  Enum5::YKPIV_OK
              })
         })
    }
}

unsafe extern fn _read_metadata(
    mut state : *mut ykpiv_state,
    mut tag : u8,
    mut data : *mut u8,
    mut pcb_data : *mut usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut p_temp
        : *mut u8
        = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8);
    let mut cb_temp : usize = 0usize;
    let mut obj_id : i32 = 0i32;
    if data.is_null() || pcb_data.is_null() || 3072usize > *pcb_data {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        if tag as (i32) == 0x88i32 {
            obj_id = 0x5fc109i32;
        } else if tag as (i32) == 0x80i32 {
            obj_id = 0x5fff00i32;
        } else {
            return Enum5::YKPIV_INVALID_OBJECT;
        }
        cb_temp = *pcb_data;
        *pcb_data = 0usize;
        (if Enum5::YKPIV_OK as (i32) != {
                                            res = _ykpiv_fetch_object(
                                                      state,
                                                      obj_id,
                                                      data,
                                                      &mut cb_temp as (*mut usize)
                                                  );
                                            res
                                        } as (i32) {
             res
         } else if cb_temp < 2usize {
             Enum5::YKPIV_GENERIC_ERROR
         } else {
             p_temp = data;
             (if tag as (i32) != *{
                                      let _old = p_temp;
                                      p_temp = p_temp.offset(1isize);
                                      _old
                                  } as (i32) {
                  Enum5::YKPIV_GENERIC_ERROR
              } else {
                  p_temp = p_temp.offset(
                               _ykpiv_get_length(p_temp as (*const u8),pcb_data) as (isize)
                           );
                  (if *pcb_data > cb_temp.wrapping_sub(
                                      ((p_temp as (isize)).wrapping_sub(
                                           data as (isize)
                                       ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                                  ) {
                       *pcb_data = 0usize;
                       Enum5::YKPIV_GENERIC_ERROR
                   } else {
                       memmove(
                           data as (*mut ::std::os::raw::c_void),
                           p_temp as (*const ::std::os::raw::c_void),
                           *pcb_data
                       );
                       Enum5::YKPIV_OK
                   })
              })
         })
    }
}

unsafe extern fn _write_metadata(
    mut state : *mut ykpiv_state,
    mut tag : u8,
    mut data : *mut u8,
    mut cb_data : usize
) -> Enum5 {
    let mut res : Enum5 = Enum5::YKPIV_OK;
    let mut buf = [0u8; 3063];
    let mut pTemp : *mut u8 = buf.as_mut_ptr();
    let mut obj_id : i32 = 0i32;
    if cb_data > _obj_size_max(state).wrapping_sub(
                     (2i32 + 2i32) as (usize)
                 ) {
        Enum5::YKPIV_GENERIC_ERROR
    } else {
        if tag as (i32) == 0x88i32 {
            obj_id = 0x5fc109i32;
        } else if tag as (i32) == 0x80i32 {
            obj_id = 0x5fff00i32;
        } else {
            return Enum5::YKPIV_INVALID_OBJECT;
        }
        if data.is_null() || 0usize == cb_data {
            res = _ykpiv_save_object(
                      state,
                      obj_id,
                      0i32 as (*mut ::std::os::raw::c_void) as (*mut u8),
                      0usize
                  );
        } else {
            *{
                 let _old = pTemp;
                 pTemp = pTemp.offset(1isize);
                 _old
             } = tag;
            pTemp = pTemp.offset(_ykpiv_set_length(pTemp,cb_data) as (isize));
            memcpy(
                pTemp as (*mut ::std::os::raw::c_void),
                data as (*const ::std::os::raw::c_void),
                cb_data
            );
            pTemp = pTemp.offset(cb_data as (isize));
            res = _ykpiv_save_object(
                      state,
                      obj_id,
                      buf.as_mut_ptr(),
                      ((pTemp as (isize)).wrapping_sub(
                           buf.as_mut_ptr() as (isize)
                       ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
                  );
        }
        res
    }
}