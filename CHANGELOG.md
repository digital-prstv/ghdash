# Changelog

All notable changes to this project will be documented in this file.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Refactor

- Use HashMap instead of Vec
- Use HashMap instead of Vec
- Only need issues count pulls seperatly
- Use confy_table for better formatting

## [0.1.4] - 2023-02-11

### Features

- Tracing with jaeger at trace level
- Don't publish (at least no yet :) )

### Miscellaneous Tasks

- Hide any performance related files

### Refactor

- Restrict console to ghdash logs
- Grooming order and don't log  token
- Make Anyhow transparent
- Spawn threads for  data collection

## [0.1.3] - 2022-12-26

### Bug Fixes

- Remove redundant use

### Features

- Verbosity, logging, octorust fix
- Add logging and refacto
- Logging,  private_repos

### Refactor

- Clippy suggested changes
- Make pub(crate); remove getters
- Use Display trait to produce Dashboard

### Testing

- Update tests

## [0.1.2] - 2022-11-27

### Features

- Option to show forked repos
- Option to show all public repos

### Miscellaneous Tasks

- Release documentation

## [0.1.1] - 2022-11-23

### Features

- Add pr_count to dasboard

### Miscellaneous Tasks

- Exclude coverage reports

### Testing

- Update tests to include user option

## [0.1.0] - 2022-11-23

### Documentation

- Documentation and config option
- Update progress on features in Readme

### Features

- Basic clap application
- Support libraty
- Initial struct for basic dashboard
- Config for binary using confy
- Config for binary using confy
- Set alternate config
- Error enum for library and binary
- Capture user and store to config
- Get token from cli
- Extract authored repository list for dashboard
- Extract authored repository list for dashboard
- Print list of public repos authored by the user in a grid

### Refactor

- Seperate clap cli configuration

### Testing

- Testing for cli options
- Update tests for change in cli spec

<!-- next-url -->
[Unreleased]: https://github.com/jerusdp/ghdash/compare/v0.1.4...HEAD
[0.1.4]: https://github.com/jerusdp/ghdash/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerusdp/ghdash/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerusdp/ghdash/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/jerusdp/ghdash/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/jerusdp/ghdash/compare/...v0.1.1

