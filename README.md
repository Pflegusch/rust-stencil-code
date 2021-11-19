# rust-stencil-code
Stencil code example written in Rust making use of various techniques for efficient hardware-aware code like blocking, SIMD and multiple threads for fast execution. This project is intended as a beginner project for the Rust programming language and simply a (yet unfinished) port of the C++ version.

## Build and Run
To build the project, make sure you have the ``nightly`` version of rust installed, you can do so by updating the current version with ``rustup update -- nightly``. After that, you can build and run the project with ``cargo +nightly build`` or ``cargo +nightly run``. The current version makes use of full compiler optimizations, if you want a more debug kind of version, change the respective parameters in the ``Cargo.toml`` under ``[profile.dev]``.