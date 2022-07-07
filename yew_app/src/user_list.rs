use std::cell::RefCell;

use crate::user_list_item::UserListItem;
use crate::user_list_page::UserListPage;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use yew::prelude::*;
use yew::Properties;

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const BASE_API_URL: &str = "http://localhost:8080";

async fn get<'a, Res, T, V>(path: &str, params: T) -> Result<Res, gloo_net::Error>
where
    Res: DeserializeOwned,
    T: IntoIterator<Item = (&'a str, V)>,
    V: AsRef<str>,
{
    let url = format!("{}{}", BASE_API_URL, path);
    let request = Request::get(&url).query(params);
    let res: Res = request.send().await?.json().await?;
    Ok(res)
}

pub struct UserList;

pub enum Msg {
    GetVal(PageData),
    GetError,
    GetData,
    Reload(Page),
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct PageData {
    pub data: Vec<User>,
    pub code: String,
    pub page: Page,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Page {
    pub total: i64,
    pub index: i64,
    pub size: i64,
}

const SPAN: i64 = 10;

impl Page {
    pub fn page_total(&self) -> i64 {
        (self.total - 1).max(0) / self.size + 1
    }

    pub fn page_start(&self) -> i64 {
        ((self.index - 1) / SPAN) * SPAN + 1
    }

    pub fn page_end(&self) -> i64 {
        (self.page_start() + SPAN - 1).min(self.page_total())
    }
}

// #[derive(Serialize, Component)]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub r#type: String,
    pub email: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserListProps {
    pub page_data: RefCell<PageData>,
}

impl Component for UserList {
    type Message = Msg;

    type Properties = UserListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetData => {
                let page = ctx.props().page_data.borrow().page.clone();
                ctx.link().send_future(async move {
                    match get::<PageData, _, _>(
                        "/user/query",
                        vec![
                            ("index", page.index.to_string()),
                            ("size", page.size.to_string()),
                        ],
                    )
                    .await
                    {
                        Ok(md) => Msg::GetVal(md),
                        Err(err) => {
                            log::error!("user query error: {}", err);
                            Msg::GetError
                        }
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
            Msg::Reload(page) => {
                ctx.props().page_data.borrow_mut().page = page;
                Component::update(self, ctx, Msg::GetData);
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            Component::update(self, ctx, Msg::GetData);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="layout">
        <div class="alert alert-warning alert-dismissible fade show" role="alert">
        <strong>{"Holy guacamole!"}</strong> {"You should check in on some of those fields below."}
        <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close"></button>
        </div>
        <div class="table container-fluid">
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


        </div>
        <div class="footer">
        {
            html!{
                <UserListPage value = { RefCell::new(ctx.props().page_data.borrow().page.clone())} reload = {ctx.link().callback(|page|Msg::Reload(page))}/>
            }
        }
        </div>
        </div>
                        }
    }
}
