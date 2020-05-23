#[cfg(feature = "gitlab12")]
use gitlab12 as gitlab;

use colored::Colorize;
use gitlab::Gitlab;
use gitlabctl::{
    error::{Error, GitlabCtlError},
    gitlabctl::GitlabCtl,
    label::Label,
};
use log::debug;
use std::{fs, path::PathBuf};
use structopt::StructOpt;

/// Manage labels
#[derive(StructOpt, Debug, Clone)]
enum Labels {
    Create(CreateLabels),
}

///Create labels from json file
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "labels")]
struct CreateLabels {
    #[structopt(parse(from_os_str))]
    /// path to json file with defined labels
    labels: PathBuf,
    #[structopt(long)]
    /// project name (for example `group/project`)
    project: String,
}

impl CreateLabels {
    pub fn labels(&self) -> Result<Vec<Label>, Error> {
        let file = fs::File::open(&self.labels)?;
        serde_json::from_reader(file).map_err(Error::from)
    }
}

/// List issues
///
/// list all issues that can be accessed by authorized user (if you want all use root Private Key).
#[derive(StructOpt, Debug, Clone)]
struct ListIssues {
    /// show only issues assigned to some user
    #[structopt(long)]
    assigned_to: Option<String>,
    #[structopt(long)]
    /// show only for some project
    project: Option<String>,
}

/// Manage issues
#[derive(StructOpt, Debug, Clone)]
enum Issues {
    List(ListIssues),
}

#[derive(StructOpt, Debug, Clone)]
enum Cmd {
    Labels(Labels),
    Issues(Issues),
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(about, author)]
struct GitlabCtlOpt {
    /// gitlab host name
    #[structopt(long, default_value = "gitlab.com")]
    host: String,
    /// your private token API with api scope
    #[structopt(long)]
    token: String,
    #[structopt(subcommand)]
    cmd: Cmd,
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let opt = GitlabCtlOpt::from_args();
    let gitlab = Gitlab::new(opt.host, opt.token)?;
    let gitlabctl = GitlabCtl::new(gitlab);

    match opt.cmd {
        Cmd::Labels(Labels::Create(l)) => {
            let labels_to_create = l.labels()?;
            debug!("Labels to create {:#?}", labels_to_create);
            let create_result = gitlabctl.create_project_labels(&l.project, labels_to_create)?;
            let mut r = Ok(());
            for label_result in create_result {
                match label_result {
                    Ok(created_label) => {
                        println!("{:<30} label created successfully", created_label.name)
                    }
                    Err((label, err)) => {
                        eprintln!(
                            "{:<30} label couldn't be created because => {}",
                            label.name, err
                        );
                        r = Err(err);
                    }
                }
            }
            r.map_err(Error::from)
        }
        Cmd::Issues(Issues::List(issues_opt)) => {
            debug!("Issues list {:?}", issues_opt);

            let assigned_to = issues_opt
                .assigned_to
                .map(|user| gitlabctl.user_by_name(&user).map(|r| (user, r)))
                .transpose()? //  here we have Option<Option<BasicUser>>
                .map(|(user, opt_found_user)| {
                    opt_found_user.ok_or(GitlabCtlError::UserNotFound { user })
                })
                .transpose()?;
            let project_id = issues_opt
                .project
                .map(|project_name| gitlabctl.project_name_to_id(&project_name))
                .transpose()?;
            debug!("assigned to => {:?}", assigned_to);
            debug!("project_id  => {:?}", project_id);

            let issues = gitlabctl.all_issues()?;
            debug!("issues:\n {:#?}", issues);
            for issue in issues
                .into_iter()
                .filter(|issue| {
                    issue.assignee.as_ref().map(|i| i.id) == assigned_to.as_ref().map(|i| i.id)
                    //TODO there is also issue.assignees
                })
                .filter(|i| project_id.map(|id| id == i.project_id).unwrap_or(true))
            {
                println!("{} ({})", issue.title.bold(), issue.web_url.italic());
            }
            return Ok(());
        }
    }
}
