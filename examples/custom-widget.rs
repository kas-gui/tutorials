use kas::prelude::*;
use kas::widgets::{format_value, AccessLabel, Button, Row, Text};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget{
        layout = column![
            align!(center, self.display),
            self.buttons,
        ];
    }]
    struct Counter {
        core: widget_core!(),
        #[widget(&self.count)]
        display: Text<i32, String>,
        #[widget]
        buttons: Row<Button<AccessLabel>>,
        count: i32,
    }
    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                core: Default::default(),
                display: format_value!("{}"),
                buttons: Row::new([
                    Button::label_msg("-", Increment(-1)),
                    Button::label_msg("+", Increment(1)),
                ]),
                count,
            }
        }
    }
    impl Events for Self {
        type Data = ();

        fn handle_messages(&mut self, cx: &mut EventCx, data: &()) {
            if let Some(Increment(incr)) = cx.try_pop() {
                self.count += incr;
                cx.update(self.as_node(data));
            }
        }
    }
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);
    kas::app::Default::with_theme(theme)
        .build(())?
        .with(Window::new(Counter::new(0), "Counter"))
        .run()
}
