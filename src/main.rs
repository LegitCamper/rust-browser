use iced::event::{self, Event};
use iced::Theme;
use iced::{window, Element, Settings, Subscription, Task};
use icy_browser::{
    get_fonts, widgets, BrowserWidget, KeyType, Message as WidgetMessage, ShortcutBuilder,
    ShortcutModifier, Ultralight,
};
use std::time::Duration;

fn main() -> iced::Result {
    let settings = Settings {
        fonts: get_fonts(),
        ..Default::default()
    };
    let window = window::Settings {
        decorations: false,
        // icon: todo!(),
        ..Default::default()
    };
    iced::application("Icy Rust Browser", Browser::update, Browser::view)
        .subscription(Browser::subscription)
        .settings(settings)
        .window(window)
        .antialiasing(true)
        .theme(|_| Theme::Dark)
        .run_with(Browser::new)
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message),
    Update,
    Event(Event),
}

struct Browser {
    widgets: BrowserWidget<Ultralight>,
}

impl Browser {
    fn new() -> (Self, Task<Message>) {
        let shortcuts = ShortcutBuilder::new()
            .add_shortcut(
                WidgetMessage::ToggleOverlay,
                vec![
                    KeyType::Modifier(ShortcutModifier::Ctrl),
                    KeyType::Key(iced::keyboard::Key::Character("e".into())),
                ],
            )
            .build();
        let widgets = BrowserWidget::new_with_ultralight()
            .with_custom_shortcuts(shortcuts)
            .with_homepage("https://search.sawyer.services")
            .build();

        (Self { widgets }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BrowserWidget(msg) => self.widgets.update(msg).map(Message::BrowserWidget),
            Message::Update => self.widgets.force_update().map(Message::BrowserWidget),
            Message::Event(event) => self
                .widgets
                .update(widgets::Message::Event(Some(event)))
                .map(Message::BrowserWidget),
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::BrowserWidget)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(Duration::from_millis(10)).map(move |_| Message::Update),
            event::listen().map(Message::Event),
        ])
    }
}
