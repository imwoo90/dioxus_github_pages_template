# Rust & Dioxus Blog for GitHub Pages

A high-performance, developer-centric blog template built with Rust and Dioxus, optimized for seamless deployment on GitHub Pages.

## 🚀 Features

-   **Rust & Dioxus**: Built on [Dioxus](https://dioxuslabs.com/), a type-safe and performant frontend framework.
-   **Wasm-Powered**: Runs directly in the browser using WebAssembly for a near-native experience.
-   **Markdown Support**: Content managed via Markdown files in `posts/` and `projects/` using `pulldown-cmark`.
-   **Tailwind CSS**: Modern, responsive design system.
-   **Theme Switching**: Support for Light/Dark mode based on user preference or system settings.
-   **Automated Deployment**: Built-in CI/CD with GitHub Actions.

## 🛠 Tech Stack

-   **Language**: Rust
-   **Frontend**: Dioxus (Web platform)
-   **Styling**: Tailwind CSS
-   **Content**: Markdown
-   **Deployment**: GitHub Actions & GitHub Pages

## 📂 Project Structure

```bash
my_blog/
├── assets/         # Static assets like images and fonts
├── posts/          # Markdown files for blog posts
├── projects/       # Markdown files for project descriptions
├── src/            # Rust source code
│   ├── components/ # Reusable UI components
│   ├── views/      # Page views for different routes
│   ├── posts.rs    # Data handling and loading logic for posts
│   └── main.rs     # Application entry point and router setup
├── tailwind.css    # Tailwind CSS input styles
├── Dioxus.toml     # Dioxus project configuration
└── Cargo.toml      # Rust dependencies and build configuration
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
2.  The app is built and bundled using `dx bundle`.
3.  The output is deployed to the `gh-pages` branch.
4.  Since GitHub Pages hosts the site at `https://<username>.github.io/<repository_name>/`, the `base_path` ensures that all assets and routes are correctly linked.

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

