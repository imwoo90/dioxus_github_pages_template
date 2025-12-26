use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn WasmProject() -> Element {
    use_effect(move || {
         document::eval("if (window.hljs) window.hljs.highlightAll();");
    });

    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 bg-background-darker",
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                article { class: "flex flex-col gap-8",
                    header { class: "flex flex-col gap-4",
                        div { class: "flex items-center gap-2 text-sm",
                            Link { class: "text-primary-light hover:underline", to: Route::Projects {}, "Projects" }
                            span { class: "material-symbols-outlined text-gray-500", "chevron_right" }
                            span { class: "text-text-heading", "Wasm Data-Viz Engine" }
                        }
                        h1 { class: "text-text-heading text-4xl font-bold leading-tight tracking-[-0.015em]",
                            "Wasm Data-Viz Engine"
                        }
                        p { class: "text-text-light text-lg",
                            "A high-performance data visualization library that runs in the browser, powered by Rust compiled to WebAssembly for native-like speed."
                        }
                        div { class: "flex flex-wrap items-center gap-2",
                            span { class: "text-xs font-mono bg-white/10 text-gray-300 px-2 py-1 rounded", "webassembly" }
                            span { class: "text-xs font-mono bg-white/10 text-gray-300 px-2 py-1 rounded", "frontend" }
                            span { class: "text-xs font-mono bg-white/10 text-gray-300 px-2 py-1 rounded", "data-viz" }
                            span { class: "text-xs font-mono bg-white/10 text-gray-300 px-2 py-1 rounded", "performance" }
                        }
                    }
                    div { class: "prose prose-invert max-w-none text-text-light prose-headings:text-text-heading prose-a:text-primary-light hover:prose-a:underline",
                        div { class: "w-full aspect-video rounded-lg overflow-hidden border border-border-dark mb-8",
                            img { class: "w-full h-full object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu", alt: "Data visualization dashboard showing complex charts" }
                        }
                        h2 { class: "text-text-heading", "The Problem: Slow, Clunky Browser Visualizations" }
                        p {
                            "Traditional JavaScript libraries for data visualization can struggle with large datasets, leading to slow rendering times, unresponsive UIs, and a frustrating user experience. When dealing with real-time data streams or complex interactive charts, the performance limitations of JavaScript become a significant bottleneck. This project aimed to overcome these hurdles by leveraging the raw power of Rust."
                        }
                        h2 { class: "text-text-heading", "The Solution: Rust + WebAssembly" }
                        p {
                            "The solution was to build a rendering engine in Rust, a language renowned for its performance and memory safety. By compiling this engine to WebAssembly (Wasm), we can execute near-native speed code directly in the browser. This approach bypasses the JavaScript performance tax for heavy computations, allowing for fluid animations and real-time updates even with millions of data points."
                        }
                        p {
                            "The core of the library handles the heavy lifting:"
                        }
                        ul {
                            li { strong { "Data Processing:" } " Efficiently parsing and transforming large JSON or CSV datasets." }
                            li { strong { "Layout Calculation:" } " Performing complex geometric calculations for chart layouts (e.g., force-directed graphs, treemaps)." }
                            li { strong { "Rendering:" } " Drawing directly to an HTML5 Canvas element with a custom rendering pipeline optimized for speed." }
                        }
                        h2 { class: "text-text-heading", "Code Spotlight: The Rust Core" }
                        p {
                            "Here's a snippet from the Rust code that demonstrates how we define a simple data point and a function to process a vector of these points. This is the kind of logic that gets compiled to hyper-efficient Wasm."
                        }
                        pre {
                            code { class: "language-rust",
                                "use wasm_bindgen::prelude::*;
// Define a simple data structure for our points.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct DataPoint {{
    pub x: f64,
    pub y: f64,
    pub value: f64,
}}
// A function to calculate the average value from a slice of data points.
// This is a simplified example of the kind of computation done in Rust.
#[wasm_bindgen]
pub fn calculate_average(data: &[DataPoint]) -> f64 {{
    let sum: f64 = data.iter().map(|p| p.value).sum();
    if data.is_empty() {{
        0.0
    }} else {{
        sum / data.len() as f64
    }}
}}"
                            }
                        }
                        p {
                            "The ", code { "#[wasm_bindgen]" }, " attribute is the magic that makes these Rust functions and structs available to JavaScript, creating a seamless bridge between the two languages."
                        }
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 my-8",
                            img { class: "rounded-lg border border-border-dark w-full h-auto object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDn1UYUGXZA0JSs_rAfERLePldD4OcbrusEaOmmqaYOEeppGFb-k8UgQ0NjR2pcuX8xnOLdwExx49rb0O8xNDB5qQ-w9UgDV1wvRMOI5fYJYHP39Nsxjc4hY3EkGXCQQlVAR5MsiQsU4XizQuTY6SzawO11zpA54rvWWPNKD9G77guy5rnDonoIyjmWLa4q_nCjTHWwY-1hk_y4VBiCiJsNJeYeLiKB55vAoJ5NQeXPsAN_fRh3mnIXnFKgLoxbMslJpYyq4EFq9xv4", alt: "Architectural diagram of the Wasm engine" }
                            img { class: "rounded-lg border border-border-dark w-full h-auto object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuCYq39SdXr4TMm7kO_Uy2xyaUFPK1uvdOICjaA2ADD3_vc2InSy1JSEAuoVrxImuP5dRw3ficRdKA9GU2KSp3Pqsnsbs6n3ISv0Ne9RUlQzeYkEd1ryMJrSOhllPrGFz5MV2MODT1R2yln-ixaVZsIl7yC2WMVZ22Pj72o9giQ13hixtw-ahBR53KQo4UAnv87qErJQUbR5W4t3ED_VlnRmLiq63ADnGpCQYUy5dVxBZcEj0uaeD6lOTpIJCXxeRvtyvSc4W4u263Vd", alt: "Screenshot of a complex graph rendered by the engine" }
                        }
                        h2 { class: "text-text-heading", "Results & Impact" }
                        p {
                            "The final library is capable of rendering interactive charts with over 1 million data points at a consistent 60 frames per second, a feat that is challenging to achieve with pure JavaScript solutions. This project serves as a powerful demonstration of Rust's potential in the frontend space, proving that it's a viable and compelling choice for performance-critical web applications."
                        }
                        div { class: "bg-surface-dark rounded-lg p-6 flex flex-col sm:flex-row items-center gap-6 mt-8 border border-border-dark",
                            div { class: "grow",
                                h3 { class: "text-text-heading text-xl font-bold", "Explore the Project" }
                                p { class: "text-text-light mt-1", "Dive into the source code, check out the live demo, and see the performance for yourself." }
                            }
                            a { class: "inline-flex items-center justify-center gap-2 whitespace-nowrap bg-primary-light text-text-dark font-bold text-sm px-6 py-3 rounded-md hover:opacity-90 transition-opacity w-full sm:w-auto", href: "#",
                                svg { class: "size-5", fill: "currentColor", view_box: "0 0 24 24", "aria-hidden": "true",
                                    path { clip_rule: "evenodd", d: "M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.168 6.839 9.49.5.092.682-.217.682-.482 0-.237-.009-.868-.014-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.031-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.03 1.595 1.03 2.688 0 3.848-2.338 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.001 10.001 0 0022 12c0-5.523-4.477-10-10-10z", fill_rule: "evenodd" }
                                }
                                "View on GitHub"
                            }
                        }
                    }
                }
                section { class: "border-t border-border-dark pt-12",
                    div { class: "flex flex-col gap-8",
                        h2 { class: "text-text-heading text-2xl font-bold", "Comments (2)" }
                        div { class: "flex flex-col gap-6",
                            div { class: "flex items-start gap-4",
                                img { class: "size-10 rounded-full", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAXB_uuet1wxQCv_HNA5OKhsrKNttI8Dk57pwThxYJLYpDEhDatb8_4amKO5dJoR4O8HAHGGn8jVpIVajTVY8eCPjeKRM5t_Aggqg9fArFKgIVqXzO8ZDcbE0u4pVmB7Bjtsv-wrwcbyaJ9EoGIkN3o6xZiUODixQ5gTySSUTNdK82w6pgxS0-z0DqPuQlZN3SIDAKTZ9mDWrNRfKjvDn87afEIg7cFXg0Fi82r1O1F1jh0GPPhHK6RRionBitolD-t_x3Duw-DtzHw", alt: "User avatar" }
                                div { class: "flex-1",
                                    form {
                                        onsubmit: |e| e.prevent_default(),
                                        textarea { class: "w-full bg-surface-dark border border-border-dark rounded-lg p-3 text-text-light placeholder-gray-500 focus:ring-primary-light focus:border-primary-light transition", placeholder: "Add a comment...", rows: "4" }
                                        div { class: "mt-4 flex justify-end",
                                            button { class: "inline-flex items-center justify-center gap-2 bg-primary-light text-text-dark font-bold text-sm px-4 py-2 rounded-md hover:opacity-90 transition-opacity", r#type: "submit",
                                                "Post Comment"
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "flex flex-col gap-8",
                                div { class: "flex items-start gap-4",
                                    img { class: "size-10 rounded-full", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAKTYnKgoWMM15UTvOugoMXvYWs9d-Yo9RCvP6v_ilmnnp8-_OLVqoz1-1AXhD1nNrSq9Z6DfjYY84gVr6eNvJB9O-GYThPTVr5TKapPERZQYYJqdPdks41NivF_GEpX82s4WZ3YZR39bKzgBc7MnkRyKSpauNcQoLJE1pg6IgE5PeMQOMCD0-4TATNGCc_JqpTcEdqQl_9Xelzn2FMFigdAiJ3_Vlsl9CvsliwUySKm-99ilP7IdYUSYQ0v9A6FapxMTzVqSRGWpI7", alt: "User avatar" }
                                    div { class: "flex-1",
                                        div { class: "flex items-center gap-2",
                                            span { class: "font-bold text-text-heading", "Alice" }
                                            span { class: "text-xs text-gray-400", "2 days ago" }
                                        }
                                        p { class: "mt-2 text-text-light",
                                            "This is an incredible write-up. The performance gains you've achieved by using Rust and Wasm are seriously impressive. I've been considering a similar approach for a project, and this post just convinced me. Great work!"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
