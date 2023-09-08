use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod clerk;
mod engine;
mod home;

use clerk::*;
use engine::*;
use home::*;

#[derive(Clone)]
struct AppState {
    clerk: RwSignal<Option<Clerk>>,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    provide_context(
        cx,
        AppState {
            clerk: create_rw_signal(cx, None),
        },
    );

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // Load Ionic components. The capitol Script and Link will inject into the header.
        <Script type_="module" src="https://cdn.jsdelivr.net/npm/@ionic/core/dist/ionic/ionic.esm.js"></Script>
        <Link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@ionic/core/css/ionic.bundle.css" />

        <ion-app>
            <Header/>

            <ion-content>
                <Router>

                    // Load Clerk
                    <Clerk/>

                    <main>
                        <Routes>
                            <Route path="/" view=HomePage/>
                                <Route path="engine" view=EnginePage/>
                            <Route path="/*any" view=NotFound/>
                        </Routes>
                    </main>
                </Router>
            </ion-content>

            <Footer/>
        </ion-app>
    }
}

#[component]
fn Header(cx: Scope) -> impl IntoView {
    view! { cx,
        <ion-header>
            <ion-toolbar color="primary">
                <ion-buttons slot="start">
                    <img src="https://donationengine.g3tech.net/assets/egearpure.svg" class="h-8" />
                </ion-buttons>
                <ion-title class="text-center">
                    "Donation Engine"
                </ion-title>
                <ion-buttons slot="end">
                    <ClerkButtons/>
                </ion-buttons>
            </ion-toolbar>
        </ion-header>
    }
}

#[component]
fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <ion-footer><div style="background-color: var(--ion-color-secondary)" class="flex justify-center">"© 2023 Gateway3. All rights reserved."</div></ion-footer>
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
