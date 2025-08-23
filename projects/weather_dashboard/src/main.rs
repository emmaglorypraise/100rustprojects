use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures; 

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Main {
    temp: f64,
    humidity: u8,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Weather {
    description: String,
}

#[function_component(App)]
fn app() -> Html {
    let city = use_state(|| "London".to_string());
    let weather = use_state(|| None::<WeatherData>);
    let input = use_state(|| city.to_string());
    let fetch_counter = use_state(|| 0); 

    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
            input.set(val);
        })
    };

    let on_click = {
        let city = city.clone();
        let input = input.clone();
        let fetch_counter = fetch_counter.clone();
        Callback::from(move |_| {
            city.set((*input).clone());
            fetch_counter.set(*fetch_counter + 1); 
        })
    };

    {
        let weather = weather.clone();
        let current_city = (*city).clone();
        
        use_effect_with(*fetch_counter, move |_| {
            if !current_city.is_empty() {
                let weather = weather.clone();
                let city_clone = current_city.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    // Replace with your actual API key
                    let url = format!(
                        "https://api.openweathermap.org/data/2.5/weather?q={}&units=metric&appid=YOUR_API_KEY
",
                        city_clone
                    );
                    
                    weather.set(None);
                    
                    match Request::get(&url).send().await {
                        Ok(response) => {
                            if response.status() == 200 {
                                match response.json::<WeatherData>().await {
                                    Ok(data) => {
                                        weather.set(Some(data));
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(&format!("JSON parse error: {:?}", e).into());
                                    }
                                }
                            } else {
                                web_sys::console::error_1(&format!("HTTP error: {}", response.status()).into());
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Request error: {:?}", e).into());
                        }
                    }
                });
            }
            
            || ()
        });
    }

    html! {
        <div style="font-family: sans-serif; text-align: center; padding-top: 2em;">
            <h1>{ "🌦️ Weather Dashboard" }</h1>
            <input 
                value={(*input).clone()} 
                oninput={on_input} 
                placeholder="Enter city..." 
                style="padding: 8px; margin-right: 8px;"
            />
            <button 
                onclick={on_click} 
                style="padding: 8px 16px; background: #007acc; color: white; border: none; border-radius: 4px; cursor: pointer;"
            >
                { "Get Weather" }
            </button>

            {
                if let Some(data) = &*weather {
                    html! {
                        <div style="margin-top: 2em; padding: 20px; background: #f5f5f5; border-radius: 8px; max-width: 400px; margin-left: auto; margin-right: auto;">
                            <h2>{ format!("Weather in {}", data.name) }</h2>
                            <p>{ format!("🌡️ Temp: {:.1}°C", data.main.temp) }</p>
                            <p>{ format!("💧 Humidity: {}%", data.main.humidity) }</p>
                            <p>{ format!("🌤️ Description: {}", data.weather[0].description) }</p>
                        </div>
                    }
                } else if *fetch_counter > 0 {
                    html! { <p style="margin-top: 2em;">{ "🔄 Loading weather data..." }</p> }
                } else {
                    html! { <p style="margin-top: 2em; color: #666;">{ "Enter a city and click 'Get Weather'" }</p> }
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}