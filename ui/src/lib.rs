//! Yew UI Single Page Application entrypoint and router.

pub mod components;
pub mod routes;
pub mod storage;
pub mod store;
pub mod utilities;
pub mod ws;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::navbar::Navbar;
use crate::routes::index::Index;
use crate::routes::login::Login;
use crate::routes::Route;
use crate::store::user::{User, UserContext};
use crate::ws::WsService;

fn switch(route: Route) -> Html {
    match route {
        Route::Index => html! { <Index /> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! {
            <div class="flex-1 flex flex-col items-center justify-center p-6 text-center">
                <h1 class="text-xl font-bold text-gray-800">{"404 - Not Found"}</h1>
                <Link<Route> to={Route::Index} classes="mt-4 text-sm text-indigo-600 font-semibold hover:underline">
                    {"Back to Home"}
                </Link<Route>>
            </div>
        },
    }
}

#[function_component(GlobalWs)]
fn global_ws() -> Html {
    let user_ctx = use_context::<UserContext>().expect("UserContext not found");
    let is_logged_in = user_ctx.is_logged_in();
    let token = user_ctx.token.clone();

    use_effect_with((is_logged_in, token), move |(logged_in, tok)| {
        let mut ws_service = None;
        if *logged_in && !tok.is_empty() {
            if let Ok(service) = WsService::connect(tok) {
                ws_service = Some(service);
            }
        }
        move || {
            drop(ws_service);
        }
    });

    html! {}
}

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_reducer(|| {
        if let Some(token) = crate::storage::read(crate::store::user::KEY_USER) {
            if !token.is_empty() {
                if let Some(mut user) = crate::utilities::from_jwt::<User>(&token) {
                    user.token = token;
                    return user;
                }
            }
        }
        User::default()
    });

    html! {
        <ContextProvider<UserContext> context={user_state}>
            <GlobalWs />
            <HashRouter>
                <div class="w-full max-w-[430px] h-full min-h-screen bg-white shadow-2xl relative flex flex-col mx-auto overflow-hidden">
                    <Navbar />
                    <main class="flex flex-col flex-1 overflow-y-auto">
                        <Switch<Route> render={switch} />
                    </main>
                </div>
            </HashRouter>
        </ContextProvider<UserContext>>
    }
}

#[wasm_bindgen(start)]
pub fn main_js() {
    yew::Renderer::<App>::new().render();
}
