# Native Installation

There are multiple ways to YRBA natively on your system.

1. System Packages (Linux)
2. Download executables (Linux, macOS, Windows)
3. Nix
4. Rust Cargo


## 1. System Packages (Linux)

You can find packages for your Linux distribution either on the release page, in the package repositories of your
distribution.

Currently, we offer packages for the following linux distributions. If yours is not one of the following distributions,
please create an [issue on the GitHub](https://github.com/lilith-roth/yrba/issues) page, and it will likely get added
very soon.

### Arch Linux AUR

An easy to install Arch Linux AUR package can be installed using any AUR helper of your choice. Check out
[AUR Instructions](./arch-aur.md) for further information.


## 2. Download Binaries

Ready to run executables can be downloaded from the GitHub releases page, and are always up to date with the latest
release of YRBA.

[You can find the download of the latest release here.](https://github.com/lilith-roth/yrba/releases)


## 3. Nix

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


## 4. Rust Cargo

If you have Rust's Cargo installed on your system, you can easily compile and run the latest version easily on your
machine, with this simple command.

Install YRBA:
```shell
cargo install yrba
```

Afterwards, you can call YRBA just by calling `yrba` in your terminal.
```shell
yrba
```


---
**Now that we installed YRBA we can continue to [configuring](../../configuration/configuration.md) YRBA.** 
