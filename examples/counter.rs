use kas::prelude::*;
use kas::widgets::{Button, column, format_value, row};

#[derive(Clone, Debug)]
struct Increment(i32);

fn counter() -> impl Widget<Data = ()> {
    let tree = column![
        format_value!("{}").align(AlignHints::CENTER),
        row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];

    tree.with_state(0)
        .on_message(|_, count, Increment(add)| *count += add)
}

fn main() -> kas::runner::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new();
    let mut app = kas::runner::Runner::with_theme(theme).build(())?;
    let _ = app.config_mut().font.set_size(24.0);
    let window = Window::new(counter(), "Counter").escapable();
    app.with(window).run()
}
