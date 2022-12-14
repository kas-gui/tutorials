use kas::model::SharedRc;
use kas::prelude::*;
use kas::view::SingleView;
use kas::widgets::TextButton;

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
    #[derive(Clone, Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget] display: SingleView<SharedRc<i32>>,
    }
    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                core: Default::default(),
                display: SingleView::new(SharedRc::new(count)),
            }
        }
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(Increment(incr)) = mgr.try_pop_msg() {
                self.display.update_value(mgr, |count| *count += incr);
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
    kas::shell::Toolkit::new(theme)?.with(counter)?.run()
}
