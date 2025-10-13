use iced::{
    Length::Fill,
    widget::{button, column, container, row, text, text_editor, text_input, combo_box},
};

#[derive(Default)]
struct AppState {
    url_content: String,
    editor_content: text_editor::Content,
    editor_disabled: bool,
    verbs: combo_box::State<Verb>,
    selected_verb: Verb,
}

#[derive(Default, Clone, Debug)]
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
    ButtonPressed,
    VerbSelected(Verb),
}

fn update(state: &mut AppState, message: Message) {
    match message {
        Message::UrlFieldChanged(new_value) => {
            state.url_content = new_value;
        }
        Message::ResponseContentChanged(action) => {
            state.editor_content.perform(action);
        }
        Message::ButtonPressed => {
            let verb_and_url = format!("{} {}", state.selected_verb, state.url_content);
            state.editor_content = text_editor::Content::with_text(&verb_and_url);
        }
        Message::VerbSelected(verb) => {
            state.selected_verb = verb;
        }
    }
}

fn view(state: &'_ AppState) -> iced::Element<'_, Message> {
    container(
        column![
            row![
                combo_box(&state.verbs, "VERB", Some(&state.selected_verb), Message::VerbSelected)
                    .width(100),
                text_input("URL", &state.url_content)
                    .on_input(Message::UrlFieldChanged)
                    .width(Fill),
                button(text("Run").center())
                    .width(100)
                    .on_press(Message::ButtonPressed)
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
