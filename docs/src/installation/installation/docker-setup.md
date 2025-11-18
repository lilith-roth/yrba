# Docker Setup

There are two different, easy to set up, ways to use YRBA with Docker.

1. Single-run Container
2. Automatic cron setup

Both setups are configured almost exactly the same way. 
Scroll down to see the installation instructions.


## Single-run Container

The single run container runs the application as usual in a docker container once,
and exits after the backup process is finished.

## Automatic cron setup

The automatic cron setup will periodically run YRBA on a defined schedule.
This is useful if you want easy to set up automatic recurring backups.


## Installation

### Docker Compose (recommended)

The setup with docker compose is recommended as it is completely preconfigured, and ready to use.

1. Clone the GitHub repository to your system `git clone https://github.com/lilith-roth/yrba`
2. Copy `config.example.toml` to `config.toml`
3. Adjust `config.toml` as described in [Configuration](../../configuration/configuration.md)
4. (Optional) If using the cron schedule based setup, adjust your automatic backup schedule by adjusting the 
`CRON_SCHEDULE` line in `docker/docker-compose-cron.yml`
5. Adjust `docker/docker-compose.yml` or `docker/docker-compose-cron.yml` with correct mount paths for your backup folder, and your
authentication keys if SFTP private keys are used
6. Start the docker container with `docker compose -f docker/docker-compose.yml up` or `docker compose -f docker/docker-compose-cron.yml up` if the cron
container is used


### Manual container setup

To set up YRBA with docker, but without using docker compose follow the steps below.

1. Get the YRBA configuration file template from [GitHub](https://github.com/lilith-roth/yrba/blob/main/config.example.toml)
2. Run the YRBA docker container with the following command
```shell
docker run \
  --rm \
  --name yrba \
  -v ./config.toml:/app/config.toml \
  -v ./folder-to-backup:/backup \
  -v ~/.ssh/:/auth \
  dcpacky/yrba-official:latest \
  /app/release/yrba -c /app/config.toml
```
Adjust the command with your mount paths for the configuration file, the folder to back up, and your authentication keys
if private key authentication is desired.

### Building the docker image locally

You can easily build the docker image on your local machine either by using the `just` command, or by using the full
`docker build` command.

**Note:** Make sure you're running these commands from the root folder of the repository.

```shell
just build-docker
```
or
```shell
docker build . -f docker/Dockerfile -t dcpacky/yrba-official:latest
```
