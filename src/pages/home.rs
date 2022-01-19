use yew::{function_component, html, use_context, classes};

use crate::{TodoApp, todo};


#[function_component(Home)]
fn home() -> Html {
    let app_ctx = use_context::<TodoApp>().expect("Todo Context");
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
