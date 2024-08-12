# Changelog

All notable changes to this project are documented in this file.

## [Unreleased]

### Added

- Prepare for github app integration with second binary(pr [#127])
- add GitHub API integration with octocrate(pr [#128])

### Changed

- ci-adopt toolkit and standardise changelog (pr [#95])
- chore-add opentelemetry packages group(pr [#120])
- ci-upgrade jerusdp/circleci-toolkit orb version from 0.24.1 to 1.0.0(pr [#129])

### Security

- Dependencies: update rust crate tokio to v1.38.1(pr [#99])
- Dependencies: update rust crate thiserror to v1.0.63(pr [#100])
- Dependencies: bump openssl from 0.10.64 to 0.10.66 in the cargo group across 1 directory(pr [#101])
- Dependencies: update rust crate trycmd to v0.15.5(pr [#103])
- Dependencies: update rust crate clap to v4.5.10(pr [#102])
- Dependencies: update rust crate tokio to v1.39.1(pr [#104])
- Dependencies: update rust crate clap to v4.5.11(pr [#105])
- Dependencies: update rust crate anyhow to 1.0.86(pr [#108])
- Dependencies: update rust crate clap to 4.5.11(pr [#109])
- Dependencies: update rust crate log to 0.4.22(pr [#110])
- Dependencies: update rust crate reqwest to 0.12.5(pr [#111])
- Dependencies: update rust crate tokio to 1.39.1(pr [#113])
- Dependencies: update opentelemetry packages(pr [#121])
- Dependencies: update rust crate thiserror to 1.0.63(pr [#112])
- Dependencies: update rust crate trycmd to 0.15.5(pr [#114])
- Dependencies: update rust crate trycmd to 0.15.6(pr [#123])
- Dependencies: update serde packages to 1.0.204(pr [#115])
- Dependencies: update rust crate opentelemetry_sdk to 0.24.1(pr [#122])
- Dependencies: update rust crate bollard to 0.17.0(pr [#124])
- Dependencies: update rust crate clap-verbosity-flag to 2.2.1(pr [#125])
- Dependencies: update rust crate tokio to 1.39.2(pr [#126])

## [0.1.7] - 2024-03-07

### Added

- Add octorust client setup error to the errors to allow use of '?'
- Only enable tracing if zipkin container is running
- Added logging without trace level
- Added fallback to default connection on failure in main
- Get user and token config values from the env

### Changed

- Create public readme to jerus-org standards
- Updated documentation for clarity and structure
- Update rust crate trycmd to 0.14.19
- Update rust crate trycmd to 0.14.20
- Update min rust to 1.70 to assure compatibility with dependencies
- Update rust crate trycmd to 0.14.21
- Update rust crate trycmd to 0.15.0
- Switched patch from path to git
- Scripted commands to duplicate files into without_zipkin
- Update spelling words
- Tracking of results returned in docker connect
- Testing CI with new feature enum to provide or deny docker connection.
- Println of result to see what is coming out of the connect_docker
- Removed final print spoiling testing
- List command to list the containers and step in CI
- Added image
- Tidy up container list presentation and a ensure all tests passing
- Use existing container  (with code checks)
- Stop the running container
- Clean-up and hide the list command
- Updated all opentelemetry crates
- Extract html response to helper function
- Replaced jaeger with zipkin as tracing target
- Restructured logging testing to confirm version of verbosity set
- Logging testing to use single source files
- Moved logging to separate module and cleaned out println macros
- Moved logging module to lib and publish docker connection
- Moved logging module to lib and publish docker connection
- Renamed logging module as log module
- Updated test result expectations
- Added test for run of ghdash without any configuration
- Duplicated cmd tests for with and without zipkin
- Refactored to split out testing cases and test runners
- Added log module to ghdash logging text as now part of a module.
- Removed table content from dashboard
- Circle CI testing
- Align MSRV with that required for clap
- Removed all-features as not required and causing failure of CI.
- Removed all-features as not required and causing failure of CI.
- Added setup_remote_docker to allow testing "with_zipkin"
- Corrected setup_remote_docker to use default image.
- Added step to pull down the openzipkin/zipkin image
- Removed run and added image as a second image in the executor
- Installation step for docker cli to support testing steps
- Used apk add instead of apt install

### Fixed

- Update rust crate anyhow to 1.0.75
- Update rust crate log to 0.4.20
- Update rust crate comfy-table to v7
- Update rust crate tracing-opentelemetry to 0.22.0
- Update rust crate tokio to 1.35.0
- Update rust crate opentelemetry-jaeger to 0.20.0
- Update rust crate clap-verbosity-flag to 2.1.1
- Update rust crate opentelemetry to 0.21.0
- Update rust crate tracing to 0.1.40
- Update rust crate clap to 4.4.11
- Update rust crate tracing-subscriber to 0.3.18
- Update rust crate thiserror to 1.0.51
- Update serde monorepo to 1.0.193
- Update rust crate tokio to 1.35.1
- Update rust crate anyhow to 1.0.76
- Update rust crate thiserror to 1.0.52
- Update rust crate anyhow to 1.0.77
- Update rust crate clap to 4.4.12
- Update rust crate thiserror to 1.0.53
- Update rust crate anyhow to 1.0.78
- Update serde monorepo to 1.0.194
- Update rust crate thiserror to 1.0.56
- Update rust crate anyhow to 1.0.79
- Octarust update to crates v0.7
- Update rust crate clap to 4.4.13
- Update serde monorepo to 1.0.195
- Update rust crate clap to 4.4.14
- Update rust crate confy to 0.6.0
- Update rust crate clap to 4.4.16
- Update rust crate clap to 4.4.17
- Update rust crate clap to 4.4.18
- Update rust crate clap-verbosity-flag to 2.1.2
- List_all_for _authenticated or
- Extract  issues_list from html Response
- Resolve opentelemetry changes
- Update rust crate thiserror to 1.0.57
- Update rust crate tokio to 1.36.0
- Update rust crate clap to 4.5.0
- Update serde monorepo to 1.0.196
- Update rust crate clap-verbosity-flag to 2.2.0
- Update rust crate clap to 4.5.1
- Fix octorust causing 422 error selecting the prs and issues
- Fix list_all_for_authenticated_user & table data underlined
- Update rust crate anyhow to 1.0.80
- Update serde monorepo to 1.0.197
- Update rust crate log to 0.4.21
- Update rust crate bollard to 0.16.0
- Update rust crate confy to 0.6.1
- Update rust crate clap to 4.5.2
- Update rust crate opentelemetry to 0.22.0

## [0.1.6] - 2023-02-19

### Fixed

- Colour selection for issues wrong

## [0.1.5] - 2023-02-18

### Changed

- Use HashMap instead of Vec
- Use HashMap instead of Vec
- Only need issues count pulls separately
- Use confy_table for better formatting

## [0.1.4] - 2023-02-11

### Added

- Tracing with jaeger at trace level
- Don't publish (at least no yet 😄 )

### Changed

- Hide any performance related files
- Restrict console to ghdash logs
- Grooming order and don't log  token
- Make Anyhow transparent
- Spawn threads for  data collection

## [0.1.3] - 2022-12-26

### Added

- Verbosity, logging, octorust fix
- Add logging and refactor
- Logging,  private_repos

### Changed

- Clippy suggested changes
- Make pub(crate); remove getters
- Use Display trait to produce Dashboard
- Update tests

### Fixed

- Remove redundant use

## [0.1.2] - 2022-11-27

### Added

- Option to show forked repos
- Option to show all public repos

### Changed

- Release documentation

## [0.1.1] - 2022-11-23

### Added

- Add pr_count to dashboard

### Changed

- Exclude coverage reports
- Update tests to include user option

## [0.1.0] - 2022-11-23

### Changed

- Documentation and config option
- Update progress on features in Readme
- Basic clap application
- Support library
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
- Separate clap cli configuration
- Testing for cli options
- Update tests for change in cli spec

[#95]: https://github.com/jerusdp/ghdash/pull/95
[#99]: https://github.com/jerusdp/ghdash/pull/99
[#100]: https://github.com/jerusdp/ghdash/pull/100
[#101]: https://github.com/jerusdp/ghdash/pull/101
[#103]: https://github.com/jerusdp/ghdash/pull/103
[#102]: https://github.com/jerusdp/ghdash/pull/102
[#104]: https://github.com/jerusdp/ghdash/pull/104
[#105]: https://github.com/jerusdp/ghdash/pull/105
[#108]: https://github.com/jerusdp/ghdash/pull/108
[#109]: https://github.com/jerusdp/ghdash/pull/109
[#110]: https://github.com/jerusdp/ghdash/pull/110
[#111]: https://github.com/jerusdp/ghdash/pull/111
[#113]: https://github.com/jerusdp/ghdash/pull/113
[#120]: https://github.com/jerusdp/ghdash/pull/120
[#121]: https://github.com/jerusdp/ghdash/pull/121
[#112]: https://github.com/jerusdp/ghdash/pull/112
[#114]: https://github.com/jerusdp/ghdash/pull/114
[#123]: https://github.com/jerusdp/ghdash/pull/123
[#115]: https://github.com/jerusdp/ghdash/pull/115
[#122]: https://github.com/jerusdp/ghdash/pull/122
[#124]: https://github.com/jerusdp/ghdash/pull/124
[#125]: https://github.com/jerusdp/ghdash/pull/125
[#126]: https://github.com/jerusdp/ghdash/pull/126
[#127]: https://github.com/jerusdp/ghdash/pull/127
[#128]: https://github.com/jerusdp/ghdash/pull/128
[#129]: https://github.com/jerusdp/ghdash/pull/129
[Unreleased]: https://github.com/jerusdp/ghdash/compare/0.1.7...HEAD
[0.1.7]: https://github.com/jerusdp/ghdash/compare/0.1.6...0.1.7
[0.1.6]: https://github.com/jerusdp/ghdash/compare/0.1.5...0.1.6
[0.1.5]: https://github.com/jerusdp/ghdash/compare/0.1.4...0.1.5
[0.1.4]: https://github.com/jerusdp/ghdash/compare/0.1.3...0.1.4
[0.1.3]: https://github.com/jerusdp/ghdash/compare/0.1.2...0.1.3
[0.1.2]: https://github.com/jerusdp/ghdash/compare/0.1.1...0.1.2
[0.1.1]: https://github.com/jerusdp/ghdash/compare/0.1.0...0.1.1
[0.1.0]: https://github.com/jerusdp/ghdash/releases/tag/0.1.0
