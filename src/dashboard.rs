//! Dashboard module represents that data presented in the dashboard
//!

use std::ops::Deref;

use crate::error::Error;
use ansi_term::{Colour, Style};
use clap::ValueEnum;
use octorust::types::{Order, PullsListSort, ReposListOrgSort, ReposListUserType};
use octorust::{auth::Credentials, Client};
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};

/// Options for the scope of the repositories listed in the dashboard
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum RepoScope {
    /// Public repositories that are original written by the user
    Authored,
    /// Public repositories that are forked from other authors
    Forked,
    /// All public repositories
    Public,
    /// All private repositories
    Private,
}

#[derive(Debug, Default)]
struct Repo {
    name: String,
    pr_count: usize,
}

impl Repo {
    fn new(name: Option<String>, pr_count: Option<usize>) -> Repo {
        match (name, pr_count) {
            (None, None) => Repo::default(),
            (Some(name), None) => Repo { name, pr_count: 0 },
            (None, Some(pr_count)) => Repo {
                name: String::from(""),
                pr_count,
            },
            (Some(name), Some(pr_count)) => Repo { name, pr_count },
        }
    }
}
/// Struct Representing a Dashboard and key data required to create the dashboard
///
/// ## Fields
/// - user - the github user for which the dashboard is created
/// - token - a personal access token for the user that provides access to the github API
/// - repositories - a list of the user's repositories presented in the dashboard
///
#[derive(Debug)]
pub struct Dashboard {
    user: String,
    token: String,
    repositories: Vec<Repo>,
    repo_scope: RepoScope,
}

impl Dashboard {
    /// Create a dashboard by setting the user and token strings to access Github
    /// Without a user and token to get data from Github the dashboard is meaningless
    /// therefore a new struct without this data is not meaningful
    ///
    pub async fn new(user: &str, token: &str, repo_scope: RepoScope) -> Result<Dashboard, Error> {
        if user.is_empty() {
            return Err(Error::MustHaveUser);
        }

        if token.is_empty() {
            return Err(Error::MustHaveToken);
        }

        let github = Client::new(String::from(user), Credentials::Token(String::from(token)))?;

        let repos = github.repos();
        let pulls = github.pulls();

        let repos_list = repos
            .list_all_for_user(
                user,
                ReposListUserType::Owner,
                ReposListOrgSort::FullName,
                Order::Asc,
            )
            .await?;

        let mut repositories: Vec<Repo> = vec![];

        for repo in repos_list {
            match repo_scope {
                RepoScope::Authored => {
                    if !repo.fork {
                        let repo_name = repo.name.as_str();
                        let pr_count = pulls
                            .list_all(
                                user,
                                repo_name,
                                octorust::types::IssuesListState::Open,
                                "",
                                "",
                                PullsListSort::Created,
                                Order::Asc,
                            )
                            .await?
                            .len();
                        repositories.push(Repo::new(Some(String::from(repo_name)), Some(pr_count)));
                    }
                }
                RepoScope::Forked => {
                    if repo.fork {
                        let repo_name = repo.name.as_str();
                        let pr_count = pulls
                            .list_all(
                                user,
                                repo_name,
                                octorust::types::IssuesListState::Open,
                                "",
                                "",
                                PullsListSort::Created,
                                Order::Asc,
                            )
                            .await?
                            .len();
                        repositories.push(Repo::new(Some(String::from(repo_name)), Some(pr_count)));
                    }
                }
                RepoScope::Public => {
                    let repo_name = repo.name.as_str();
                    let pr_count = pulls
                        .list_all(
                            user,
                            repo_name,
                            octorust::types::IssuesListState::Open,
                            "",
                            "",
                            PullsListSort::Created,
                            Order::Asc,
                        )
                        .await?
                        .len();
                    repositories.push(Repo::new(Some(String::from(repo_name)), Some(pr_count)));
                }
                RepoScope::Private => {
                    unreachable!()
                }
            }
        }

        Ok(Dashboard {
            user: user.to_string(),
            token: token.to_string(),
            repositories,
            repo_scope,
        })
    }

    /// Build a table from the dashboard configuration and data
    ///
    pub fn build_dashboard(&self) -> String {
        let mut grid = Grid::new(GridOptions {
            filling: Filling::Spaces(1),
            direction: Direction::LeftToRight,
        });

        // Add the headings
        grid.add(Cell::from(heading("Repository        ")));
        grid.add(Cell::from(heading("PR Count")));

        for repo in self.repositories.deref() {
            grid.add(Cell::from(repo.name.clone()));
            let count = repo.pr_count;
            if 0 < count {
                grid.add(Cell::from(bold_yellow(count)));
            } else {
                grid.add(Cell::from(repo.pr_count.to_string()));
            }
        }

        format!("{}", grid.fit_into_columns(2))
    }

    /// Get the user name for the dashboard
    ///
    pub fn user(&self) -> &str {
        &self.user
    }

    /// Set the user name for the Dashboard
    ///
    pub fn set_user(&mut self, user: &str) -> &mut Self {
        self.user = user.to_string();
        self
    }

    /// Get the token for the Dashboard
    ///
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Set the token for the Dashboard
    ///
    pub fn set_token(&mut self, token: &str) -> &mut Self {
        self.token = token.to_string();
        self
    }

    /// Get the repo_scope for the Dashboard
    ///
    pub fn repo_scope(&self) -> RepoScope {
        self.repo_scope
    }

    /// Set the repo_scope for the Dashboard
    ///
    pub fn set_repo_scope(&mut self, repo_scope: RepoScope) -> &mut Self {
        self.repo_scope = repo_scope;
        self
    }

    // /// Add a repo to the Dashboard
    // ///
    // pub fn add_repo(&mut self, repo: &str) -> &mut Self {
    //     self.repositories.push(repo.to_string());
    //     self
    // }
}

fn heading(heading: &str) -> String {
    format!("{}", Style::new().bold().paint(heading))
}

fn bold_yellow<T: ToString>(text: T) -> String {
    format!(
        "{}",
        Style::new()
            .bold()
            .fg(Colour::Fixed(220))
            .paint(text.to_string())
    )
}
