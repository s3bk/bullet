# bullet 
An attempt at a computer algbra system written in Rust

(It also contains a few useful traits, a proc_macro to turn mathematical notation into Rust code, avx-assembly or executable code.)


### Building:
You will need nightly rust distribution (not stable). You can use [ustup](https://rust-lang.github.io/rustup/overrides.html#directory-overrides) to get the nightly versions.
To build it, you will need a CPU supporting `avx` and enable it:

Write the following into `math/.cargo/config` to enable it just here:
```
[build]
rustflags = "-C target-cpu=native"
```
(It also works without avx, but then the JIT insn't avaible.)
