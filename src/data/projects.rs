use crate::data::utils::{get_read_time as shared_get_read_time, parse_frontmatter};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

static PROJECTS_DIR: Dir<'_> = include_dir!("projects");

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ProjectMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
    pub link: Option<String>,
    pub link_text: Option<String>,
    pub route: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub meta: ProjectMeta,
    pub content: String,
}

impl Project {
    pub fn get_read_time(&self) -> String {
        shared_get_read_time(&self.content)
    }
}

pub fn get_all_projects() -> Vec<ProjectMeta> {
    let mut projects = Vec::new();

    for entry in PROJECTS_DIR.files() {
        if let Some(content_str) = entry.contents_utf8() {
            if let Ok(meta) = parse_project_meta(
                content_str,
                entry
                    .path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
            ) {
                projects.push(meta);
            }
        }
    }

    // Sort by date descending
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    projects
}

pub fn get_project_by_id(id: &str) -> Option<Project> {
    for entry in PROJECTS_DIR.files() {
        let file_id = entry.path().file_stem()?.to_str()?;
        if file_id == id {
            let content_str = entry.contents_utf8()?;
            return parse_project_full(content_str, id.to_string()).ok();
        }
    }
    None
}

fn parse_project_meta(content: &str, id: String) -> Result<ProjectMeta, String> {
    let project = parse_project_full(content, id)?;
    Ok(project.meta)
}

fn parse_project_full(content: &str, id: String) -> Result<Project, String> {
    let (mut meta, markdown): (ProjectMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Project {
        meta,
        content: markdown.to_string(),
    })
}
