use kas::class::HasString;
use kas::event::{Manager, Response, VoidMsg};
use kas::macros::make_widget;
use kas::widget::{Label, TextButton, Window};

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let content = make_widget! {
        #[layout(column)]
        #[handler(msg = VoidMsg)]
        struct {
            #[widget(halign = centre)] display: impl HasString = Label::new("0".to_string()),
            #[widget(handler = count)] _ = TextButton::new_msg("&count", ()),
            counter: u32 = 0,
        }
        impl {
            fn count(&mut self, mgr: &mut Manager, _: ()) -> Response<VoidMsg> {
                self.counter += 1;
                *mgr |= self.display.set_string(self.counter.to_string());
                Response::None
            }
        }
    };
    let window = Window::new("Counter", content);

    let theme = kas_theme::ShadedTheme::new();
    kas_wgpu::Toolkit::new(theme)?.with(window)?.run()
}
