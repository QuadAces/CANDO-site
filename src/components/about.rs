use dioxus::prelude::*;
use dioxus_material_icons::{IconColor, MaterialIcon};

struct Benefit {
    icon: &'static str,
    description: &'static str,
    title: &'static str,
}

#[component]
pub fn About() -> Element {
    let howItWorks = [
            Benefit {
                    icon: "double_arrow",
                    description: "We motivate students by giving an immediate credit to their student accounts for recycling.",
                    title: "Motivating Students"
                },
                Benefit {
                        icon: "accessibility",
                        description: "By making bins more accessible and appealing across campus, we can increase recycling volume.",
                        title: "Super Accessible Bins"
                },
                Benefit {
                        icon: "build",
                        description: "Our bins are made of HDPE and are designed to be super easy to clean, empty and upkeep. Our technology module is plug and play - so any tech problems can be 'hotswapped' out!",
                        title: "Light Work"
                },
                Benefit {
                        icon: "done",
                        description: "Less clunky than other similar 'smart' bins (no scanning)",
                        title: "Simple"
                },
                Benefit {
                        icon: "attach_money",
                        description: "Our bins are FREE (yes seriously)! We are backed by the United Nations.",
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
                        h2 { class: "my-6 text-lg md:text-3xl", "How does it work?" }
                        h4 {
                                "Our bin improves sustainability by a) increasaing volume by being easier to use than similar, traditional 'smart bin' solutions and b) keeping the recycling stream for 10c containers clean. We achieve this by the following:"
                        }
                        div { class: "flex flex-row flex-wrap justify-center pt-8",
                                {howItWorks.iter().map(|benefit| rsx! {
                                        div { class: "w-1/3 shrink-0 shadow-sm py-1",
                                                // adding a margin or gap above will mean 3 cards cannot fit in one row
                                                // instead create another inside container and add margin to that
                                                div { class: "card m-1 h-full",

                                                        figure { class: "px-10 pt-0",

                                                                div { class: "bg-(--color-primary) rounded-2xl w-18 h-18 flex items-center justify-center",

                                                                        MaterialIcon {
                                                                                name: benefit.icon,
                                                                                size: 64,
                                                                                // otherwise formatter messes it up
                                                                                color: (IconColor("var(--color-secondary-content)")),
                                                                        }
                                                                }
                                                        }

                                                        div { class: "card-body items-center",

                                                                h2 { class: "card-title", "{benefit.title}" }

                                                                p { "{benefit.description}" }
                                                        }
                                                }
                                        }

                                })}
                        }
                }
                // chuck some small callout banner here
                div { class: "my-12",
                        h2 { class: "my-6 text-lg md:text-3xl", "The Benefits:" }
                        h4 {
                                "By ordering a bin, you are taking plastic out of the would-be landfill - each bin is made of recycled, rerecycled plastic. You also make your campus more sustainable, and associate your campus with sustainable practices. It encourages youth led sustainability programs (volunteering and leadership). It also considerably cleans the supply chain of recycling by providing a more direct and transparent means of recycling."
                        }
                        div { class: "flex flex-row gap-6",
                                {benefits.iter().map(|benefit| rsx! {
                                        div { class: "card bg-base-100 w-96 shadow-sm",

                                                figure { class: "px-10 pt-10",

                                                        div { class: "bg-gray-200 rounded-2xl w-18 h-18 flex items-center justify-center",

                                                                MaterialIcon {
                                                                        name: benefit.icon,
                                                                        size: 64,
                                                                        // otherwise formatter messes it up
                                                                        color: (IconColor("var(--color-primary)")),
                                                                }
                                                        }
                                                }

                                                div { class: "card-body items-center text-center",

                                                        h2 { class: "card-title", "{benefit.title}" }

                                                        p { "{benefit.description}" }
                                                }
                                        }
                                })}
                        }
                }
        
        }

}
}
