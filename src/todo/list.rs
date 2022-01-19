use yew::{Component, html, Html, classes, Properties};

use crate::{Todo};


pub struct  List;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub todos: Option<Vec<Todo>>
}

impl Component for List {
    type Properties = Props;
    type Message = ();

    fn view(&self, ctx: &yew::Context<Self>) -> Html {

        html! {
            <div>
                { self.render_list(&ctx.props().todos) }
            </div>
        }
    }

    fn create(_ctx: &yew::Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
       true
    }
}

impl List {
    fn render_list(&self, todos: &Option<Vec<Todo>>) -> Html {
        if let Some(t) = todos {
            html! {
                <div class={ classes!("list") }>
                    { t.iter().map(|todo| self.view_todo(todo)).collect::<Html>() }
                </div>
            }
        } else {
            html! {
                <div class={ classes!("loading") }>{ "loading..." }</div>
            }
        }
    }

    fn view_todo(&self, todo: &Todo) -> Html {
        let completed = if todo.completed {
            Some("completed")
        } else {
            None
        };

        html! {
            <div class={ classes!("list-item", completed) }>
                { &todo.title }
            </div>
        }
    }
}
