// Thanks Bart Massey for providing a frontend template
use crate::*;

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionStruct {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<HashSet<String>>,
}

impl QuestionStruct {
    pub async fn get_question(key: Option<String>) -> Msg {
        let request: String = match &key {
            None => "http://localhost:3000/api/v1/question".to_string(),
            Some(ref key) => format!("http://localhost:3060/api/v1/question/{}", key,),
        };
        let response: Result<http::Response, gloo_net::Error> =
            http::Request::get(&request).send().await;
        match response {
            Err(e) => Msg::GotQuestion(Err(e)),
            Ok(data) => Msg::GotQuestion(data.json().await),
        }
    }
}
pub fn format_tags(tags: &HashSet<String>) -> String {
    let taglist: Vec<&str> = tags.iter().map(String::as_ref).collect();
    taglist.join(", ")
}

#[derive(Properties, Clone, PartialEq, serde::Deserialize)]
pub struct QuestionProps {
    pub question: QuestionStruct,
}

#[function_component(Question)]
pub fn question(question: &QuestionProps) -> Html {
    let question = &question.question;
    html! { <>
        <div class="question">
            <span class="title">{question.title.clone()}</span><br/>
            <span class="content">{&question.content}</span><br/>
        </div>
        <span class="annotation">
            {format!("[id: {}", &question.id)}
            if let Some(ref tags) = question.tags {
                {format!("; tags: {}", &format_tags(tags))}
            }
            {"]"}
        </span>
    </> }
}
