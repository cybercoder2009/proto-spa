use common::rest::login::PostReq;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::store::user::{UserAction, UserContext};

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_context::<UserContext>().expect("UserContext not found");
    let navigator = use_navigator().unwrap();
    let username_node = use_node_ref();
    let password_node = use_node_ref();
    let error_state = use_state(|| None::<String>);
    let loading = use_state(|| false);

    if user_ctx.is_logged_in() {
        navigator.push(&Route::Index);
    }

    let on_submit = {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        let username_node = username_node.clone();
        let password_node = password_node.clone();
        let error_state = error_state.clone();
        let loading = loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username_input = username_node.cast::<HtmlInputElement>();
            let password_input = password_node.cast::<HtmlInputElement>();

            if let (Some(u_in), Some(p_in)) = (username_input, password_input) {
                let username = u_in.value().trim().to_string();
                let password = p_in.value();

                if username.is_empty() || password.is_empty() {
                    error_state.set(Some("Username and password are required".into()));
                    return;
                }

                let user_ctx = user_ctx.clone();
                let navigator = navigator.clone();
                let error_state = error_state.clone();
                let loading = loading.clone();

                loading.set(true);
                error_state.set(None);

                spawn_local(async move {
                    let req_body = PostReq {
                        username: username.clone(),
                        password,
                    };

                    match do_login(&req_body).await {
                        Ok(token) => {
                            user_ctx.dispatch(UserAction::SetJwt(token));
                            navigator.push(&Route::Index);
                        }
                        Err(err_msg) => {
                            error_state.set(Some(err_msg));
                        }
                    }
                    loading.set(false);
                });
            }
        })
    };

    html! {
        <div class="flex flex-col p-4">
            <h1 class="text-xl font-bold mb-4">{"Sign In"}</h1>

            {
                if let Some(ref err) = *error_state {
                    html! {
                        <div class="mb-4 text-sm border p-2">
                            {err}
                        </div>
                    }
                } else {
                    html! {}
                }
            }

            <form class="flex flex-col gap-3" onsubmit={on_submit}>
                <label for="username">{"Username"}</label>
                <input
                    id="username"
                    type="text"
                    ref={username_node}
                    required=true
                    class="border p-2"
                />

                <label for="password">{"Password"}</label>
                <input
                    id="password"
                    type="password"
                    ref={password_node}
                    required=true
                    class="border p-2"
                />

                <button
                    type="submit"
                    disabled={*loading}
                    class="border py-2 mt-2"
                >
                    {
                        if *loading {
                            "Signing in..."
                        } else {
                            "Sign In"
                        }
                    }
                </button>
            </form>
        </div>
    }
}

async fn do_login(req: &PostReq) -> Result<String, String> {
    let window = web_sys::window().ok_or("No window object")?;
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);

    let json_str = serde_json::to_string(req).map_err(|e| e.to_string())?;
    opts.set_body(&wasm_bindgen::JsValue::from_str(&json_str));

    let request = Request::new_with_str_and_init("/rest/login", &opts)
        .map_err(|e| format!("Request failed: {:?}", e))?;

    request.headers().set("Content-Type", "application/json").map_err(|e| format!("{:?}", e))?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {:?}", e))?;

    let response: Response = resp_value.dyn_into().map_err(|e| format!("{:?}", e))?;

    match response.status() {
        200 => {
            let text = JsFuture::from(response.text().map_err(|e| format!("{:?}", e))?)
                .await
                .map_err(|e| format!("{:?}", e))?;
            Ok(text.as_string().unwrap_or_default())
        }
        403 => Err("Invalid username or password".into()),
        400 => Err("Invalid request".into()),
        status => Err(format!("Server returned status: {status}")),
    }
}
