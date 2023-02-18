//! Dashboard module represents that data presented in the dashboard
//!

use std::fmt::{self};
use std::sync::Arc;

use crate::error::Error;
use clap::ValueEnum;
use comfy_table::presets::NOTHING;
use comfy_table::{Attribute, Cell, CellAlignment, Color, Table, TableComponent};
use octorust::issues::Issues;
use octorust::types::{
    IssuesListSort, IssuesListState, Order, ReposListOrgSort, ReposListType, Repository,
};
use octorust::{auth::Credentials, Client};
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
    issue_count: usize,
}

struct RepoResult {
    name: String,
    count_res: Result<(usize, usize), Error>,
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
        let issues = Arc::new(github.issues());

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

        let mut repositories = vec![]; // : Vec<Repo> // HashMap::new();
        let mut tasks = vec![];

        info!("Building list of repositories ({:#?}).", &repositories);

        for repo in repos_list {
            let t_issues = issues.clone();
            let t_repo = repo.name.clone();
            let t_user = self.user.clone();

            info!("Counting issue requests for {:?}", &repo.name);
            let res: JoinHandle<RepoResult> = tokio::spawn(async move {
                let count_res = count_issues(&t_issues, t_user.as_ref(), t_repo.as_ref()).await;
                RepoResult {
                    name: t_repo,
                    count_res,
                }
            });
            tasks.push(res);
        }

        for task in tasks {
            match task.await {
                Ok(repo_res) => {
                    let name = repo_res.name;
                    match repo_res.count_res {
                        Ok(counts) => {
                            info!("The counts found are: {:?}", &counts);
                            repositories.push(Repo {
                                name,
                                pr_count: counts.0,
                                issue_count: counts.1,
                            });
                        }
                        Err(e) => warn!(
                            "Error returned while fetching pull data for {:#?}: {:#?}",
                            name, e
                        ),
                        // },
                    }
                }
                Err(e) => warn!("Join Error on task: {e}"),
            }
        }
        self.repositories = repositories;

        Ok(self.to_owned())
    }
}

impl fmt::Display for Dashboard {
    /// Build a table from the dashboard configuration and data
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        table
            .load_preset(NOTHING)
            .set_style(TableComponent::HeaderLines, '═')
            .set_style(TableComponent::MiddleHeaderIntersections, '═')
            .set_style(TableComponent::BottomBorder, '─')
            .set_style(TableComponent::BottomBorderIntersections, '─')
            .set_header(vec![
                Cell::new("Repository")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("PRs")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
                Cell::new("Issues")
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center),
            ]);

        for repo in self.repositories.clone().into_iter() {
            let repo_name = Cell::new(repo.name);
            let prs = if 0 < repo.pr_count {
                Cell::new(repo.pr_count)
                    .fg(Color::Yellow)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center)
            } else {
                Cell::new(repo.pr_count)
                    .fg(Color::White)
                    .add_attribute(Attribute::NoBold)
                    .set_alignment(CellAlignment::Center)
            };
            let issues = if 0 < repo.pr_count {
                Cell::new(repo.issue_count)
                    .fg(Color::Yellow)
                    .add_attribute(Attribute::Bold)
                    .set_alignment(CellAlignment::Center)
            } else {
                Cell::new(repo.issue_count)
                    .fg(Color::White)
                    .add_attribute(Attribute::NoBold)
                    .set_alignment(CellAlignment::Center)
            };
            table.add_row(vec![repo_name, prs, issues]);
        }

        writeln!(f, "{table}")
    }
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

#[instrument(skip(issues))]
async fn count_issues(
    issues: &Arc<Issues>,
    user: &str,
    repo: &str,
) -> Result<(usize, usize), Error> {
    let all_issues = issues
        .list_all_for_repo(
            user,
            repo,
            "",
            IssuesListState::Open,
            "",
            "",
            "",
            "",
            IssuesListSort::Created,
            Order::Asc,
            None,
        )
        .await;

    debug!(
        "Success of request for all issues: {:?}",
        &all_issues.is_ok()
    );

    match all_issues {
        Ok(v) => {
            let mut pr_count = 0;
            let mut issue_count = 0;

            for issue in v {
                match issue.pull_request {
                    Some(_) => pr_count += 1,
                    None => issue_count += 1,
                }
            }
            Ok((pr_count, issue_count))
        }
        Err(e) => {
            debug!(
                "Error returned seeking list of all open pull requests:{:?}",
                &e
            );
            Err(e.into())
        }
    }
}
