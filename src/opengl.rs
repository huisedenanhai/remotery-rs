use crate::{ffi, Remotery};

pub trait RemoteryOpenGL {
    fn begin_opengl_sample(&self, name: &str, hash_cache: &mut u32);
    fn end_opengl_sample(&self);
    fn begin_opengl_sample_scoped(&self, name: &str, hash_cache: &mut u32) -> OpenGLSample;
}

pub struct OpenGLSample;

fn begin_opengl_sample(name: &str, hash_cache: &mut u32) {
    unsafe {
        ffi::opengl::_rmt_BeginOpenGLSample(
            std::ffi::CString::new(name).unwrap().as_ptr(),
            hash_cache as *mut ffi::U32,
        );
    }
}

fn end_opengl_sample() {
    unsafe {
        ffi::opengl::_rmt_EndOpenGLSample();
    }
}

impl RemoteryOpenGL for Remotery {
    fn begin_opengl_sample(&self, name: &str, hash_cache: &mut u32) {
        begin_opengl_sample(name, hash_cache)
    }

    fn end_opengl_sample(&self) {
        end_opengl_sample()
    }

    fn begin_opengl_sample_scoped(&self, name: &str, hash_cache: &mut u32) -> OpenGLSample {
        self.begin_opengl_sample(name, hash_cache);
        OpenGLSample
    }
}

impl Drop for OpenGLSample {
    fn drop(&mut self) {
        end_opengl_sample();
    }
}

#[macro_export]
macro_rules! opengl_sample {
    ($rmt: expr, $name: expr) => {
        static mut HASH_CACHE: u32 = 0;
        let _sample = unsafe { $rmt.begin_opengl_sample_scoped($name, &mut HASH_CACHE) };
    };
}
