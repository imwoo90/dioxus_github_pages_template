---
title: "Building a Rust Blog on GitHub Pages with Dioxus"
date: "2026-01-11"
author: "imwoo90"
description: "A comprehensive guide on how we built this Rust-powered blog and deployed it to GitHub Pages using Dioxus and GitHub Actions."
image_url: "/my_blog/content/posts/hosting-dioxus-on-github-pages/dioxus_gh_pages_hosting.png"
tags: ["rust", "dioxus", "github-pages", "tutorial"]
---

Dioxus is an incredible framework that brings the power of Rust to the web via WebAssembly. In this post, I'll share the unique development workflow I used to create this blog—from initial AI-assisted design to final deployment on GitHub Pages.

## The Vision: A New Era of Development

Building a modern web application today isn't just about writing code; it's about orchestrating powerful tools. This project serves as a blueprint for how **AI-augmented design** meets **high-performance systems programming**.

---

## 🎨 Phase 1: Designing with Stitch

The journey started with a visual vision. Instead of coding CSS from scratch, I used [Stitch](https://stitch.withgoogle.com/) to design the blog's interface. 

- **Stitch Design Project**: [Explore the Canvas](https://stitch.withgoogle.com/projects/12286301471497023600)
- **Outcome**: A premium, high-fidelity UI mockup with tokens ready for the next stage.

## 🏗️ Phase 2: Setting up the Foundation

While the design was being finalized, I prepared a clean **Dioxus** base project. I initialized a standard Dioxus web app and pushed it to a GitHub repository. This acted as the "skeleton" where the design and logic would eventually live.

## 🔗 Phase 3: The Bridge - Jules

It's important to note that **Stitch** doesn't directly generate Dioxus `rsx!` macros; its output is typically clean HTML/CSS. To bridge this gap, I used **Jules** (Google's autonomous coding assistant). 

Jules took the design output from Stitch and intelligently **merged** the layout and styles into my pre-prepared Dioxus GitHub repository. This automated merge saved hours of manual translation, giving me a solid UI foundation inside the Rust workspace.

## 🤖 Phase 4: Refinement with Antigravity

With the design-driven code in place, I turned to **Antigravity**. As a powerful agentic AI coding assistant, Antigravity handled the heavy lifting of the application's core logic:
-   **Dynamic Content Architecture**: Transitioning from static embedding (`include_dir!`) to a scalable folder-based system.
-   **Automated Indexing**: Implementing a `build.rs` script that scans `public/content/` and generates JSON metadata indexes at compile time.
-   **Runtime Fetching**: Using `gloo-net` to asynchronously fetch Markdown content only when a user navigates to a specific post.
-   **Image Resolution**: Crafting a custom Markdown-to-HTML pipeline that automatically resolves relative image paths inside post folders to their correct URLs on GitHub Pages.

---

## 🚀 Phase 5: Hosting on GitHub Pages

The final step was ensuring the world could see it. We refined the deployment workflow to handle the technical requirements of GitHub Pages and Dioxus 0.7:

### 1. The `base_path` Configuration
Crucial for GitHub's subpath hosting (`username.github.io/repo/`):
```toml
[web.app]
base_path = "your_repo_name"
```

### 2. Serving through `public/`
By moving our content to the `public/` directory, we ensure that `dx serve` and `dx bundle` treat our Markdown and images as root-level assets, making them easily fetchable by the Wasm binary.

### 3. Automated CI/CD
Our GitHub Actions workflow handles the `dx bundle --release` process and ensures that the generated `docs/` or `gh-pages` branch contains a `404.html` (copied from `index.html`) to support client-side routing.

---

## 💡 The Synergy of Agents

The true magic of this project isn't just in the tech stack (Rust/Dioxus), but in the **Synergy of AI Agents**:
- **Stitch** focuses on the *Aesthetics*.
- **Jules** handles the *Integration*.
- **Antigravity** masters the *Implementation*.

This workflow allows a single developer to build state-of-the-art applications with the polish and performance of a full engineering team.

## Conclusion

This blog is more than a place for my thoughts; it's a testament to the future of software development. By combining **Rust's raw power** with **AI's creative speed**, we can reach new horizons in web development.

Happy coding!
