use iced::event::{self, Event};
use iced::Theme;
use iced::{window, Element, Settings, Subscription, Task};
use icy_browser::{
    get_fonts, widgets, BrowserWidget, KeyType, Message as WidgetMessage, NoCustomView,
    ShortcutBuilder, ShortcutModifier, Shortcuts, Ultralight,
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
        .theme(|_| Theme::CatppuccinMacchiato)
        .run_with(Browser::new)
}

#[derive(Debug, Clone)]
pub enum Message {
    BrowserWidget(widgets::Message),
    Update,
    Event(Event),
}

struct Browser {
    widgets: BrowserWidget<Ultralight, NoCustomView>,
}

impl Browser {
    fn new() -> (Self, Task<Message>) {
        let shortcuts = create_shortcuts();
        let widgets = BrowserWidget::new()
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

fn create_shortcuts() -> Shortcuts {
    ShortcutBuilder::new()
        .add_shortcut(
            WidgetMessage::ToggleOverlay,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("e".into())),
            ],
        )
        .add_shortcut(
            WidgetMessage::CreateTab,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("t".into())),
            ],
        )
        .add_shortcut(
            WidgetMessage::CloseCurrentTab,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("q".into())),
            ],
        )
        .build()
}
