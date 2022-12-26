//! Dashboard module represents that data presented in the dashboard
//!

use std::ops::Deref;

use crate::error::Error;
use ansi_term::{Colour, Style};
use clap::ValueEnum;
use octorust::types::{Order, PullsListSort, ReposListOrgSort, ReposListType, Repository};
use octorust::{auth::Credentials, Client};
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use tracing::{info, instrument};

/// Options for the scope of the repositories listed in the dashboard
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
pub enum RepoScope {
    /// Public repositories that are original written by the user
    #[default]
    Authored,
    /// Public repositories that are forked from other authors
    Forked,
    /// All public repositories
    Public,
    /// All private repositories
    Private,
}

#[derive(Debug, Clone, Default)]
struct Repo {
    name: String,
    fork: bool,
    archived: bool,
    pr_count: usize,
}

/// Struct Representing a Dashboard and key data required to create the dashboard
///
/// ## Fields
/// - user - the github user for which the dashboard is created
/// - token - a personal access token for the user that provides access to the github API
/// - repositories - a list of the user's repositories presented in the dashboard
///
#[derive(Debug, Clone, Default)]
pub struct Dashboard {
    user: String,
    token: String,
    repositories: Vec<Repo>,
    repo_scope: RepoScope,
    archived: bool,
}

impl Dashboard {
    /// Create a dashboard by setting the user and token strings to access Github
    ///
    /// Without a user and token to get data from Github the dashboard is meaningless
    /// therefore a new struct without this data is not meaningful
    ///
    #[instrument]
    pub fn builder(user: &str, token: &str) -> Result<Self, Error> {
        if user.is_empty() {
            return Err(Error::MustHaveUser);
        }

        if token.is_empty() {
            return Err(Error::MustHaveToken);
        }

        info!("Starting to build the dashboard.");

        Ok(Dashboard {
            user: user.to_string(),
            token: token.to_string(),
            ..Default::default()
        })
    }

    /// Set the repo_scope for the Dashboard
    ///
    #[instrument]
    pub fn set_repo_scope(&mut self, repo_scope: RepoScope) -> &mut Self {
        info!("set the scope to {:#?}", &repo_scope);
        self.repo_scope = repo_scope;
        self
    }

    /// Gather the data for the report from Github
    ///
    #[instrument]
    pub async fn finish(&mut self) -> Result<Self, Error> {
        info!("Finishing the dashboard configuration build.");
        let list_type = match self.repo_scope {
            RepoScope::Authored => ReposListType::Public,
            RepoScope::Forked => ReposListType::Public,
            RepoScope::Private => ReposListType::Private,
            RepoScope::Public => ReposListType::Public,
        };

        let github = Client::new(
            String::from(&self.user),
            Credentials::Token(String::from(&self.token)),
        )?;

        let repos = github.repos();
        let pulls = github.pulls();

        info!("Access secured to github repositories and pull requests.\nGetting the base list of repositories.");
        let repos_list = repos
            .list_all_for_authenticated_user(
                None,
                "",
                Some(list_type),
                ReposListOrgSort::FullName,
                Order::Asc,
                None,
                None,
            )
            .await?;

        let mut repositories: Vec<Repo> = vec![];

        info!("Building list of repositories ({:#?}).", &repositories);

        for repo in repos_list {
            let repo_name = repo.name.as_str();
            let fork = repo.fork;
            let archived = repo.archived;
            let mut pr_count = 0;
            if owned_by(&repo, &self.user) {
                pr_count = pulls
                    .list_all(
                        &self.user,
                        repo_name,
                        octorust::types::IssuesListState::Open,
                        "",
                        "",
                        PullsListSort::Created,
                        Order::Asc,
                    )
                    .await?
                    .len();
            }
            repositories.push(Repo {
                name: String::from(repo_name),
                fork,
                archived,
                pr_count,
            });
        }

        info!(
            "Check if archived should be retained or removed ({:#?}).",
            self.archived
        );
        if !self.archived {
            repositories.retain(|repo| !repo.archived);
        }

        match self.repo_scope {
            RepoScope::Authored => {
                info!("Remove forked repositories.");
                repositories.retain(|repo| !repo.fork);
            }
            RepoScope::Forked => {
                info!("Retain only forked repositories.");
                repositories.retain(|repo| repo.fork);
            }
            _ => {}
        }

        self.repositories = repositories;

        Ok(self.to_owned())
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

fn owned_by(repo: &Repository, user: &str) -> bool {
    if let Some(owner) = repo.owner.clone() {
        owner.login == user
    } else {
        false
    }
}
