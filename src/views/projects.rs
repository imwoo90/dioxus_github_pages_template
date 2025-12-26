use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                section { class: "flex flex-col gap-6",
                    div { class: "px-4 pb-2 pt-5",
                        h1 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em]", "Projects" }
                        p { class: "text-[#D4D4D4] mt-2", "A collection of my work, showcasing the versatility of Rust across the full stack." }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        div { class: "bg-white/5 rounded-lg overflow-hidden flex flex-col group",
                            div {
                                class: "w-full aspect-video bg-cover bg-center",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC')"
                            }
                            div { class: "p-6 flex flex-col grow",
                                h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "FerrOS: A Bare-Metal RTOS"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 grow",
                                    "A lightweight, real-time operating system for ARM Cortex-M microcontrollers, written entirely in Rust for maximum safety and performance."
                                }
                                div { class: "flex flex-wrap items-center gap-2 mb-4",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "bare-metal" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "embedded" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "RTOS" }
                                }
                                a { class: "inline-flex items-center gap-2 text-primary-light text-sm font-medium hover:underline", href: "#",
                                    "View on GitHub"
                                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                                }
                            }
                        }
                        Link { to: Route::WasmProject {}, class: "bg-white/5 rounded-lg overflow-hidden flex flex-col group",
                            div {
                                class: "w-full aspect-video bg-cover bg-center",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu')"
                            }
                            div { class: "p-6 flex flex-col grow",
                                h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "Wasm Data-Viz Engine"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 grow",
                                    "A high-performance data visualization library that runs in the browser, powered by Rust compiled to WebAssembly for native-like speed."
                                }
                                div { class: "flex flex-wrap items-center gap-2 mb-4",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "webassembly" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "frontend" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "data-viz" }
                                }
                                div { class: "inline-flex items-center gap-2 text-primary-light text-sm font-medium hover:underline",
                                    "View Project Details"
                                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                                }
                            }
                        }
                         div { class: "bg-white/5 rounded-lg overflow-hidden flex flex-col group",
                            div {
                                class: "w-full aspect-video bg-cover bg-center",
                                style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a')"
                            }
                            div { class: "p-6 flex flex-col grow",
                                h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                    "Crabby Sync Mobile"
                                }
                                p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 grow",
                                    "A cross-platform mobile app for secure file synchronization, using a shared Rust core on both iOS and Android for consistent logic and performance."
                                }
                                div { class: "flex flex-wrap items-center gap-2 mb-4",
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "mobile" }
                                    span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "cross-platform" }
                                }
                                a { class: "inline-flex items-center gap-2 text-primary-light text-sm font-medium hover:underline", href: "#",
                                    "Read Case Study"
                                    span { class: "material-symbols-outlined text-base", "arrow_forward" }
                                }
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
