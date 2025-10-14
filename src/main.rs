use iced::{
    Length::Fill,
    widget::{button, column, container, row, text, text_editor, text_input, pick_list},
};

#[derive(Default)]
struct AppState {
    url_content: String,
    editor_content: text_editor::Content,
    selected_verb: Verb,
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
enum Verb {
    #[default]
    Get,
    Post,
    Patch,
    Put,
    Delete
}

impl std::fmt::Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verb::Get => write!(f, "GET"),
            Verb::Post => write!(f, "POST"),
            Verb::Patch => write!(f, "PATCH"),
            Verb::Put => write!(f, "PUT"),
            Verb::Delete => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    UrlFieldChanged(String),
    ResponseContentChanged(text_editor::Action),
    SendRequest,
    RequestCompleted(Result<String, std::sync::Arc<reqwest::Error>>),
    VerbSelected(Verb),
}

fn update(state: &mut AppState, message: Message) -> iced::Task<Message>{
    match message {
        Message::UrlFieldChanged(new_value) => {
            state.url_content = new_value;
            iced::Task::none()
        }
        Message::ResponseContentChanged(action) => {
            state.editor_content.perform(action);
            iced::Task::none()
        }
        Message::SendRequest => {
            let url = state.url_content.clone();
            let verb = state.selected_verb.clone();
            iced::Task::perform(
                async move {
                    fetch_url(url, verb)
                        .await
                        .map_err(|e| std::sync::Arc::new(e))
                },
                Message::RequestCompleted,
            )
        },
        Message::RequestCompleted(Ok(response_body)) => {
            state.editor_content = text_editor::Content::with_text(&response_body);
            iced::Task::none()
        }
        Message::RequestCompleted(Err(error)) => {
            state.editor_content = text_editor::Content::with_text(&format!("Error: {}", error));
            iced::Task::none()
        }
        Message::VerbSelected(verb) => {
            state.selected_verb = verb;
            iced::Task::none()
        }
    }
}

fn view(state: &'_ AppState) -> iced::Element<'_, Message> {
    let verbs = [
        Verb::Get,
        Verb::Post,
        Verb::Patch,
        Verb::Put,
        Verb::Delete,
    ];


    container(
        column![
            row![
                pick_list(verbs, Some(&state.selected_verb), Message::VerbSelected)
                    .width(100),
                text_input("URL", &state.url_content)
                    .on_input(Message::UrlFieldChanged)
                    .width(Fill),
                button(text("Run").center())
                    .width(100)
                    .on_press(Message::SendRequest)
            ]
            .spacing(10),
            text_editor(&state.editor_content)
                .on_action(Message::ResponseContentChanged)
                .height(Fill)
        ]
        .spacing(10),
    )
    .padding(10)
    .into()
}

pub fn main() -> iced::Result {
    iced::application("maildude", update, view)
        .theme(|_| iced::Theme::KanagawaDragon)
        .run()
}

async fn fetch_url(_url: String, _verb: Verb) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let request = match _verb {
        Verb::Get => client.get(_url),
        Verb::Post => client.post(_url),
        Verb::Patch => client.patch(_url),
        Verb::Put => client.put(_url),
        Verb::Delete => client.delete(_url),
    };
    let response = request.send().await?;
    let body = response.text().await?;
    Ok(body)
}