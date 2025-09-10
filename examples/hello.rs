use kas::widgets::{column, Button};
use kas::window::Window;

fn main() -> kas::runner::Result<()> {
    let ui = column![
        "Hello, world!",
        Button::label("&Close").with(|cx, _| cx.exit())
    ];
    let window = Window::new(ui, "Hello").escapable();

    kas::runner::Runner::new(())?.with(window).run()
}
