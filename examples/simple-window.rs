use kas::widget::{EditBox, Window};

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let content = EditBox::new("0");
    let window = Window::new("Simple window", content);

    let theme = kas_theme::ShadedTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
