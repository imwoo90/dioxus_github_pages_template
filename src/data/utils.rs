use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CommonMeta {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,
    pub image_url: String,
    pub tags: Vec<String>,
}

pub fn parse_frontmatter<T: for<'de> Deserialize<'de>>(content: &str) -> Result<(T, &str), String> {
    if !content.starts_with("---") {
        return Err("No frontmatter found".to_string());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format".to_string());
    }

    let yaml = parts[1];
    let markdown = parts[2];

    let meta: T = serde_yaml::from_str(yaml).map_err(|e| e.to_string())?;

    Ok((meta, markdown.trim()))
}

pub fn get_read_time(content: &str) -> String {
    let words = content.split_whitespace().count();
    let minutes = (words as f32 / 200.0).ceil() as u32;
    if minutes <= 1 {
        "1 min read".to_string()
    } else {
        format!("{} min read", minutes)
    }
}

pub fn markdown_to_html(markdown: &str, post_id: &str, content_type: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown, options).map(|event| match event {
        Event::Start(Tag::Image {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let mut new_dest = dest_url.to_string();

            if !new_dest.starts_with("http") && !new_dest.starts_with("/") {
                let clean_path = new_dest.trim_start_matches("./");
                new_dest = format!("content/{}/{}/{}", content_type, post_id, clean_path);
            }

            Event::Start(Tag::Image {
                link_type,
                dest_url: new_dest.into(),
                title,
                id,
            })
        }
        _ => event,
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
