extern crate l4;
extern crate l4_derive;
extern crate l4re;
extern crate libc;

use l4::{cap::Cap,
    error::{Error, Result},
    ipc, l4_err,
    sys::l4_utcb,
};
use l4_derive::{iface, l4_server};
use l4re::{
    mem::DataspaceProvider,
    OwnedCap
};

// include IPC interface definition
include!("../interface.rs");

#[l4_server(Shm)]
struct ShmServer;

impl Shm for ShmServer {
    fn witter(&mut self, length: u32, ds: Cap<Dataspace>) -> Result<()> {
        let mut ds = OwnedCap::from(ds)?;
        let msg = read_message(&mut ds, length as usize)?;
        println!("{}...", msg.chars().take(20).collect::<String>());
        Ok(())
    }
}

fn read_message(ds: &mut Cap<Dataspace>, length: usize)
        -> Result<String> {
            // ToDo: error handling
            use l4::sys::*;
    let info = ds.info()?;
    // be double sure what the client sent
    if info.size != l4::sys::round_page(length) as u64 {
        l4_err!(Generic, OutOfBounds);
    }
    let msg = unsafe {
        let mut virt_addr: *mut libc::c_void = std::mem::uninitialized();
        // reserve area to allow attaching a data space; flags = 0 must be set
        let page_size = round_page(length);
        println!("attaching memory: {:x} bytes", page_size);
        match l4re::sys::l4re_rm_attach((&mut virt_addr) as *mut *mut _,
                page_size as u64, l4re::sys::l4re_rm_flags_t::L4RE_RM_SEARCH_ADDR as u64,
                ds.raw(), 0, l4::sys::L4_PAGESHIFT as u8) {
            0 => (),
            err => return Err(Error::Unknown(err as i64)),
        };
        println!("copying string, length: {:x}; base: {:p}", length, virt_addr);
        // copy bytes into a vector
        let mut msg: Vec<u8> = Vec::with_capacity(length as usize);
        msg.set_len(length);
        (virt_addr as *mut u8).copy_to(msg.as_mut_ptr(), length);
        println!("copied");

        println!("detaching memory…");
        l4re::sys::l4re_rm_detach(virt_addr as _);
        msg
    };
    String::from_utf8(msg).map_err(|e| Error::from(e))
}

fn main() {
    let chan = l4re::sys::l4re_env_get_cap("channel").expect(
            "Received invalid channel capability.");
    let mut srv_impl = ShmServer::new(chan);
    let mut srv_loop = unsafe {
        ipc::LoopBuilder::new_at((*l4re::sys::l4re_env()).main_thread,
            l4_utcb())
        }.build();
    srv_loop.register(&mut srv_impl).expect("Failed to register server in loop");
    println!("Waiting for incoming connections");
    srv_loop.start();
}
