fn main() {
    cc::Build::new()
        .file("Remotery/lib/Remotery.c")
        .compile("remotery");
}
