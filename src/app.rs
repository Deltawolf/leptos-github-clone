use leptos::{*, ev::MouseEvent};
use leptos_meta::*;
use leptos_router::*;
use crate::octicons::octicons::Octicons;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusthub.css"/>

        // sets the document title
        <Title text="RustHub"/>

        // content for this welcome page
        <Router>
            <main>
                <AppHeader/>
                <div class="max-w-screen-xl mt-16 mx-auto ">
                    <div class="main flex gap-16">
                        <LayoutSidebar />
                        <Routes>
                            <Route path="/" view=Overview>
                                <Route path="/:id" view=Overview/>
                            </Route>
                        </Routes>
                    </div>
                    <AppFooter />
                </div>
            </main>
        </Router>
    }
}



/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    view! { cx,
        <AppHeader/>
        <ContentPanel/>
    }
}


//Can I use routes to change the USERNAME? www.github.com/username 
#[component]
fn AppHeader(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let menu_icon = icons.get_icon("menu").unwrap();
    let logo_icon = icons.get_icon("logo").unwrap();
    let search_icon = icons.get_icon("search").unwrap();
    let command_icon = icons.get_icon("command-palette").unwrap();
    let plus_icon = icons.get_icon("plus").unwrap();
    let triangle_down_icon = icons.get_icon("triangle-down").unwrap();
    let notifications_icon = icons.get_icon("notifications").unwrap();
    let issues_icon_1 = icons.get_icon("issues-1").unwrap();
    let issues_icon_2 = icons.get_icon("issues-2").unwrap();
    let pull_requests_icon = icons.get_icon("pull-requests").unwrap();

    let params = use_params_map(cx);

    let username = move|| {
        params.with(|params| params.get("id").cloned())
    };
    
    view! { cx,

        <header class="AppHeader font-sans leading-normal text-sm shadow-inner bg-[#f6f8fa]">
            <div class="AppHeader-globalBar pb-2 flex justify-between p-4 gap-4">
                <div class="AppHeader-globalBar-start flex gap-2 width-full">
                    <deferred-side-panel>
                        <button class="header-button">
                            <Octicon edge_length=16 paths=vec![menu_icon]/>
                        </button>
                    </deferred-side-panel>
                    <a class="AppHeader-logo ml-2">
                        <Octicon edge_length=32
                        paths=vec![logo_icon]/>
                    </a>
                    <div class="AppHeader-Context flex flex-col justify-center">
                        <nav role="navigation">
                            <span class="AppHeader-context-item-label font-bold">
                            {username}
                            </span>
                        </nav>
                    </div>

                </div>
                <div class="AppHeader-globalBar-end flex gap-2 width-full">
                    <div class="AppHeader-search">
                    <button class="AppHeader-searchButton header-button">
                        <label>
                            <Octicon edge_length=16 paths=vec![search_icon]/>
                        </label>
                        <Octicon edge_length=16 paths=vec![command_icon]/>
                        </button>
                    </div>
                    <div class="AppHeader-actions">
                        <button class="header-button">
                            <Octicon edge_length=16 paths=vec![plus_icon]/>
                            <Octicon edge_length=16 paths=vec![triangle_down_icon]/>
                        </button>
                    </div>
                    <div class="AppHeader-issues">
                        <button class="header-button">
                            <Octicon edge_length=16 paths=vec![issues_icon_1, issues_icon_2]/>
                        </button>
                    </div>
                    <div class="AppHeader-pull">
                        <button class="header-button">
                            <Octicon edge_length=16 paths=vec![pull_requests_icon]/>
                        </button>
                    </div>
                    <div class="mr-0">
                        <button class="header-button">
                            <Octicon edge_length=16 paths=vec![notifications_icon]/>
                        </button>
                    </div>
                    <div class="header-item">
                        <button class="header-button overflow-hidden p-0 rounded-full w-9 h-9">
                            <img class="w-full h-full object-center" src="/assets/ryanPic.png"/>
                        </button>
                    </div>
                </div>
            </div>
            <NavigationHeader/>
        </header>
    }
}


