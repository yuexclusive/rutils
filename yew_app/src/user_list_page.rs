use std::cell::RefCell;

use crate::user_list::Page;
use crate::user_list_page_item::UserListPageItem;
use yew::prelude::*;
use yew::Properties;

pub struct UserListPage;

pub enum UserListPageMsg {
    PageItemClick(i64),
    Next,
    Pre,
    First,
    Last,
    SizeChanged(web_sys::Event),
}

#[derive(Clone, PartialEq, Properties)]
pub struct UserListPageProps {
    pub value: RefCell<Page>,
    pub reload: Callback<Page>,
}
impl Component for UserListPage {
    type Message = UserListPageMsg;

    type Properties = UserListPageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            UserListPageMsg::PageItemClick(index) => {
                ctx.props().value.borrow_mut().index = index;
            }
            UserListPageMsg::Next => {
                ctx.props().value.borrow_mut().index += 1;
            }
            UserListPageMsg::Pre => {
                ctx.props().value.borrow_mut().index -= 1;
            }
            UserListPageMsg::First => {
                ctx.props().value.borrow_mut().index = 1;
            }
            UserListPageMsg::Last => {
                let last = ctx.props().value.borrow().page_total();
                ctx.props().value.borrow_mut().index = last;
            }
            UserListPageMsg::SizeChanged(e) => {
                let el: web_sys::HtmlInputElement = e.target_unchecked_into();
                ctx.props().value.borrow_mut().size = el.value().parse().unwrap();
            }
        }
        let page = ctx.props().value.borrow().clone();
        ctx.props().reload.emit(page);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let page = ctx.props().value.borrow();

        html! {
            <nav aria-label="Page navigation example">
            <ul class="pagination">
                <li class={if page.index==1 {"page-item disabled"} else { "page-item" }}>
                <a class="page-link"  aria-label="Previous" href="javascript:void(0);" onclick = {ctx.link().callback(|_|UserListPageMsg::First)}>
                    <span aria-hidden="true">{"<<"}</span>
                </a>
                </li>
                <li class={if page.index==1 {"page-item disabled"} else { "page-item" }}>
                <a class="page-link"  aria-label="Previous" href="javascript:void(0);" onclick = {ctx.link().callback(|_|UserListPageMsg::Pre)}>
                    <span aria-hidden="true">{"<"}</span>
                </a>
                </li>
                {
                    (page.page_start()..= page.page_end()).map(|x|{
                        let onclick = ctx.link().callback(|index:i64|UserListPageMsg::PageItemClick(index));
                        html!{
                            <UserListPageItem page_index={x} active={page.index==x} {onclick}/>
                        }
                    }).collect::<Html>()
                }
                <li class={if page.index==page.page_total() {"page-item disabled"} else { "page-item" }}>
                <a class="page-link"  aria-label="Next" href="javascript:void(0);" onclick = {ctx.link().callback(|_|UserListPageMsg::Next)}>
                    <span aria-hidden="true">{">"}</span>
                </a>
                </li>
                <li class={if page.index==page.page_total() {"page-item disabled"} else { "page-item" }}>
                <a class="page-link"  aria-label="Next" href="javascript:void(0);" onclick = {ctx.link().callback(|_|UserListPageMsg::Last)}>
                    <span aria-hidden="true">{">>"}</span>
                </a>
                </li>
                <li class="page-item">
                <select class="form-select" onchange={ctx.link().callback(|e:web_sys::Event|UserListPageMsg::SizeChanged(e))} aria-label="Default select example">
                    <option selected = {true}>{15}</option>
                    <option>{20}</option>
                    <option>{30}</option>
                    <option>{50}</option>
                </select>
                </li>
                <li class="page-item">
                <a href="javascript:void(0);" class="page-link"  aria-label="Next">
                    <span aria-hidden="true">{format!("page total: {}",page.page_total())}</span>
                </a>
                </li>
                <li class="page-item">
                <a href="javascript:void(0);" class="page-link"  aria-label="Next">
                    <span aria-hidden="true">{format!("total: {}",page.total)}</span>
                </a>
                </li>
            </ul>
            </nav>
        }
    }
}
