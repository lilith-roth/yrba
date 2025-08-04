# Native Installation

There are multiple ways to YRBA natively on your system.

1. Download executables
2. Nix
3. Rust Cargo

## 1. Download Binaries

Ready to run executables can be downloaded from the GitHub releases page, and are always up to date with the latest
release of YRBA.

[You can find the download of the latest release here.](https://github.com/lilith-roth/yrba/releases)

## 2. Nix

If you have Nix installed on your system be it Linux, macOS or Windows, you can easily install & run YRBA with a single
command.

```shell
nix run github:lilith-roth/yrba
```

In case you need to add parameters while calling the application, you have to append a `--` at the end of the command,
before the parameters.

Example:
```shell
nix run github:lilith-roth/yrba -- --config ./config.toml
```


## 3. Rust Cargo

If you have Rust's Cargo installed on your system, you can easily compile and run the latest version easily on your
machine, with this simple command.

Install YRBA:
```shell
cargo add yrba
```

Afterwards, you can call YRBA just by calling `yrba` in your terminal.
```shell
yrba
```
