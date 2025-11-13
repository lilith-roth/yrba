# YRBA - Your Remote Backup Assistant

YRBA makes backing up your systems easy, by automating incremental backups of defined folders, and uploading them to a server of your choice.

**Documentation:** [https://lilith-roth.github.io/yrba/](https://lilith-roth.github.io/yrba/)


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

Detailed installation instructions can be found on the official documentation over at: [https://lilith-roth.github.io/yrba/](https://lilith-roth.github.io/yrba/)


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


### Note
Make sure the system you're uploading your backups to has the following shell commands available:
- tail
- ls
- grep
- xargs
- cd


## Contributing
Any kind of support is appreciated.

This can range from suggesting new features, to finding bugs, to coding on the project itself.

To suggest new features or report bugs, please leave a GitHub issue on this project.

For detailed guides on how to contribute, and how to build the project check out the 
[official documentation](https://lilith-roth.github.io/yrba/).


## License
[GPL-3.0](LICENSE)
