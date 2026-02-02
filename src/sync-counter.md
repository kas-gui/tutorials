# Sync-counter: data models

*Topics: top-level `AppData`, multiple windows*

![Counter 1](screenshots/sync-counter-1.png)
![Counter 2](screenshots/sync-counter-2.png)

We complicate the previous example just a little bit!

```rust
# extern crate kas;
use kas::widgets::{AdaptWidget, Button, Label, Slider, column, format_label, row};
use kas::window::Window;

#[derive(Clone, Debug)]
struct Increment(i32);

#[derive(Clone, Copy, Debug)]
struct Count(i32);
impl kas::runner::AppData for Count {
    fn handle_message(&mut self, messages: &mut impl kas::runner::ReadMessage) {
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
        format_label!(data: &Data, "Count: {}", data.0.0),
        row![slider, format_label!(data: &Data, "{}", data.1)],
        row![
            Button::new(Label::new_any("Sub")).with(|cx, data: &Data| cx.push(Increment(-data.1))),
            Button::new(Label::new_any("Add")).with(|cx, data: &Data| cx.push(Increment(data.1))),
        ],
    ];

    let ui = ui
        .with_state(initial)
        .on_update(|_, _, state, count| state.0 = *count)
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
```

## AppData

In the previous example, our top-level `AppData` was `()` and our mutable state was stored in an [`Adapt`] widget. This time, we will store our counter in top-level `AppData`, in a custom type which includes a message handler:
```rust
# extern crate kas;
# #[derive(Clone, Debug)]
# struct Increment(i32);

#[derive(Clone, Copy, Debug)]
struct Count(i32);

impl kas::runner::AppData for Count {
    fn handle_message(&mut self, messages: &mut impl kas::runner::ReadMessage) {
        if let Some(Increment(add)) = messages.try_pop() {
            self.0 += add;
        }
    }
}
```
[`AppData::handle_message`] is more verbose than [`Adapt::on_message`], but does the same job.

To integrate this into our example, we pass a `Count` object into [`kas::runner::Builder::build`] and adjust the prototype of `counter` to:
```rust
# extern crate kas;
# #[derive(Clone, Copy, Debug)]
# struct Count(i32);
# impl kas::runner::AppData for Count {
#     fn handle_message(&mut self, messages: &mut impl kas::runner::ReadMessage) {}
# }
fn counter() -> impl kas::Widget<Data = Count> {
    // ...
    # kas::widgets::Label::new_any("")
}
```

### Adapting app data

We could at this point simply repeat the previous example, skipping the [`Adapt`] node since our [`AppData`] implementation already does the work. But lets make things more interesting by combining top-level state with local state.

We define a new data type for local state and construct an initial instance:
```rust
# #[derive(Clone, Copy, Debug)]
# struct Count(i32);
// Per window state: (count, increment).
type Data = (Count, i32);
let initial: Data = (Count(0), 1);
```
Note that our local data includes a *copy* of the top-level data `Count` (along with an initial value, `Count(0)`, which will be replaced before it is ever used).

We'll skip right over the widget declarations to the new [`Adapt`] node:
```rust
# extern crate kas;
# use kas::widgets::{Adapt, AdaptWidget, Label};
# #[derive(Clone, Copy, Debug)]
# struct Count(i32);
# fn counter() -> impl kas::Widget<Data = Count> {
# #[derive(Clone, Debug)]
# struct SetValue(i32);
# let ui = Label::new_any("");
# let initial = (Count(0), 1);
    let ui = ui
        .with_state(initial)
        .on_update(|_, _, state, count| state.0 = *count)
        .on_message(|_, state, SetValue(v)| state.1 = v);
    # ui
# }
```
The notable addition here is [`Adapt::on_update`], which takes a closure over the expected mutable reference to local `state` as well as *input* data `count` (i.e. the top-level data), allowing us to update local state with the latest top-level `count`.

Aside: you may wonder why we store `count` in [`Adapt`]'s state at all. Why not simply pass `(&Count, &i32)` (count, increment) down to the local UI? The answer is that we can't, because of lifetimes. To be specific, the input data type is formalized as an associated type, [`Widget::Data`], which must outlive instances of that type: that is any references embedded in an input data type must outlive the instances of the widgets they are passed to. Moreover, [`AppData`] requires lifetime `'static` (more as a simplification than because we truely couldn't support non-static lifetimes here, though there really isn't much use for them).

Aside aside: could we not make [`Widget::Data`] into a Generic Associated Type (GAT) to support lifetimes shorter than that of the widget object? Well, yes, but traits with GATs are not (yet) object-safe. This is a problem because object-safe widget types are important (both for variadic layout — e.g. a `TabStack` where pages use different widget types — and more fundamentally, namely to make [`Node`] work). So *maybe* this will be possible eventually, dependent on future Rust development.

## Running multiple windows

Constructing multiple windows under a UI runner is simple:
```rust
# extern crate kas;
# use kas::window::Window;
# #[derive(Clone, Copy, Debug)]
# struct Count(i32);
# impl kas::runner::AppData for Count {
#     fn handle_message(&mut self, messages: &mut impl kas::runner::ReadMessage) {}
# }
# fn counter(title: &str) -> Window<Count> {
#     Window::new(kas::widgets::Label::new_any(""), title)
# }
# fn main() -> kas::runner::Result<()> {
    # let count = Count(0);
    # let theme = kas_wgpu::ShadedTheme::new();
    let mut runner = kas::runner::Runner::with_theme(theme).build(count)?;
    let _ = runner.config_mut().font.set_size(24.0);
    runner
        .with(counter("Counter 1"))
        .with(counter("Counter 2"))
        .run()
# }
```
Each window has its own local state stored in its [`Adapt`] node (the `increment`) while sharing the top-level `Count`.

[`AppData`]: https://docs.rs/kas/latest/kas/app/trait.AppData.html
[`AppData::handle_message`]: https://docs.rs/kas/latest/kas/runner/trait.AppData.html#tymethod.handle_message
[`Adapt`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html
[`Adapt::on_message`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html#method.on_message
[`Action::UPDATE`]: https://docs.rs/kas/latest/kas/struct.Action.html#associatedconstant.UPDATE
[`Adapt::on_update`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html#method.on_update
[`kas::runner::Builder::build`]: https://docs.rs/kas/latest/kas/runner/struct.Builder.html#method.build
[`Widget::Data`]: https://docs.rs/kas/latest/kas/trait.Widget.html#associatedtype.Data
[`Node`]: https://docs.rs/kas/latest/kas/struct.Node.html
