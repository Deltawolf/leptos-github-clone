use leptos::*;

#[component]
pub fn Packages(cx: Scope) -> impl IntoView {

    view!{cx,
        
        <div class="flex flex-col items-center justify-around gap-4">
            <img src="/assets/packages/packages.svg"/>
            <span class="text-3xl font-bold"> Get started with GitHub Packages </span>
            <span class="text-slate-500 text-sm font-medium">Safely publish packages, store your packages alongside your code, and share your packages privately with your team.</span>
            <div class="flex flex-col items-center gap-2 pt-4">
                <span class="text-slate-500 text-xl">Choose a registry </span>
                <div class="grid grid-cols-3 w-full gap-4">
                    <PackagesCard 
                        header="Docker".to_string()
                        image_source="/assets/packages/docker.svg".to_string()
                        content="A software platform used for building applications based on containers — small and lightweight execution environments. ".to_string()/>
                    <PackagesCard 
                        header="Apache Maven".to_string()
                        image_source="/assets/packages/maven.svg".to_string()
                        content="A default package manager used for the Java programming language and the Java runtime environment. ".to_string()/> 

                    <PackagesCard 
                        header="NuGet".to_string()
                        image_source="/assets/packages/nuget.svg".to_string()
                        content="A free and open source package manager used for the Microsoft development platforms including .NET. ".to_string()/> 
 
                    <PackagesCard 
                        header="RubyGems".to_string()
                        image_source="/assets/packages/ruby.svg".to_string()
                        content="A standard format for distributing Ruby programs and libraries used for the Ruby programming language. ".to_string()/> 

                    <PackagesCard 
                        header="npm".to_string()
                        image_source="/assets/packages/npm.svg".to_string()
                        content="A package manager for JavaScript, included with Node.js. npm makes it easy for developers to share and reuse code. ".to_string()/> 
                    <PackagesCard 
                        header="Containers".to_string()
                        image_source="/assets/packages/docker.svg".to_string()
                        content="A single place for your team to manage Docker images and decide who can see and access your images. ".to_string()/> 
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PackagesCard(cx: Scope,
                image_source: String,
                header: String,
                content: String,
                ) -> impl IntoView {

    view!{cx, 
            <div class="flex flex-col w-full p-4 border-[1px] rounded-md border-slate-300 gap-4">
                <div class="inline-flex w-full gap-2">
                    <img src=image_source/>
                   <span class="font-bold">{header}</span> 
                </div>
                <span class="text-xs text-slate-500">{content}</span>
                <button class="content-button text-xs w-fit py-2 px-4">Learn more</button>
            </div>
    }

}
