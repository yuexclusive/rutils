use yew::prelude::*;
use yew::Properties;
use crate::user_list::User;
use std::ops::Deref;

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
