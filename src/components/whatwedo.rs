use dioxus::prelude::*;

#[component]
pub fn WhatWeDo() -> Element {
    rsx! {
        div { id: "about",
                h2 { class: "text-4xl", "What we do & How it works" }
                p {
                        "We build bins that encourage students to recycle. In it's essence, when a student recycles a 10c return n earn can, they get 10c credited to their student account (which could be used to pay for student ammenities like printing or laundry/washing) or instead could get donated to charity. While simple, our solution provides remarkably effective in
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
                div { class: "flex md:flex-row",
                        div {

                                h3 { "Why it works" }
                                ul { class: "",
                                        li {
                                                "Using one or many of our bins make 10c recycling bins more accessible -> increases volume"
                                        }
                                        li {
                                                "It motivates students to recycle because they can either gain immediate value in recycling, or can see an impact in donating to charity -> increases volume"
                                        }
                                        li {
                                                "They are less clunky than other simlar products (no scanning) -> increases volume"
                                        }
                                        li { "Easy to maintain " }
                                        li {
                                                "Our bins are FREE (yes, we're serious!) for any Australian University or school that makes them (we are funded philanthropically and backed by the United Nations)"
                                        }
                                }
                        }
                        div {
                                h3 { "How it benefits" }
                                ul { class: "",
                                        li {
                                                "You make your campus more sustainable, and associate your campus with sustainable practices"
                                        }
                                        li {
                                                "It encourages youth led sustainability programs (volunteering and leadership)."
                                        }
                                        li {
                                                "By ordering a bin, you are taking plastic out of the would-be landfill - each bin is made of recycled, rerecycled plastic."
                                        }
                                        li { " Cleans the means of production" }
                                }
                        }
                }
        }

}
}
