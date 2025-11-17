# YRBA - Your Remote Backup Assistant

![GitHub Release Date](https://img.shields.io/github/release-date/lilith-roth/yrba?label=Release)
![GitHub branch check runs](https://img.shields.io/github/check-runs/lilith-roth/yrba/main?label=Checks)
![GitHub Sponsors](https://img.shields.io/github/sponsors/lilith-roth?label=Sponsors)
![GitHub License](https://img.shields.io/github/license/lilith-roth/yrba?label=License)

![Crates.io Total Downloads](https://img.shields.io/crates/d/yrba?label=Downloads%20crates.io)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/lilith-roth/yrba/total?label=Downloads%20GitHub)
![Docker Pulls](https://img.shields.io/docker/pulls/dcpacky/yrba-official?label=Docker%20pulls)
![AUR Votes](https://img.shields.io/aur/votes/yrba-git?label=AUR%20votes)


YRBA makes backing up your systems easy, by automating incremental backups of defined folders, 
and uploading them to a server of your choice.

**Documentation:** [https://yrba.roth.systems/](https://yrba.roth.systems/)


## Features

- Archives your backup as .tar.gz
- Incremental backups automatically keeps the last N backups
- Can back up directories on any OS
- - GNU/Linux
- - macOS
- - Windows (untested)
- Automatic uploads with SFTP (NFS & file copy are planned)
- Can upload backups to Unix systems
- - GNU/Linux
- - macOS


## Installation

Detailed installation instructions can be found on the official documentation over at:
[https://yrba.roth.systems/](https://yrba.roth.systems/)


### Usage
```
Usage: yrba [OPTIONS]

Options:
  -v, --verbose...                 Increase logging verbosity
  -q, --quiet...                   Decrease logging verbosity
  -c, --config <CONFIG_FILE_PATH>  [default: ~/.config/yrba/config.toml]
  -h, --help                       Print help
  -V, --version                    Print version
```


## Contributing
Any kind of support is appreciated.

This can range from suggesting new features, to finding bugs, to coding on the project itself.

To suggest new features or report bugs, please leave a GitHub issue on this project.

For detailed guides on how to contribute, and how to build the project check out the 
[official documentation](https://yrba.roth.systems/).


## License
[GPL-3.0](LICENSE)
