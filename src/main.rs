use dioxus::prelude::*;

use views::{About, BlogList, BlogPost, Contact, Home, NotFound, Projects, WasmProject, Navbar};

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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Tailwind Script for CDN fallback if build fails (optional, but good for proto)
        // script { src: "https://cdn.tailwindcss.com?plugins=forms,container-queries" }
        // document::Script {
        //     id: "tailwind-config",
        //     "tailwind.config = {{ darkMode: 'class', theme: {{ extend: {{ colors: {{ 'primary': '#ec5b13', 'primary-light': '#DEA584', 'background-light': '#f8f6f6', 'background-dark': '#1E1E1E', 'background-darker': '#121212' }} }} }} }}"
        // }

        // Fonts
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "true" }
        document::Link { href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700;900&display=swap", rel: "stylesheet" }
        document::Link { href: "https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@400;500&display=swap", rel: "stylesheet" }
        document::Link { href: "https://fonts.googleapis.com/css2?family=Fira+Code:wght@400;600&display=swap", rel: "stylesheet" }
        document::Link { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:wght,FILL@100..700,0..1&display=swap", rel: "stylesheet" }

        // Highlight.js
        document::Link { href: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/atom-one-dark.min.css", rel: "stylesheet" }
        document::Script { src: "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js" }

        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}
