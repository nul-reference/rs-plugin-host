# rs-plugin-host

This is an experiment of using a C API to load dynamic libraries via a Rust binary, wherein the binary has no working knowledge of the structs may want to send one another.

__Note__: I realize this is likely not fruitful, almost certainly not a good idea, and there may be dragons, with respect to performance, ergonomics, etc. This is less a tutorial on what _should_ be done, and more an example of what _could_ be done.
