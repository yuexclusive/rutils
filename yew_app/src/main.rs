#![allow(unused)]
use std::ops::Deref;

use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::BrowserRouter;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

struct UserComponent {
    res: QueryRes,
}

const BASE_API_URL: &str = "http://localhost:8080";

async fn get<T>(path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let url = format!("{}{}", BASE_API_URL, path);
    let request = Request::get(&url);
    let res: T = request.send().await?.json().await?;
    Ok(res)
}

enum Msg {
    GetVal(QueryRes),
    GetError,
    GetMarkdown,
}

// #[derive(Serialize, Component)]
#[derive(Deserialize, Debug)]
pub struct User {
    id: i64,
    r#type: String,
    email: String,
    name: Option<String>,
    mobile: Option<String>,
    laston: Option<String>,
    created_at: String,
    updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct QueryRes {
    data: Vec<User>,
    total: i64,
    code: String,
}

impl Component for UserComponent {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            res: QueryRes {
                data: Vec::new(),
                total: 0,
                code: String::from(""),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetMarkdown => {
                ctx.link().send_future(async {
                    match get::<QueryRes>("/user/query?index=1&size=10").await {
                        Ok(md) => Msg::GetVal(md),
                        Err(err) => Msg::GetError,
                    }
                });
                false
            }
            Msg::GetError => false,
            Msg::GetVal(s) => {
                self.res = s;
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            yew::Component::update(self, ctx, Msg::GetMarkdown);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let data = &self.res.data;
        html! {
            <>
                <div class="container">
                <div class="row">
                <div class="col">{"type"}</div>
                <div class="col">{"email"}</div>
                <div class="col">{"name"}</div>
                <div class="col">{"mobile"}</div>
                <div class="col">{"laston"}</div>
                <div class="col">{"created_at"}</div>
                <div class="col">{"updated_at"}</div>
                </div>
                {
                    data.iter().map(|x| {
                    html!{
                        <div class="row">
                        <div class="col">{ x.r#type.deref()}</div>
                        <div class="col">{ x.email.deref()}</div>
                        <div class="col">{ x.name.as_deref().unwrap_or("")}</div>
                        <div class="col">{ x.mobile.as_deref().unwrap_or("")}</div>
                        <div class="col">{ x.laston.as_deref().unwrap_or("")}</div>
                        <div class="col">{ x.created_at.deref()}</div>
                        <div class="col">{ x.updated_at.as_deref().unwrap_or("")}</div>
                        </div>
                    }
                    }).collect::<Html>()
                }
                </div>

                <nav aria-label="Page navigation example">
                <ul class="pagination">
                    <li class="page-item">
                    <a class="page-link" href="#" aria-label="Previous">
                        <span aria-hidden="true">{"<"}</span>
                    </a>
                    </li>
                    <li class="page-item"><a class="page-link" href="#">{1}</a></li>
                    <li class="page-item"><a class="page-link" href="#">{2}</a></li>
                    <li class="page-item"><a class="page-link" href="#">{3}</a></li>
                    <li class="page-item">
                    <a class="page-link" href="#" aria-label="Next">
                        <span aria-hidden="true">{">"}</span>
                    </a>
                    </li>
                </ul>
                </nav>
            </>
        }
    }
}


#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    User,
    #[at("/secure")]
    Secure,
    // #[at("/hello")]
    // Hello,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::User));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        // Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::User => html! {
            <UserComponent />
        },
        Route::Secure => html! {
            <Secure />
        },
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
    yew::Renderer::<Main>::new().render();
}
