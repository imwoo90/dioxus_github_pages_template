use crate::components::*;
use crate::views::Footer;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BlogList() -> Element {
    rsx! {
        Container {
            main { class: "flex flex-col gap-12 mt-8 md:mt-16",
                Hero {
                    title: "From the Horizon",
                    subtitle: "Exploring the frontiers of Rust, from bare-metal to the web. Find articles, tutorials, and deep dives here.",
                }

                // Filter / Search Section
                section { class: "flex flex-col md:flex-row gap-4 px-4 items-center",
                    BlogSearch { placeholder: "Search articles..." }
                    BlogCategories {
                        categories: vec!["bare-metal".to_string(), "mobile".to_string(), "web".to_string(), "backend".to_string()],
                        active: "All".to_string()
                    }
                }

                Section { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
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
                        tags: vec!["web".to_string(), "frontend".to_string()],
                        link_to: Route::WasmProject {}
                    }
                    Card {
                        title: "Building Native Mobile Apps with Rust",
                        description: "Exploring how to create cross-platform native mobile applications using a Rust backend and web-based frontend.",
                        image_url: "https://lh3.googleusercontent.com/aida-public/AB6AXuBw0O5mBTIGcNEI5F7O7cvNFLBqR3vtVXX0xpBIqdxMjQzqsflRIpLkWfHeYCzI6JbOoWtv8PgIDgXtmk1807iK6WPBXbslbZdBkFcanHrmA96bfNqegGjfk7wDF_Fgn7ElE5a0jPvGvSK1_wtTHkbPQQoK4e-SABmKm54PP006iDJN_lfNqiAhZmVUi5cWTvpPT-VFpEG77Ne324il3ltYhpF1B-kqpgMWymESqPg9jer8niQofq_Q8vPoEPCPZvuSOYEHJklGZa8a",
                        tags: vec!["mobile".to_string(), "tauri".to_string()],
                        external_link: "#".to_string()
                    }
                }
            }
        }
        Footer {}
    }
}

#[component]
pub fn BlogPost(id: String) -> Element {
    use_effect(move || {
        document::eval("if (window.hljs) window.hljs.highlightAll();");
    });

    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-16",
            article { class: "w-full max-w-3xl flex flex-col gap-10",
                BlogHero {
                    title: "Bare-Metal Rust: Blinking an LED",
                    author: "John Doe",
                    date: "October 26, 2023",
                    read_time: "5 min read"
                }

                div { class: "prose max-w-none dark:prose-invert",
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

                // Share Section
                div { class: "mt-8 border-t border-text-dark/5 dark:border-white/5 pt-8",
                     div { class: "flex flex-col sm:flex-row justify-between items-center gap-4 bg-text-dark/5 dark:bg-white/5 p-6 rounded-xl",
                        h3 { class: "text-lg font-bold text-text-dark dark:text-white", "Share this article" }
                        div { class: "flex items-center gap-3",
                            button {
                                class: "text-text-dark/40 dark:text-gray-400 hover:text-text-dark dark:hover:text-white hover:bg-text-dark/10 dark:hover:bg-[#1DA1F2]/20 p-2.5 rounded-lg transition-all",
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
