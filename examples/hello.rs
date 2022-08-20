use kas::widgets::dialog::MessageBox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = MessageBox::new("Message", "Hello world");

    let theme = kas::theme::FlatTheme::new();
    kas::shell::Toolkit::new(theme)?.with(window)?.run()
}
