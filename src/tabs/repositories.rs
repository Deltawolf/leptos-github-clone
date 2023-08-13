use leptos::{*, ev::MouseEvent};
use crate::components::*;

#[component]
pub fn Repository(cx: Scope) -> impl IntoView {

    let num: Vec<i32> = (1..=10).collect();
    view!{cx,
        <div class="grid grid-cols-1 gap-y-4 w-full">
        {num.into_iter().map(|_| view!{cx,
                <RepositoryRepo />
            })
            .collect::<Vec<_>>()
        }
        </div>
    }
}

#[component]
pub fn RepositoryRepo(cx: Scope) -> impl IntoView {

    use rand::Rng;

    let icons = Octicons::new();
    let repositories_icon = icons.get_icon("repositories").unwrap();
    let stars_icon = icons.get_icon("stars").unwrap();
    let stars1_icon = icons.get_icon("stars").unwrap();
    let stars_filled_icon = icons.get_icon("stars-filled").unwrap();
    let fork_icon = icons.get_icon("fork").unwrap();
    let triangle_down_icon = icons.get_icon("triangle-down").unwrap();
    let edge = 12;
    let lines = vec!["/assets/sparklines/sp1.svg", "/assets/sparklines/sp2.svg", "/assets/sparklines/sp3.svg", "/assets/sparklines/sp4.svg", "/assets/sparklines/sp5.svg"];
    

    let mut rng = rand::thread_rng();
    let sparkline = lines[rng.gen_range(0..5)];
    let (star_fill, set_star_fill) = create_signal(cx, String::from(""));
    let on_click = move |_: MouseEvent| 
        set_star_fill.update(|status|
            if *status == "" {
                *status = String::from("gold"); 
            } else {
                *status=String::from("");}
        );
    let is_star_filled = move || 
        if star_fill().as_str() == "gold" {
            stars_filled_icon.clone()
        } else {
            stars1_icon.clone()
        };



    view! {cx,

       
        <div class="repo-card flex justify-between">
            <div class="flex flex-col p-4 items-start w-3/4">

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
                        <Octicon edge_length=edge paths=vec![stars_icon.clone()]/>
                        <span class="">11.2k</span>
                    </div>
                    <div class="inline-flex gap-1 items-center">
                        <Octicon edge_length=edge paths=vec![fork_icon]/>
                        <span class="">383</span>
                    </div>
                </div>
            </div>
            <div class="flex flex-col justify-center items-end gap-8 w-1/5 text-xs">
                <div class="flex flex-row w-full justify-end">
                    <button on:click=on_click class="content-button flex justify-around rounded-e-none w-1/2">
                        <svg height=16 width=16>
                            <path d=is_star_filled fill-rule="nonzero" fill=move || star_fill()/>
                        </svg>
                    Star</button>
                    <button class="content-button rounded-s-none w-1/5">
                        <Octicon edge_length=16 paths=vec![triangle_down_icon]/>
                    </button>
                </div>
                <img class="h-1/3 self-end" src=sparkline />
            </div>
        </div>

    }

}

