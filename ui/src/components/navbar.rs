//! Navigation bar using Tailwind CSS classes.

use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::Route;
use crate::store::user::{UserAction, UserContext};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_ctx = use_context::<UserContext>().expect("UserContext not found");
    let is_logged_in = user_ctx.is_logged_in();
    let display_name = user_ctx.display_name().to_string();
    let sidebar_open = use_state(|| false);

    let on_logout = {
        let user_ctx = user_ctx.clone();
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            sidebar_open.set(false);
            user_ctx.dispatch(UserAction::Logout);
        })
    };

    let toggle_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: MouseEvent| {
            sidebar_open.set(!*sidebar_open);
        })
    };

    let close_sidebar = {
        let sidebar_open = sidebar_open.clone();
        Callback::from(move |_: MouseEvent| {
            sidebar_open.set(false);
        })
    };

    html! {
        <>
            <header class="flex justify-between items-center px-4 py-3 border-b">
                <div class="w-8 flex items-center">
                    {
                        if is_logged_in {
                            html! {
                                <button onclick={toggle_sidebar} aria-label="Open menu">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                                    </svg>
                                </button>
                            }
                        } else {
                            html! {
                                <Link<Route> to={Route::Login}>
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1" />
                                    </svg>
                                </Link<Route>>
                            }
                        }
                    }
                </div>

                <div class="font-bold text-lg tracking-wider">
                    <Link<Route> to={Route::Index}>
                        {"spa"}
                    </Link<Route>>
                </div>

                <div class="w-8"></div>
            </header>

            {
                if *sidebar_open {
                    html! {
                        <div class="fixed inset-0 z-50 flex">
                            <div class="fixed inset-0 bg-black/50" onclick={&close_sidebar}></div>
                            <div class="relative w-64 bg-white h-full flex flex-col justify-between p-4 z-10">
                                <div>
                                    <div class="flex justify-between items-center border-b pb-3">
                                        <span class="font-bold">{display_name}</span>
                                        <button onclick={&close_sidebar}>{"✕"}</button>
                                    </div>
                                </div>
                                <div class="border-t pt-3">
                                    <button class="w-full text-left py-2 font-medium" onclick={on_logout}>
                                        {"Logout"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}
