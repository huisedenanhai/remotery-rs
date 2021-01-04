fn main() {
    let mut build = cc::Build::new();

    #[cfg(feature = "assume-little-endian")]
    build.define("RMT_ASSUME_LITTLE_ENDIAN", "1");

    #[cfg(feature = "cuda")]
    build.define("RMT_USE_CUDA", "1");

    #[cfg(feature = "dx11")]
    build.define("RMT_USE_D3D11", "1");

    #[cfg(feature = "metal")]
    build
        .define("RMT_USE_METAL", "1")
        .file("Remotery/lib/RemoteryMetal.mm");

    #[cfg(feature = "opengl")]
    build.define("RMT_USE_OPENGL", "1");

    #[cfg(feature = "posix-thread-names")]
    build.define("RMT_USE_POSIX_THREADNAMES", "1");

    build.file("Remotery/lib/Remotery.c").compile("remotery");
}
