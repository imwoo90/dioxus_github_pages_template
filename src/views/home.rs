use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                div { class: "w-full @container",
                    div { class: "flex flex-col gap-6 text-center items-center px-4 py-10",
                        div { class: "flex flex-col gap-4",
                            h1 { class: "text-white text-5xl md:text-6xl font-black leading-tight tracking-[-0.033em]",
                                "Rust's Horizon"
                            }
                            h2 { class: "text-[#D4D4D4] text-lg md:text-xl font-normal leading-normal max-w-3xl mx-auto",
                                "Navigating the Rust ecosystem from silicon to screen. I document the journey of building high-performance software across the entire stack—spanning bare-metal MCUs, backend services, and native mobile apps."
                            }
                        }
                        Link {
                            to: Route::BlogList {},
                            class: "flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-6 bg-primary-light text-[#1E1E1E] text-base font-bold leading-normal tracking-[0.015em] hover:opacity-90 transition-opacity",
                            span { class: "truncate", "Explore the Blog" }
                        }
                    }
                }
                section { class: "flex flex-col gap-6",
                    h2 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em] px-4 pb-2 pt-5 border-b border-white/10",
                        "Latest Articles"
                    }

                    div { class: "p-4 @container group",
                        Link { to: Route::BlogPost { id: "post-1".to_string() }, class: "flex flex-col items-stretch justify-start rounded-lg transition-all duration-300 group-hover:bg-white/5 @xl:flex-row @xl:items-start gap-6",
                            div {
                                class: "w-full @xl:w-1/3 aspect-video bg-cover bg-center rounded-lg",
                                "data-alt": "Abstract gradient representing embedded systems",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC')"
                            }
                            div { class: "flex w-full min-w-0 grow flex-col items-stretch justify-center gap-2 @xl:w-2/3",
                                p { class: "text-gray-400 text-sm font-normal leading-normal", "October 26, 2023" }
                                p { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "Bare-Metal Rust: Blinking an LED with Zero Abstractions"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal",
                                    "A deep dive into writing Rust for microcontrollers, starting from the ground up without any standard library."
                                }
                                div { class: "flex items-center gap-2 pt-2",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "bare-metal" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "embedded" }
                                }
                            }
                        }
                    }
                    hr { class: "border-white/10 mx-4" }
                    div { class: "p-4 @container group",
                        Link { to: Route::WasmProject {}, class: "flex flex-col items-stretch justify-start rounded-lg transition-all duration-300 group-hover:bg-white/5 @xl:flex-row @xl:items-start gap-6",
                            div {
                                class: "w-full @xl:w-1/3 aspect-video bg-cover bg-center rounded-lg",
                                "data-alt": "Abstract code on a screen representing WebAssembly",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu')"
                            }
                            div { class: "flex w-full min-w-0 grow flex-col items-stretch justify-center gap-2 @xl:w-2/3",
                                p { class: "text-gray-400 text-sm font-normal leading-normal", "September 15, 2023" }
                                p { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "Compiling Rust to WebAssembly: A Practical Guide"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal",
                                    "Learn how to leverage Rust's performance and safety in the browser by compiling it to WebAssembly."
                                }
                                div { class: "flex items-center gap-2 pt-2",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "webassembly" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "frontend" }
                                }
                            }
                        }
                    }
                    hr { class: "border-white/10 mx-4" }
                    div { class: "p-4 @container group",
                        div { class: "flex flex-col items-stretch justify-start rounded-lg transition-all duration-300 group-hover:bg-white/5 @xl:flex-row @xl:items-start gap-6",
                            div {
                                class: "w-full @xl:w-1/3 aspect-video bg-cover bg-center rounded-lg",
                                "data-alt": "Smartphone displaying code, representing mobile development",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a')"
                            }
                            div { class: "flex w-full min-w-0 grow flex-col items-stretch justify-center gap-2 @xl:w-2/3",
                                p { class: "text-gray-400 text-sm font-normal leading-normal", "August 02, 2023" }
                                p { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "Building Native Mobile Apps with Rust and Tauri"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal",
                                    "Exploring how to create cross-platform native mobile applications using a Rust backend and web-based frontend with Tauri Mobile."
                                }
                                div { class: "flex items-center gap-2 pt-2",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "mobile" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "tauri" }
                                }
                            }
                        }
                    }
                }
                section { class: "flex flex-col items-center text-center gap-4 bg-white/5 p-8 sm:p-12 rounded-lg",
                    h2 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em]",
                        "Let's Build Together"
                    }
                    p { class: "text-[#D4D4D4] text-base font-normal leading-normal max-w-2xl",
                        "I'm passionate about tackling challenging projects with Rust. If you're looking for a developer with deep experience in embedded systems, performance optimization, and cross-platform development, let's talk."
                    }
                    Link {
                        to: Route::Contact {},
                        class: "text-primary-light text-lg font-medium hover:underline",
                        "contact@rustshorizon.com"
                    }
                }
            }
        }
        Footer {}
    }
}
