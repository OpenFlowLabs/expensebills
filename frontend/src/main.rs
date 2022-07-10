mod errors;
mod components;
mod state;

use errors::Error;
use log::info;
use reqwest::{Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yew::prelude::*;
use yew_hooks::use_async;
use components::atoms::button::Button;
use ulid::Ulid;
use chrono::NaiveDate;
use gloo_file::{File};
use web_sys::{Event, HtmlInputElement};


static BASE_LOCAL_SERVER_ADDRESS: &str = "http://127.0.0.1:8000/api/v1";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
enum ReceiptOrError {
    Receipt(Receipt),
    Error(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Receipt {
    id: Ulid,
    name: String,
    state: ReceiptState,
    recipient: Option<Recipient>,
    category: Option<String>,
    payment_date: Option<NaiveDate>
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ReceiptState {
    Inbox,
    Valid,
    Payed,
    Declined,
    Process,
    Done
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Recipient {
    id: Ulid,
    name: String,
    iban: String,
    address_line1: String,
    address_line2: String,
    address_line3: String,
    address_line4: String,
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

/// You can use reqwest or other crates to post to your api.
async fn post<T>(url: String, form: reqwest::multipart::Form) -> Result<T, Error>
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
        client.request(Method::POST, url)
        .header("Accept", "application/json")
        .multipart(form);
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

async fn upload_file(file: reqwest::multipart::Form) -> Result<Receipt, Error> {
    post(format!("{}/receipts/upload", BASE_LOCAL_SERVER_ADDRESS), file).await
}

#[function_component(Greeter)]
fn greeter() -> Html {
    let input_ref = use_node_ref();

    let upload_handler = { 
        let input_ref = input_ref.clone();
        use_async(async move {
            let input = input_ref
            .cast::<HtmlInputElement>()
            .expect("input_ref not attached to input element");
            
            if let Some(files) = input.files() {
                let mut result: Vec<ReceiptOrError> = vec![];
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from)
                    .collect::<Vec<File>>();
                for file in files {
                    let bytes = gloo_file::futures::read_as_bytes(&file).await.unwrap();
                    let file_part = reqwest::multipart::Part::bytes(bytes).file_name(file.name());
                    let multipart = reqwest::multipart::Form::new()
                        .part("file", file_part)
                        .text("name", file.name());
                    match upload_file(multipart).await {
                        Ok(v) => result.push(ReceiptOrError::Receipt(v)),
                        Err(e) => result.push(ReceiptOrError::Error(e.to_string())),
                    }   
                }

                input.set_files(None);
                input.set_value("");
                Ok(result)
            } else {

                input.set_files(None);
                input.set_value("");
                Err(Error::FrontendError("no files selected".to_string()))
            }
        })
    };

    let onclick = {
        let upload_handler = upload_handler.clone();
        Callback::from(move |_| {
            // You can trigger to run in callback or use_effect.
            upload_handler.run();
        })
    };

    html! {
        <div class={vec!["p-6", "mx-auto",
                "rounded-md", "shadow-lg", "flex", "items-center", "space-x-12"]}>
            <input ref={input_ref} type={"file"} name={"receipts"} />
            <Button onclick={onclick} disabled={upload_handler.loading} text={"Start to send greeting"}></Button>
            <p>
                {
                    if upload_handler.loading {
                        html! { "Getting greeting from Server" }
                    } else {
                        html! {}
                    }
                }
            </p>
            {
                if let Some(receipts) = &upload_handler.data {
                    html! {
                        <>
                        {
                            receipts.iter().map(|receipt_or_error| {
                                    match receipt_or_error {
                                        ReceiptOrError::Receipt(receipt) => html!{<p>{ "Receipt Id: " }<b>{ &receipt.id }</b></p>},
                                        ReceiptOrError::Error(err) => html!{<p>{ "Error: " }<b>{ &err }</b></p>}                                        ,
                                    }
                            }).collect::<Html>()
                        }
                        </>
                        }
                } else {
                    html! {}
                }
            }
            <p>
                {
                    if let Some(error) = &upload_handler.error {
                        match error {
                            Error::RequestError(t)=>html!{<span>{"RequestError: "}{&t}</span>},
                            Error::DeserializeError(s)=>html!{<span>{"DeserializeError: "}{&s}</span>},
                            Error::FrontendError(e) => html!{<span>{"FrontendError: "}{&e}</span>}, }
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
