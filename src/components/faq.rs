use dioxus::prelude::*;

#[component]
pub fn FAQ() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div { class: "h-screen w-screen p-[16%] items-center justify-center",
                div { class: "w-4/5",
                        h1 { class: "text-2xl md:text-5xl w-4/5", "Frequently Asked Questions" }

                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2", checked: true }
                                div { class: "collapse-title font-semibold", "Why are your bins free?" }
                                div { class: "collapse-content text-sm",
                                        "We are backed by the United Nations and other sponsors. Right now, we are more focused on adoption"
                                }
                        }

                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2" }
                                div { class: "collapse-title font-semibold", "How will the credit system work?" }
                                div { class: "collapse-content text-sm",
                                        "Click on \"Forgot Password\" on the login page and follow the instructions sent to your email."
                                }
                        }

                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2" }
                                div { class: "collapse-title font-semibold", "What if there is an issue?" }
                                div { class: "collapse-content text-sm",
                                        "Go to \"My Account\" settings and select \"Edit Profile\" to make changes."
                                }
                        }
                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2" }
                                div { class: "collapse-title font-semibold", "Who takes out the cans?" }
                                div { class: "collapse-content text-sm",
                                        "Go to \"My Account\" settings and select \"Edit Profile\" to make changes."
                                }
                        }
                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2" }
                                div { class: "collapse-title font-semibold",
                                        "How can you afford to give bins away for free?"
                                }
                                div { class: "collapse-content text-sm",
                                        "Go to \"My Account\" settings and select \"Edit Profile\" to make changes."
                                }
                        }
                        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
                                input { r#type: "radio", name: "my-accordion-2" }
                                div { class: "collapse-title font-semibold", "How can I order a bin?" }
                                div { class: "collapse-content text-sm",
                                        "Go to \"My Account\" settings and select \"Edit Profile\" to make changes."
                                }
                        }
                
                }
        }
}
}
