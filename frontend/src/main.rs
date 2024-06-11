// Thanks Bart Massey for providing a frontend template
mod cookie;
mod finder;
mod question;

use cookie::*;
use finder::*;
use question::*;

use std::collections::HashSet;

extern crate serde;
// use gloo_console::log;
use gloo_net::http;
extern crate wasm_bindgen_futures;
use wasm_cookies as cookies;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

/// Represents the result of a question operation.
/// It can either be a successful QuestionStruct or an error.
pub type QuestionResult = Result<QuestionStruct, gloo_net::Error>;

/// Represents the main application state.
/// It contains a cookie string and a question result.
struct App {
    cookie: String,
    question: QuestionResult,
}

/// Represents the different types of messages that can be processed by the application.
pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

impl App {
    /// Refreshes the question in the application.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the App component.
    /// * `key` - An optional string that may be used to get a specific question.
    ///
    /// This function sends a future to the component's link, which will be resolved when the question is retrieved.
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        let got_question = QuestionStruct::get_question(key);
        ctx.link().send_future(got_question);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    /// Creates a new App component.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the App component.
    ///
    /// # Returns
    ///
    /// * `Self` - The newly created App component.
    fn create(ctx: &Context<Self>) -> Self {
        let cookie: String = acquire_cookie();
        App::refresh_question(ctx, None);
        let question: Result<_, gloo_net::Error> =
            Err(gloo_net::Error::GlooError("Loading Question…".to_string()));
        Self { cookie, question }
    }

    /// Handles the messages sent to the App component.
    ///
    /// # Arguments
    ///
    /// * `msg` - The message to be handled.
    /// * `ctx` - The context of the App component.
    ///
    /// # Returns
    ///
    /// * `ShouldRender` - A boolean indicating whether the component should be re-rendered.
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GotQuestion(question) => {
                self.question = question;
                true
            }
            Msg::GetQuestion(key) => {
                // log!(format!("GetQuestion: {:?}", key));
                App::refresh_question(ctx, key);
                false
            }
        }
    }

    /// Renders the App component.
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the App component.
    ///
    /// # Returns
    ///
    /// * `Html` - The HTML representation of the App component.
    fn view(&self, ctx: &Context<Self>) -> Html {
        let cookie: &String = &self.cookie;
        let question: &Result<_, gloo_net::Error> = &self.question;
        html! {
        <>
            <h1>{ "question" }</h1>
            if false {
                {render_cookie(cookie)}
            }
            if let Ok(ref question) = question {
                <Question question={question.clone()}/>
            }
            if let Err(ref error) = question {
                <div>
                    <span class="error">{format!("Server Error: {error}")}</span>
                </div>
            }
            <div>
                <button onclick={ctx.link().callback(|_| Msg::GetQuestion(None))}>{"Ask another question."}</button>
            </div>
            <Finder on_find={ctx.link().callback(Msg::GetQuestion)}/>
        </>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
