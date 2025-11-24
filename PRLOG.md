# Changelog

All notable changes to this project are documented in this file.

## [Unreleased]

### Added

- Prepare for github app integration with second binary(pr [#127])
- add GitHub API integration with octocrate(pr [#128])
- add AuthSource enum for authentication source and refactor(pr [#136])
- add new_with_github_app function to create GitHub API client(pr [#138])
- add release-hook.sh script for automated changelog generation(pr [#288])

### Changed

- ci-adopt toolkit and standardise changelog (pr [#95])
- chore-add opentelemetry packages group(pr [#120])
- ci-upgrade jerusdp/circleci-toolkit orb version from 0.24.1 to 1.0.0(pr [#129])
- refactor-move config from bin to library so that it can be shared with other bins(pr [#130])
- refactor-move cli to library so that it can be shared with other bins(pr [#131])
- refactor-rename GhDashCli struct to Cli(pr [#132])
- refactor-rename builder to new and finish to generate in Dashboard struct(pr [#133])
- refactor-replace octorust library with octocrate for GitHub API calls(pr [#134])
- refactor-remove AuthSource enum and refactor Dashboard struct with pat new(pr [#137])
- chore-clean-up after adding GitHub App authorisation(pr [#141])
- ci-add bot-check context to make_release workflow(pr [#142])
- ci-add pcu-app context and install_from_github, pcu_verbosity parameters to workflows(pr [#143])
- ci-add publish parameter with false value(pr [#144])
- ci-update jerusdp/circleci-toolkit orb version from 1.0.0 to 1.1.0(pr [#145])
- ci-upgrade jerusdp/circleci-toolkit from 1.1.0 to 1.2.0(pr [#146])
- ci-enable pcu_push in workflows configuration(pr [#147])
- ci-update toolkit version from 1.2.0 to 1.2.1(pr [#148])
- ci-add update_pcu and pcu_commit_message options to workflows configuration(pr [#149])
- ci-update toolkit and fix pcu_commit_message(pr [#150])
- ci-upgrade jerusdp/circleci-toolkit orb from 1.2.2 to 1.3.0(pr [#151])
- ci-upgrade toolkit version from 1.3.0 to 1.4.0(pr [#152])
- ci-upgrade jerusdp/circleci-toolkit orb from 1.4.0 to 1.4.1(pr [#153])
- chore-remove unnecessary comments and assign @jerusdp as default code owner(pr [#154])
- ci-update circleci-toolkit version from 1.4.1 to 1.4.2(pr [#155])
- chore-update CircleCI toolkit version and enable renovate for toolkit(pr [#169])
- Renovate-on-fridays-only(pr [#197])
- 🔧 chore(renovate)-merge tracing and opentelemetry package groups(pr [#249])
- 👷 ci(circleci)-update circleci toolkit orb version(pr [#260])
- 👷 ci(circleci)-add release-flag parameter and update workflows(pr [#261])
- 👷 ci(circleci)-rename pcu parameters for clarity(pr [#262])
- 👷 ci(config)-update circleci toolkit orb version(pr [#263])
- 👷 ci(circleci)-update toolkit version in circleci config(pr [#264])
- 👷 ci(circleci)-remove unused config parameter(pr [#265])
- 👷 ci(circleci)-update toolkit orb version(pr [#266])
- 👷 ci(circleci)-update circleci-toolkit orb version(pr [#267])
- 🔧 chore(config)-update renovate schedule(pr [#268])
- 🔧 chore(config)-update renovate schedule(pr [#270])
- chore-rename CHANGELOG.md to PRLOG.md(pr [#286])
- chore-update release.toml to reference PRLOG.md instead of CHANGELOG.md(pr [#287])

### Fixed

- remove redundant octorust dependency(pr [#135])
- deps: update rust crate tokio to 1.40.0(pr [#156])
- deps: update opentelemetry packages to 0.25.0(pr [#157])
- deps: update rust crate anyhow to 1.0.87(pr [#158])
- deps: update rust crate bollard to 0.17.1(pr [#159])
- deps: update rust crate clap to 4.5.17(pr [#160])
- deps: update rust crate reqwest to 0.12.7(pr [#161])
- deps: update rust crate trycmd to 0.15.7(pr [#162])
- deps: update serde packages to 1.0.210(pr [#163])
- deps: update rust crate anyhow to 1.0.88(pr [#165])
- deps: update rust crate anyhow to 1.0.89(pr [#166])
- deps: update rust crate clap to 4.5.18(pr [#167])
- deps: update rust crate thiserror to 1.0.64(pr [#168])
- deps: update rust crate clap-verbosity-flag to 2.2.2(pr [#170])
- deps: update rust crate reqwest to 0.12.8(pr [#171])
- deps: update rust crate clap to 4.5.19(pr [#172])
- deps: update rust crate clap to 4.5.20(pr [#174])
- deps: update opentelemetry packages(pr [#175])
- deps: update rust crate anyhow to 1.0.90(pr [#176])
- deps: update serde packages to 1.0.211(pr [#177])
- deps: update rust crate tokio to 1.41.0(pr [#178])
- deps: update rust crate anyhow to 1.0.91(pr [#179])
- deps: update serde packages to 1.0.213(pr [#181])
- deps: update rust crate thiserror to 1.0.65(pr [#180])
- deps: update rust crate reqwest to 0.12.9(pr [#182])
- deps: update serde packages to 1.0.214(pr [#183])
- deps: update rust crate trycmd to 0.15.8(pr [#184])
- deps: update rust crate thiserror to 1.0.66(pr [#185])
- deps: update rust crate anyhow to 1.0.92(pr [#186])
- deps: update rust crate thiserror to 1.0.67(pr [#187])
- deps: update rust crate thiserror to 1.0.68(pr [#188])
- deps: update rust crate anyhow to 1.0.93(pr [#189])
- deps: update rust crate tokio to 1.41.1(pr [#191])
- deps: update rust crate thiserror to v2(pr [#190])
- deps: update rust crate thiserror to 2.0.1(pr [#192])
- deps: update rust crate thiserror to 2.0.2(pr [#193])
- deps: update rust crate thiserror to 2.0.3(pr [#194])
- deps: update serde packages to 1.0.215(pr [#195])
- deps: update rust crate clap to 4.5.21(pr [#198])
- deps: update rust crate comfy-table to 7.1.3(pr [#199])
- deps: update opentelemetry packages(pr [#200])
- deps: update rust crate bollard to 0.18.0(pr [#201])
- deps: update rust crate bollard to 0.18.1(pr [#202])
- deps: update rust crate clap-verbosity-flag to 2.2.3(pr [#203])
- deps: update rust crate clap-verbosity-flag to v3(pr [#204])
- deps: update opentelemetry packages to 0.27.1(pr [#206])
- deps: update rust crate clap-verbosity-flag to 3.0.1(pr [#207])
- deps: update rust crate tracing to 0.1.41(pr [#208])
- deps: update rust crate anyhow to 1.0.94(pr [#209])
- deps: update rust crate clap to 4.5.23(pr [#210])
- deps: update rust crate thiserror to 2.0.4(pr [#211])
- deps: update rust crate tracing-subscriber to 0.3.19(pr [#212])
- deps: update rust crate thiserror to 2.0.6(pr [#213])
- deps: update serde packages to 1.0.216(pr [#214])
- deps: update rust crate tokio to 1.42.0(pr [#215])
- deps: update rust crate clap-verbosity-flag to 3.0.2(pr [#216])
- deps: update rust crate thiserror to 2.0.8(pr [#217])
- deps: update rust crate anyhow to 1.0.95(pr [#218])
- deps: update rust crate reqwest to 0.12.10(pr [#219])
- deps: update rust crate thiserror to 2.0.9(pr [#220])
- deps: update rust crate reqwest to 0.12.12(pr [#221])
- deps: update serde packages to 1.0.217(pr [#222])
- deps: update rust crate clap to 4.5.26(pr [#223])
- deps: update rust crate thiserror to 2.0.10(pr [#224])
- deps: update rust crate tokio to 1.43.0(pr [#225])
- deps: update rust crate log to 0.4.25(pr [#226])
- deps: update rust crate thiserror to 2.0.11(pr [#227])
- deps: update rust crate clap to 4.5.27(pr [#228])
- config: migrate renovate config(pr [#229])
- deps: update rust crate clap to 4.5.28(pr [#231])
- deps: update rust crate clap to 4.5.29(pr [#232])
- deps: update rust crate comfy-table to 7.1.4(pr [#233])
- deps: update rust crate anyhow to 1.0.96(pr [#234])
- deps: update rust crate clap to 4.5.30(pr [#235])
- deps: update rust crate clap to 4.5.31(pr [#236])
- deps: update rust crate log to 0.4.26(pr [#237])
- deps: update rust crate anyhow to 1.0.97(pr [#238])
- deps: update rust crate thiserror to 2.0.12(pr [#239])
- deps: update rust crate trycmd to 0.15.9(pr [#240])
- deps: update serde packages to 1.0.218(pr [#241])
- deps: update rust crate clap to 4.5.32(pr [#243])
- deps: update rust crate reqwest to 0.12.14(pr [#244])
- deps: update rust crate reqwest to 0.12.15(pr [#245])
- deps: update serde packages to 1.0.219(pr [#246])
- deps: update rust crate octocrate to 2.2.0(pr [#248])
- deps: update rust crate clap to 4.5.34(pr [#250])
- deps: update rust crate log to 0.4.27(pr [#251])
- deps: update rust crate tokio to 1.44.1(pr [#252])
- deps: update rust crate clap to 4.5.35(pr [#254])
- deps: update rust crate tokio to v1.44.2 [security](pr [#255])
- deps: update rust crate tokio to 1.44.2(pr [#258])
- deps: update tracing and opentelemetry packages(pr [#253])
- deps: update rust crate anyhow to 1.0.98(pr [#256])
- deps: update rust crate clap to 4.5.36(pr [#257])
- deps: update rust crate clap to 4.5.37(pr [#259])
- deps: update rust crate clap to 4.5.38(pr [#271])
- deps: update rust crate clap-verbosity-flag to 3.0.3(pr [#272])
- deps: update rust crate clap to 4.5.40(pr [#273])
- deps: update rust crate futures-util to 0.3.31(pr [#274])
- deps: update rust crate reqwest to 0.12.20(pr [#275])
- deps: update rust crate bollard to 0.19.1(pr [#276])
- deps: update rust crate clap to 4.5.41(pr [#277])
- deps: update rust crate reqwest to 0.12.22(pr [#278])
- deps: update rust crate trycmd to 0.15.10(pr [#279])
- deps: update rust crate tokio to 1.46.1(pr [#280])
- deps: update rust crate clap-verbosity-flag to 3.0.4(pr [#284])
- deps: update rust crate anyhow to 1.0.99(pr [#281])
- deps: update rust crate bollard to 0.19.2(pr [#282])
- deps: update rust crate clap to 4.5.45(pr [#283])
- deps: update rust crate tracing-subscriber to v0.3.20 [security](pr [#285])
- deps: update rust crate anyhow to 1.0.100(pr [#289])
- deps: update rust crate clap to 4.5.48(pr [#290])
- deps: update rust crate log to 0.4.28(pr [#291])
- deps: update rust crate reqwest to 0.12.23(pr [#292])
- deps: update rust crate thiserror to 2.0.16(pr [#293])
- deps: update rust crate tokio to 1.47.1(pr [#294])
- deps: update rust crate bollard to 0.19.3(pr [#295])
- deps: update rust crate clap to 4.5.50(pr [#296])
- deps: update rust crate bollard to 0.19.4(pr [#297])
- deps: update rust crate clap to 4.5.53(pr [#298])

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
- Dependencies: update serde packages to 1.0.207(pr [#140])
- Dependencies: update rust crate clap to 4.5.15(pr [#139])
- Dependencies: bump rustls from 0.23.16 to 0.23.18 in the cargo group across 1 directory(pr [#205])
- Dependencies: bump openssl from 0.10.68 to 0.10.70 in the cargo group across 1 directory(pr [#230])

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
[#130]: https://github.com/jerusdp/ghdash/pull/130
[#131]: https://github.com/jerusdp/ghdash/pull/131
[#132]: https://github.com/jerusdp/ghdash/pull/132
[#133]: https://github.com/jerusdp/ghdash/pull/133
[#134]: https://github.com/jerusdp/ghdash/pull/134
[#135]: https://github.com/jerusdp/ghdash/pull/135
[#136]: https://github.com/jerusdp/ghdash/pull/136
[#137]: https://github.com/jerusdp/ghdash/pull/137
[#138]: https://github.com/jerusdp/ghdash/pull/138
[#140]: https://github.com/jerusdp/ghdash/pull/140
[#139]: https://github.com/jerusdp/ghdash/pull/139
[#141]: https://github.com/jerusdp/ghdash/pull/141
[#142]: https://github.com/jerusdp/ghdash/pull/142
[#143]: https://github.com/jerusdp/ghdash/pull/143
[#144]: https://github.com/jerusdp/ghdash/pull/144
[#145]: https://github.com/jerusdp/ghdash/pull/145
[#146]: https://github.com/jerusdp/ghdash/pull/146
[#147]: https://github.com/jerusdp/ghdash/pull/147
[#148]: https://github.com/jerusdp/ghdash/pull/148
[#149]: https://github.com/jerusdp/ghdash/pull/149
[#150]: https://github.com/jerusdp/ghdash/pull/150
[#151]: https://github.com/jerusdp/ghdash/pull/151
[#152]: https://github.com/jerusdp/ghdash/pull/152
[#153]: https://github.com/jerusdp/ghdash/pull/153
[#154]: https://github.com/jerusdp/ghdash/pull/154
[#155]: https://github.com/jerusdp/ghdash/pull/155
[#156]: https://github.com/jerusdp/ghdash/pull/156
[#157]: https://github.com/jerusdp/ghdash/pull/157
[#158]: https://github.com/jerusdp/ghdash/pull/158
[#159]: https://github.com/jerusdp/ghdash/pull/159
[#160]: https://github.com/jerusdp/ghdash/pull/160
[#161]: https://github.com/jerusdp/ghdash/pull/161
[#162]: https://github.com/jerusdp/ghdash/pull/162
[#163]: https://github.com/jerusdp/ghdash/pull/163
[#165]: https://github.com/jerusdp/ghdash/pull/165
[#166]: https://github.com/jerusdp/ghdash/pull/166
[#167]: https://github.com/jerusdp/ghdash/pull/167
[#168]: https://github.com/jerusdp/ghdash/pull/168
[#169]: https://github.com/jerusdp/ghdash/pull/169
[#170]: https://github.com/jerusdp/ghdash/pull/170
[#171]: https://github.com/jerusdp/ghdash/pull/171
[#172]: https://github.com/jerusdp/ghdash/pull/172
[#174]: https://github.com/jerusdp/ghdash/pull/174
[#175]: https://github.com/jerusdp/ghdash/pull/175
[#176]: https://github.com/jerusdp/ghdash/pull/176
[#177]: https://github.com/jerusdp/ghdash/pull/177
[#178]: https://github.com/jerusdp/ghdash/pull/178
[#179]: https://github.com/jerusdp/ghdash/pull/179
[#181]: https://github.com/jerusdp/ghdash/pull/181
[#180]: https://github.com/jerusdp/ghdash/pull/180
[#182]: https://github.com/jerusdp/ghdash/pull/182
[#183]: https://github.com/jerusdp/ghdash/pull/183
[#184]: https://github.com/jerusdp/ghdash/pull/184
[#185]: https://github.com/jerusdp/ghdash/pull/185
[#186]: https://github.com/jerusdp/ghdash/pull/186
[#187]: https://github.com/jerusdp/ghdash/pull/187
[#188]: https://github.com/jerusdp/ghdash/pull/188
[#189]: https://github.com/jerusdp/ghdash/pull/189
[#191]: https://github.com/jerusdp/ghdash/pull/191
[#190]: https://github.com/jerusdp/ghdash/pull/190
[#192]: https://github.com/jerusdp/ghdash/pull/192
[#193]: https://github.com/jerusdp/ghdash/pull/193
[#194]: https://github.com/jerusdp/ghdash/pull/194
[#195]: https://github.com/jerusdp/ghdash/pull/195
[#197]: https://github.com/jerusdp/ghdash/pull/197
[#198]: https://github.com/jerusdp/ghdash/pull/198
[#199]: https://github.com/jerusdp/ghdash/pull/199
[#200]: https://github.com/jerusdp/ghdash/pull/200
[#201]: https://github.com/jerusdp/ghdash/pull/201
[#202]: https://github.com/jerusdp/ghdash/pull/202
[#203]: https://github.com/jerusdp/ghdash/pull/203
[#204]: https://github.com/jerusdp/ghdash/pull/204
[#205]: https://github.com/jerusdp/ghdash/pull/205
[#206]: https://github.com/jerusdp/ghdash/pull/206
[#207]: https://github.com/jerusdp/ghdash/pull/207
[#208]: https://github.com/jerusdp/ghdash/pull/208
[#209]: https://github.com/jerusdp/ghdash/pull/209
[#210]: https://github.com/jerusdp/ghdash/pull/210
[#211]: https://github.com/jerusdp/ghdash/pull/211
[#212]: https://github.com/jerusdp/ghdash/pull/212
[#213]: https://github.com/jerusdp/ghdash/pull/213
[#214]: https://github.com/jerusdp/ghdash/pull/214
[#215]: https://github.com/jerusdp/ghdash/pull/215
[#216]: https://github.com/jerusdp/ghdash/pull/216
[#217]: https://github.com/jerusdp/ghdash/pull/217
[#218]: https://github.com/jerusdp/ghdash/pull/218
[#219]: https://github.com/jerusdp/ghdash/pull/219
[#220]: https://github.com/jerusdp/ghdash/pull/220
[#221]: https://github.com/jerusdp/ghdash/pull/221
[#222]: https://github.com/jerusdp/ghdash/pull/222
[#223]: https://github.com/jerusdp/ghdash/pull/223
[#224]: https://github.com/jerusdp/ghdash/pull/224
[#225]: https://github.com/jerusdp/ghdash/pull/225
[#226]: https://github.com/jerusdp/ghdash/pull/226
[#227]: https://github.com/jerusdp/ghdash/pull/227
[#228]: https://github.com/jerusdp/ghdash/pull/228
[#229]: https://github.com/jerusdp/ghdash/pull/229
[#230]: https://github.com/jerusdp/ghdash/pull/230
[#231]: https://github.com/jerusdp/ghdash/pull/231
[#232]: https://github.com/jerusdp/ghdash/pull/232
[#233]: https://github.com/jerusdp/ghdash/pull/233
[#234]: https://github.com/jerusdp/ghdash/pull/234
[#235]: https://github.com/jerusdp/ghdash/pull/235
[#236]: https://github.com/jerusdp/ghdash/pull/236
[#237]: https://github.com/jerusdp/ghdash/pull/237
[#238]: https://github.com/jerusdp/ghdash/pull/238
[#239]: https://github.com/jerusdp/ghdash/pull/239
[#240]: https://github.com/jerusdp/ghdash/pull/240
[#241]: https://github.com/jerusdp/ghdash/pull/241
[#243]: https://github.com/jerusdp/ghdash/pull/243
[#244]: https://github.com/jerusdp/ghdash/pull/244
[#245]: https://github.com/jerusdp/ghdash/pull/245
[#246]: https://github.com/jerusdp/ghdash/pull/246
[#248]: https://github.com/jerusdp/ghdash/pull/248
[#249]: https://github.com/jerusdp/ghdash/pull/249
[#250]: https://github.com/jerusdp/ghdash/pull/250
[#251]: https://github.com/jerusdp/ghdash/pull/251
[#252]: https://github.com/jerusdp/ghdash/pull/252
[#254]: https://github.com/jerusdp/ghdash/pull/254
[#255]: https://github.com/jerusdp/ghdash/pull/255
[#258]: https://github.com/jerusdp/ghdash/pull/258
[#253]: https://github.com/jerusdp/ghdash/pull/253
[#256]: https://github.com/jerusdp/ghdash/pull/256
[#257]: https://github.com/jerusdp/ghdash/pull/257
[#259]: https://github.com/jerusdp/ghdash/pull/259
[#260]: https://github.com/jerusdp/ghdash/pull/260
[#261]: https://github.com/jerusdp/ghdash/pull/261
[#262]: https://github.com/jerusdp/ghdash/pull/262
[#263]: https://github.com/jerusdp/ghdash/pull/263
[#264]: https://github.com/jerusdp/ghdash/pull/264
[#265]: https://github.com/jerusdp/ghdash/pull/265
[#266]: https://github.com/jerusdp/ghdash/pull/266
[#267]: https://github.com/jerusdp/ghdash/pull/267
[#268]: https://github.com/jerusdp/ghdash/pull/268
[#270]: https://github.com/jerusdp/ghdash/pull/270
[#271]: https://github.com/jerusdp/ghdash/pull/271
[#272]: https://github.com/jerusdp/ghdash/pull/272
[#273]: https://github.com/jerusdp/ghdash/pull/273
[#274]: https://github.com/jerusdp/ghdash/pull/274
[#275]: https://github.com/jerusdp/ghdash/pull/275
[#276]: https://github.com/jerusdp/ghdash/pull/276
[#277]: https://github.com/jerusdp/ghdash/pull/277
[#278]: https://github.com/jerusdp/ghdash/pull/278
[#279]: https://github.com/jerusdp/ghdash/pull/279
[#280]: https://github.com/jerusdp/ghdash/pull/280
[#284]: https://github.com/jerusdp/ghdash/pull/284
[#281]: https://github.com/jerusdp/ghdash/pull/281
[#282]: https://github.com/jerusdp/ghdash/pull/282
[#283]: https://github.com/jerusdp/ghdash/pull/283
[#285]: https://github.com/jerusdp/ghdash/pull/285
[#286]: https://github.com/jerusdp/ghdash/pull/286
[#287]: https://github.com/jerusdp/ghdash/pull/287
[#288]: https://github.com/jerusdp/ghdash/pull/288
[#289]: https://github.com/jerusdp/ghdash/pull/289
[#290]: https://github.com/jerusdp/ghdash/pull/290
[#291]: https://github.com/jerusdp/ghdash/pull/291
[#292]: https://github.com/jerusdp/ghdash/pull/292
[#293]: https://github.com/jerusdp/ghdash/pull/293
[#294]: https://github.com/jerusdp/ghdash/pull/294
[#295]: https://github.com/jerusdp/ghdash/pull/295
[#296]: https://github.com/jerusdp/ghdash/pull/296
[#297]: https://github.com/jerusdp/ghdash/pull/297
[#298]: https://github.com/jerusdp/ghdash/pull/298
[Unreleased]: https://github.com/jerusdp/ghdash/compare/v0.1.7...HEAD
[0.1.7]: https://github.com/jerusdp/ghdash/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/jerusdp/ghdash/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/jerusdp/ghdash/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/jerusdp/ghdash/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/jerusdp/ghdash/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/jerusdp/ghdash/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/jerusdp/ghdash/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/jerusdp/ghdash/releases/tag/v0.1.0
