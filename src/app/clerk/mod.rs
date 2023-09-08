use leptos::*;
use leptos_router::use_navigate;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::{wasm_bindgen, Closure};

use super::AppState;

#[server(IsLoggedIn, "/isloggedin")]
pub async fn is_logged_in(cx: Scope, session_id: String) -> Result<bool, ServerFnError> {
    use actix_web::HttpRequest;
    use reqwest::StatusCode;
    use std::collections::HashMap;

    leptos_actix::extract(cx, |req: HttpRequest| async move {
        let session_cookie = req.cookie("__session").unwrap();
        println!("{}", session_cookie.to_string());

        let mut map = HashMap::new();
        map.insert("token", session_cookie.value().to_owned());

        let status = reqwest::Client::new()
            .post(format!(
                r#"https://api.clerk.com/v1/sessions/{session_id}/verify"#
            ))
            .json(&map)
            .send()
            .await
            .unwrap()
            .status();

        println!("{}", status.to_string());
        status == StatusCode::OK
    })
    .await
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clerk {
    pub client: ClientResource,
    pub session: Option<ActiveSessionResource>,
    pub user: Option<UserResource>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResource {
    pub path_root: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveSessionResource {
    pub id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResource {}

use wasm_bindgen::JsValue;
#[wasm_bindgen(js_namespace = ["window", "Clerk"])]
extern "C" {
    #[wasm_bindgen]
    type ClerkJsObj;

    #[wasm_bindgen(method)]
    fn load(this: &ClerkJsObj);

    #[wasm_bindgen(method, js_name = "addListener")]
    fn add_listener(this: &ClerkJsObj, listener: &Closure<dyn Fn(JsValue)>);

    #[wasm_bindgen(method, js_name = "openSignIn")]
    fn open_sign_in(this: &ClerkJsObj);

    #[wasm_bindgen(method, js_name = "openUserProfile")]
    fn open_user_profile(this: &ClerkJsObj);

    #[wasm_bindgen(method, js_name = "signOut")]
    fn sign_out(this: &ClerkJsObj);
}

fn get_clerk_from_js() -> ClerkJsObj {
    let clerk = window().get("Clerk").unwrap();
    use wasm_bindgen::JsCast;
    clerk.unchecked_into::<ClerkJsObj>()
}

#[component]
pub fn Clerk(cx: Scope) -> impl IntoView {
    let app_state = expect_context::<AppState>(cx);

    let clerk_load_handler = move |_| {
        let clerk = get_clerk_from_js();

        clerk.load();

        // https://stackoverflow.com/questions/63640557/clousure-invocation-error-when-creating-a-callback-with-a-clousure-on-mouse-inpu
        let closure = Closure::wrap(Box::new(move |resources: JsValue| {
            let clerk: Clerk =
                from_value(resources).expect("Unable to load Clerk object from window.");
            app_state.clerk.set(Some(clerk));
            log!("{:?}", app_state.clerk.get_untracked());
        }) as Box<dyn Fn(JsValue)>);

        clerk.add_listener(&closure);
        closure.forget();
    };

    create_effect(cx, move |_| {
        let is_logged_in = app_state
            .clerk
            .with(|clerk| clerk.as_ref().is_some_and(|clerk| clerk.user.is_some()));

        let navigate = use_navigate(cx);
        if is_logged_in {
            let _ = navigate("/engine", Default::default());
        } else {
            let _ = navigate("/", Default::default());
        }
    });

    view! { cx,
        // Load Clerk
        <script
            async
            crossorigin="anonymous"
            data-clerk-publishable-key="pk_test_d2FybS1tYW50aXMtMzQuY2xlcmsuYWNjb3VudHMuZGV2JA"
            on:load=clerk_load_handler
            // onload="window.Clerk.load().then(() => console.log('hi'))"
            src="https://warm-mantis-34.clerk.accounts.dev/npm/@clerk/clerk-js@4/dist/clerk.browser.js"
            type="text/javascript">
        </script>
    }
}

#[component]
pub fn ClerkButtons(cx: Scope) -> impl IntoView {
    let app_state = expect_context::<AppState>(cx);

    view! { cx,
        <Show
            when= move || app_state.clerk.get().is_some_and(|clerk| clerk.session.is_some())
            fallback=|cx| view! { cx, <ClerkSignIn/> }
        >
            <ClerkUser/>
        </Show>
    }
}

#[component]
fn ClerkSignIn(cx: Scope) -> impl IntoView {
    let sign_in_handler = |_| {
        let clerk = get_clerk_from_js();
        clerk.open_sign_in();
    };

    view! { cx,
        <ion-button
            id="sign-in-button"
            color="secondary"
            fill="solid"
            on:click=sign_in_handler
        >
            <div class="hidden md:block">"Login/Signup"</div>
            <ion-icon class="hidden md:block" slot="end" name="log-in-outline"></ion-icon>
            <ion-icon class="md:hidden" name="log-in-outline"></ion-icon>
        </ion-button>
    }
}

#[component]
fn ClerkUser(cx: Scope) -> impl IntoView {
    let open_user_profile_handler = |_| {
        let clerk = get_clerk_from_js();
        clerk.open_user_profile();
    };

    let sign_out_handler = |_| {
        let clerk = get_clerk_from_js();
        clerk.sign_out();
    };

    view! { cx,
        <ion-button
            id="user-button"
            color="secondary"
            fill="solid"
        >
            "User"
            <ion-icon slot="end" name="chevron-down-circle"></ion-icon>
        </ion-button>
        <ion-popover trigger="user-button" prop:dismissOnSelect="true">
            <ion-content>
                <ion-list>
                    <ion-item
                        id="user-button"
                        button="true"
                        detail="false"
                        on:click=open_user_profile_handler
                    >
                        "Settings"
                        <ion-icon slot="end" name="person-circle-outline"></ion-icon>
                    </ion-item>
                    <ion-item
                        id="sign-out-button"
                        button="true"
                        detail="false"
                        on:click=sign_out_handler
                    >
                        "Logout"
                        <ion-icon slot="end" name="log-out-outline"></ion-icon>
                    </ion-item>
                </ion-list>
            </ion-content>
        </ion-popover>
    }
}
