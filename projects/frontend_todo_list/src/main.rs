use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use web_sys::console;
use wasm_bindgen_futures; // Add this import

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(|| vec![]);
    let loading = use_state(|| true);

   {
        let todos = todos.clone();
        let loading = loading.clone();
        
        use_effect_with((), move |_| {
            // Spawn the async task
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://localhost:8080/todos")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.json::<Vec<Todo>>().await {
                            Ok(data) => {
                                todos.set(data);
                                loading.set(false);
                            }
                            Err(e) => {
                                console::error_1(&format!("Failed to parse JSON: {:?}", e).into());
                                loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        console::error_1(&format!("Failed to fetch todos: {:?}", e).into());
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }


    html! {
        <div style="font-family: sans-serif; padding: 2em;">
            <h1>{ "📝 Yew Todo App" }</h1>
            if *loading {
                <p>{ "Loading todos..." }</p>
            } else {
                <div>
                    <h2>{ "Your Todos:" }</h2>
                    <ul style="list-style: none; padding: 0;">
                        { for todos.iter().map(|todo| html! {
                            <li key={todo.id} style="padding: 10px; border-bottom: 1px solid #eee;">
                                <span style={if todo.completed {"text-decoration: line-through; color: #888;"} else {""}}>
                                    { &todo.title }
                                </span>
                                <span style="margin-left: 10px;">
                                    { if todo.completed { "✅" } else { "❌" } }
                                </span>
                            </li>
                        })}
                    </ul>
                    if todos.is_empty() {
                        <p>{ "No todos found. Make sure your backend is running!" }</p>
                    }
                </div>
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}