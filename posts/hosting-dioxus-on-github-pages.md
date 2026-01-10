---
title: "Building a Rust Blog on GitHub Pages with Dioxus"
date: "2026-01-11"
author: "imwoo90"
description: "A comprehensive guide on how we built this Rust-powered blog and deployed it to GitHub Pages using Dioxus and GitHub Actions."
image_url: "https://raw.githubusercontent.com/DioxusLabs/dioxus/master/packages/desktop/examples/assets/logo.png"
tags: ["rust", "dioxus", "github-pages", "tutorial"]
---

Dioxus is an incredible framework that brings the power of Rust to the web via WebAssembly. In this post, I'll share the unique development workflow I used to create this blog—from initial AI-assisted design to final deployment on GitHub Pages.

## Phase 1: Designing with Stitch

The journey started with a visual vision. Instead of coding CSS from scratch, I used [Stitch](https://stitch.withgoogle.com/) to design the blog's interface. It allowed for rapid prototyping of the modern, dark-themed aesthetic you see now.

- **Stitch Design Project**: [View Design](https://stitch.withgoogle.com/projects/12286301471497023600)

## Phase 2: Setting up the Foundation

While the design was being finalized, I prepared a clean **Dioxus** base project. I initialized a standard Dioxus web app and pushed it to a GitHub repository. This acted as the "skeleton" where the design and logic would eventually live.

## Phase 3: Merging Design via Jules

It's important to note that **Stitch** doesn't directly generate Dioxus `rsx!` macros; its output is typically clean HTML, CSS, or specialized design tokens. To bridge this gap, I used **Jules** (Google's autonomous coding assistant). Jules took the design output from Stitch and intelligently merged the layout, styles, and components into my pre-prepared Dioxus GitHub repository. This automated merge saved hours of manual CSS-to-RSX conversion, giving me a solid UI foundation inside the Dioxus workspace.

## Phase 4: Refinement with Antigravity

With the design-driven code in place, I used **Antigravity** (a powerful agentic AI coding assistant) to handle the complex logic and fine-grained details. Antigravity helped:
- Implement the Markdown parsing engine (`pulldown-cmark`).
- Set up the dynamic routing for blog posts and project pages.
- Refine the responsive behavior and interactive elements.

## Phase 5: Hosting on GitHub Pages (The Technical Polish)

The final step was ensuring the world could see it. Here are the key technical details for GitHub Pages:

### ⚙️ Configuring `Dioxus.toml`
Since GitHub Pages hosts sites at `username.github.io/repo-name/`, we must set the `base_path`:
```toml
[web.app]
base_path = "your_repo_name"
```

### 🛰 Handling the SPA 404 Issue
Because it's a Single Page Application (SPA), refreshes on sub-routes would normally return 404. We handle this in our deployment workflow by copying the entry point:
```bash
cp docs/index.html docs/404.html
```

### 🚢 Automated deployment
Whenever I push to the `main` branch, a GitHub Action automatically builds the Wasm binary, bundles the assets, and deploys everything to the `gh-pages` branch.

## Conclusion

By combining the design power of **Stitch**, the automation of **Jules**, the coding intelligence of **Antigravity**, and the performance of **Dioxus**, building a premium blog has never been more efficient.

Happy coding!
