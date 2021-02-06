# SealPIR (Rust): Rust wrappers for SealPIR

SealPIR is a research library and should not be used in production systems. SealPIR allows a client to download an element from a database stored by a server without revealing which element was downloaded. SealPIR was introduced in our [paper](https://eprint.iacr.org/2017/1142.pdf).

SealPIR relies on SEAL v3.2.0. The rest of this README assumes that the SEAL v3.2.0 is installed natively. You can get SEAL v3.2.0 from this [link](http://sealcrypto.org).


# Compiling SealPIR-Rust

SealPIR's Rust wrapper works with [Rust](https://www.rust-lang.org/) nightly (we have tested with Rust 1.49.0). It also depends on the C++ version of SealPIR (included as a submodule) and SEAL (see above). We have tested these versions with g++ 9.3.0.

To compile SealPIR-Rust just do:

```sh
$ git submodule init
$ git submodule update
$ cargo build
```
