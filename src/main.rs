

mod todo;
mod pages;

use reqwasm::http::{Request};
use serde::Deserialize;
use yew::{Component, html, Html, classes, Callback, ContextProvider, MouseEvent};
use yew_router::prelude::*;

#[derive(Clone, PartialEq)]
struct TodoApp {
    todos: Option<Vec<Todo>>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool
}


#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/todo/:id")]
    Detail { id: String },
    #[at("/")]
    Home,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, reqwasm::Error>),
}

fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => {
            html! {
                <h1>{ "Home" }</h1>
            }
            // html! {
            //     <div>
            //         <div class={ classes!("refresh") }>
            //     </div>
            //         <todo::list::List todos={ todos.clone() }/>
            //     </div>
            // }
        }
        AppRoute::Detail { id } => {
            html! {
                <h1>{ "Todo" }</h1>
            }
            // html! {
            //     <div>
            //         <todo::detail::Detail todo_id={ id }/>
            //     </div>
            // }
        }
    }
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        ctx.link().send_message(Msg::MakeReq);

        Self {
            todos: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                let link = ctx.link().clone();
                self.todos = None;

                wasm_bindgen_futures::spawn_local(async move {
                    let req = Request::get("https://jsonplaceholder.typicode.com/todos");
                    let resp = req.send().await.unwrap();
                    let result: Result<Vec<Todo>, reqwasm::Error> = resp.json().await;

                    link.send_message(Msg::Resp(result));
                });
                true
            }

            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <ContextProvider<TodoApp> context={self.clone()} >
                <div class={ classes!("todo") }>
                    <div class={ classes!("nav")}>
                        <Link<AppRoute> to={ AppRoute::Home}>{"Home"}</Link<AppRoute>>
                    </div>
                    <div class={ classes!("content")}>
                        <BrowserRouter>
                            <Switch<AppRoute> render={Switch::render(switch)} />
                        </BrowserRouter>
                    </div>
                </div>
            </ ContextProvider<TodoApp>>
        }
    }
}

fn main() {
    yew::start_app::<TodoApp>();
}
