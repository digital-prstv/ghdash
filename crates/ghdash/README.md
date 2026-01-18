# ghdash

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][circleci-badge]][circleci-url]
[![Rust 1.81+][version-badge]][version-url]
[![Docs][docs-badge]][docs-url]
[![BuyMeaCoffee][bmac-badge]][bmac-url]
[![GitHubSponsors][ghub-badge]][ghub-url]

[crates-badge]: https://img.shields.io/crates/v/ghdash.svg
[crates-url]: https://crates.io/crates/ghdash
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/jerusdp/ghdash/blob/main/LICENSE
[circleci-badge]:https://circleci.com/gh/jerusdp/ghdash/tree/main.svg?style=svg
[circleci-url]: https://circleci.com/gh/jerusdp/ghdash/tree/?branch=main
[version-badge]: https://img.shields.io/badge/rust-1.81+-orange.svg
[version-url]: https://www.rust-lang.org
[docs-badge]:  https://docs.rs/ghdash/badge.svg
[docs-url]:  https://docs.rs/ghdash
[bmac-badge]: https://badgen.net/badge/icon/buymeacoffee?color=yellow&icon=buymeacoffee&label
[bmac-url]: https://buymeacoffee.com/jerusdp
[ghub-badge]: https://img.shields.io/badge/sponsor-30363D?logo=GitHub-Sponsors&logoColor=#white
[ghub-url]: https://github.com/sponsors/jerusdp

Provide a command line application to display a dashboard of statistics about a user's repositories.

## Features

- [X] Get user and authentication
- [X] Get list of user's public repos (repos::list_all_for_user)
- [X] Pull Request Dashboard
  - [X] For each repo in the repos list
  - [X] List pull requests (pulls::list)
  - [X] Display dashboard with repository name, pull request count
  - [X] Show forked repos only
  - [X] Show public repos
  - [X] Show private repos only
  - [ ] Show all repos including archived

## CLI Usage

Install the CLI using cargo install.

```sh

cargo install ghdash

```

Run in your project directory and check the version

```console
$ ghdash --version
ghdash 0.1.6

```

Help provides all the options

```sh

$ ghdash -h
Dashboard for Github account

Usage: ghdash [OPTIONS]

Options:
  -v, --verbose...                   More output per occurrence
  -q, --quiet...                     Less output per occurrence
  -c, --config <CONFIG>              Force the calculation of the version number alternate configuration file
  -u, --user <USER>                  github user name
  -t, --token <TOKEN>                github personal access token
  -r, --repositories <REPOSITORIES>  scope for the repositories shown in the dashboard [possible values: authored, forked, public, private]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version

```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
