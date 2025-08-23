use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

#[function_component(App)]
pub fn app() -> Html {
    let messages = use_state(|| vec![]);
    let input = use_state(|| String::new());
    let ws_ref = use_mut_ref(|| None::<WebSocket>);

    let append_msg = {
        let messages = messages.clone();
        Callback::from(move |msg: String| {
            let mut new = (*messages).clone();
            new.push(msg);
            messages.set(new);
        })
    };

    {
        let append_msg = append_msg.clone();
        let ws_ref = ws_ref.clone();
        use_effect(move || {
            let ws = WebSocket::new("ws://localhost:9001").expect("Failed to connect to WebSocket");
            
            let onmessage = Closure::<dyn FnMut(_)>::wrap(Box::new(move |e: MessageEvent| {
                if let Some(text) = e.data().as_string() {
                    append_msg.emit(text);
                }
            }) as _);

            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            onmessage.forget();
            *ws_ref.borrow_mut() = Some(ws);
            
            || ()
        });
    }

    let oninput = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            input.set(val);
        })
    };

    let onclick = {
        let input = input.clone();
        let ws_ref = ws_ref.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(ws) = &*ws_ref.borrow() {
                if !input.is_empty() {
                    ws.send_with_str(&input).expect("Failed to send message");
                }
            }
            input.set(String::new());
        })
    };

    html! {
        <div style="font-family: sans-serif; max-width: 500px; margin: 3em auto;">
            <h1>{ "💬 Yew Chat Client" }</h1>
            <div style="border: 1px solid #ccc; padding: 1em; height: 300px; overflow-y: scroll;">
                { for messages.iter().map(|m| html! {
                    <p>{ m }</p>
                })}
            </div>
            <input 
                style="width: 70%;" 
                value={(*input).clone()} 
                oninput={oninput} 
                onkeypress={move |e: KeyboardEvent| {
                    if e.key() == "Enter" {
                        if let Some(ws) = &*ws_ref.borrow() {
                            if !input.is_empty() {
                                ws.send_with_str(&input).expect("Failed to send message");
                                input.set(String::new());
                            }
                        }
                    }
                }}
            />
            <button onclick={onclick} style="width: 28%; margin-left: 2%;">{ "Send" }</button>
        </div>
    }
}