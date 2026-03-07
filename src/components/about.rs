use daisy_rsx::marketing::benefits;
use dioxus::prelude::*;
use dioxus_material_icons::{MaterialIcon, MaterialIconColor};

struct Benefit {
    icon: &'static str,
    description: &'static str,
    title: &'static str,
}

#[component]
pub fn About() -> Element {
    let howItWorks = [
                Benefit {
                        icon: "accessibility",
                        description: "Bins are more accessible -> increases volume",
                        title: "Accessibility"
                },
                Benefit {
                        icon: "double_arrow",
                        description: "It motivates students by gaining immediate value in recycling, or from donating to charity -> increases volume",
                        title: "Motivation"
                },
                Benefit {
                        icon: "done",
                        description: "Less clunky than other similar bins (no scanning) -> increases volume",
                        title: "Ease of use"
                },
                Benefit {
                        icon: "build",
                        description: "Easy to maintain",
                        title: "Maintenance"
                },
                Benefit {
                        icon: "attach_money",
                        description: "Our bins are FREE (yes seriously)! We are backed by the UN.",
                        title: "Cost"
                }
        ];
    let benefits = [
                Benefit {
                        icon: "eco",
                        description: "You make your campus more sustainable, and associate your campus with sustainable practices",
                        title: "Sustainability"
                },
                Benefit {
                        icon: "volunteer_activism",
                        description: "It encourages youth led sustainability programs (volunteering and leadership).",
                        title: "Youth Leadership"
                },
                Benefit {
                        icon: "recycling",
                        description: "By ordering a bin, you are taking plastic out of the would-be landfill - each bin is made of recycled, rerecycled plastic.",
                        title: "Environmental impact"   
                },
                Benefit {
                        icon: "clean_hands",
                        description: " Cleans the means of production",
                        title: "Supply chain impact"
                }
];
    rsx! {
        div { id: "about", class: "w-screen p-[16%]",
                h2 { class: "text-4xl", "What we do & How it works" }
                p {
                        "Our goal is to improve the sustainability across campuses. We do this by building smart bins for 10c cans that immediately credit students' account. This money could be used to pay for student ammenities (like printing or laundry/washing) OR instead could get donated to charity! While simple, our solution provides remarkably effective in
                                                                                                                                                                        a) Massively increasing recycling volume and b) massively increasing a clean chain of recycling process"
                
                // 10 c to student account, pay for student amenities like washing, printing etc
                // why this works -> motivates students adequately -> higher volume
                // makes bins more accessible -> higher volume
                // not clunky to use (like charopy) -> higher volume;
                // easy to maintain
                // self sutaining and philantronipcally funded (its 100% free);
                // benefits ->
                //  associates your campus with sustainability;
                // encourages youth led sustainability programs
                // physically takes plastic out of the environment, (the bin itself is also 100% recyclable and also made from 100% recycled plastic)
                // considerably cleans supply issues
                }
                br {}
                br {}

                div { class: "my-12",
                        h2 { class: "my-6", "How does it work?" }

                        div { class: "flex flex-row gap-6",
                                {howItWorks.iter().map(|benefit| rsx! {
                                        div { class: "card bg-base-100 w-96 shadow-sm",

                                                figure { class: "px-10 pt-10",

                                                        div { class: "bg-gray-200 rounded-2xl w-18 h-18 flex items-center justify-center",

                                                                MaterialIcon { name: benefit.icon, size: 64 }
                                                        }
                                                }

                                                div { class: "card-body items-center text-center",

                                                        h2 { class: "card-title", "{benefit.title}" }

                                                        p { "{benefit.description}" }

                                                        div { class: "card-actions",

                                                                button { class: "btn btn-primary", "Learn More" }
                                                        }
                                                }
                                        }
                                })}
                        }
                }

                div { class: "my-12",
                        h2 { class: "my-6", "How does it work?" }

                        div { class: "flex flex-row gap-6",
                                {benefits.iter().map(|benefit| rsx! {
                                        div { class: "card bg-base-100 w-96 shadow-sm",

                                                figure { class: "px-10 pt-10",

                                                        div { class: "bg-gray-200 rounded-2xl w-18 h-18 flex items-center justify-center",

                                                                MaterialIcon { name: benefit.icon, size: 64 }
                                                        }
                                                }

                                                div { class: "card-body items-center text-center",

                                                        h2 { class: "card-title", "{benefit.title}" }

                                                        p { "{benefit.description}" }

                                                        div { class: "card-actions",

                                                                button { class: "btn btn-primary", "Learn More" }
                                                        }
                                                }
                                        }
                                })}
                        }
                }
        
        }

}
}
