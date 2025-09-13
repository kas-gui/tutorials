use kas::widgets::{AdaptWidget, Button, Label, Slider, column, format_data, row};
use kas::window::Window;

#[derive(Clone, Debug)]
struct Increment(i32);

#[derive(Clone, Copy, Debug)]
struct Count(i32);
impl kas::runner::AppData for Count {
    fn handle_messages(&mut self, messages: &mut kas::runner::MessageStack) {
        if let Some(Increment(add)) = messages.try_pop() {
            self.0 += add;
        }
    }
}

fn counter(title: &str) -> Window<Count> {
    // Per window state: (count, increment).
    type Data = (Count, i32);
    let initial: Data = (Count(0), 1);

    #[derive(Clone, Debug)]
    struct SetValue(i32);

    let slider = Slider::right(1..=10, |_, data: &Data| data.1).with_msg(SetValue);
    let ui = column![
        format_data!(data: &Data, "Count: {}", data.0.0),
        row![slider, format_data!(data: &Data, "{}", data.1)],
        row![
            Button::new(Label::new_any("Sub")).with(|cx, data: &Data| cx.push(Increment(-data.1))),
            Button::new(Label::new_any("Add")).with(|cx, data: &Data| cx.push(Increment(data.1))),
        ],
    ];

    let ui = ui
        .with_state(initial)
        .on_update(|_, state, count| state.0 = *count)
        .on_message(|_, state, SetValue(v)| state.1 = v);
    Window::new(ui, title).escapable()
}

fn main() -> kas::runner::Result<()> {
    env_logger::init();

    let count = Count(0);
    let theme = kas_wgpu::ShadedTheme::new();

    let mut runner = kas::runner::Runner::with_theme(theme).build(count)?;
    let _ = runner.config_mut().font.set_size(24.0);
    runner
        .with(counter("Counter 1"))
        .with(counter("Counter 2"))
        .run()
}
