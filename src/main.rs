

mod todo;

use reqwasm::http::{Request};
use serde::Deserialize;
use yew::{Component, html, Html, classes};

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
        let todos = self.todos.clone();
        let cb = ctx.link().callback(|_| Msg::MakeReq);

        html! {
            <div>
                <div class={ classes!("refresh") }>
                    <button onclick={cb.clone()}>
                        { "Refresh" }
                    </button>
                </div>
                <todo::list::List todos={ todos.clone() }/>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<TodoApp>();
}
