// #![allow(unused)]
mod user_list;
mod user_list_item;
mod user_list_page;
mod user_list_page_item;

use std::cell::RefCell;
use user_list::Page;
use user_list::PageData;
use user_list::UserList;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;
use yew_router::BrowserRouter;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    User,
    // #[at("/hello")]
    // Hello,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> VNode {
    match routes {
        Route::User => {
            html! {
                <UserList page_data = {
                    RefCell::new(PageData {
                        data: Vec::new(),
                        code: String::from(""),
                        page: Page{total:0,index:1,size:15}
                    })
                }/>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Main>::new().render();
}
