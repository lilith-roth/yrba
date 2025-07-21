# YRBA - Your Remote Backup Assistant

YRBA makes backing up your systems easy, by automating incremental backups of defined folders, and uploading them to a server of your choice.

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

### Docker Compose Deploy (recommended)

The easiest way to deploy YRBA is using docker-compose.

1. Get a copy of the [docker-compose.yml](./docker-compose.yml) (manual start), or [docker-compose-cron.yml](docker-compose-cron.yml) (automated cron job based backups) file, alternatively you can also clone this repository.
2. Adjust the line `- ./folder-to-backup:/backup` in the docker compose file, and replace `./folder-to-backup` with the path to the folder you want to back up.
3. Make a copy of [config.toml](./config.toml) in the folder where your `docker-compose.yml` resides.
4. Adjust `config.toml` with the remote path to your backup server, and make further changes as desired.
 
Now you can choose which docker compose reference to run, one will run the backup a single time, and exit.
While the other option runs based on a cron job to automatically make backups on a defined schedule.

### One time run
- Run the setup using `docker compose up`
### Automated cron job setup
Per default this runs once a week, to adjust the schedule, modify the `CRON_SCHEDULE` variable in the [docker-compose-cron.yml](docker-compose-cron.yml) file.
- Run the setup using `docker compose up -d -f docker-compose-cron.yml`



### Native binary 
1. Download the latest release from the GitHub release page.
2. Copy `config.example.toml` to `config.toml`, and adjust with your remote backup path, and the folders you want to backup
3. Run software


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

### Set up development environment

1. Clone repository
2. Install dependencies & build project
```bash 
cargo build
```
3. A formatting and lint check is automatically run on every pull request.
To check this locally, and reformat your code use:
```bash
cargo clippy --verbose -- WADF
cargo fmt 
```


## License
[GPL-3.0](LICENSE)