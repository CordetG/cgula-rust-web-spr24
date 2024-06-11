//! `finder.rs`
//!
//! This module contains the Finder component and its associated properties.
//! The Finder component is a functional component that represents a Finder.
//! It takes a FinderProps struct as properties, which includes a callback function to be called when a find operation is performed.
//!
//! The Finder function returns an Html representation of the Finder component.
//!
//! The FinderProps struct is derived from Properties, Clone, and PartialEq traits.
//! It has a single field `on_find` which is a Callback function that takes an Option<String> as an argument.
//!
//! The Finder function uses a state hook for key which is an Option<String>.
//! It also defines a callback function `change_key` that is triggered on an InputEvent.
//! This function takes the event, extracts the target as an HtmlTextAreaElement, and gets its value as a String.
//! This value is then trimmed and checked if it's empty. If it is, None is set to the state, otherwise, the trimmed value is set.
use crate::*;

#[derive(Properties, Clone, PartialEq)]
/// Represents the properties for the Finder component.
pub struct FinderProps {
    /// Callback function to be called when a find operation is performed.
    pub on_find: Callback<Option<String>>,
}

#[function_component]
/// A functional component that represents a Finder.
///
/// # Arguments
///
/// * `props` - A reference to the properties for the Finder component.
///
/// # Returns
///
/// * `Html` - The HTML representation of the Finder component.
pub fn Finder(props: &FinderProps) -> Html {
    let key: UseStateHandle<Option<String>> = use_state(|| <Option<String>>::None);
    let change_key: Callback<InputEvent> = {
        let key: UseStateHandle<Option<String>> = key.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            let value: String = input.value();
            // log!(format!("key change: {:?}", value));
            let value: &str = value.trim();
            let value: Option<String> = if value.is_empty() {
                None
            } else {
                Some(value.to_string())
            };
            // log!(format!("key change final: {:?}", value));
            key.set(value);
        })
    };
    let props: FinderProps = props.clone();
    html! { <>
        <div>
            <input type="text" placeholder="question id" oninput={change_key}/>
            <button onclick={move |_| props.on_find.emit((*key).clone())}>
                {"Find this question"}
            </button>
        </div>
    </> }
}
