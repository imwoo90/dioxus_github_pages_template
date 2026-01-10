// Forces recompilation to refresh included files
use crate::content::{get_read_time as shared_get_read_time, parse_frontmatter};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

static POSTS_DIR: Dir<'_> = include_dir!("posts");

/// Metadata for a blog post.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PostMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

/// A complete blog post including metadata and markdown content.
#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub meta: PostMeta,
    pub content: String,
}

impl Post {
    pub fn get_read_time(&self) -> String {
        shared_get_read_time(&self.content)
    }
}

/// Retrieves all blog posts metadata, sorted by date descending.
pub fn get_all_posts() -> Vec<PostMeta> {
    let mut posts = Vec::new();

    for entry in POSTS_DIR.files() {
        if let Some(content) = entry.contents_utf8() {
            let id = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();

            if let Ok(post) = parse_markdown(content, id) {
                posts.push(post.meta);
            }
        }
    }

    // Sort by date descending
    posts.sort_by(|a, b| b.date.cmp(&a.date));
    posts
}

pub fn get_all_categories() -> Vec<String> {
    let posts = get_all_posts();
    let mut categories = std::collections::HashSet::new();
    for post in posts {
        for tag in post.tags {
            categories.insert(tag);
        }
    }
    let mut categories: Vec<String> = categories.into_iter().collect();
    categories.sort();
    categories
}

pub fn get_post_by_id(id: &str) -> Option<Post> {
    for entry in POSTS_DIR.files() {
        let file_id = entry.path().file_stem()?.to_str()?;
        if file_id == id {
            let content = entry.contents_utf8()?;
            return parse_markdown(content, id.to_string()).ok();
        }
    }
    None
}

fn parse_markdown(content: &str, id: String) -> Result<Post, String> {
    let (mut meta, markdown): (PostMeta, &str) = parse_frontmatter(content)?;
    meta.id = id;

    Ok(Post {
        meta,
        content: markdown.to_string(),
    })
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new(markdown);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}
