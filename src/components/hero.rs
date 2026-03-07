use daisy_rsx::Button;
use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            class: "h-screen w-screen p-4 md:p-12",
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            // img { src: HEADER_SVG, id: "header" }
            h2 { class: "text-2xl md:text-5xl w-4/5",
                "We are a Youth Led organisation that motivates students to recycle"
            }
            h3 { class: "text-sm md:text-xl w-4/5 pt-4",
                "We give 10c Smart Recycling bins to Australian Universities, Colleges and Schools, 100% free"
            }

            div { class: "flex flex-row gap-6 md:gap-12 py-8 w-4/5",
                Button { class: "btn-sm md:btn-lg btn-secondary btn-outline w-1/2 bg-transparent p-6",
                    "Order Now"
                }
                Button { class: "btn-xs md:btn-lg btn-primary btn-outline w-1/2 bg-transparent p-6",
                    "Learn More"
                }
            }
            div { id: "links" }
        }
    }
}
