use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        // We can create elements inside the rsx macro with the element name followed by a block of attributes and children.
        div {
            class: "h-screen w-screen",
            // Attributes should be defined in the element before any children
            id: "hero",
            // After all attributes are defined, we can define child elements and components
            // img { src: HEADER_SVG, id: "header" }
            h1 { class: "text-4xl",
                "We are a Youth Led organisation that motivates students to recycle"
            }
            div { id: "links" }
        }
    }
}
