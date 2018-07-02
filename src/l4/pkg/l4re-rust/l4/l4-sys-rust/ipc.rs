use std::os::raw::{c_int, c_uint, c_long, c_ulong};
use c_api::*;

////////////////////////////////////////////////////////////////////////////////
// redefined constants and enums with (wrongly) generated type
#[cfg(target_arch = "x86_64")]
const UTCB_GENERIC_DATA_SIZE: usize = L4_UTCB_GENERIC_DATA_SIZE as usize;
#[cfg(target_arch = "x86_64")]
const UTCB_BUF_REGS_OFFSET: isize = L4_UTCB_BUF_REGS_OFFSET as isize;

// Constants for message items.
//
// These constants have been redefined as Rust enum, because the constants from
// Bindgen carry constatly the wrong type, the implementor may now choose which
// one to pick. In this library, usize as type is required.
// The names have been stripped of their L4 prefix and the _t suffix, but are
// identical otherwise.
/// Identify a message item as **map** item.
const MSG_ITEM_CONSTS_ITEM_MAP: u64 = 8;
/// Denote that the following item shall be put into the same receive item as this one.
const MSG_ITEM_CONSTS_ITEM_CONT: u64 = 1;
///< Flag as usual **map** operation.
// receive
/// Mark the receive buffer to be a small receive item that describes a buffer for a single
/// capability.
/// The receiver requests to receive a local ID instead of a mapping whenever possible.
const MSG_ITEM_MAP: l4_umword_t = L4_ITEM_MAP as l4_umword_t;
const MSGTAG_ERROR: i64 = L4_MSGTAG_ERROR as i64;

////////////////////////////////////////////////////////////////////////////////
// simple wrappers from lib-l4re-rust-wrapper

#[inline]
pub unsafe fn l4_ipc_call(dest: l4_cap_idx_t, utcb: *mut l4_utcb_t,
            tag: l4_msgtag_t, timeout: l4_timeout_t) -> l4_msgtag_t {
    l4_ipc_call_w(dest, utcb, tag, timeout)
}

#[inline]
pub unsafe fn l4_ipc_error(tag: l4_msgtag_t, utcb: *mut l4_utcb_t) -> l4_umword_t {
    l4_ipc_error_w(tag, utcb)
}

#[inline]
pub unsafe fn l4_ipc_receive(object: l4_cap_idx_t, utcb: *mut l4_utcb_t,
        timeout: l4_timeout_t) -> l4_msgtag_t {
    l4_ipc_receive_w(object, utcb, timeout)
}


#[inline]
pub unsafe fn l4_ipc_reply_and_wait(utcb: *mut l4_utcb_t, tag: l4_msgtag_t,
        src: *mut l4_umword_t, timeout: l4_timeout_t) -> l4_msgtag_t {
    l4_ipc_reply_and_wait_w(utcb, tag, src, timeout)
}


#[inline]
pub unsafe fn l4_ipc_wait(utcb: *mut l4_utcb_t, label: *mut l4_umword_t,
        timeout: l4_timeout_t) -> l4_msgtag_t {
    l4_ipc_wait_w(utcb, label, timeout)
}

#[inline]
pub unsafe fn l4_msgtag(label: ::std::os::raw::c_long, words: c_uint,
        items: c_uint, flags: c_uint) -> l4_msgtag_t {
    l4_msgtag_w(label, words, items, flags)
}

#[inline]
pub unsafe fn l4_rcv_ep_bind_thread(gate: l4_cap_idx_t, thread: l4_cap_idx_t,
        label: l4_umword_t) -> l4_msgtag_t {
    l4_rcv_ep_bind_thread_w(gate, thread, label)
}


////////////////////////////////////////////////////////////////////////////////
// re-implemented inline functions from l4/sys/ipc.h:

#[inline]
pub unsafe fn l4_msgtag_flags(t: l4_msgtag_t) -> c_ulong {
    (t.raw & 0xf000) as c_ulong
}

/// Re-implementation returning bool instead of int
#[inline]
pub fn l4_msgtag_has_error(t: l4_msgtag_t) -> bool {
    (t.raw & MSGTAG_ERROR) != 0
}

/// This function's return type is altered in comparison to the orig: c_uint -> usize, return value
/// is used as index and this needs to be usize in Rust world.
#[inline]
pub unsafe fn l4_msgtag_items(t: l4_msgtag_t) -> usize {
    ((t.raw >> 6) & 0x3f) as usize
}

#[inline]
pub unsafe fn l4_msgtag_label(t: l4_msgtag_t) -> c_long {
    t.raw >> 16
}

#[inline]
pub unsafe fn l4_msgtag_words(t: l4_msgtag_t) -> u32 {
    (t.raw & 0x3f) as u32
}

