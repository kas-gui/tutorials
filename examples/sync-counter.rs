use kas::widgets::{format_data, label_any, Adapt, Button, Slider};
use kas::{messages::MessageStack, Action, Window};

#[derive(Clone, Debug)]
struct Increment(i32);

#[derive(Clone, Copy, Debug)]
struct Count(i32);

impl kas::app::AppData for Count {
    fn handle_messages(&mut self, messages: &mut MessageStack) -> Action {
        if let Some(Increment(add)) = messages.try_pop() {
            self.0 += add;
            Action::UPDATE
        } else {
            Action::empty()
        }
    }
}

fn counter() -> impl kas::Widget<Data = Count> {
    // Per window state: (count, increment).
    type Data = (Count, i32);
    let initial: Data = (Count(0), 1);

    #[derive(Clone, Debug)]
    struct SetValue(i32);

    let slider = Slider::right(1..=10, |_, data: &Data| data.1).with_msg(SetValue);
    let ui = kas::column![
        format_data!(data: &Data, "Count: {}", data.0.0),
        row![slider, format_data!(data: &Data, "{}", data.1)],
        row![
            Button::new(label_any("Sub")).with(|cx, data: &Data| cx.push(Increment(-data.1))),
            Button::new(label_any("Add")).with(|cx, data: &Data| cx.push(Increment(data.1))),
        ],
    ];

    Adapt::new(ui, initial)
        .on_update(|_, state, count| state.0 = *count)
        .on_message(|_, state, SetValue(v)| state.1 = v)
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas_wgpu::ShadedTheme::new().with_font_size(24.0);

    kas::app::Default::with_theme(theme)
        .build(Count(0))?
        .with(Window::new(counter(), "Counter 1"))
        .with(Window::new(counter(), "Counter 2"))
        .run()
}
