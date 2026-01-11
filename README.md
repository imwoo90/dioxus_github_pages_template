# Rust & Dioxus Blog for GitHub Pages

A high-performance, developer-centric blog template built with Rust and Dioxus, optimized for seamless deployment on GitHub Pages.

## 🚀 Features

-   **Rust & Dioxus**: Built on [Dioxus 0.7+](https://dioxuslabs.com/), a type-safe and performant frontend framework.
-   **Wasm-Powered**: Runs directly in the browser using WebAssembly for a near-native experience.
-   **Dynamic Content Management**: Supports folder-based Markdown content fetching at runtime via `gloo-net`.
-   **Automatic Indexing**: A robust `build.rs` script automatically generates metadata indexes (`posts_index.json`, `projects_index.json`) for seamless discovery.
-   **Local Image Support**: Content-specific images are managed alongside Markdown files in dedicated folders.
-   **Tailwind CSS**: Modern, responsive design system.
-   **Theme Switching**: Support for Light/Dark mode based on user preference or system settings.
-   **Automated Deployment**: Built-in CI/CD with GitHub Actions, tailored for Dioxus 0.7 bundle structure.

## 🛠 Tech Stack

-   **Language**: Rust
-   **Frontend**: Dioxus (Web platform)
-   **Styling**: Tailwind CSS
-   **Content**: Markdown (`pulldown-cmark`)
-   **Fetching**: `gloo-net` (Runtime async fetch)
-   **Deployment**: GitHub Actions & GitHub Pages

## 📂 Project Structure

```bash
my_blog/
├── public/                 # Static assets served at the root
│   └── content/            # Dynamic content directory
│       ├── posts/          # Blog posts (each in its own folder)
│       │   └── post-id/
│       │       ├── index.md
│       │       └── image.png
│       └── projects/       # Projects (each in its own folder)
├── src/                    # Rust source code
│   ├── components/         # Reusable UI components
│   ├── data/               # Data models and fetching logic
│   │   ├── blog.rs         # Blog-related data & logic
│   │   ├── projects.rs     # Project-related data & logic
│   │   └── utils.rs        # Shared utilities (Markdown parsing)
│   ├── views/              # Page views and routing
│   └── main.rs             # App entry point & Route definition
├── build.rs                # Component indexing logic (generates _index.json)
├── Dioxus.toml             # Dioxus configuration
└── Cargo.toml              # Dependencies
```

## 🚀 Getting Started

### 1. Prerequisites

Ensure you have Rust installed. Then, install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

### 2. Local Development

Run the following command to start the development server:

```bash
dx serve
```

> **Note on Base Path**: This project is configured with a `base_path` of `my_blog` in `Dioxus.toml`. When running locally, you can access the site at:
> `http://localhost:8080/my_blog`

### 3. Build

To generate static files for production deployment:

```bash
dx build --release
```

## 🚢 Deployment

This project is configured for automated deployment via GitHub Actions (`.github/workflows/deploy.yml`). When you push to the `main` branch:

1.  The environment is set up with Rust and Dioxus CLI.
2.  The app is built and bundled using `dx bundle --release`.
3.  The workflow ensures all generated content and assets in `public/` are correctly mapped to the deployment root.
4.  The output is deployed to the `gh-pages` (or `docs/`) branch.

### ⚙️ Customization for Forks

If you fork this repository, you **must** update the configuration to match your repository name for GitHub Pages to work correctly:

1.  **Update `Dioxus.toml`**: 
    Change the `base_path` value to your repository name:
    ```toml
    [web.app]
    base_path = "your_repository_name"
    ```
2.  **Update Local Access**:
    After changing the `base_path`, your local development URL will also change to:
    `http://localhost:8080/your_repository_name`

## 📝 Content Management

### Adding a Blog Post
Simply add a new `.md` file to the `posts/` directory.

### Adding a Project
Add a Markdown file to the `projects/` directory and update the data source (e.g., `src/projects_data.rs`) to include the new project metadata.

---

Built with ❤️ using **Rust** and **Dioxus**.

