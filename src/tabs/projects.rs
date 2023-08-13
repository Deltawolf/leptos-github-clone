use leptos::*;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::leptos_dom::console_log;
use leptos_router::{Form, use_query_map, use_params_map};

use crate::components::Octicons;
use crate::components::Octicon;

#[component]
pub fn ProjectsList(cx: Scope) -> impl IntoView {

    let icons = Octicons::new();
    let triangle_down_icon = icons.get_icon("triangle-down").unwrap();
    let open_icon_1 = icons.get_icon("issues-1").unwrap();
    let open_icon_2 = icons.get_icon("issues-2").unwrap();
    let check_icon = icons.get_icon("check").unwrap();
    let (projects, set_projects) = create_signal(cx, vec![
            ("Leptos".to_string(),"Build fast web applications with Rust.".to_string()),
            ("Custom-Elements".to_string(),"Build fast web applications with Rust.".to_string()),
            ("Venite".to_string(), "Spec and rendering components for the Liturgy Document Format (LDF)".to_string()),
        ]);  

    view! {cx,
        <div class="flex flex-col gap-8">
         <div class="flex flex-col w-full">
             <div class="flex bg-[#f6f8fa] border-[1px] border-slate-300 rounded-t-md justify-between py-2 px-4 gap-4">
                <div class="left flex gap-4">
                    <div class="inline-flex items-center gap-2">
                        <Octicon edge_length=16 paths=vec![open_icon_1, open_icon_2] />
                        <span>Open</span>
                    </div>
                    <div class="inline-flex items-center gap-2">
                        <Octicon edge_length=16 paths=vec![check_icon] />
                        <span>Closed</span>
                    </div>
                </div>
                <div class="right">
                    <div class="inline-flex items-center gap-2">
                        <span>Sort</span>
                        <Octicon edge_length=16 paths=vec![triangle_down_icon] />
                    </div>
                </div>
            </div>
            { move || projects().into_iter().map(|project|
                view! {cx,
                    <Project header=project.0 desc=project.1/>
                })
                .collect::<Vec<_>>()
            }
           <span class="border-b-[1px] border-slate-300 rounded-md"/>
        </div>
        <Add_Project set_projects=set_projects/>
        </div>
    }
}

#[component]
fn Project(cx: Scope, header: String, desc: String) -> impl IntoView {

    let icons = Octicons::new();
    let ellipses_icon: String = icons.get_icon("issues-1").unwrap();
    view! {cx, 
        <div class="border-slate-300 p-4 border-[1px] border-b-0">
            <div class="grid grid-cols-5">
                <div class="flex flex-col col-span-2 gap-1">
                    <span class="font-bold">{header}</span>
                    <span class="text-xs text-slate-500">{updated_generator()}</span>
                </div>
                <span class="self-center col-span-2 text-sm">{desc}</span>
                <div class="flex justify-end items-center">
                    <Octicon edge_length=8 paths=vec![ellipses_icon.clone()]/>
                    <Octicon edge_length=8 paths=vec![ellipses_icon.clone()]/>
                    <Octicon edge_length=8 paths=vec![ellipses_icon]/>
                </div>
            </div>
        </div>
    }
}

fn updated_generator() -> String {

    use rand::Rng;
    let mut rng = rand::thread_rng();
        match rng.gen_range(0..=3) {
            0 => format!("Updated {} seconds ago", rng.gen_range(0..60)),
            1 => format!("Updated {} minutes ago", rng.gen_range(1..60)),
            2 => format!("Updated {} hours ago", rng.gen_range(1..24)),
            3 => format!("Updated {} days ago", rng.gen_range(1..100)),
            _ => format!("")
        }
}

