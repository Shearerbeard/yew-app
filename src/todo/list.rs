use yew::{Component, html, Html, classes, Properties};

use crate::Todo;


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub todos: Option<Vec<Todo>>,
}


pub struct  List {
    pub props: Props,
}

pub enum  Msg {}

impl Component for List {
    type Properties = Props;
    type Message = Msg;

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        html! {
            <div>
                { self.render_list(&self.props.todos) }
            </div>
        }
    }

    fn create(_ctx: &yew::Context<Self>) -> Self {

        let props = Props {
            todos: None
        };

        Self { props }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, _msg: Self::Message) -> bool {
       true
    }

    fn changed(&mut self, ctx: &yew::Context<Self>) -> bool {
        self.props = ctx.props().clone();
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
