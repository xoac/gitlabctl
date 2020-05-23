use gitlab::{
    types::{Issue as ApiIssue, Label as ApiLabel, Project as ApiProject, ProjectId},
    Gitlab,
};

use gitlab::types as api_types;

use gitlab::api::Query;

use crate::{error, label::Label};
use log::{debug, trace};

type ApiGitlabError = gitlab::api::ApiError<self::error::ApiRestError>;

type Result<T> = std::result::Result<T, ApiGitlabError>;
type LabelCreateErr = (Label, ApiGitlabError);

#[derive(Debug)]
pub struct GitlabCtl {
    gitlab: Gitlab,
}

impl GitlabCtl {
    pub fn new(gitlab: Gitlab) -> Self {
        Self { gitlab }
    }
}

impl GitlabCtl {
    pub fn user_by_name(&self, name: &str) -> Result<Option<api_types::UserBasic>> {
        trace!("gitlabctl::user_by_name called");

        let r: Vec<api_types::UserBasic> = gitlab::api::users::Users::builder()
            .username(name)
            .build()
            .expect("Correct user name")
            .query(&self.gitlab)?;

        assert!(r.len() <= 1);
        Ok(r.get(0).cloned()) //FIXME why we need here clone?
    }

    pub fn all_projects(&self) -> Result<Vec<ApiProject>> {
        gitlab::api::projects::Projects::builder()
            .owned(false)
            .build()
            .expect("all projects query should be valid")
            .query(&self.gitlab)
    }

    pub fn all_issues(&self) -> Result<Vec<ApiIssue>> {
        let projects = self.all_projects()?;

        Ok(projects
            .into_iter()
            .map(|api_project| {
                gitlab::api::projects::issues::Issues::builder()
                    .project(api_project.id.value())
                    .build()
                    .expect("correct query for issues")
                    .query(&self.gitlab)
            })
            .collect::<std::result::Result<Vec<Vec<api_types::Issue>>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    pub fn project_name_to_id(&self, name: &str) -> Result<ProjectId> {
        trace!("gitlabctl::project_name_to_id called");

        gitlab::api::projects::Project::builder()
            .project(name)
            .build()
            .expect("Correct project query")
            .query(&self.gitlab)
            .map(|project: api_types::Project| project.id)
    }

    /// Create labels for `project_name` returning result Result for every label
    pub fn create_project_labels(
        &self,
        project_name: &str,
        labels: Vec<Label>,
    ) -> Result<Vec<std::result::Result<ApiLabel, LabelCreateErr>>> {
        trace!("gitlabctl::create_project_labels called");
        let project_id = self.project_name_to_id(project_name).unwrap();

        debug!("ProjectId {}", project_id);
        let result = labels
            .into_iter()
            .map(|label| {
                label
                    .clone()
                    .to_create_label(project_id.value())
                    .query(&self.gitlab)
                    .map_err(|e| (label, e))
            })
            .collect();
        Ok(result)
    }
}
