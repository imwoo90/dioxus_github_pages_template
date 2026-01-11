---
title: "Rust & Dioxus Blog Template"
date: "2026-01-11"
author: "imwoo90"
description: "A high-performance blog and portfolio template built with Dioxus and Rust, optimized for GitHub Pages with automated CI/CD workflows."
image_url: "dioxus_template.png"
tags: ["rust", "dioxus", "webassembly", "github-pages"]
link: "https://github.com/imwoo90/my_blog"
link_text: "View on GitHub"
---

# Rust & Dioxus Blog for GitHub Pages

This project is a modern, developer-centric blog and portfolio template. It leverages the power of **Rust** and **Dioxus** to provide a lightning-fast, type-safe, and highly customizable web application that can be hosted entirely on **GitHub Pages**.

## Key Features

- **Blazing Fast**: Compiles to WebAssembly (Wasm) for near-native performance in the browser.
- **Type-Safe**: Developed entirely in Rust, ensuring memory safety and robustness.
- **Markdown-Driven**: Posts and projects are managed through static Markdown files, making content updates simple and git-friendly.
- **Automated Deployment**: Includes a battle-tested GitHub Actions workflow for automatic building and deployment.
- **Responsive Design**: Styled with Tailwind CSS for a clean, modern, and mobile-friendly UI.
- **SEO Optimized**: Built-in support for meta tags and semantic HTML.

## Why Dioxus?

Dioxus provides a React-like developer experience but with the benefits of Rust. It allows for high-level component-based architecture while maintaining extreme efficiency. By using Dioxus for this blog template, we achieve:

1. **Easy State Management**: Using Rust's ownership and signals.
2. **Zero Runtime Latency**: No heavy JavaScript bundles.
3. **Cross-Platform Potential**: While this template targets the web, Dioxus makes it easy to port to desktop or mobile in the future.

## Zero-Config Automation

One of the main goals of this project was to make it truly "plug and play." By using advanced CI/CD techniques and dynamic runtime detection, the blog automatically adapts its configuration to your repository name. 

- **No manual edits** needed in `Dioxus.toml` for deployment.
- **Dynamic asset loading** that works everywhere.
- **Automated GitHub Actions** that handle the build, base-tag injection, and deployment.
