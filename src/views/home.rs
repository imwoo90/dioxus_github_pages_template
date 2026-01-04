use crate::components::*;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        Container {
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                Hero {
                    title: "Rust's Horizon",
                    subtitle: "Navigating the Rust ecosystem from silicon to screen. I document the journey of building high-performance software across the entire stack—spanning bare-metal MCUs, backend services, and native mobile apps.",
                    PrimaryButton {
                        to: Route::BlogList {},
                        text: "Explore the Blog"
                    }
                }

                Section {
                    SectionTitle { title: "Latest Articles" }

                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                        Card {
                            title: "Bare-Metal Rust: Blinking an LED",
                            description: "A deep dive into writing Rust for microcontrollers, starting from the ground up without any standard library.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC",
                            tags: vec!["bare-metal".to_string(), "embedded".to_string()],
                            link_to: Route::BlogPost { id: "post-1".to_string() }
                        }
                        Card {
                            title: "Compiling Rust to WebAssembly",
                            description: "Learn how to leverage Rust's performance and safety in the browser by compiling it to WebAssembly.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu",
                            tags: vec!["webassembly".to_string(), "frontend".to_string()],
                            link_to: Route::WasmProject {}
                        }
                        Card {
                            title: "Building Native Mobile Apps with Rust",
                            description: "Exploring how to create cross-platform native mobile applications using a Rust backend and web-based frontend with Tauri Mobile.",
                            image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a",
                            tags: vec!["mobile".to_string(), "tauri".to_string()],
                            external_link: "#".to_string()
                        }
                    }
                }

                section { class: "flex flex-col items-center text-center gap-6 bg-white/5 p-8 sm:p-12 rounded-lg transition-colors border border-white/5",
                    SectionTitle { title: "Let's Build Together" }
                    p { class: "text-text-dark/80 dark:text-[#D4D4D4] text-base font-normal leading-normal max-w-2xl",
                        "I'm passionate about tackling challenging projects with Rust. If you're looking for a developer with deep experience in embedded systems, performance optimization, and cross-platform development, let's talk."
                    }
                    PrimaryButton {
                        to: Route::Contact {},
                        text: "Get in Touch"
                    }
                }
            }
        }
        Footer {}
    }
}
