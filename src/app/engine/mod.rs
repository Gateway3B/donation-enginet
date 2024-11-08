use crate::{app::AppState, list::*};
use leptos::*;

#[server(GetList, "/api")]
pub async fn get_list(cx: Scope, user_id: String) -> Result<List, ServerFnError> {
    use actix_web::web::Data;
    use sea_orm::DatabaseConnection;

    leptos_actix::extract(cx, |db: Data<DatabaseConnection>| async move {
        let list: Option<List> = List::from_user_id(db.get_ref(), user_id.clone()).await;

        if let Some(list) = list {
            return Ok(list);
        }

        let list: Option<List> = List::init_list(db.get_ref(), user_id).await;

        if let Some(list) = list {
            return Ok(list);
        } else {
            return Err(ServerFnError::ServerError(
                "Issue retrieving list from database.".to_owned(),
            ));
        }
    })
    .await?
}

#[component]
pub fn EnginePage(cx: Scope) -> impl IntoView {
    let app_state = expect_context::<AppState>(cx);

    let load_list = create_resource(
        cx,
        app_state.clerk,
        // every time `count` changes, this will run
        move |clerk| async move {
            let clerk = if let Some(clerk) = clerk {
                clerk
            } else {
                return;
            };

            let user = if let Some(user) = clerk.user {
                user
            } else {
                return;
            };

            let list = get_list(cx, user.id).await?;

            app_state.list.set(list);
        },
    );

    view! { cx,
        <h1>"This is the engine page."</h1>
    }
}
