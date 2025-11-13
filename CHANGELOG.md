# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.3.1](https://github.com/lilith-roth/yrba/compare/v1.3.0...v1.3.1) - 2025-11-13

### Other

- README.md cleanup
- improve documentation
- minor code improvement

## [1.3.0](https://github.com/lilith-roth/yrba/compare/v1.2.1...v1.3.0) - 2025-11-12

### Added

- automatic creation of config file ([#41](https://github.com/lilith-roth/yrba/pull/41))

## [1.2.1](https://github.com/lilith-roth/yrba/compare/v1.2.0...v1.2.1) - 2025-11-12

### Fixed

- *(systemd-service)* use system config

## [1.2.0](https://github.com/lilith-roth/yrba/compare/v1.1.8...v1.2.0) - 2025-11-12

### Added

- systemd service & timers ([#38](https://github.com/lilith-roth/yrba/pull/38))

## [1.1.8](https://github.com/lilith-roth/yrba/compare/v1.1.7...v1.1.8) - 2025-11-11

### Other

- fix windows binary builds

## [1.1.7](https://github.com/lilith-roth/yrba/compare/v1.1.6...v1.1.7) - 2025-11-11

### Other

- fixed binary release builds

## [1.1.6](https://github.com/lilith-roth/yrba/compare/v1.1.5...v1.1.6) - 2025-11-10

### Fixed

- *(docs)* linking of index page

### Other

- fix release builds
- *(flake.nix)* allow running on aarch64-darwin
- *(TODO.md)* update todo
- *(TODO.md)* update todo
- *(readme.md)* added mention of the official documentation

## [1.1.5](https://github.com/lilith-roth/yrba/compare/v1.1.4...v1.1.5) - 2025-11-06

### Other

- mdbook documentation added ([#18](https://github.com/lilith-roth/yrba/pull/18))

## [1.1.4](https://github.com/lilith-roth/yrba/compare/v1.1.3...v1.1.4) - 2025-11-06

### Other

- dep update

## [1.1.3](https://github.com/lilith-roth/yrba/compare/v1.1.2...v1.1.3) - 2025-11-06

### Other

- *(release-plz)* updated release pr config
- *(docker)* moved command params to docker compose file ([#15](https://github.com/lilith-roth/yrba/pull/15))
- explicit typing & improved logging ([#14](https://github.com/lilith-roth/yrba/pull/14))

## [1.1.2](https://github.com/lilith-roth/yrba/compare/v1.1.1...v1.1.2) - 2025-11-06

### Fixed

- temporary archives did not get deleted ([#10](https://github.com/lilith-roth/yrba/pull/10))

### Other

- *(gitignore)* minor gitignore update ([#13](https://github.com/lilith-roth/yrba/pull/13))
- *(sftp upload)* minor code cleanup & impr. log msg ([#12](https://github.com/lilith-roth/yrba/pull/12))
- *(sftp upload)* limited buffer size for SFTP uploads ([#8](https://github.com/lilith-roth/yrba/pull/8))
- *(gitignore)* added working folders to gitignore ([#9](https://github.com/lilith-roth/yrba/pull/9))

## [1.1.1](https://github.com/lilith-roth/yrba/compare/v1.1.0...v1.1.1) - 2025-09-24

### Fixed

- memory allocation for upload

### Other

- typo in README.md

## [1.1.0](https://github.com/lilith-roth/yrba/compare/v1.0.1...v1.1.0) - 2025-07-21

### Added

- docker deployment ([#5](https://github.com/lilith-roth/yrba/pull/5))

## [1.0.1](https://github.com/lilith-roth/yrba/releases/tag/v1.0.1) - 2025-07-11

### Added

- base features ([#1](https://github.com/lilith-roth/yrba/pull/1))

### Fixed

- release not working due to broken license field
- relative home paths did not work for config file ([#2](https://github.com/lilith-roth/yrba/pull/2))

### Other

- bump version for bugfix & release trigger
- updated release-plz.toml to allow release on fix commits
- binary release CD pipeline
- release v1.0.0 ([#3](https://github.com/lilith-roth/yrba/pull/3))
- release-plz integration
- Initial commit

## [1.0.0](https://github.com/lilith-roth/yrba/releases/tag/v1.0.0) - 2025-07-11

### Added

- base features ([#1](https://github.com/lilith-roth/yrba/pull/1))

### Fixed

- relative home paths did not work for config file ([#2](https://github.com/lilith-roth/yrba/pull/2))

### Other

- release-plz integration
- Initial commit
