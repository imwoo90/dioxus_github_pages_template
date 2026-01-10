use dioxus::prelude::*;
use views::{About, BlogList, BlogPost, Contact, Home, Navbar, NotFound, Projects, WasmProject};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},

        #[route("/blog")]
        BlogList {},

        #[route("/blog/:id")]
        BlogPost { id: String },

        #[route("/projects")]
        Projects {},

        #[route("/projects/wasm")]
        WasmProject {},

        #[route("/about")]
        About {},

        #[route("/contact")]
        Contact {},
    #[end_layout]

    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
#[server(endpoint = "static_routes", output = server_fn::codec::Json)]
async fn static_routes() -> Result<Vec<String>, ServerFnError> {
    let mut routes = Route::static_routes()
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>();

    // Add dynamic routes that you want to pre-render
    routes.push(
        Route::BlogPost {
            id: "post-1".to_string(),
        }
        .to_string(),
    );

    Ok(routes)
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("assets/tailwind.css");

fn main() {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(App);
}

/// Detect initial theme (Pure Rust abstraction)
fn get_initial_theme() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(saved)) = storage.get_item("theme") {
                    return saved == "dark";
                }
            }
            if let Ok(Some(mql)) = window.match_media("(prefers-color-scheme: dark)") {
                return mql.matches();
            }
        }
    }
    // Default for Desktop/SSR
    false
}

/// Sync theme to storage and document root (Pure Rust abstraction)
#[allow(unused_variables)]
fn sync_theme(is_dark: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // 1. Sync with document root (html tag) so that body background and Tailwind variants work correctly
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    let _ = root.class_list().toggle_with_force("dark", is_dark);
                }
            }
            // 2. Persist to localStorage
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if is_dark { "dark" } else { "light" });
            }
        }
    }
}

#[allow(non_snake_case)]
#[component]
fn App() -> Element {
    let is_dark = use_signal(get_initial_theme);
    use_context_provider(|| is_dark);

    // Platform-agnostic effect
    use_effect(move || sync_theme(is_dark()));

    rsx! {

        // Standard Links
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "true",
        }
        document::Link {
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;700;900&family=Roboto+Mono&family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap",
            rel: "stylesheet",
        }
        document::Link {
            href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css",
            rel: "stylesheet",
        }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        // Root Wrapper: Reacts to is_dark signal
        div { class: if is_dark() { "dark" } else { "" },
            div { class: "bg-background-light dark:bg-background-dark text-text-dark dark:text-text-light min-h-screen transition-colors duration-300",
                Router::<Route> {}
            }
        }
    }
}
