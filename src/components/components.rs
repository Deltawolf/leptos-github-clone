use leptos::*;

#[component]
pub fn Octicon(cx: Scope, 
           edge_length: i32, 
           paths: Vec<String>,
           ) -> impl IntoView {


    view! {cx, 
        <svg fill="currentColor" viewBox="0 0 16 16" height=edge_length width=edge_length>

        {paths.into_iter()
            .map(|defined_path| view! {cx,
                <path d=defined_path></path>
            })
            .collect::<Vec<_>>()
        }
            
        </svg>
    }

    
}

#[component]
pub fn Avatar(cx: Scope, source: String) -> impl IntoView {

    view! {cx,
        <div class="header-item m-0">
            <button class="header-button overflow-hidden p-0 m-0 rounded-full w-9 h-9">
                <img class="w-full h-full object-center" src=source.clone()/>
            </button>
        </div>
    }

}