//Set signal or query to enable underline class. BOOM. 
#[component]
fn NavigationHeader(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let overview_icon = icons.get_icon("overview").unwrap();
    let repositories_icon = icons.get_icon("repositories").unwrap();
    let projects_icon = icons.get_icon("projects").unwrap();
    let packages_icon = icons.get_icon("packages").unwrap();
    let stars_icon = icons.get_icon("stars").unwrap();

    view! {cx, 

        <div class="AppHeader-localBar mt-2 px-4 pt-4 gap-4">
            <nav class="UnderlineNav">
            <ul class="UnderlineNav-body list-none flex items-center gap-6">
                <li class="inline-flex">
                    <A href="" id="overview-tab" class="UnderlineNav-item p-2">
                        <Octicon edge_length=16 paths=vec![overview_icon]/>
                        <span>Overview</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=repositories" id="repositories-tab" class="UnderlineNav-item p-2 pl-0">
                        <Octicon edge_length=16 paths=vec![repositories_icon]/>
                        <span>Repositories</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=projects" id="projects-tab" class="UnderlineNav-item p-2">
                        <Octicon edge_length=16 paths=vec![projects_icon]/>
                        <span>Projects</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=packages" id="packages-tab" class="UnderlineNav-item p-2">
                        <Octicon edge_length=16 paths=vec![packages_icon]/>
                        <span>Packages</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=stars" id="stars-tab" class="UnderlineNav-item p-2"> 
                        <Octicon edge_length=16 paths=vec![stars_icon]/>
                        <span>Stars</span>
                    </A>
                </li>
                </ul>
            </nav>
        </div>



    }

}

#[component]
fn ContentPanel(cx: Scope) -> impl IntoView {


    let query = use_query_map(cx);

    let tab = move|| {
        query.with(|query| query.get("tab").cloned())
    };

    view!{cx, 
        <div class="max-w-screen-xl mt-16 mx-auto ">

            <div class="main flex gap-16">
                <LayoutSidebar />
                <Outlet/>
//                {
//                    match tab().unwrap_or("".to_string()).as_str() {
//                        "repositories" => view!{cx, <Repository/> },
//                        _ => view!{cx, <Overview/>}
//                    }
//                }
            </div>
            <AppFooter />
        </div>
    }
}


#[component]
fn AppFooter(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let logo_icon = icons.get_icon("logo").unwrap();
    let links = vec!["Terms","Privacy","Security","Status","Docs","Contact Github", "Pricing", "API",
        "Training", "Blog", "About"];


    view! {cx, 
        <div class="relative flex items-center pb-2 mt-6 pt-6 border-t-2 width-full justify-between text-xs">
            <div class="w-1/3 text-slate-600">   
                <div class="flex w-full justify-start mb-2">
                    <div class="flex items-center justify-between">
                        <a class="mt-2 flex mx-4 items-center">
                            <Octicon edge_length=32 paths=vec![logo_icon]/>
                        </a>
                        <span>"© 2023 GitHub, Inc."</span>
                    </div>
                    
                </div>
            </div>
                <nav class="w-full">
                    <ul class="list-none flex flex-nowrap justify-between mb-2 text-[#0969da] text-sm">
                    {links.into_iter().map(|link| view! {cx,
                            <li class ="mr-3">
                                <a href="#">{link}</a>
                            </li>
                        })
                        .collect::<Vec<_>>()
                    }
                    </ul>
                    </nav>

        </div>
    }

}