#[component]
fn Add_Project(cx: Scope, set_projects: WriteSignal<Vec<(String,String)>>) -> impl IntoView {

    let triangle_down_icon = Octicons::new().get_icon("triangle-down").unwrap();
    let repository_icon = Octicons::new().get_icon("repositories").unwrap();
    let lock_icon = Octicons::new().get_icon("lock").unwrap();
    let info_icon = Octicons::new().get_icon("info").unwrap();


    let params = use_params_map(cx);

    let username = move|| {
        params.with(|params| params.get("id").cloned())
    };

    let repo_ref = create_node_ref::<Input>(cx);
    let desc_ref = create_node_ref::<Input>(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        set_projects.update(
            |projects| projects.push(
                (repo_ref().expect("repo").value(), desc_ref().expect("desc").value())
            ));
    };

    let (public, set_public) = create_signal(cx, true);

    let on_public = move |ev| set_public.update(|current| {
        console_log(event_target_value(&ev).as_str());
        if event_target_value(&ev).as_str() == "on" {
            *current = true; 
        } else {
            *current = false; 
        }});
    
    let on_private = move |ev| set_public.update(|current| {
        if event_target_value(&ev).as_str() == "on" {
            *current = false; 
        } else {
            *current = true; 
        }});




    view! {cx,
        <Form method="POST" action="" on:submit=on_submit>
            <div class="flex flex-col gap-2 p-2">
                
                <section class="flex flex-col border-b-[1px] pb-2">
                    <span class="font-semibold text-2xl mb-1">Create a new repository</span>
                    <span class="text-slate-500 text-sm">
                        A repository contains all project files, 
                        including the revision history. Already have a 
                        project repository elsewhere? 
                    </span>
                    <a class="text-sm text-blue-600" href="">Import a repository.</a>
                </section>

                <section class="flex flex-col gap-2 w-full border-b-[1px] pb-4">
                    <span class="text-sm mb-2"><em>Required fields are marked with an asterisk (*).</em></span>
                    <div class="flex gap-2 items-end mb-4">
                        <div class="flex flex-col gap-1">
                            <span class="font-semibold text-sm">Owner<sup>*</sup></span>
                            <button class="content-button text-sm flex items-center gap-1 flex-row h-8">
                                <div class="header-button p-0 overflow-hidden rounded-full w-6 h-6">
                                    <img class="w-full h-full object-center" src="/assets/ryanPic.png"/>
                                </div>
                                <span>{username}</span>
                                <Octicon edge_length=16 paths=vec![triangle_down_icon.clone()]/>
                            </button>
                        </div>
                        <span class="text-[1.5rem]">/</span>
                        <div class="flex flex-col h-full gap-1">
                            <span class="font-semibold text-sm">Repository name<sup>*</sup></span>
                            <input node_ref=repo_ref class="border-[1px] border-slate-300 rounded-md h-8 px-1" type="text"/>
                        </div>
                    </div>
                    <span class="text-sm">
                        Great repository names are short and memorable. Need inspiration? 
                        How about <span class="text-green-700 font-semibold">curly-fiesta</span> ?
                    </span>
                    <div class="mt-4 flex flex-col gap-1">
                        <span class="text-sm font-semibold">
                            Description 
                            <span class="text-xs text-gray-400">
                                (optional)
                            </span>
                        </span>
                        <input node_ref=desc_ref class="h-8 p-2 border-[1px] border-slate-300 rounded-md" type="text"/>
                    </div>
                </section>
                
                <section class="flex flex-col gap-4 border-b-[1px] border-slate-300 py-4">
                    <div class="inline-flex items-center gap-[0.75rem]">
                    <input on:change=on_public class="project-radio" name="privacyGroup" type="radio"/> 
                        <Octicon edge_length=24 paths=vec![repository_icon]/>
                        <div class="flex flex-col">
                            <span class="font-bold text-sm">Public</span>
                            <span class="text-xs text-slate-500">
                                Anyone on the internet can see this repository. You choose who can commit.
                            </span>
                        </div>
                    </div>
                    <div class="inline-flex items-center gap-[0.75rem]">
                    <input on:change=on_private class="project-radio" name="privacyGroup" type="radio"/> 
                        <Octicon edge_length=24 paths=vec![lock_icon]/>
                        <div class="flex flex-col">
                            <span class="text-sm font-bold">Private</span>
                            <span class="text-xs text-slate-500">
                                You can choose who can see and commit to this repository.
                            </span>
                        </div>
                    </div>
                </section>
                <section class="flex flex-col gap-4 border-b-[1px] border-slate-300 py-4">

                    <span class="text-sm font-semibold">Initialize this repository with:</span>
                    <div class="inline-flex items-center gap-[0.75rem]">
                        <input class="mr-1 w-4"  type="checkbox"/> 
                        <div class="flex flex-col">
                            <span class="font-bold text-sm">Add a README file</span>
                            <span class="text-xs text-slate-500">
                                "This is where you can write a long description for your project. " 
                                <span class="text-blue-600">Learn more about READMEs.</span>
                            </span>
                        </div>
                    </div>
                    <div class="inline-flex items-center gap-[0.75rem]">
                        <div class="flex flex-col gap-2">
                            <span class="font-bold text-sm">Add .gitignore</span>
                            <button class="content-button inline-flex gap-1 items-center h-8 w-fit">
                                <span class="text-xs text-slate-500">
                                    .gitignore template:<span class="text-black">None</span>
                                </span>
                                <Octicon edge_length=16 paths=vec![triangle_down_icon.clone()]/>
                            </button> 
                            <span class="text-xs text-slate-500">
                                "Choose which files not to track from a list of templates. "
                                <span class="text-blue-600">Learn more about ignoring files.</span>
                            </span>
                        </div>
                    </div>
                    <div class="inline-flex items-center gap-[0.75rem]">
                        <div class="flex flex-col gap-2">
                            <span class="font-bold text-sm">Choose a license</span>
                            <button class="content-button inline-flex gap-1 items-center h-8 w-fit">
                                <span class="text-xs text-slate-500">
                                    License:<span class="text-black">None</span>
                                </span>
                                <Octicon edge_length=16 paths=vec![triangle_down_icon]/>
                            </button> 
                            <span class="text-xs text-slate-500">
                                "A license tells others what they can and can't do with your code. " 
                                <span class="text-blue-600">"Learn more about licenses."</span>
                            </span>
                        </div>
                    </div>
                </section>
                <section class="inline-flex items-center gap-4 border-b-[1px] border-slate-300 py-4 text-slate-500">
                

                <Octicon edge_length=16 paths=vec![info_icon]/>
                { move || if public() { 
                        view!{cx, 
                            <span class="text-slate-500">You are creating a public repository in your personal account</span>}
                    } else {
                        view!{cx, <span class="text-slate-500" >You are creating a private repository in your personal account</span>}
                    }
                }

                </section>
                <input class="self-end font-medium text-sm text-white bg-[#1f883d] 
                    rounded-md px-3 h-8" type="submit" value="Create repository"/>
            </div>
        </Form>
    }
}

#[component]
pub fn FormExample2(cx: Scope) -> impl IntoView {
    // reactive access to URL query strings
    let query = use_query_map(cx);
    // search stored as ?q=
    let search = move || query().get("q").cloned().unwrap_or_default();
    // a resource driven by the search string

        view! { cx,
        <Form method="POST" action="">
            <input type="text" name="search" value=search> <label>Goodbye</label> </input>
            <input type="submit"/>
        </Form>
    }

}

