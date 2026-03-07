use crate::{views::Footer, Route};
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { class: "drawer",

            input {
                id: "nav-drawer",
                r#type: "checkbox",
                class: "drawer-toggle",
            }

            div { class: "drawer-content flex flex-col",

                // Top Navbar
                div { class: "navbar bg-black text-white w-full px-6",

                    // Mobile menu button
                    div { class: "flex-none lg:hidden",
                        label {
                            r#for: "nav-drawer",
                            class: "btn btn-square btn-ghost",
                            aria_label: "open sidebar",

                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                class: "inline-block h-6 w-6 stroke-current",

                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M4 6h16M4 12h16M4 18h16",
                                }
                            }
                        }
                    }

                    // Logo
                    div { class: "flex-1 px-2 text-xl font-bold",
                        Link { to: Route::Home {}, "CANDO" }
                    }

                    // Desktop menu
                    div { class: "hidden lg:flex flex-none",
                        ul { class: "menu menu-horizontal gap-6",

                            li {
                                a { href: "/#about", "About us" }
                            }
                            li {
                                Link { to: Route::Blog { id: 1 }, "How does it work?" }
                            }
                        }
                    }
                }

                // Page content
                Outlet::<Route> {}
                Footer {}
            }

            // Mobile drawer sidebar
            div { class: "drawer-side",

                label {
                    r#for: "nav-drawer",
                    class: "drawer-overlay",
                    aria_label: "close sidebar",
                }

                ul { class: "menu bg-black text-white min-h-full w-80 p-6 gap-4",

                    li {
                        Link { to: Route::Home {}, "Home" }
                    }

                    li {
                        a { href: "/#about", "About us" }
                    }
                    li {
                        Link { to: Route::Blog { id: 1 }, "How does it work?" }
                    }
                
                }
            }
        }
    }
}
