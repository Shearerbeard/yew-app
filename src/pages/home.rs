use yew::{function_component, html, use_context, classes};

use crate::{todo, TodoContext};


#[function_component(Home)]
pub fn home() -> Html {
    let app_ctx = use_context::<TodoContext>().expect("Todo Context");
    let onclick = app_ctx.refresh;

    html! {
        <div>
            <div class={classes!("refresh")}>
                <button { onclick }>
                { "refresh" }
                </button>
            </div>
            <todo::list::List todos={app_ctx.todos.clone()}/>
        </div>
    }
}
