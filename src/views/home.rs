use crate::components::*;
use crate::posts::get_all_posts;
use crate::projects_data::get_all_projects;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let posts = get_all_posts();
    let projects = get_all_projects();

    // Create a unified list of recent items
    let mut recent_items = Vec::new();

    for post in posts {
        recent_items.push((
            post.date.clone(),
            rsx! {
                Card {
                    title: post.title.clone(),
                    description: post.description.clone(),
                    image_url: post.image_url.clone(),
                    tags: post.tags.clone(),
                    link_to: Route::BlogPost { id: post.id.clone() },
                }
            },
        ));
    }

    for project in projects {
        recent_items.push((
            project.date.clone(),
            rsx! {
                Card {
                    title: project.title.clone(),
                    description: project.description.clone(),
                    image_url: project.image_url.clone(),
                    tags: project.tags.clone(),
                    link_to: Route::ProjectDetail { id: project.id.clone() },
                }
            },
        ));
    }

    // Sort by date descending
    recent_items.sort_by(|a, b| b.0.cmp(&a.0));

    // Take the 3 most recent
    let latest_elements = recent_items.into_iter().take(3).map(|(_, el)| el);

    rsx! {
        Container {
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                Hero {
                    title: "Rust's Horizon",
                    subtitle: "Navigating the Rust ecosystem from silicon to screen. I document the journey of building high-performance software across the entire stack—spanning bare-metal MCUs, backend services, and native mobile apps.",
                    PrimaryButton { to: Route::BlogList {}, text: "Explore the Blog" }
                }

                Section {
                    SectionTitle { title: "Latest Articles" }

                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        for el in latest_elements {
                            {el}
                        }
                    }
                }

                section { class: "flex flex-col items-center text-center gap-6 bg-white dark:bg-white/5 p-8 sm:p-12 rounded-lg transition-colors border border-text-dark/5 dark:border-white/5 shadow-sm dark:shadow-none",
                    SectionTitle { title: "Let's Build Together" }
                    p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-base font-normal leading-normal max-w-2xl",
                        "I'm passionate about tackling challenging projects with Rust. If you're looking for a developer with deep experience in embedded systems, performance optimization, and cross-platform development, let's talk."
                    }
                    PrimaryButton { to: Route::Contact {}, text: "Get in Touch" }
                }
            }
        }
        Footer {}
    }
}
