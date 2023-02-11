//! Dashboard module represents that data presented in the dashboard
//!

use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

use crate::error::Error;
use ansi_term::{Colour, Style};
use clap::ValueEnum;
use octorust::pulls::Pulls;
use octorust::types::{Order, PullsListSort, ReposListOrgSort, ReposListType, Repository};
use octorust::{auth::Credentials, Client};
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use tokio::task::JoinHandle;
use tracing::{debug, info, instrument, warn};

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
    #[instrument(skip(token))]
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
    #[instrument(skip(self), fields(self.user = %self.user, self.repo_scope = ?self.repo_scope))]
    pub fn set_repo_scope(&mut self, repo_scope: RepoScope) -> &mut Self {
        info!("set the scope to {:#?}", &repo_scope);
        self.repo_scope = repo_scope;
        self
    }

    /// Gather the data for the report from Github
    ///
    #[instrument(name = "Extract_dashboard_data", skip(self), fields(self.user = %self.user, self.repo_scope = ?self.repo_scope))]
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
        let pulls = Arc::new(github.pulls());

        info!("Access secured to github repositories and pull requests.\nGetting the base list of repositories.");
        let mut repos_list = repos
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

        info!("Remove un-owned repositories.");
        repos_list.retain(|repo| owned_by(repo, &self.user));

        info!(
            "Check if archived should be retained or removed ({:#?}).",
            self.archived
        );

        if !self.archived {
            info!("Removing archived repos from the list");
            repos_list.retain(|repo| !repo.archived);
        }

        match self.repo_scope {
            RepoScope::Authored => {
                info!("Remove forked repositories.");
                repos_list.retain(|repo| !repo.fork);
            }
            RepoScope::Forked => {
                info!("Retain only forked repositories.");
                repos_list.retain(|repo| repo.fork);
            }
            _ => {}
        }

        let mut repositories = vec![]; // : Vec<Repo>
        let mut tasks = vec![];

        info!("Building list of repositories ({:#?}).", &repositories);

        for repo in repos_list {
            let t_repo = repo.name.clone();
            let t_pulls = pulls.clone();
            let t_user = self.user.clone();
            info!("Counting pull requests for {:?}", &repo.name);
            let res: JoinHandle<RepoResult> = tokio::spawn(async move {
                let pr_count_res =
                    pull_request_count(&t_pulls, t_user.as_ref(), t_repo.as_ref()).await;
                RepoResult {
                    name: t_repo,
                    pr_count_res,
                }
            });
            tasks.push(res);
        }

        for task in tasks {
            match task.await {
                Ok(repo_res) => {
                    let name = repo_res.name;
                    match repo_res.pr_count_res {
                        Ok(pr_count) => repositories.push(Repo { name, pr_count }),
                        Err(e) => warn!(
                            "Error returned while fetching pull data for {:#?}: {:#?}",
                            name, e
                        ),
                    };
                }
                Err(e) => warn!("Join Error on task: {e}"),
            }
        }
        self.repositories = repositories;

        Ok(self.to_owned())
    }
}

struct RepoResult {
    name: String,
    pr_count_res: Result<usize, Error>,
}

impl fmt::Display for Dashboard {
    /// Build a table from the dashboard configuration and data
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

        write!(f, "{}", grid.fit_into_columns(2))
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

#[instrument(skip(repo) fields(repo.name))]
fn owned_by(repo: &Repository, user: &str) -> bool {
    if let Some(owner) = repo.owner.clone() {
        debug!(
            "checking {} owned by {} for user {}",
            &repo.name, &owner.login, &user
        );
        owner.login == user
    } else {
        false
    }
}

#[instrument(skip(pulls))]
async fn pull_request_count(pulls: &Pulls, user: &str, repo: &str) -> Result<usize, Error> {
    let all_pulls = pulls
        .list_all(
            user,
            repo,
            octorust::types::IssuesListState::Open,
            "",
            "",
            PullsListSort::Created,
            Order::Asc,
        )
        .await;

    debug!("Success of request for all pulls: {:?}", &all_pulls.is_ok());

    match all_pulls {
        Ok(v) => Ok(v.len()),
        Err(e) => {
            debug!(
                "Error returned seeking list of all open pull requests:{:?}",
                &e
            );
            Err(e.into())
        }
    }
}
