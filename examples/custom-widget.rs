use kas::prelude::*;
use kas::widgets::{AccessLabel, Button, Row, Text, format_value};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget]
    #[layout(column![
        self.display.align(AlignHints::CENTER),
        self.buttons,
    ])]
    struct Counter {
        core: widget_core!(),
        #[widget(&self.count)]
        display: Text<i32, String>,
        #[widget]
        buttons: Row<[Button<AccessLabel>; 2]>,
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

fn main() -> kas::runner::Result<()> {
    env_logger::init();

    let window = Window::new(Counter::new(0), "Counter");

    let theme = kas::theme::SimpleTheme::new();
    let mut app = kas::runner::Runner::with_theme(theme).build(())?;
    let _ = app.config_mut().font.set_size(24.0);
    app.with(window).run()
}
