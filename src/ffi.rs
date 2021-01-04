pub type Bool = ::std::os::raw::c_uint;
pub type U16 = ::std::os::raw::c_ushort;
pub type U32 = ::std::os::raw::c_uint;
pub type PStr = *const ::std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Remotery {
    _unused: [u8; 0],
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    None = 0,
    RecursiveSample = 1,
    MallocFail = 2,
    TlsAllocFail = 3,
    VirtualMemoryBufferFail = 4,
    CreateThreadFail = 5,
    SocketInitNetworkFail = 6,
    SocketCreateFail = 7,
    SocketBindFail = 8,
    SocketListenFail = 9,
    SocketSetNonBlockingFail = 10,
    SocketInvalidPoll = 11,
    SocketSelectFail = 12,
    SocketPollErrors = 13,
    SocketAcceptFail = 14,
    SocketSendTimeout = 15,
    SocketSendFail = 16,
    SocketRecvNoData = 17,
    SocketRecvTimeout = 18,
    SocketRecvFailed = 19,
    WebsocketHandshakeNotGet = 20,
    WebsocketHandshakeNoVersion = 21,
    WebsocketHandshakeBadVersion = 22,
    WebsocketHandshakeNoHost = 23,
    WebsocketHandshakeBadHost = 24,
    WebsocketHandshakeNoKey = 25,
    WebsocketHandshakeBadKey = 26,
    WebsocketHandshakeStringFail = 27,
    WebsocketDisconnected = 28,
    WebsocketBadFrameHeader = 29,
    WebsocketBadFrameHeaderSize = 30,
    WebsocketBadFrameHeaderMask = 31,
    WebsocketReceiveTimeout = 32,
    RemoteryNotCreated = 33,
    SendOnIncompleteProfile = 34,
    CudaDeinitialized = 35,
    CudaNotInitialized = 36,
    CudaInvalidContext = 37,
    CudaInvalidValue = 38,
    CudaInvalidHandle = 39,
    CudaOutOfMemory = 40,
    ErrorNotReady = 41,
    D3d11FailedToCreateQuery = 42,
    OpenglError = 43,
    CudaUnknown = 44,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::None => f.write_str("None"),
            Error::RecursiveSample => f.write_str("RecursiveSample"),
            Error::MallocFail => f.write_str("MallocFail"),
            Error::TlsAllocFail => f.write_str("TlsAllocFail"),
            Error::VirtualMemoryBufferFail => f.write_str("VirtualMemoryBufferFail"),
            Error::CreateThreadFail => f.write_str("CreateThreadFail"),
            Error::SocketInitNetworkFail => f.write_str("SocketInitNetworkFail"),
            Error::SocketCreateFail => f.write_str("SocketCreateFail"),
            Error::SocketBindFail => f.write_str("SocketBindFail"),
            Error::SocketListenFail => f.write_str("SocketListenFail"),
            Error::SocketSetNonBlockingFail => f.write_str("SocketSetNonBlockingFail"),
            Error::SocketInvalidPoll => f.write_str("SocketInvalidPoll"),
            Error::SocketSelectFail => f.write_str("SocketSelectFail"),
            Error::SocketPollErrors => f.write_str("SocketPollErrors"),
            Error::SocketAcceptFail => f.write_str("SocketAcceptFail"),
            Error::SocketSendTimeout => f.write_str("SocketSendTimeout"),
            Error::SocketSendFail => f.write_str("SocketSendFail"),
            Error::SocketRecvNoData => f.write_str("SocketRecvNoData"),
            Error::SocketRecvTimeout => f.write_str("SocketRecvTimeout"),
            Error::SocketRecvFailed => f.write_str("SocketRecvFailed"),
            Error::WebsocketHandshakeNotGet => f.write_str("WebsocketHandshakeNotGet"),
            Error::WebsocketHandshakeNoVersion => f.write_str("WebsocketHandshakeNoVersion"),
            Error::WebsocketHandshakeBadVersion => f.write_str("WebsocketHandshakeBadVersion"),
            Error::WebsocketHandshakeNoHost => f.write_str("WebsocketHandshakeNoHost"),
            Error::WebsocketHandshakeBadHost => f.write_str("WebsocketHandshakeBadHost"),
            Error::WebsocketHandshakeNoKey => f.write_str("WebsocketHandshakeNoKey"),
            Error::WebsocketHandshakeBadKey => f.write_str("WebsocketHandshakeBadKey"),
            Error::WebsocketHandshakeStringFail => f.write_str("WebsocketHandshakeStringFail"),
            Error::WebsocketDisconnected => f.write_str("WebsocketDisconnected"),
            Error::WebsocketBadFrameHeader => f.write_str("WebsocketBadFrameHeader"),
            Error::WebsocketBadFrameHeaderSize => f.write_str("WebsocketBadFrameHeaderSize"),
            Error::WebsocketBadFrameHeaderMask => f.write_str("WebsocketBadFrameHeaderMask"),
            Error::WebsocketReceiveTimeout => f.write_str("WebsocketReceiveTimeout"),
            Error::RemoteryNotCreated => f.write_str("RemoteryNotCreated"),
            Error::SendOnIncompleteProfile => f.write_str("SendOnIncompleteProfile"),
            Error::CudaDeinitialized => f.write_str("CudaDeinitialized"),
            Error::CudaNotInitialized => f.write_str("CudaNotInitialized"),
            Error::CudaInvalidContext => f.write_str("CudaInvalidContext"),
            Error::CudaInvalidValue => f.write_str("CudaInvalidValue"),
            Error::CudaInvalidHandle => f.write_str("CudaInvalidHandle"),
            Error::CudaOutOfMemory => f.write_str("CudaOutOfMemory"),
            Error::ErrorNotReady => f.write_str("ErrorNotReady"),
            Error::D3d11FailedToCreateQuery => f.write_str("D3d11FailedToCreateQuery"),
            Error::OpenglError => f.write_str("OpenglError"),
            Error::CudaUnknown => f.write_str("CudaUnknown"),
        }
    }
}

