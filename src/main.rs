use iced::event::{self, Event};
use iced::Theme;
use iced::{window, Element, Settings, Subscription, Task};
use icy_browser::{
    get_fonts, IcyBrowser, KeyType, 
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
    IcyBrowser(icy_browser::Message),
    Update,
    Event(Event),
}

struct Browser {
    widgets: IcyBrowser<Ultralight>,
}

impl Browser {
    fn new() -> (Self, Task<Message>) {
        let shortcuts = create_shortcuts();
        let widgets = IcyBrowser::new()
            .with_custom_shortcuts(shortcuts)
            .with_homepage("https://search.sawyer.services")
            .build();

        (Self { widgets }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::IcyBrowser(msg) => self.widgets.update(msg).map(Message::IcyBrowser),
            Message::Update => self.widgets.force_update().map(Message::IcyBrowser),
            Message::Event(event) => self
                .widgets
                .update(icy_browser::Message::IcedEvent(Some(event)))
                .map(Message::IcyBrowser),
        }
    }

    fn view(&self) -> Element<Message> {
        self.widgets.view().map(Message::IcyBrowser)
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
            icy_browser::Message::ToggleOverlay,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("e".into())),
            ],
        )
        .add_shortcut(
            icy_browser::Message::CreateTab,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("t".into())),
            ],
        )
        .add_shortcut(
            icy_browser::Message::CloseCurrentTab,
            vec![
                KeyType::Modifier(ShortcutModifier::Ctrl),
                KeyType::Key(iced::keyboard::Key::Character("q".into())),
            ],
        )
        .build()
}
