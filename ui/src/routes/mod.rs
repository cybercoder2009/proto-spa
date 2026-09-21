pub mod index;
pub mod login;

use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Eq, Debug)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
