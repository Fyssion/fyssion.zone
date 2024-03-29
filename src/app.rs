use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{
    blog::Blog,
    home::Home,
    not_found::NotFound,
    // about::About,
    post::BlogPost,
};

static SITE_TITLE: &'static str = "fyssion's zone";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fyssion_zone.css"/>

        <Title text=SITE_TITLE/>

        <Link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/favicon-16x16.png" />
        <Link rel="manifest" href="/site.webmanifest" />
        <Meta name="msapplication-TileColor" content="#EB9486" />
        <Meta name="theme-color" content="#EB9486" />

        <Script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js" />
        <Link rel="preconnect" href="https://rsms.me/" />
        <Link rel="stylesheet" href="https://rsms.me/inter/inter.css" />

        //  TODO: add real error handling
        <Router fallback=move || {view!{ <NotFound />}.into_view()}>
        <div class="app">
                <header>
                    <nav>
                        <div class="logo">
                            <A exact=true href="/">"fyssion.zone"</A>
                            // <A href="about">"about"</A>  // not yet!!
                        </div>
                        <ul>
                            <li>
                                <A href="blog">"blog"</A>
                            </li>
                            <li>
                                <a href="https://github.com/Fyssion">"github"</a>
                            </li>
                        </ul>
                    </nav>
                </header>

                <main>
                    <Routes>
                        <Route path="" view=|| view! { <Home/> }/>
                        // <Route path="about" view=|| view! { <About/> }/>
                        <Route path="blog" view=|| view! { <Blog/> }/>
                        <Route
                            path="/blog/:id"
                            view=|| view! { <BlogPost/> }
                            ssr=leptos_router::SsrMode::Async
                        />
                    </Routes>
                </main>
                // <footer>
                //     <p>"Made with Rust using Leptos!"</p>
                // </footer>
            </div>
        </Router>
    }
}
