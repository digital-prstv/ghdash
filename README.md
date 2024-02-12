# ghdash

## Objective

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
