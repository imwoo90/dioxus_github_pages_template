use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-5xl mx-auto px-4 sm:px-6 lg:px-8",
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                div { class: "w-full flex flex-col gap-6 text-center items-center px-4",
                    h1 { class: "text-white text-4xl md:text-5xl font-black leading-tight tracking-[-0.033em]",
                        "From the Horizon"
                    }
                    p { class: "text-[#D4D4D4] text-lg md:text-xl font-normal leading-normal max-w-3xl mx-auto",
                        "Exploring the frontiers of Rust, from bare-metal to the web. Find articles, tutorials, and deep dives here."
                    }
                }
                section { class: "flex flex-col md:flex-row gap-4 px-4 items-center",
                    div { class: "relative w-full md:flex-1",
                        span { class: "material-symbols-outlined absolute left-3 top-1/2 -translate-y-1/2 text-gray-400", "search" }
                        input {
                            class: "w-full bg-[#2a2a2a] border border-white/10 rounded-md h-12 pl-10 pr-4 text-white placeholder:text-gray-400 focus:ring-primary-light focus:border-primary-light",
                            placeholder: "Search articles...",
                            r#type: "text",
                        }
                    }
                    div { class: "flex flex-wrap gap-2 justify-center",
                        button { class: "px-4 py-2 text-sm font-medium rounded-md bg-white/10 text-white hover:bg-white/20 transition-colors", "All" }
                        button { class: "px-4 py-2 text-sm font-medium rounded-md bg-transparent text-gray-400 hover:bg-white/10 hover:text-white transition-colors", "bare-metal" }
                        button { class: "px-4 py-2 text-sm font-medium rounded-md bg-transparent text-gray-400 hover:bg-white/10 hover:text-white transition-colors", "mobile" }
                        button { class: "px-4 py-2 text-sm font-medium rounded-md bg-transparent text-gray-400 hover:bg-white/10 hover:text-white transition-colors", "web" }
                        button { class: "px-4 py-2 text-sm font-medium rounded-md bg-transparent text-gray-400 hover:bg-white/10 hover:text-white transition-colors", "backend" }
                    }
                }
                section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
                    div { class: "flex flex-col rounded-lg overflow-hidden bg-[#2a2a2a] border border-white/10 group",
                        div {
                            class: "w-full aspect-video bg-cover bg-center",
                            style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBCi_pzNBVAhdhyYbqWap1-J9p4C5e7Qg6V8ZzF0E8aKgg5gcyH6nlx6u8TcJznHXhdlE6wILKxlF_rVI-dsKeRThWqbJXnybVHkiS879RY2kqgOxaj8lGDFgqgIlrbsxFIjg_SsQ7ddoeF67K9JgN6ZeqobfwkcRHjFnG2BbRCumKTZTwKS1bcPJuFg6X5cEc4popXC2x7h-Hg6_A_WJ1ZEyEQg9cnRfLTod-WujsdskNkUlBc-SRxRmTO7k0sa4JyR-4QUQ4or0YC')"
                        }
                        div { class: "p-6 flex flex-col flex-grow",
                            h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                "Bare-Metal Rust: Blinking an LED"
                            }
                            p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 flex-grow",
                                "A deep dive into writing Rust for microcontrollers, starting from the ground up without any standard library."
                            }
                            div { class: "flex items-center gap-2 mb-4",
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "bare-metal" }
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "embedded" }
                            }
                            Link {
                                to: Route::BlogPost { id: "post-1".to_string() },
                                class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                                "Read More "
                                span { class: "material-symbols-outlined text-base", "arrow_forward" }
                            }
                        }
                    }
                    div { class: "flex flex-col rounded-lg overflow-hidden bg-[#2a2a2a] border border-white/10 group",
                        div {
                            class: "w-full aspect-video bg-cover bg-center",
                            style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuAI1Cr-l7Wo5cgHt_zfG7BK8WvHPYWk5oEm-9oEQ6_MylTr-gO7ZrldH3pUrirQ5dYe4yZDhwcV-arSM4h3WA2urB99awKGkr9SLyWeteJEQExthZdm_fK0mqi3c95QNudHSRPVSZIyywMm-LADZqGhXreY55_EqUksKyzJXbGh43v6TWyfjFjAPn1a4OPM0KYZ-1joKgoI6uEnbR-6-cn4GYPzcL8ari8x_9XuWah3PJcoY7eqIyd4R-RNA0bwyrgkJRXbMCMI7Qeu')"
                        }
                        div { class: "p-6 flex flex-col flex-grow",
                            h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                "Compiling Rust to WebAssembly"
                            }
                            p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 flex-grow",
                                "Learn how to leverage Rust's performance and safety in the browser by compiling it to WebAssembly."
                            }
                            div { class: "flex items-center gap-2 mb-4",
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "web" }
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "frontend" }
                            }
                            Link {
                                to: Route::WasmProject {},
                                class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                                "Read More "
                                span { class: "material-symbols-outlined text-base", "arrow_forward" }
                            }
                        }
                    }
                    div { class: "flex flex-col rounded-lg overflow-hidden bg-[#2a2a2a] border border-white/10 group",
                        div {
                            class: "w-full aspect-video bg-cover bg-center",
                            style: "background-image: url('https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a')"
                        }
                        div { class: "p-6 flex flex-col flex-grow",
                            h3 { class: "text-white text-xl font-bold leading-tight tracking-[-0.015em] group-hover:text-primary-light transition-colors",
                                "Building Native Mobile Apps with Rust"
                            }
                            p { class: "text-[#D4D4D4] text-base font-normal leading-normal mt-2 mb-4 flex-grow",
                                "Exploring how to create cross-platform native mobile applications using a Rust backend and web-based frontend."
                            }
                            div { class: "flex items-center gap-2 mb-4",
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "mobile" }
                                span { class: "text-xs font-mono bg-gray-700/50 text-gray-300 px-2 py-1 rounded", "tauri" }
                            }
                            a {
                                class: "text-primary-light text-sm font-semibold hover:underline flex items-center gap-1",
                                href: "#",
                                "Read More "
                                span { class: "material-symbols-outlined text-base", "arrow_forward" }
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn BlogPost(id: String) -> Element {
    // In a real app, we would fetch the post based on ID.
    // Here we hardcode the specific post content provided in the example for all IDs,
    // or just checking if id == "post-1".

    use_effect(move || {
         // This is a browser-only effect to trigger highlight.js
         // Dioxus 0.6+ often handles this differently but for prototype this is fine if web-sys is available
         // or we just inject script.
         // Since I added the script in index, window.hljs might be available.
         // For now, I'll rely on the script being present.
         // JS eval is needed to call highlightAll.
         document::eval("if (window.hljs) window.hljs.highlightAll();");
    });

    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
            article { class: "w-full max-w-3xl flex flex-col gap-10",
                header { class: "flex flex-col gap-6 border-b border-white/5 pb-10",
                    div { class: "flex items-center gap-2 text-sm text-gray-500 font-medium",
                        Link { class: "hover:text-primary-light transition-colors", to: Route::BlogList {}, "Blog" }
                        span { "›" }
                        span { class: "truncate text-gray-400", "Bare-Metal Rust" }
                    }
                    h1 { class: "text-white text-4xl sm:text-5xl font-extrabold leading-tight tracking-tight",
                        "Bare-Metal Rust: Blinking an LED"
                    }
                    div { class: "flex items-center gap-4 text-sm text-gray-400",
                        span { class: "font-medium text-primary-light", "John Doe" }
                        span { class: "text-gray-600", "•" }
                        time { datetime: "2023-10-26", "October 26, 2023" }
                        span { class: "text-gray-600", "•" }
                        span { "5 min read" }
                    }
                }
                div { class: "prose max-w-none",
                    p {
                        "Welcome to the fascinating world of bare-metal programming with Rust! In this post, we'll embark on a journey to the very foundations of embedded systems. Our goal is simple but fundamental: to blink an LED on a microcontroller using Rust, without relying on any operating system or standard library (no-std). This is the \"Hello, World!\" of the embedded world, and it's a perfect starting point to understand what makes Rust a powerful choice for low-level development."
                    }
                    h2 { "Why Bare-Metal Rust?" }
                    p {
                        "Rust's emphasis on safety, performance, and concurrency makes it an excellent candidate for embedded systems. Its zero-cost abstractions mean you don't pay for what you don't use, which is critical in resource-constrained environments. The powerful type system and ownership model help prevent common bugs like null pointer dereferences and data races at compile time, saving hours of debugging on hardware."
                    }
                    pre {
                        code { class: "language-rust",
                            "#[entry]
fn main() -> ! {{
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
    loop {{
        led.set_high().unwrap();
        // Delay for a bit
        led.set_low().unwrap();
        // Delay for a bit
    }}
}}"
                        }
                    }
                    p {
                        "The code snippet above shows a simplified version of what we're aiming for. We take control of the device peripherals, configure a specific GPIO pin (in this case, PC13, which is often connected to an onboard LED on STM32 boards) as an output, and then enter an infinite loop to toggle its state."
                    }
                    h2 { "Setting Up the Environment" }
                    p {
                        "Before we can blink, we need to set up our development environment. This involves installing the Rust compiler, adding the correct target for our microcontroller (e.g., ",
                        code { "thumbv7em-none-eabihf" },
                        " for an STM32F4 series), and getting the necessary tools for flashing and debugging, like ",
                        code { "probe-rs" },
                        " or ",
                        code { "openocd" },
                        "."
                    }
                    blockquote {
                        p {
                            "This tutorial assumes you are using an STM32 \"Blue Pill\" board, but the concepts are transferable to other microcontrollers with their respective Hardware Abstraction Layer (HAL) crates."
                        }
                    }
                    p {
                        "Once the environment is ready, we'll create a new no-std Rust project and add the required dependencies to our ",
                        code { "Cargo.toml" },
                        ". This will include the Peripheral Access Crate (PAC) for our specific MCU, and a higher-level Hardware Abstraction Layer (HAL) crate that provides safer and more ergonomic APIs to control the hardware."
                    }
                }
                div { class: "mt-8 border-t border-white/5 pt-8",
                     div { class: "flex flex-col sm:flex-row justify-between items-center gap-4 bg-white/5 p-6 rounded-xl",
                        h3 { class: "text-lg font-bold text-white", "Share this article" }
                        div { class: "flex items-center gap-3",
                            button { class: "text-gray-400 hover:text-white hover:bg-[#1DA1F2]/20 p-2.5 rounded-lg transition-all",
                                span { class: "material-symbols-outlined", "share" }
                            }
                        }
                     }
                }
            }
        }
        Footer {}
    }
}
