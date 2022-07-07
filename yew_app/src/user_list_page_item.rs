use yew::prelude::*;
use yew::Properties;

pub struct UserListPageItem;

#[derive(Clone, PartialEq, Properties)]
pub struct UserListPageItemProps {
    pub onclick: Callback<i64>,
    pub page_index: i64,
    pub active: bool,
}

pub enum UserListPageItemMsg {
    Click,
}

impl Component for UserListPageItem {
    type Message = UserListPageItemMsg;

    type Properties = UserListPageItemProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListPageItemMsg::Click => {
                ctx.props().onclick.emit(ctx.props().page_index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let index = ctx.props().page_index;
        let active = ctx.props().active;
        let onclick = ctx.link().callback(|_| UserListPageItemMsg::Click);
        html! {
            <>
            <li class={if active {"page-item active"} else { "page-item" }}><a {onclick} class="page-link" href="javascript:void(0);">{index}</a></li>
            </>
        }
    }
}
