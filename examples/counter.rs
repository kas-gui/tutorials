use kas::prelude::*;
use kas::widgets::{format_value, Adapt, Button};

#[derive(Clone, Debug)]
struct Increment(i32);

fn counter() -> impl Widget<Data = ()> {
    let tree = kas::column![
        align!(center, format_value!("{}")),
        kas::row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];

    Adapt::new(tree, 0).on_message(|_, count, Increment(add)| *count += add)
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);
    kas::app::Default::with_theme(theme)
        .build(())?
        .with(Window::new(counter(), "Counter"))
        .run()
}
