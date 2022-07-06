use web_sys::{MouseEvent};
use yew::{function_component, html, Callback, Properties};


#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub text: String,
    pub onclick: Option<Callback<MouseEvent>>,
    pub disabled: Option<bool>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = vec!["bg-gray-400", 
        "dark:text-slate-50", "dark:bg-slate-900", 
        "hover:bg-gray-600", "hover:dark:bg-slate-700",
        "disabled:bg-gray-400", "disabled:dark:bg-gray-400",
        "rounded-md", 
        "px-4", "py-8"];

    let disabled = if let Some(disabled) = props.disabled {
        disabled
    } else {
        false
    };

    let onclick = props.onclick.clone();

    html!{
        <button
            {disabled}
            {onclick}
            {class}>
            {&props.text}
        </button>
    }
}