

use reqwasm::http::{Request};
use serde::Deserialize;
use yew::{Component, html, Html};

struct TodoApp {
    // link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>,
    // fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, reqwasm::Error>),
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
        let link = ctx.link();

        let todo_list = match &self.todos {
            Some(todos) => todos.clone(),
            None => vec![]
        };

        html! {
            <div>
                <h1>{ "Hello World!" }</h1>
                <button onclick={link.callback(|_| Msg::MakeReq)}></button>
            { todo_list.into_iter().map(|_| {
                html! { <p>{ "1" }</p> }
            }).collect::<Html>() }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<TodoApp>();
}
