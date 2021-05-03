use kas::class::HasString;
use kas::event::{Manager, Response, VoidMsg};
use kas::macros::Widget;
use kas::widget::{Label, TextButton, Window};

#[derive(Debug, Widget)]
#[layout(column)]
struct Counter {
    #[widget_core]
    core: kas::CoreData,

    #[layout_data]
    layout_data: <Self as kas::LayoutData>::Data,

    #[widget(halign = centre)]
    display: Label<String>,

    #[widget(handler = increment)]
    button: TextButton<()>,

    counter: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            core: Default::default(),
            layout_data: Default::default(),
            display: Label::new("0".to_string()),
            button: TextButton::new_msg("&count", ()),
            counter: 0,
        }
    }

    fn increment(&mut self, mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
        self.counter += 1;
        *mgr |= self.display.set_string(self.counter.to_string());
        Response::None
    }
}

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let window = Window::new("Counter", Counter::new());

    let theme = kas_theme::ShadedTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
