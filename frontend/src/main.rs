mod errors;

use errors::Error;
use log::info;
use reqwest::Method;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Greeting {
    name: String,
    greeting: String,
}

/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let url = match reqwest::Url::parse(&url) {
        Ok(v) => Ok(v),
        Err(err) => Err(Error::RequestError(err.to_string())),
    }?;
    info!("{:?}", url);
    let client = reqwest::Client::new();
    let request =
        client.request(Method::GET, url).header("Accept", "application/json");
    info!("{:?}", &request);
    let response = request.send().await;
    info!("{:?}", &response);
    match response {
        Ok(resp) => match resp.json::<T>().await {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::DeserializeError(err.to_string())),
        },
        Err(err) => Err(Error::RequestError(err.to_string())),
    }
}

async fn fetch_greeting(name: String) -> Result<Greeting, Error> {
    fetch::<Greeting>(format!("http://127.0.0.1:8000/hello/{}", name)).await
}

#[function_component(Greeter)]
fn greeter() -> Html {
    let state = use_async(async move { fetch_greeting("Yew".into()).await });

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            // You can trigger to run in callback or use_effect.
            state.run();
        })
    };

    html! {
        <div>
            <button {onclick} disabled={state.loading}>{ "Start to send greeting" }</button>
            <p>
                {
                    if state.loading {
                        html! { "Getting greeting from Server" }
                    } else {
                        html! {}
                    }
                }
            </p>
            {
                if let Some(greeting) = &state.data {
                    html! {
                        <>
                            <p>{ "Greeting from server: " }<b>{ &greeting.greeting }</b></p>
                        </>
                        }
                } else {
                    html! {}
                }
            }
            <p>
                {
                    if let Some(error) = &state.error {
                        match error {
                            Error::RequestError(t) => html! { <span>{"RequestError: "} {&t}</span> },
                            Error::DeserializeError(s) => html! { <span>{"DeserializeError: "} {&s}</span> },
                        }
                    } else {
                        html! {}
                    }
                }
            </p>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Greeter>();
}
