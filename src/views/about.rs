use crate::views::Footer;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                section { class: "flex flex-col md:flex-row items-center gap-8 md:gap-12",
                    div { class: "w-48 h-48 md:w-60 md:h-60 flex-shrink-0",
                        img {
                            class: "w-full h-full rounded-full object-cover border-4 border-primary-light/50 shadow-lg",
                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAPY0CCVN_4GL3-hGi_oTrltehQY07eSFKotu0R7VEFER7S3HpAsYPIVhi9tPeKSEfsqsbJDH-pUOecDZ1_9LKTtOUxtVEzozK1Mg5PTH34O5gL4fQqpGTXDDdohpSDkoCMHma6B6i_LW633qtLmpDbSQRBMyRE_h90oqvCdv2jK4_ToQG3ZlHKwnorWLecafDGcNgiVkbjeGfDCyqoR6ADi6nytdDEqrwSVszYU0i1h-BRbHH4V5fdVO9QZZq5i0QXVNIKKabPaWwV",
                            alt: "Professional headshot of the developer"
                        }
                    }
                    div { class: "flex flex-col gap-4 text-center md:text-left",
                        h1 { class: "text-white text-4xl md:text-5xl font-black leading-tight tracking-[-0.033em]",
                            "Hi, I'm Alex."
                        }
                        p { class: "text-lg md:text-xl font-normal leading-normal text-[#D4D4D4]",
                            "I'm a full-stack embedded developer with a singular passion: leveraging the power of Rust to build robust, efficient, and secure software across every conceivable platform. From the tight constraints of bare-metal microcontrollers to the vast scale of cloud backends, I believe Rust is the key to a new era of reliable systems."
                        }
                    }
                }
                section { class: "grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12",
                    div { class: "flex flex-col gap-4",
                        h2 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em] border-b border-white/10 pb-3",
                            "My Philosophy"
                        }
                        p { class: "text-base font-normal leading-relaxed text-[#D4D4D4]",
                            "The \"Rust-for-everything\" philosophy isn't just a technical preference; it's a commitment to quality. It means applying the principles of memory safety, zero-cost abstractions, and fearless concurrency to every layer of the stack. This approach minimizes bugs, maximizes performance, and creates software that is a pleasure to maintain and extend, whether it's firmware for a tiny IoT device or a high-traffic web service."
                        }
                    }
                    div { class: "flex flex-col gap-4",
                        h2 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em] border-b border-white/10 pb-3",
                            "Core Skills"
                        }
                        div { class: "flex flex-wrap gap-3",
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Embedded Rust" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Bare-Metal Firmware" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "RTOS Integration" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "WebAssembly (WASM)" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Async Rust (Tokio)" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Backend APIs (axum)" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Cross-Platform Mobile" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "CI/CD & DevOps" }
                            span { class: "text-sm font-mono bg-gray-700/50 text-gray-300 px-3 py-1.5 rounded-lg", "Linux Systems" }
                        }
                    }
                }
                section { class: "flex flex-col gap-8",
                    h2 { class: "text-white text-3xl font-bold leading-tight tracking-[-0.015em] border-b border-white/10 pb-3",
                        "My Journey"
                    }
                    div { class: "relative pl-6 border-l-2 border-primary-light/30",
                        div { class: "mb-10 ml-4",
                            div { class: "absolute w-4 h-4 bg-primary-light rounded-full mt-1.5 -left-2.5 border border-background-dark" }
                            time { class: "mb-1 text-sm font-normal leading-none text-gray-400", "2022 - Present" }
                            h3 { class: "text-lg font-semibold text-white", "Lead Embedded Engineer, Innovatech Dynamics" }
                            p { class: "text-base font-normal text-[#D4D4D4]",
                                "Architected and developed a new generation of IoT devices, migrating the entire firmware from C to Rust. Achieved a 40% reduction in memory usage and eliminated a whole class of memory corruption bugs."
                            }
                        }
                        div { class: "mb-10 ml-4",
                            div { class: "absolute w-4 h-4 bg-primary-light rounded-full mt-1.5 -left-2.5 border border-background-dark" }
                            time { class: "mb-1 text-sm font-normal leading-none text-gray-400", "2020 - 2022" }
                            h3 { class: "text-lg font-semibold text-white", "Full-Stack Developer, QuantumLeap Solutions" }
                            p { class: "text-base font-normal text-[#D4D4D4]",
                                "Built high-performance backend services in Rust and explored its potential for frontend development using WebAssembly, creating interactive data visualization tools that ran entirely in the browser."
                            }
                        }
                        div { class: "mb-10 ml-4",
                            div { class: "absolute w-4 h-4 bg-primary-light rounded-full mt-1.5 -left-2.5 border border-background-dark" }
                            time { class: "mb-1 text-sm font-normal leading-none text-gray-400", "2018" }
                            h3 { class: "text-lg font-semibold text-white", "The Spark: Discovering Rust" }
                            p { class: "text-base font-normal text-[#D4D4D4]",
                                "While working on a complex C++ project, I discovered Rust. Its promise of safety without sacrificing performance was a revelation that set the course for my entire career."
                            }
                        }
                        div { class: "ml-4",
                            div { class: "absolute w-4 h-4 bg-primary-light rounded-full mt-1.5 -left-2.5 border border-background-dark" }
                            time { class: "mb-1 text-sm font-normal leading-none text-gray-400", "2016 - 2018" }
                            h3 { class: "text-lg font-semibold text-white", "Firmware Engineer, Core Systems Inc." }
                            p { class: "text-base font-normal text-[#D4D4D4]",
                                "My professional journey began here, writing C and C++ for industrial control systems. It was here I learned the criticality of robust, reliable code in resource-constrained environments."
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}
