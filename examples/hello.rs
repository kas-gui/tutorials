use kas::widgets::{TextButton, Window};

fn main() -> Result<(), kas::shell::Error> {
    env_logger::init();

    let content = TextButton::new("Push me");
    let window = Window::new("Hello", content);

    let theme = kas::theme::FlatTheme::new();
    kas::shell::Toolkit::new(theme)?.with(window)?.run()
}
