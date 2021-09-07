use kas::event::VoidMsg;
use kas::widgets::{MessageBox, TextButton, Window};

fn main() -> Result<(), kas::shell::Error> {
    env_logger::init();

    let content = TextButton::new("&Push me").on_push::<VoidMsg, _>(|mgr| {
        let mbox = MessageBox::new("Message", "You pushed the button.");
        mgr.add_window(Box::new(mbox));
        None
    });
    let window = Window::new("Hello", content);

    let theme = kas::theme::ShadedTheme::new();
    kas::shell::Toolkit::new(theme)?.with(window)?.run()
}