impl std::error::Error for Error {}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SampleFlags {
    None = 0,
    Aggregate = 1,
    Recursive = 2,
}

pub type MallocPtr = ::std::option::Option<
    unsafe extern "C" fn(
        mm_context: *mut ::std::os::raw::c_void,
        size: U32,
    ) -> *mut ::std::os::raw::c_void,
>;

pub type ReallocPtr = ::std::option::Option<
    unsafe extern "C" fn(
        mm_context: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
        size: U32,
    ) -> *mut ::std::os::raw::c_void,
>;

pub type FreePtr = ::std::option::Option<
    unsafe extern "C" fn(mm_context: *mut ::std::os::raw::c_void, ptr: *mut ::std::os::raw::c_void),
>;

pub type InputHandlerPtr = ::std::option::Option<
    unsafe extern "C" fn(text: *const ::std::os::raw::c_char, context: *mut ::std::os::raw::c_void),
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Settings {
    pub port: U16,
    pub reuse_open_port: Bool,
    pub limit_connections_to_localhost: Bool,
    pub ms_sleep_between_server_updates: U32,
    pub message_queue_size_in_bytes: U32,
    pub max_nb_messages_per_update: U32,
    pub malloc: MallocPtr,
    pub realloc: ReallocPtr,
    pub free: FreePtr,
    pub mm_context: *mut ::std::os::raw::c_void,
    pub input_handler: InputHandlerPtr,
    pub input_handler_context: *mut ::std::os::raw::c_void,
    pub log_filename: PStr,
}

#[test]
fn bindgen_test_layout_settings() {
    assert_eq!(
        ::std::mem::size_of::<Settings>(),
        80usize,
        concat!("Size of: ", stringify!(rmtSettings))
    );
    assert_eq!(
        ::std::mem::align_of::<Settings>(),
        8usize,
        concat!("Alignment of ", stringify!(rmtSettings))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).port as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(port)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).reuse_open_port as *const _ as usize },
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
            &(*(::std::ptr::null::<Settings>())).limit_connections_to_localhost as *const _ as usize
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
            &(*(::std::ptr::null::<Settings>())).ms_sleep_between_server_updates as *const _
                as usize
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
            &(*(::std::ptr::null::<Settings>())).message_queue_size_in_bytes as *const _ as usize
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
            &(*(::std::ptr::null::<Settings>())).max_nb_messages_per_update as *const _ as usize
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
        unsafe { &(*(::std::ptr::null::<Settings>())).malloc as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(malloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).realloc as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(realloc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).free as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(free)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).mm_context as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(mm_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).input_handler as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(input_handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).input_handler_context as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(input_handler_context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Settings>())).log_filename as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rmtSettings),
            "::",
            stringify!(logFilename)
        )
    );
}

