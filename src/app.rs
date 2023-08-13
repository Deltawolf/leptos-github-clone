use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::tabs::*;
use crate::components::*;

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
            <Routes>

                <Route path="/:id" view=|cx| view!{cx,
                    <AppHeader/>
                    <div class="max-w-screen-xl mt-16 mx-auto ">
                        <div class="main flex gap-16">
                            <LayoutSidebar />
                            <Content />
                        </div>
                        <AppFooter />
                    </div>
                }/>
            </Routes>
            </main>
        </Router>
    }
}

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

        <header class="AppHeader leading-normal text-sm shadow-inner bg-[#f6f8fa]">
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
                        <span class="text-sm">Overview</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=repositories" id="repositories-tab" class="UnderlineNav-item p-2 pl-0">
                        <Octicon edge_length=16 paths=vec![repositories_icon]/>
                        <span class="text-sm">Repositories</span>

                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=projects" id="projects-tab" class="UnderlineNav-item p-2">
                        <Octicon edge_length=16 paths=vec![projects_icon]/>
                        <span class="text-sm">Projects</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=packages" id="packages-tab" class="UnderlineNav-item p-2">
                        <Octicon edge_length=16 paths=vec![packages_icon]/>
                        <span class="text-sm">Packages</span>
                    </A>
                </li>
                <li class="inline-flex items-center">
                    <A href="?tab=stars" id="stars-tab" class="UnderlineNav-item p-2"> 
                        <Octicon edge_length=16 paths=vec![stars_icon]/>
                        <span class="text-sm text-[#1f2328]">Stars</span>
                    </A>
                </li>
                </ul>
            </nav>
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
                <img src="/assets/ryanPic.png" class="rounded-full object-cover w-[296px] h-[296px]"/>
                <span class="text-lg text-[#656d76]">{username}</span>
                <div class="flex text-sm gap-2">
                    <button class="content-button w-full">Follow</button>
                    <button class="content-button flex justify-center gap-2 items-center w-full">
                    <div class="text-pink-400">
                    <Octicon edge_length=16 paths=vec![heart_icon]/>
                    </div>
                    Sponsors</button>
                </div>
                <div class="inline-flex items-center text-sm text-[#656d76] gap-2 mb-4">
                    <Octicon edge_length=16 paths=vec![people_icon]/>
                    <span><b class="text-black">69</b> followers " · " <b class="text-black">420</b> following</span>
                </div>
            </div>
            <div id="achievements" class="flex flex-col border-t-2 my-2 ">
                <span class="font-bold mt-4 mb-1">Sponsors</span>
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
fn Content(cx: Scope) -> impl IntoView {

    let query = use_query_map(cx);

    let tab = move || {
        query.with(|queries| queries.get("tab").cloned())
            .unwrap_or_else(|| "".to_string())
    };


    view! {cx, {move || 
            match tab().as_str() {
                "" => view!{cx, <Overview />},
                "repositories" => view!{cx, <Repository />},
                "packages" => view!{cx, <Packages />},
                "projects" => view!{cx, <ProjectsList />},
                _ => view!{cx, <Overview />}
            }
        }
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

