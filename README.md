# Monero Atomic Swap Utilities

A collection of C++ utilities to execute atomic swaps with Monero.

# Dependencies

* [COMIT](https://github.com/comit-network) (written in Rust)

# Compiling from source
1. Install Rust for your system<br>
  Mac: `brew install rustup && rustup-init`
2. `rustup install nightly`
3. `rustup override set nightly`
4. `source $HOME/.cargo/env` to configure current shell
5. `./bin/build_xmr_atomic_swap_cpp`