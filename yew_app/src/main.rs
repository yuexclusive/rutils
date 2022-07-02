use gloo_console as console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

struct Model {}
impl Component for Model {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>{"hello"}</div>
        }
    }
}

// struct Msg {}

struct Model2 {
    content: String,
}
impl Component for Model2 {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            content: "world".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: ()) -> bool {
        self.content = format!("{}, 1", self.content);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
           <>
             <div>{self.content.as_str()}</div>
             <input type="button" value="click" onclick = {ctx.link().callback(|_|())}/>
           </>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
    yew::Renderer::<Model2>::new().render();
}
