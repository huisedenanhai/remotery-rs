fn main() {
    unsafe {
        let mut remotery: *mut Remotery = std::ptr::null_mut();
        _rmt_CreateGlobalInstance(&mut remotery as *mut *mut Remotery);
        loop {
            let mut hash: U32 = 0;
            let name = CString::new("hello rust").unwrap();
            _rmt_BeginCPUSample(name.as_ptr(), SampleFlags::None as u32, &mut hash);
            _rmt_EndCPUSample();
        }
        _rmt_DestroyGlobalInstance(remotery);
    }
    println!("Hello, world!");
}
