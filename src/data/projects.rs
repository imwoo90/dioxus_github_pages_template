use crate::data::utils::parse_frontmatter;
use serde::{Deserialize, Serialize};

const PROJECTS_INDEX: &str = include_str!("../../public/content/projects_index.json");

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub meta: ProjectMeta,
    pub content: String,
}

impl Project {
    pub fn get_read_time(&self) -> String {
        crate::data::utils::get_read_time(&self.content)
    }
}

pub fn get_all_projects() -> Vec<ProjectMeta> {
    let mut projects: Vec<ProjectMeta> = serde_json::from_str(PROJECTS_INDEX).unwrap_or_default();
    // Sort by date descending
    projects.sort_by(|a, b| b.date.cmp(&a.date));
    projects
}

pub fn get_all_categories() -> Vec<String> {
    let projects = get_all_projects();
    let mut categories = std::collections::HashSet::new();
    for project in projects {
        for tag in project.tags {
            categories.insert(tag);
        }
    }
    let mut categories: Vec<String> = categories.into_iter().collect();
    categories.sort();
    categories
}

pub async fn get_project_by_id(id: &str) -> Option<Project> {
    let url = format!("/my_blog/content/projects/{}/index.md", id);
    let content = match gloo_net::http::Request::get(&url).send().await {
        Ok(resp) => resp.text().await.ok()?,
        Err(_) => return None,
    };

    parse_project_full(&content, id.to_string()).ok()
}

fn parse_project_full(content: &str, id: String) -> Result<Project, String> {
    let (mut meta, markdown): (ProjectMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Project {
        meta,
        content: markdown.to_string(),
    })
}
