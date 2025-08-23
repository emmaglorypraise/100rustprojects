use yew::prelude::*;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;

#[derive(Clone, PartialEq)]
struct Metrics {
    users: u32,
    orders: u32,
    revenue: f64,
}

#[derive(Properties, PartialEq, Clone)]
struct MetricCardProps {
    pub label: String,
    pub value: String,
}

#[function_component(MetricCard)]
fn metric_card(props: &MetricCardProps) -> Html {
    html! {
        <div style="padding: 1em; border-radius: 8px; background: #f4f4f4; width: 30%; margin: 1%;">
            <h3>{ &props.label }</h3>
            <p style="font-size: 1.4em; font-weight: bold;">{ &props.value }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let data = use_state(|| Metrics {
        users: 0,
        orders: 0,
        revenue: 0.0,
    });

    {
        let data = data.clone();
        use_effect(move || {
            Timeout::new(1000, move || {
                data.set(Metrics {
                    users: 1324,
                    orders: 768,
                    revenue: 125430.75,
                });
            })
            .forget();
            || ()
        });
    }

    html! {
        <div style="font-family: sans-serif; padding: 2em;">
            <h1>{ "📊 Admin Dashboard" }</h1>
            <div style="display: flex; justify-content: space-between;">
                <MetricCard 
                    label={"Users".to_string()}
                    value={format!("{}", data.users)} 
                />
                <MetricCard 
                    label={"Orders".to_string()}
                    value={format!("{}", data.orders)} 
                />
                <MetricCard 
                    label={"Revenue".to_string()}
                    value={format!("${:.2}", data.revenue)} 
                />
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}