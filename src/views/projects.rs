use crate::components::{
    BlogHero, CallToAction, Card, Comment, Comments, Container, Hero, Section,
};
use crate::hooks::use_syntax_highlighting;
use crate::posts::markdown_to_html;
use crate::projects_data::{get_all_projects, get_project_by_id};
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    let projects = get_all_projects();

    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "The Workshop",
                    subtitle: "A collection of my work, showcasing the versatility of Rust across the full stack.",
                    centered: Some(false),
                    children: rsx! {},
                }
                Section { class: "px-4 mb-20",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        for project in projects {
                            Card {
                                title: project.title.clone(),
                                description: project.description.clone(),
                                image_url: project.image_url.clone(),
                                tags: project.tags.clone(),
                                link_text: project.link_text.clone().unwrap_or_else(|| "View Details".to_string()),
                                external_link: if project.route.is_none() && project.link.is_some() { project.link.clone().unwrap_or_else(|| "#".to_string()) } else { "".to_string() },
                                link_to: Some(Route::ProjectDetail {
                                    id: project.id.clone(),
                                }),
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn ProjectDetail(id: String) -> Element {
    let project = get_project_by_id(&id);
    use_syntax_highlighting();

    match project {
        Some(project) => {
            let html_content = markdown_to_html(&project.content);

            rsx! {
                div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
                    article { class: "w-full max-w-3xl flex flex-col gap-10",
                        BlogHero {
                            title: project.meta.title.clone(),
                            author: project.meta.author.clone(),
                            date: project.meta.date.clone(),
                            read_time: project.get_read_time(),
                        }

                        div {
                            class: "prose max-w-none dark:prose-invert",
                            dangerous_inner_html: "{html_content}",
                        }

                        if let Some(link) = &project.meta.link {
                            div { class: "mt-8",
                                a {
                                    href: "{link}",
                                    class: "inline-flex items-center gap-2 bg-primary-light text-text-dark px-6 py-3 rounded-lg font-bold hover:opacity-90 transition-all shadow-md active:scale-95",
                                    target: "_blank",
                                    span { class: "material-symbols-outlined", "launch" }
                                    "{project.meta.link_text.clone().unwrap_or_else(|| \"Visit Project\".to_string())}"
                                }
                            }
                        }

                        Comments {
                            comments: vec![
                                Comment {
                                    author: "Alice".to_string(),
                                    date: "2 days ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuAKTYnKgoWMM15UTvOugoMXvYWs9d-Yo9RCvP6v_ilmnnp8-_OLVqoz1-1AXhD1nNrSq9Z6DfjYY84gVr6eNvJB9O-GYThPTVr5TKapPERZQYYJqdPdks41NivF_GEpX82s4WZ3YZR39bKzgBc7MnkRyKSpauNcQoLJE1pg6IgE5PeMQOMCD0-4TATNGCc_JqpTcEdqQl_9Xelzn2FMFigdAiJ3_Vlsl9CvsliwUySKm-99ilP7IdYUSYQ0v9A6FapxMTzVqSRGWpI7"
                                        .to_string(),
                                    text: "This is an incredible write-up. The performance gains you've achieved by using Rust and Wasm are seriously impressive. I've been considering a similar approach for a project, and this post just convinced me. Great work!"
                                        .to_string(),
                                },
                                Comment {
                                    author: "Bob".to_string(),
                                    date: "1 day ago".to_string(),
                                    avatar_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBBY54kL1b9FE1c05fMLGziuoEeOH4QsqnruT9ad7rlIs0rLWzKrS_Z9-RWZkSuiCXnRcqQS5Rt2JoqkiKOL6VQSYzlHEl35OgBa9EOvwBmD5ypYtvZyL9nfx7eKDMA5PDHBV1Rf2ROcDvuF7FABBmdLvY4qL1tG20C_4JssGJPKbDdANUcfv11LJL_8s-67IVXzTFr2uo3ApFtU-PxGgkRp3IhLgB8AlonuubXq7sMAgiS8-BZq3EyMP3qr2pbtOYx7fz15rmwrsHX"
                                        .to_string(),
                                    text: "Fascinating project. I'm curious about the bundle size. How did you manage to keep it small enough for a good web experience? Any tips on optimizing the Wasm binary?"
                                        .to_string(),
                                },
                            ],
                        }

                        CallToAction {}
                    }
                }
                Footer {}
            }
        }
        None => rsx! {
            div { class: "flex flex-col items-center justify-center min-h-[60vh]",
                h1 { class: "text-4xl font-bold", "Project Not Found" }
                Link {
                    to: Route::Projects {},
                    class: "mt-4 text-primary-light hover:underline",
                    "Back to Projects"
                }
            }
            Footer {}
        },
    }
}
