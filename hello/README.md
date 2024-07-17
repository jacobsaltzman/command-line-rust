# hello

## project notes

- Used `cargo new` to create a new Rust program that prints “Hello, world!”
- The Rust compiler `rustc` compiled Rust source code into a machine-executable file.
- Using `cargo run` will bundle the process of compiling, testing, and running.
- Put crate dependencies into `Cargo.toml`
- Used `#[test]` to mark functions that should be executed as tests.
- Created source code files in the `src/bin` directory.
- Check the text printed to STDOUT from something like `cargo run --quiet --bin false` then `echo $?` to check the output.
- Implemented the `true` and `false` programs along with tests to verify that they succeed and fail as expected. 
- Rust program will exit with the value zero
- The `std::process::exit` function can be used to explicitly exit with a given code. 
- Additionally, the `std::process::abort` function can be used to exit with a nonzero error code.