use crate::ffi;

pub use ffi::{Error, SampleFlags, Settings};

pub struct Remotery {
    ptr: *mut ffi::Remotery,
}

impl Drop for Remotery {
    fn drop(&mut self) {
        unsafe {
            #[cfg(feature = "opengl")]
            ffi::opengl::_rmt_UnbindOpenGL();

            ffi::_rmt_DestroyGlobalInstance(self.ptr);
        }
    }
}

impl Remotery {
    // there can only be one Remotery instance
    pub unsafe fn new(settings: Settings) -> Result<Remotery, Error> {
        let mut remotery: *mut ffi::Remotery = std::ptr::null_mut();
        *ffi::_rmt_Settings().as_mut().unwrap() = settings;
        let error = ffi::_rmt_CreateGlobalInstance(&mut remotery as *mut *mut ffi::Remotery);
        if error == Error::None {
            #[cfg(feature = "opengl")]
            ffi::opengl::_rmt_BindOpenGL();

            Ok(Remotery { ptr: remotery })
        } else {
            Err(error)
        }
    }

    pub fn set_current_thread_name(&self, name: &str) {
        unsafe { ffi::_rmt_SetCurrentThreadName(std::ffi::CString::new(name).unwrap().as_ptr()) }
    }

    pub fn log_text(&self, text: &str) {
        unsafe { ffi::_rmt_LogText(std::ffi::CString::new(text).unwrap().as_ptr()) }
    }

    pub fn begin_cpu_sample(&self, name: &str, flags: SampleFlags, hash_cache: &mut u32) {
        unsafe {
            ffi::_rmt_BeginCPUSample(
                std::ffi::CString::new(name).unwrap().as_ptr(),
                flags as ffi::U32,
                hash_cache as *mut ffi::U32,
            )
        }
    }

    pub fn end_cpu_sample(&self) {
        unsafe {
            ffi::_rmt_EndCPUSample();
        }
    }

    pub fn begin_cpu_sample_scoped(
        &self,
        name: &str,
        flags: SampleFlags,
        hash_cache: &mut u32,
    ) -> CpuSample {
        self.begin_cpu_sample(name, flags, hash_cache);
        CpuSample { remotery: self }
    }
}

pub struct CpuSample<'a> {
    remotery: &'a Remotery,
}

impl<'a> Drop for CpuSample<'a> {
    fn drop(&mut self) {
        self.remotery.end_cpu_sample();
    }
}

#[macro_export]
macro_rules! cpu_sample {
    ($rmt: expr, $name: expr) => {
        cpu_sample!($rmt, $name, SampleFlags::None)
    };
    ($rmt: expr, $name: expr, $flags: expr) => {
        static mut HASH_CACHE: u32 = 0;
        let _sample = unsafe { $rmt.begin_cpu_sample_scoped($name, $flags, &mut HASH_CACHE) };
    };
}

#[test]
fn test_cpu_sample_macro() {
    let remotery = unsafe { Remotery::new(Settings::default()) }.unwrap();
    {
        cpu_sample!(remotery, "scope outer");
        {
            {
                cpu_sample!(remotery, "scope 1");
            }
            {
                cpu_sample!(remotery, "scope 2");
            }
        }
    }
}
