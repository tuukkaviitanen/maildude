use std::time::Duration;

use iced::{
    widget::{button, column, container::{self, Style}, horizontal_space, pick_list, rich_text, row, scrollable, text, text_editor, text_input}, Border, Length::Fill
};
use reqwest::Method;
use tokio::time::Instant;

#[derive(Default)]
struct App {
    url_content: String,
    editor_content: text_editor::Content,
    response_data: Option<ResponseData>,
    selected_method: Method,
}

#[derive(Debug, Clone)]
struct ResponseData {
    body: String,
    status: reqwest::StatusCode,
    headers: reqwest::header::HeaderMap,
    request_duration: Duration,
}

#[derive(Debug, Clone)]
enum Message {
    UrlFieldChanged(String),
    ResponseContentChanged(text_editor::Action),
    SendRequest,
    RequestCompleted(Result<ResponseData, std::sync::Arc<reqwest::Error>>),
    MethodSelected(Method),
}

impl App {
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::UrlFieldChanged(new_value) => {
                self.url_content = new_value;
                iced::Task::none()
            }
            Message::ResponseContentChanged(action) => {
                self.editor_content.perform(action);
                iced::Task::none()
            }
            Message::SendRequest => {
                let url = self.url_content.clone();
                let method = self.selected_method.clone();
                let request_body = if self.editor_content.text().is_empty() {
                    None
                } else {
                    Some(self.editor_content.text().to_string())
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
            Message::RequestCompleted(Ok(response_data)) => {
                self.response_data = Some(response_data);
                iced::Task::none()
            }
            Message::RequestCompleted(Err(error)) => {
                self.response_data = None;
                iced::Task::none()
            }
            Message::MethodSelected(method) => {
                self.selected_method = method;
                iced::Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        const METHODS: [Method; 9] = [
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

        container::Container::new(
            column![
                row![
                    pick_list(
                        METHODS,
                        Some(&self.selected_method),
                        Message::MethodSelected
                    )
                    .width(115),
                    text_input("URL", &self.url_content)
                        .on_input(Message::UrlFieldChanged)
                        .width(Fill).on_submit(Message::SendRequest),
                    button(text("Run").center())
                        .width(100)
                        .on_press(Message::SendRequest)
                ]
                .spacing(10),
                text("Request body"),
                text_editor(&self.editor_content)
                    .on_action(Message::ResponseContentChanged)
                    .height(Fill),
                row![text("Response"), 
                    self.response_data
                        .as_ref()
                        .map(|data| row!(
                            horizontal_space(),
                            text(format!("{}", data.status)),
                            horizontal_space(),
                            text(format!(
                                "{:.2?}",
                                data.request_duration
                            )),
                        ))
                        .unwrap_or(row![])
                ],
                container::Container::new(scrollable(text(
                    self.response_data
                        .as_ref()
                        .map(|data| data.body.as_str())
                        .unwrap_or_default()
                ).width(Fill)))
                .style(|_| Style {
                    border: Border {
                        width: 1.,
                        color: iced::Color::from_rgb8(100, 100, 100),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .padding(10)
                .height(Fill)
                .width(Fill)
            ]
            .spacing(10),
        )
        .padding(10)
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::application("maildude", App::update, App::view)
        .theme(|_| iced::Theme::Dark)
        .run()
}

async fn send_request(
    url: String,
    method: Method,
    body: Option<String>,
) -> Result<ResponseData, reqwest::Error> {
    let client = reqwest::Client::new();
    let request = client.request(method, url);
    let request = if let Some(body) = body {
        request.body(body)
    } else {
        request
    };

    let start = Instant::now();
    let response = request.send().await?;
    let request_duration = start.elapsed();

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;

    Ok(ResponseData {
        body,
        status,
        headers,
        request_duration,
    })
}
