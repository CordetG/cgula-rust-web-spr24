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

pub type QuestionResult = Result<QuestionStruct, gloo_net::Error>;

struct App {
    cookie: String,
    question: QuestionResult,
}

pub enum Msg {
    GotQuestion(QuestionResult),
    GetQuestion(Option<String>),
}

impl App {
    fn refresh_question(ctx: &Context<Self>, key: Option<String>) {
        let got_question = QuestionStruct::get_question(key);
        ctx.link().send_future(got_question);
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cookie: String = acquire_cookie();
        App::refresh_question(ctx, None);
        let question: Result<_, gloo_net::Error> =
            Err(gloo_net::Error::GlooError("Loading Question…".to_string()));
        Self { cookie, question }
    }

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
