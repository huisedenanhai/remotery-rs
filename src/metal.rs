use crate::{ffi, Remotery};

pub trait RemoteryMetal {
    fn begin_metal_sample(&self, name: &str, hash_cache: &mut u32);
    fn end_metal_sample(&self);
    fn begin_metal_sample_scoped(&self, name: &str, hash_cache: &mut u32) -> MetalSample;
    fn bind_metal(&self, command_buffer: *mut std::ffi::c_void);
    fn unbind_metal(&self);
    fn bind_metal_scoped(&self, command_buffer: *mut std::ffi::c_void) -> MetalBinding;
}

pub struct MetalSample;
pub struct MetalBinding;

fn begin_metal_sample(name: &str, hash_cache: &mut u32) {
    unsafe {
        let name = std::ffi::CString::new(name).unwrap();
        ffi::metal::_rmt_BeginMetalSample(name.as_ptr(), hash_cache as *mut ffi::U32);
    }
}

fn end_metal_sample() {
    unsafe {
        ffi::metal::_rmt_EndMetalSample();
    }
}

fn bind_metal(command_buffer: *mut std::ffi::c_void) {
    unsafe {
        ffi::metal::_rmt_BindMetal(command_buffer as *mut _);
    }
}

fn unbind_metal() {
    unsafe {
        ffi::metal::_rmt_UnbindMetal();
    }
}

impl RemoteryMetal for Remotery {
    fn begin_metal_sample(&self, name: &str, hash_cache: &mut u32) {
        begin_metal_sample(name, hash_cache)
    }

    fn end_metal_sample(&self) {
        end_metal_sample()
    }

    fn begin_metal_sample_scoped(&self, name: &str, hash_cache: &mut u32) -> MetalSample {
        self.begin_metal_sample(name, hash_cache);
        MetalSample
    }

    fn bind_metal(&self, command_buffer: *mut std::ffi::c_void) {
        bind_metal(command_buffer);
    }

    fn unbind_metal(&self) {
        unbind_metal();
    }

    fn bind_metal_scoped(&self, command_buffer: *mut std::ffi::c_void) -> MetalBinding {
        self.bind_metal(command_buffer);
        MetalBinding
    }
}

impl Drop for MetalSample {
    fn drop(&mut self) {
        end_metal_sample();
    }
}

impl Drop for MetalBinding {
    fn drop(&mut self) {
        unbind_metal();
    }
}

#[macro_export]
macro_rules! metal_sample {
    ($rmt: expr, $name: expr) => {
        static mut HASH_CACHE: u32 = 0;
        let _sample = unsafe { $rmt.begin_metal_sample_scoped($name, &mut HASH_CACHE) };
    };
}

#[macro_export]
macro_rules! metal_binding {
    ($rmt: expr, $command_buffer: expr) => {
        let _binding = unsafe { $rmt.bind_metal_scoped($command_buffer as *mut std::ffi::c_void) };
    };
}
