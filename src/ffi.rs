/* automatically generated by rust-bindgen */

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub const RMT_ENABLED: u32 = 1;
pub const RMT_ASSUME_LITTLE_ENDIAN: u32 = 0;
pub const RMT_USE_TINYCRT: u32 = 0;
pub const RMT_USE_CUDA: u32 = 0;
pub const RMT_USE_D3D11: u32 = 0;
pub const RMT_USE_OPENGL: u32 = 0;
pub const RMT_USE_METAL: u32 = 0;
pub const RMT_USE_POSIX_THREADNAMES: u32 = 0;
pub const RMT_GPU_CPU_SYNC_NUM_ITERATIONS: u32 = 16;
pub const RMT_GPU_CPU_SYNC_SECONDS: u32 = 30;
pub const RMT_D3D11_RESYNC_ON_DISJOINT: u32 = 1;
pub type rmtBool = ::std::os::raw::c_uint;
pub type rmtU8 = ::std::os::raw::c_uchar;
pub type rmtU16 = ::std::os::raw::c_ushort;
pub type rmtU32 = ::std::os::raw::c_uint;
pub type rmtU64 = ::std::os::raw::c_ulonglong;
pub type rmtS8 = ::std::os::raw::c_char;
pub type rmtS16 = ::std::os::raw::c_short;
pub type rmtS32 = ::std::os::raw::c_int;
pub type rmtS64 = ::std::os::raw::c_longlong;
pub type rmtPStr = *const ::std::os::raw::c_char;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Remotery {
    _unused: [u8; 0],
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rmtError {
    RMT_ERROR_NONE = 0,
    RMT_ERROR_RECURSIVE_SAMPLE = 1,
    RMT_ERROR_MALLOC_FAIL = 2,
    RMT_ERROR_TLS_ALLOC_FAIL = 3,
    RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL = 4,
    RMT_ERROR_CREATE_THREAD_FAIL = 5,
    RMT_ERROR_SOCKET_INIT_NETWORK_FAIL = 6,
    RMT_ERROR_SOCKET_CREATE_FAIL = 7,
    RMT_ERROR_SOCKET_BIND_FAIL = 8,
    RMT_ERROR_SOCKET_LISTEN_FAIL = 9,
    RMT_ERROR_SOCKET_SET_NON_BLOCKING_FAIL = 10,
    RMT_ERROR_SOCKET_INVALID_POLL = 11,
    RMT_ERROR_SOCKET_SELECT_FAIL = 12,
    RMT_ERROR_SOCKET_POLL_ERRORS = 13,
    RMT_ERROR_SOCKET_ACCEPT_FAIL = 14,
    RMT_ERROR_SOCKET_SEND_TIMEOUT = 15,
    RMT_ERROR_SOCKET_SEND_FAIL = 16,
    RMT_ERROR_SOCKET_RECV_NO_DATA = 17,
    RMT_ERROR_SOCKET_RECV_TIMEOUT = 18,
    RMT_ERROR_SOCKET_RECV_FAILED = 19,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET = 20,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION = 21,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION = 22,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST = 23,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST = 24,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY = 25,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY = 26,
    RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL = 27,
    RMT_ERROR_WEBSOCKET_DISCONNECTED = 28,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER = 29,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_SIZE = 30,
    RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_MASK = 31,
    RMT_ERROR_WEBSOCKET_RECEIVE_TIMEOUT = 32,
    RMT_ERROR_REMOTERY_NOT_CREATED = 33,
    RMT_ERROR_SEND_ON_INCOMPLETE_PROFILE = 34,
    RMT_ERROR_CUDA_DEINITIALIZED = 35,
    RMT_ERROR_CUDA_NOT_INITIALIZED = 36,
    RMT_ERROR_CUDA_INVALID_CONTEXT = 37,
    RMT_ERROR_CUDA_INVALID_VALUE = 38,
    RMT_ERROR_CUDA_INVALID_HANDLE = 39,
    RMT_ERROR_CUDA_OUT_OF_MEMORY = 40,
    RMT_ERROR_ERROR_NOT_READY = 41,
    RMT_ERROR_D3D11_FAILED_TO_CREATE_QUERY = 42,
    RMT_ERROR_OPENGL_ERROR = 43,
    RMT_ERROR_CUDA_UNKNOWN = 44,
}
pub const rmtSampleFlags_RMTSF_None: rmtSampleFlags = 0;
pub const rmtSampleFlags_RMTSF_Aggregate: rmtSampleFlags = 1;
pub const rmtSampleFlags_RMTSF_Recursive: rmtSampleFlags = 2;
pub type rmtSampleFlags = u32;
pub type rmtMallocPtr = ::std::option::Option<
    unsafe extern "C" fn(
        mm_context: *mut ::std::os::raw::c_void,
        size: rmtU32,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type rmtReallocPtr = ::std::option::Option<
    unsafe extern "C" fn(
        mm_context: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
        size: rmtU32,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type rmtFreePtr = ::std::option::Option<
    unsafe extern "C" fn(mm_context: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void),
>;
pub type rmtInputHandlerPtr = ::std::option::Option<
    unsafe extern "C" fn(text: *const ::std::os::raw::c_char, context: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rmtSettings {
    pub port: rmtU16,
    pub reuse_open_port: rmtBool,
    pub limit_connections_to_localhost: rmtBool,
    pub msSleepBetweenServerUpdates: rmtU32,
    pub messageQueueSizeInBytes: rmtU32,
    pub maxNbMessagesPerUpdate: rmtU32,
    pub malloc: rmtMallocPtr,
    pub realloc: rmtReallocPtr,
    pub free: rmtFreePtr,
    pub mm_context: *mut ::std::os::raw::c_void,
    pub input_handler: rmtInputHandlerPtr,
    pub input_handler_context: *mut ::std::os::raw::c_void,
    pub logFilename: rmtPStr,
}
#[test]
fn bindgen_test_layout_rmtSettings() {
    assert_eq!(
        ::std::mem::size_of::<rmtSettings>(),
        80usize,
        concat!("Size of: ", stringify!(rmtSettings))
    );
    assert_eq!(
        ::std::mem::align_of::<rmtSettings>(),
        8usize,
        concat!("Alignment of ", stringify!(rmtSettings))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).port as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).reuse_open_port as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(reuse_open_port)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rmtSettings>())).limit_connections_to_localhost as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(limit_connections_to_localhost)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rmtSettings>())).msSleepBetweenServerUpdates as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(msSleepBetweenServerUpdates)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rmtSettings>())).messageQueueSizeInBytes as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(messageQueueSizeInBytes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rmtSettings>())).maxNbMessagesPerUpdate as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(maxNbMessagesPerUpdate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).malloc as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(malloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).realloc as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(realloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).free as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).mm_context as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(mm_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).input_handler as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(input_handler)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rmtSettings>())).input_handler_context as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(input_handler_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtSettings>())).logFilename as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(logFilename)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rmtCUDABind {
    pub context: *mut ::std::os::raw::c_void,
    pub CtxSetCurrent: *mut ::std::os::raw::c_void,
    pub CtxGetCurrent: *mut ::std::os::raw::c_void,
    pub EventCreate: *mut ::std::os::raw::c_void,
    pub EventDestroy: *mut ::std::os::raw::c_void,
    pub EventRecord: *mut ::std::os::raw::c_void,
    pub EventQuery: *mut ::std::os::raw::c_void,
    pub EventElapsedTime: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_rmtCUDABind() {
    assert_eq!(
        ::std::mem::size_of::<rmtCUDABind>(),
        64usize,
        concat!("Size of: ", stringify!(rmtCUDABind))
    );
    assert_eq!(
        ::std::mem::align_of::<rmtCUDABind>(),
        8usize,
        concat!("Alignment of ", stringify!(rmtCUDABind))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).context as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).CtxSetCurrent as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(CtxSetCurrent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).CtxGetCurrent as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(CtxGetCurrent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).EventCreate as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(EventCreate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).EventDestroy as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(EventDestroy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).EventRecord as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(EventRecord)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).EventQuery as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(EventQuery)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rmtCUDABind>())).EventElapsedTime as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtCUDABind),
            "::",
            stringify!(EventElapsedTime)
        )
    );
}
extern "C" {
    pub fn _rmt_Settings() -> *mut rmtSettings;
}
extern "C" {
    pub fn _rmt_CreateGlobalInstance(remotery: *mut *mut Remotery) -> rmtError;
}
extern "C" {
    pub fn _rmt_DestroyGlobalInstance(remotery: *mut Remotery);
}
extern "C" {
    pub fn _rmt_SetGlobalInstance(remotery: *mut Remotery);
}
extern "C" {
    pub fn _rmt_GetGlobalInstance() -> *mut Remotery;
}
extern "C" {
    pub fn _rmt_SetCurrentThreadName(thread_name: rmtPStr);
}
extern "C" {
    pub fn _rmt_LogText(text: rmtPStr);
}
extern "C" {
    pub fn _rmt_BeginCPUSample(name: rmtPStr, flags: rmtU32, hash_cache: *mut rmtU32);
}
extern "C" {
    pub fn _rmt_EndCPUSample();
}