extern "C" {
    pub fn _rmt_Settings() -> *mut Settings;
}
extern "C" {
    pub fn _rmt_CreateGlobalInstance(remotery: *mut *mut Remotery) -> Error;
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
    pub fn _rmt_SetCurrentThreadName(thread_name: PStr);
}
extern "C" {
    pub fn _rmt_LogText(text: PStr);
}
extern "C" {
    pub fn _rmt_BeginCPUSample(name: PStr, flags: U32, hash_cache: *mut U32);
}
extern "C" {
    pub fn _rmt_EndCPUSample();
}

#[cfg(feature = "cuda")]
mod cuda {
    use super::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CUDABind {
        pub context: *mut ::std::os::raw::c_void,
        pub ctx_set_current: *mut ::std::os::raw::c_void,
        pub ctx_get_current: *mut ::std::os::raw::c_void,
        pub event_create: *mut ::std::os::raw::c_void,
        pub event_destroy: *mut ::std::os::raw::c_void,
        pub event_record: *mut ::std::os::raw::c_void,
        pub event_query: *mut ::std::os::raw::c_void,
        pub event_elapsed_time: *mut ::std::os::raw::c_void,
    }

    #[test]
    fn bindgen_test_layout_rmt_cuda_bind() {
        assert_eq!(
            ::std::mem::size_of::<CUDABind>(),
            64usize,
            concat!("Size of: ", stringify!(rmtCUDABind))
        );
        assert_eq!(
            ::std::mem::align_of::<CUDABind>(),
            8usize,
            concat!("Alignment of ", stringify!(rmtCUDABind))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).context as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(context)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).ctx_set_current as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(CtxSetCurrent)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).ctx_get_current as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(CtxGetCurrent)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).event_create as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(EventCreate)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).event_destroy as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(EventDestroy)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).event_record as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(EventRecord)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).event_query as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(rmtCUDABind),
                "::",
                stringify!(EventQuery)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CUDABind>())).event_elapsed_time as *const _ as usize },
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
        pub fn _rmt_BindCUDA(bind: *const CUDABind);
    }
    extern "C" {
        pub fn _rmt_BeginCUDASample(
            name: PStr,
            hash_cache: *mut U32,
            stream: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn _rmt_EndCUDASample(stream: *mut ::std::os::raw::c_void);
    }
}

#[cfg(feature = "dx11")]
mod dx11 {
    use super::*;
    extern "C" {
        pub fn _rmt_BindD3D11(
            device: *mut ::std::os::raw::c_void,
            context: *mut ::std::os::raw::c_void,
        );
    }
    extern "C" {
        pub fn _rmt_UnbindD3D11();
    }
    extern "C" {
        pub fn _rmt_BeginD3D11Sample(name: PStr, hash_cache: *mut U32);
    }
    extern "C" {
        pub fn _rmt_EndD3D11Sample();
    }
}

#[cfg(feature = "opengl")]
mod opengl {
    use super::*;
    extern "C" {
        pub fn _rmt_BindOpenGL();
    }
    extern "C" {
        pub fn _rmt_UnbindOpenGL();
    }
    extern "C" {
        pub fn _rmt_BeginOpenGLSample(name: PStr, hash_cache: *mut U32);
    }
    extern "C" {
        pub fn _rmt_EndOpenGLSample();
    }
}

#[cfg(feature = "metal")]
mod metal {
    use super::*;
    extern "C" {
        pub fn _rmt_BeginMetalSample(name: PStr, hash_cache: *mut U32);
    }
    extern "C" {
        pub fn _rmt_EndMetalSample();
    }
}