/// re-implementation of the inline function from l4sys/include/ipc.h
#[inline]
pub unsafe fn l4_sndfpage_add_u(snd_fpage: l4_fpage_t, snd_base: c_ulong,
                  tag: *mut l4_msgtag_t, utcb: *mut l4_utcb_t) -> c_int {
    let v = l4_utcb_mr_u(utcb);
    let i = l4_msgtag_words(*tag) as usize + 2 * l4_msgtag_items(*tag);
    if i >= (UTCB_GENERIC_DATA_SIZE - 1) {
        return L4_ENOMEM as i32 * -1;
    }

    (*v).mr[i] = snd_base | MSG_ITEM_CONSTS_ITEM_MAP | MSG_ITEM_CONSTS_ITEM_CONT;
    (*v).mr[i + 1] = snd_fpage.raw;

    *tag = l4_msgtag(l4_msgtag_label(*tag), l4_msgtag_words(*tag),
                   l4_msgtag_items(*tag) as u32 + 1, l4_msgtag_flags(*tag) as u32);
    0
}

#[inline]
pub unsafe fn l4_utcb_mr() -> *mut l4_msg_regs_t {
    l4_utcb_mr_u(l4_utcb())
}


/// re-implementation of the inline function from l4sys/include/ipc.h
#[inline]
pub unsafe fn l4_sndfpage_add(snd_fpage: l4_fpage_t, snd_base: c_ulong,
        tag: *mut l4_msgtag_t) -> c_int {
    l4_sndfpage_add_u(snd_fpage, snd_base, tag, l4_utcb())
}


#[inline]
pub unsafe fn l4_utcb() -> *mut l4_utcb_t {
    l4_utcb_w()
}

// ToDo: potentially broken (or one of the functions called
//#[inline]
//pub unsafe fn l4_rcv_ep_bind_thread(gate: l4_cap_idx_t, thread: l4_cap_idx_t,
//        label: l4_umword_t) -> l4_msgtag_t {
//    return l4_rcv_ep_bind_thread_u(gate, thread, label, l4_utcb())
//}


#[inline]
pub unsafe fn l4_map_control(snd_base: l4_umword_t, cache: u8,
         grant: u32) -> l4_umword_t {
    (snd_base & L4_FPAGE_CONTROL_MASK)
                   | ((cache as l4_umword_t) << 4)
                   | MSG_ITEM_MAP
                   | (grant as l4_umword_t)
}

// ToDo: broken
//#[inline]
//pub unsafe fn l4_rcv_ep_bind_thread_u(gate: l4_cap_idx_t, thread: l4_cap_idx_t,
//        label: l4_umword_t, utcb: *mut l4_utcb_t) -> l4_msgtag_t {
//    let m: *mut l4_msg_regs_t = l4_utcb_mr_u(utcb);
//    (*m).mr[0] = L4_fpage_control_L4_FPAGE_CONTROL_MASK;
//    (*m).mr[1] = label;
//    (*m).mr[2] = cap::l4_map_obj_control(0, 0);
//    (*m).mr[3] = cap::l4_obj_fpage(thread, 0, cap::FPAGE_RWX).raw;
//    l4_ipc_call(gate, utcb,
//                l4_msgtag(l4_msgtag_protocol_L4_PROTO_KOBJECT as i64, 2, 1, 0),
//                l4_timeout_t { raw: 0 })
//}

/// Retrieve pointer to buffer register from current UTCB
#[inline(always)]
pub unsafe fn l4_utcb_br() -> *mut l4_buf_regs_t {
    (l4_utcb() as *mut u8).offset(UTCB_BUF_REGS_OFFSET) as *mut l4_buf_regs_t
}

/// Retrieve pointer to buffer register from specified UTCB
#[inline(always)]
pub unsafe fn l4_utcb_br_u(u: *mut l4_utcb_t) -> *mut l4_buf_regs_t {
    (u as *mut u8).offset(UTCB_BUF_REGS_OFFSET) as *mut l4_buf_regs_t
}

#[inline]
pub unsafe fn l4_utcb_mr_u(u: *mut l4_utcb_t) -> *mut l4_msg_regs_t {
     (u as *mut u8).offset(L4_UTCB_MSG_REGS_OFFSET as isize)
         as *mut l4_msg_regs_t
}

////////////////////////////////////////////////////////////////////////////////
// new functions

/// Return a never-timeout (akin to L4_IPC_TIMEOUT_NEVER)
#[inline(always)]
pub fn timeout_never() -> l4_timeout_t {
    l4_timeout_t {
        raw: 0, // forever
    }
}

/// Extract IPC error code from error code
///
/// Error codes in the console output, e.g. for  page faults, contain more information than the IPC
/// error code, this function extracts this bit of information.
#[inline(always)]
pub fn ipc_error2code(code: isize) -> u64 {
    (code & L4_IPC_ERROR_MASK as isize) as u64
}