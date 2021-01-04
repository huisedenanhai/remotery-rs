#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let mut remotery: *mut Remotery = std::ptr::null_mut();
        _rmt_CreateGlobalInstance(&mut remotery as *mut *mut Remotery);
        loop {
            let mut hash: rmtU32 = 0;
            let name = CString::new("hello rust").unwrap();
            _rmt_BeginCPUSample(name.as_ptr(), rmtSampleFlags_RMTSF_None, &mut hash);
            _rmt_EndCPUSample();
        }
        _rmt_DestroyGlobalInstance(remotery);
    }
    println!("Hello, world!");
}
