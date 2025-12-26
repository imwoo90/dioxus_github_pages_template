use crate::views::Footer;
use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div { class: "layout-content-container flex flex-col w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8",
            main { class: "flex flex-col gap-16 md:gap-24 mt-8 md:mt-16",
                section { class: "flex flex-col gap-10",
                    div { class: "flex flex-col gap-4 text-center",
                        h1 { class: "text-white text-4xl md:text-5xl font-bold leading-tight tracking-[-0.033em]", "Get In Touch" }
                        p { class: "text-[#D4D4D4] text-lg font-normal leading-normal max-w-2xl mx-auto",
                            "Have a project in mind, a question about an article, or just want to connect? I'm always open to discussing new opportunities and collaborating on exciting ideas."
                        }
                    }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 md:gap-12",
                        div { class: "md:col-span-2 bg-white/5 p-8 rounded-lg",
                            form { class: "flex flex-col gap-6", onsubmit: |e| e.prevent_default(),
                                h2 { class: "text-white text-2xl font-bold leading-tight tracking-[-0.015em]", "Send a Message" }
                                div { class: "flex flex-col sm:flex-row gap-6",
                                    div { class: "flex flex-col gap-2 w-full",
                                        label { class: "text-sm font-medium text-[#D4D4D4]", r#for: "name", "Your Name" }
                                        input { class: "w-full bg-background-dark border border-white/20 rounded-md h-11 px-4 text-base text-white placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light", id: "name", placeholder: "John Doe", r#type: "text" }
                                    }
                                    div { class: "flex flex-col gap-2 w-full",
                                        label { class: "text-sm font-medium text-[#D4D4D4]", r#for: "email", "Your Email" }
                                        input { class: "w-full bg-background-dark border border-white/20 rounded-md h-11 px-4 text-base text-white placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light", id: "email", placeholder: "john.doe@email.com", r#type: "email" }
                                    }
                                }
                                div { class: "flex flex-col gap-2",
                                    label { class: "text-sm font-medium text-[#D4D4D4]", r#for: "message", "Message" }
                                    textarea { class: "w-full bg-background-dark border border-white/20 rounded-md p-4 text-base text-white placeholder:text-gray-500 focus:ring-primary-light focus:border-primary-light", id: "message", placeholder: "I'd like to discuss...", rows: "6" }
                                }
                                button { class: "flex min-w-[84px] max-w-[480px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-12 px-6 bg-primary-light text-[#1E1E1E] text-base font-bold leading-normal tracking-[0.015em] hover:opacity-90 transition-opacity self-start", r#type: "submit",
                                    span { class: "truncate", "Submit" }
                                }
                            }
                        }
                        div { class: "flex flex-col gap-6",
                            h2 { class: "text-white text-2xl font-bold leading-tight tracking-[-0.015em]", "Contact Information" }
                            div { class: "flex flex-col gap-4",
                                div { class: "flex items-center gap-4",
                                    div { class: "flex items-center justify-center size-10 bg-white/10 rounded-full text-primary-light",
                                        span { class: "material-symbols-outlined", "mail" }
                                    }
                                    div { class: "flex flex-col",
                                        p { class: "text-sm text-gray-400", "Email" }
                                        a { class: "text-base font-medium text-white hover:text-primary-light transition-colors", href: "mailto:contact@rustshorizon.com", "contact@rustshorizon.com" }
                                    }
                                }
                                div { class: "flex items-center gap-4",
                                    div { class: "flex items-center justify-center size-10 bg-white/10 rounded-full text-primary-light",
                                        svg { class: "size-6", fill: "currentColor", view_box: "0 0 24 24", "aria-hidden": "true",
                                            path { clip_rule: "evenodd", d: "M12 2C6.477 2 2 6.477 2 12c0 4.418 2.865 8.168 6.839 9.49.5.092.682-.217.682-.482 0-.237-.009-.868-.014-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.031-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.203 2.398.1 2.651.64.7 1.03 1.595 1.03 2.688 0 3.848-2.338 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.001 10.001 0 0022 12c0-5.523-4.477-10-10-10z", fill_rule: "evenodd" }
                                        }
                                    }
                                    div { class: "flex flex-col",
                                        p { class: "text-sm text-gray-400", "GitHub" }
                                        a { class: "text-base font-medium text-white hover:text-primary-light transition-colors", href: "#", "@rust-developer" }
                                    }
                                }
                                div { class: "flex items-center gap-4",
                                    div { class: "flex items-center justify-center size-10 bg-white/10 rounded-full text-primary-light",
                                        svg { class: "size-6", fill: "currentColor", view_box: "0 0 24 24", "aria-hidden": "true",
                                            path { d: "M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" }
                                        }
                                    }
                                    div { class: "flex flex-col",
                                        p { class: "text-sm text-gray-400", "LinkedIn" }
                                        a { class: "text-base font-medium text-white hover:text-primary-light transition-colors", href: "#", "Jane Doe" }
                                    }
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
