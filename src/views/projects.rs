use crate::components::*;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "The Workshop",
                    subtitle: "A collection of my work, showcasing the versatility of Rust across the full stack.",
                    centered: Some(false),
                    children: rsx! {}
                }
                Section { class: "px-4 mb-20",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        Card {
                            title: "FerrOS: A Bare-Metal RTOS",
                            description: "A lightweight, real-time operating system for ARM Cortex-M microcontrollers, written entirely in Rust for maximum safety and performance.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC",
                            tags: vec!["bare-metal".to_string(), "embedded".to_string(), "RTOS".to_string()],
                            external_link: "#".to_string(),
                            link_text: "View on GitHub".to_string()
                        }
                        Card {
                            title: "Wasm Data-Viz Engine",
                            description: "A high-performance data visualization library that runs in the browser, powered by Rust compiled to WebAssembly for native-like speed.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu",
                            tags: vec!["webassembly".to_string(), "frontend".to_string(), "data-viz".to_string()],
                            link_to: Route::WasmProject {},
                            link_text: "View Project Details".to_string()
                        }
                        Card {
                            title: "Crabby Sync Mobile",
                            description: "A cross-platform mobile app for secure file synchronization, using a shared Rust core on both iOS and Android for consistent logic and performance.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a",
                            tags: vec!["mobile".to_string(), "cross-platform".to_string()],
                            external_link: "#".to_string(),
                            link_text: "Read Case Study".to_string()
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
