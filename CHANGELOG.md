# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.2.1](https://github.com/lilith-roth/yrba/compare/v2.2.0...v2.2.1) - 2025-11-27

### Other

- small typo

## [2.2.0](https://github.com/lilith-roth/yrba/compare/v2.1.1...v2.2.0) - 2025-11-27

### Added

- smb implementation ([#75](https://github.com/lilith-roth/yrba/pull/75))

### Fixed

- *(docker)* added a tmp docker volume  ([#77](https://github.com/lilith-roth/yrba/pull/77))

### Other

- macOS builds broken due to deprecated CI runner ([#85](https://github.com/lilith-roth/yrba/pull/85))
- *(deps)* bump actions/upload-pages-artifact from 3 to 4 ([#81](https://github.com/lilith-roth/yrba/pull/81))
- *(deps)* bump actions/checkout from 4 to 6 ([#82](https://github.com/lilith-roth/yrba/pull/82))
- *(deps)* bump peter-evans/dockerhub-description from 4.0.0 to 5.0.0 ([#80](https://github.com/lilith-roth/yrba/pull/80))
- *(deps)* bump actions/upload-artifact from 4 to 5 ([#79](https://github.com/lilith-roth/yrba/pull/79))
- Two small docs changes ([#78](https://github.com/lilith-roth/yrba/pull/78))
- *(deps)* bump docker/build-push-action from 4.0.0 to 6.18.0 ([#68](https://github.com/lilith-roth/yrba/pull/68))
- *(deps)* bump docker/metadata-action from 9ec57ed1fcdbf14dcef7dfbe97b2010124a938b7 to 8d8c7c12f7b958582a5cb82ba16d5903cb27976a ([#71](https://github.com/lilith-roth/yrba/pull/71))
- *(deps)* bump actions/configure-pages from 4 to 5 ([#70](https://github.com/lilith-roth/yrba/pull/70))
- *(deps)* bump actions/attest-build-provenance from 2 to 3 ([#69](https://github.com/lilith-roth/yrba/pull/69))

- fix: added docker volume mounts 

## [2.1.1](https://github.com/lilith-roth/yrba/compare/v2.1.0...v2.1.1) - 2025-11-18

### Other

- *(deps)* bump docker/login-action from 2.1.0 to 3.6.0 ([#67](https://github.com/lilith-roth/yrba/pull/67))
- docker cleanup ([#65](https://github.com/lilith-roth/yrba/pull/65))
- improved error handling ([#66](https://github.com/lilith-roth/yrba/pull/66))

## [2.1.0](https://github.com/lilith-roth/yrba/compare/v2.0.2...v2.1.0) - 2025-11-17

### Added

- file copy backups ([#58](https://github.com/lilith-roth/yrba/pull/58))

### Other

- readme badges ([#62](https://github.com/lilith-roth/yrba/pull/62))
- improve rust checks ([#60](https://github.com/lilith-roth/yrba/pull/60))

## [2.0.2](https://github.com/lilith-roth/yrba/compare/v2.0.1...v2.0.2) - 2025-11-14

### Other

- binary deployments broken

## [2.0.1](https://github.com/lilith-roth/yrba/compare/v2.0.0...v2.0.1) - 2025-11-14

### Other

- deb packaging ([#55](https://github.com/lilith-roth/yrba/pull/55))
- packaging & PR binary builds ([#54](https://github.com/lilith-roth/yrba/pull/54))

## [2.0.0](https://github.com/lilith-roth/yrba/compare/v1.3.0...v2.0.0) - 2025-11-13

### Other

- *(deps)* bump rust from 1.88-alpine3.22 to 1.91-alpine3.22 ([#51](https://github.com/lilith-roth/yrba/pull/51))
- [**breaking**] implemented strict clippy checks & impr. abbrevations ([#45](https://github.com/lilith-roth/yrba/pull/45))
- create dependabot.yml ([#49](https://github.com/lilith-roth/yrba/pull/49))
- update for custom documentation domain
- Update issue templates
- Create FUNDING.yml
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
