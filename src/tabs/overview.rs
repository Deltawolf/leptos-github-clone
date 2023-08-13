use leptos::*;
use crate::components::*;


#[component]
pub fn Overview(cx: Scope) -> impl IntoView {

    view!{cx,
        <div id="container" class="flex flex-col">
            <span>Pinned</span>
            <div id="repos">
                <div class="flex w-full gap-4">
                    <OverviewRepo/>
                    <OverviewRepo/>
                </div>
                <div class="flex w-full gap-4 mt-4">
                    <OverviewRepo/>
                    <OverviewRepo/>
                </div>
            </div>
            <div id="calendar" class="mt-8 w-full">
                <span> 2,553 contributions in the last year</span>
                <img src="https://cdn.discordapp.com/attachments/923050495327744071/1138933480810627233/image.png"/>
            </div>
            <div id="contribution-timeline">
            </div>

        </div>
    }

}

//<Octicon edge_length=edge paths=vec![issues_icon_2]/>
#[component]
pub fn OverviewRepo(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let repositories_icon = icons.get_icon("repositories").unwrap();
    let stars_icon = icons.get_icon("stars").unwrap();
    let fork_icon = icons.get_icon("fork").unwrap();
    let edge = 12;


    view! {cx,

        
        <button class="header-button flex flex-col p-4 w-full items-start">

            <div class="inline-flex gap-2 items-center">
                <Octicon edge_length=16 paths=vec![repositories_icon]/>
                <span class="text-[#0969da]">leptos-rs/<b>leptos</b></span>
                <span class="header-button rounded-full text-xs">Public</span>
            </div>
        <span class="text-xs text-[#656d76] mt-4">Build fast web applications with Rust.</span>
        <div class="inline-flex gap-4 mt-4 text-[#656d76] text-xs">
            <div class="inline-flex gap-1 items-center">
                <svg height=edge width=edge>
                <circle cx="6" cy="6" r="6" fill="#dea584"/>
                </svg>
                <span class="">Rust</span>
            </div>
            <div class="inline-flex gap-1 items-center">
                <Octicon edge_length=edge paths=vec![stars_icon]/>
                <span class="">11.2k</span>
            </div>
            <div class="inline-flex gap-1 items-center">
                <Octicon edge_length=edge paths=vec![fork_icon]/>
                <span class="">383</span>
            </div>

        </div>
        </button>

    }

}
