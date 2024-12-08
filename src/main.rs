use iced::alignment::Horizontal::Right;
use iced::widget::{center, checkbox, column, responsive, text};
use iced::{Element, Task};

mod ellipsize;

fn main() -> iced::Result {
    iced::application("iced window handler", App::update, App::view)
        .window(iced::window::Settings {
            size: (300.0, 300.0).into(),
            min_size: Some((200.0, 200.0).into()),
            ..Default::default()
        })
        .centered()
        .run()
}

struct App {
    ellipsize: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { ellipsize: true }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ToggleEllipsize(bool),
}

impl App {
    fn view(&self) -> Element<'_, Message> {
        column![
            checkbox("Ellipsize", self.ellipsize).on_toggle(Message::ToggleEllipsize),
            center(responsive(move |size| {
                Element::from(ellipsize::Content::new(
                    LIPSUM.to_string(),
                    if self.ellipsize {
                        size
                    } else {
                        iced::Size::INFINITY
                    },
                ))
            }))
        ]
        .padding(20)
        .spacing(10)
        .align_x(Right)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToggleEllipsize(ellipsize) => {
                self.ellipsize = ellipsize;
            }
        }

        Task::none()
    }
}

// pub static LIPSUM: &str = "Lorem ipsum dolor sit amet.";

pub static LIPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing\
elit. Integer sit amet risus lorem. Fusce varius sem ut risus tincidunt mollis. \
Etiam scelerisque neque non libero suscipit, nec blandit enim maximus. Proin\
mattis diam porttitor nisl semper luctus. Vestibulum ante ipsum primis in\
faucibus orci luctus et ultrices posuere cubilia curae; Quisque cursus nec metus\
vel faucibus. Suspendisse egestas gravida dui eget consequat. Nam ut volutpat\
nibh. Quisque semper orci leo, placerat molestie nisl vestibulum tincidunt. \
Fusce non urna vel urna vestibulum blandit. Donec vulputate auctor lorem. Mauris\
vulputate vehicula rhoncus. Donec est enim, laoreet ut turpis quis, lobortis\
porttitor ex.";
