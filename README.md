# remotery-rs

Another rust binding for [remotery](https://github.com/Celtoys/Remotery). There is an [old crate](https://github.com/emoon/remotery-rs) that only exports cpu profiling methods. This one does more.

## Features

+ CPU
+ OpenGL
+ Metal

See [examples](./examples) for detail.

> Known issue: currently the metal example panics on exit as the assertion `nb_inuse == 0` failed in original C code.

## Note

Rust deserves a better graphics profiler but I found none.

Unfortunately this project does not fits my personal needs neither though I build it myself, but it might still be useful for getting a quick glimpse of performance hotspots so I make it public.