#[component]
fn Overview(cx: Scope) -> impl IntoView {

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
fn OverviewRepo(cx: Scope) -> impl IntoView {

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



#[component]
fn RepositoryRepo(cx: Scope) -> impl IntoView {

    use rand::Rng;

    let icons = Octicons::new();
    let repositories_icon = icons.get_icon("repositories").unwrap();
    let stars_icon = icons.get_icon("stars").unwrap();
    let stars1_icon = icons.get_icon("stars").unwrap();
    let stars_filled_icon = icons.get_icon("stars-filled").unwrap();
    let fork_icon = icons.get_icon("fork").unwrap();
    let triangle_down_icon = icons.get_icon("triangle-down").unwrap();
    let edge = 12;
    let lines = vec!["/assets/sp1.svg", "/assets/sp2.svg", "/assets/sp3.svg", "/assets/sp4.svg", "/assets/sp5.svg"];

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

#[component]
fn Repository(cx: Scope) -> impl IntoView {

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
fn LayoutSidebar(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let people_icon = icons.get_icon("people").unwrap();
    let heart_icon = icons.get_icon("heart").unwrap();
    let params = use_params_map(cx);

    let username = move|| {
        params.with(|params| params.get("id").cloned())
    };

    let avatars = vec![
        "https://avatars.githubusercontent.com/u/16739?s=70&v=4",
        "https://avatars.githubusercontent.com/u/11306089?v=4",
        "https://avatars.githubusercontent.com/u/1469613?s=70&v=4",
        "https://avatars.githubusercontent.com/u/67846890?s=70&v=4",
        "https://avatars.githubusercontent.com/u/7694931?s=70&v=4",
        "https://avatars.githubusercontent.com/u/34382127?s=70&v=4",
        "https://avatars.githubusercontent.com/u/34443492?s=70&v=4",
        "https://avatars.githubusercontent.com/u/22966523?s=70&v=4",
        "https://avatars.githubusercontent.com/u/1089?s=70&v=4",
        "https://avatars.githubusercontent.com/u/6953353?s=70&v=4",
        "https://avatars.githubusercontent.com/u/880421?s=60&v=4",
        "https://avatars.githubusercontent.com/u/6588942?s=60&v=4",
        "https://avatars.githubusercontent.com/u/60681675?s=60&v=4"
    ];

    view!{cx,
        <div class="layout_sidebar flex flex-col max-w-[18rem] w-full">
            <div class="h-card flex flex-col gap-4">
                <img src="/assets/ryanPic.png" class="rounded-full object-cover w-64 h-64"/>
                <span class="text-lg text-[#656d76]">{username}</span>
                <div class="flex text-sm gap-2">
                    <button class="content-button w-full">Follow</button>
                    <button class="content-button flex justify-center gap-2 items-center w-full">
                    <div class="text-pink-400">
                    <Octicon edge_length=16 paths=vec![heart_icon]/>
                    </div>
                    Sponsors</button>
                </div>
                <div class="inline-flex items-center text-sm text-[#656d76] gap-2">
                    <Octicon edge_length=16 paths=vec![people_icon]/>
                    <span><b class="text-black">69</b> followers " · " <b class="text-black">420</b> following</span>
                </div>
            </div>
            <div id="achievements" class="flex flex-col border-t-2 my-2">
                <span class="font-bold mt-4">Sponsors</span>
                <div class="tars grid grid-cols-6 grid-rows-2 gap-y-0 gap-x-0">
                    { avatars.into_iter().map(|link|
                        view! {cx, 
                            <Avatar source=String::from(link)/>
                        })
                        .collect::<Vec<_>>()
                    }
                </div>
            </div>
            <div id="achievements" class="flex flex-col border-t-2">
                <span class="font-bold mt-4">Achievements</span>
            </div>

        </div>
    }
}

#[component]
fn Octicon(cx: Scope, 
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
fn Avatar(cx: Scope, source: String) -> impl IntoView {

    view! {cx,
        <div class="header-item m-0">
            <button class="header-button overflow-hidden p-0 m-0 rounded-full w-9 h-9">
                <img class="w-full h-full object-center" src=source.clone()/>
            </button>
        </div>
    }

}


/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
