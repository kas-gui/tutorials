use kas::prelude::*;
use kas::widgets::{Label, TextButton};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            row: [
                TextButton::new_msg("−", Increment(-1)),
                TextButton::new_msg("+", Increment(1)),
            ],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget] display: Label<String>,
        count: i32,
    }

    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                core: Default::default(),
                display: Label::from(count.to_string()),
                count,
            }
        }
    }

    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr) {
            if let Some(Increment(incr)) = mgr.try_pop() {
                self.count += incr;
                *mgr |= self.display.set_string(self.count.to_string());
            }
        }
    }

    impl Window for Self {
        fn title(&self) -> &str { "Counter" }
    }
}

fn main() -> kas::shell::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);

    let counter = Counter::new(0);
    kas::shell::DefaultShell::new(theme)?.with(counter)?.run()
}
