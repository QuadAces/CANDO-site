use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Footer() -> Element {
    rsx! {

        div { class: "w-screen flex md:flex-row justify-between",
                div { class: "w-full",
                        h5 { "Company" }
                        // about us
                        ul {
                                a { href: "/#about", "About us" }
                        }
                                // what we do
                }
                div { class: "w-full",
                        h5 { "Contact" }
                        ul {
                                li { "Phone +61 476 211 867" }
                                li { "Email stuart@stuartcutbush.com" }
                        }
                                // socials + contact
                }
        
        }

}
}
