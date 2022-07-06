#![allow(unused)]
use std::cell::RefCell;
use std::ops::Deref;

use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Properties;
use yew_router::prelude::*;
use yew_router::BrowserRouter;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
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

pub struct UserListItem;

#[derive(Clone, PartialEq, Properties)]
pub struct UserListItemProps {
    pub value: User,
}

impl Component for UserListItem {
    type Message = ();
    type Properties = UserListItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let x = &ctx.props().value;
        html! {
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
    }
}

pub struct UserList;

pub enum Msg {
    GetVal(PageData),
    GetError,
    GetMarkdown,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserListProps {
    pub page_data: RefCell<PageData>,
    // pub children: ChildrenWithProps<UserListItem>,
}

impl Component for UserList {
    type Message = Msg;

    type Properties = UserListProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetMarkdown => {
                ctx.link().send_future(async {
                    match get::<PageData>("/user/query?index=1&size=10").await {
                        Ok(md) => Msg::GetVal(md),
                        Err(err) => Msg::GetError,
                    }
                });
                false
            }
            Msg::GetError => false,
            Msg::GetVal(s) => {
                *ctx.props().page_data.borrow_mut() = s;
                // self.page_data = s;
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
        html! {
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
            ctx.props()
            .page_data
            .borrow()
            .data
            .iter()
            .map(|x| {
                html! {
                    <UserListItem value = {x.clone()}/>
                }
            })
            .collect::<Html>()
        }

                        <nav aria-label="Page navigation example">
                <ul class="pagination">
                    <li class="page-item">
                    <a class="page-link"  aria-label="Previous">
                        <span aria-hidden="true">{"<"}</span>
                    </a>
                    </li>
                    <li class="page-item"><a class="page-link" >{1}</a></li>
                    <li class="page-item"><a class="page-link" >{2}</a></li>
                    <li class="page-item"><a class="page-link" >{3}</a></li>
                    <li class="page-item">
                    <a class="page-link"  aria-label="Next">
                        <span aria-hidden="true">{">"}</span>
                    </a>
                    </li>
                </ul>
                </nav>

                        </div>
                }
    }
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct PageData {
    pub data: Vec<User>,
    pub total: i64,
    pub code: String,
}
// #[derive(Serialize, Component)]
#[derive(Deserialize, PartialEq, Clone)]
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
                        total: 0,
                        code: String::from(""),
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
    yew::Renderer::<Main>::new().render();
}
