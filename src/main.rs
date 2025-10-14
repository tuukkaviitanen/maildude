use iced::{
    Length::Fill,
    widget::{button, column, container, pick_list, row, text, text_editor, text_input},
};
use reqwest::Method;

#[derive(Default)]
struct AppState {
    url_content: String,
    editor_content: text_editor::Content,
    response_content: text_editor::Content,
    selected_method: Method,
}

#[derive(Debug, Clone)]
enum Message {
    UrlFieldChanged(String),
    ResponseContentChanged(text_editor::Action),
    SendRequest,
    RequestCompleted(Result<String, std::sync::Arc<reqwest::Error>>),
    MethodSelected(Method),
}

fn update(state: &mut AppState, message: Message) -> iced::Task<Message> {
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
            let method = state.selected_method.clone();
            let request_body = if state.editor_content.text().is_empty() {
                None
            } else {
                Some(state.editor_content.text().to_string())
            };
            iced::Task::perform(
                async move {
                    send_request(url, method, request_body)
                        .await
                        .map_err(|e| std::sync::Arc::new(e))
                },
                Message::RequestCompleted,
            )
        }
        Message::RequestCompleted(Ok(response_body)) => {
            state.response_content = text_editor::Content::with_text(&response_body);
            iced::Task::none()
        }
        Message::RequestCompleted(Err(error)) => {
            state.editor_content = text_editor::Content::with_text(&format!("Error: {}", error));
            iced::Task::none()
        }
        Message::MethodSelected(method) => {
            state.selected_method = method;
            iced::Task::none()
        }
    }
}

fn view(state: &'_ AppState) -> iced::Element<'_, Message> {
    let methods = [
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::OPTIONS,
        Method::TRACE,
        Method::CONNECT,
    ];

    container(
        column![
            row![
                pick_list(
                    methods,
                    Some(&state.selected_method),
                    Message::MethodSelected
                )
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
                .height(Fill),
            text_editor(&state.response_content).height(Fill)
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

async fn send_request(
    url: String,
    method: Method,
    body: Option<String>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let request = client.request(method, url);
    let request = if let Some(body) = body {
        request.body(body)
    } else {
        request
    };
    let response = request.send().await?;
    let body = response.text().await?;
    Ok(body)
}
