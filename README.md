# math 
An attempt at a computer algbra system written in Rust

(It also contains a few useful traits, a proc_macro to turn mathematical notation into Rust code, avx-assembly or executable code.)


### Building:
To build it, you will need a CPU supporting `avx` and enable it:

Write the following into `math/.cargo/config` to enable it just here:
```
[build]
rustflags = "-C target-cpu=native"
```
(It also works without avx, but then the JIT insn't avaible.)
