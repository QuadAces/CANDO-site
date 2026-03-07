use crate::{
    views::{Footer, Navbar},
    Route,
};
use dioxus::prelude::*;
use dioxus_material_icons::MaterialIconStylesheet;

#[component]
pub fn Provider() -> Element {
    rsx! {
        MaterialIconStylesheet {}
        Navbar {}
        Outlet::<Route> {}
        Footer {}
}
}
