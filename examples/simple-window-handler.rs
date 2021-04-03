use kas::event::VoidMsg;
use kas::widget::{TextButton, Window, MessageBox};

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let content = TextButton::new("&Push me")
        .on_push::<VoidMsg, _>(|mgr| {
            let mbox = MessageBox::new("Message", "You pushed the button.");
            mgr.add_window(Box::new(mbox));
            None
        });
    let window = Window::new("Simple window", content);

    let theme = kas_theme::ShadedTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
