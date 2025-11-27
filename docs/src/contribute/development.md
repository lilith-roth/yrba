# Development

This guide will contain information to help you compiling YRBA, and general development informations.


## How to build

To build YRBA on your local systems, first make sure you have the Rust compiler & cargo installed on your local machine.

For further instructions regarding the installation of Rust check out:
[https://rust-lang.org/learn/get-started/](https://rust-lang.org/learn/get-started/)


The next step is to clone to git repository to your local machine.
```shell
git clone https://github.com/lilith-roth/yrba/
```

Now navigate a terminal to the YRBA project on your local machine, and compile the project using cargo.
```shell
cargo build
```

A release optimized build can be built using the `--release` flag.
```shell
cargo build --release
```


## Code formatting
A formatting and lint check is automatically run on every pull request. To check this locally, and reformat your code use:
```bash
cargo clippy --verbose # Runs the clippy linter on your code
cargo fmt # Automatically reformats your code files
```


## Troubleshooting

### Compilation fails with `openssl not found`

In some cases, especially when doing cross-compilation, the build process may fail with an error message something like:
`Could not find openssl via pkg-config`.

The easiest solution to this issue is to compile YRBA with the `vendored-openssl` flag. This will build openssl from
source during the compilation of YRBA.

**Note:** This may increase the compilation time substantially. The alternative to using the feature flag is to
install & configure the openssl development libraries on your system so that the Rust compiler can find them.

This is out of scope for this guide though if you run into any issues, please create a
[GitHub issue](https://github.com/lilith-roth/yrba/issues), and we can find a solution together.
