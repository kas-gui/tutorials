# Hello World!

*Topics: logging, app, run*

![Hello](screenshots/hello.png)

Lets get started with a simple message box.
[Source](https://github.com/kas-gui/tutorials/blob/master/examples/hello.rs).

```rust
# extern crate kas;
use kas::widgets::{Button, column};
use kas::window::Window;

fn main() -> kas::runner::Result<()> {
    let ui = column![
        "Hello, world!",
        Button::label("&Close").with(|cx, _| cx.exit())
    ];
    let window = Window::new(ui, "Hello").escapable();

    kas::runner::Runner::new(())?.with(window).run()
}
```

```sh
cargo run --example hello
```

## The UI

We use the [`column!`] macro to construct our layout. This macro turns string literals into label widgets for us, ensuring that "Hello, world!" will appear on the screen.

For the button, we use a [`Button`] widget. The button's action handler calls [`EventState::exit`] to terminate the UI. (To close the window without terminating the UI, we would instead call `cx.window_action(Action::CLOSE);`.)

## The Window

We construct a [`Window`] over the `ui` and a title. We also call [`Window::escapable`] to allow our window to be closed using the <kbd>Escape</kbd> key.

## The Runner

Every UI needs a [`Runner`]. In this example we simply construct a runner over data `()`, add a single window, and run. In later examples you will see how we can select a theme, use input data, multiple windows and tweak the configuration.

Finally, [`Runner::run`] starts our UI. This method runs the event-loop internally, returning `Ok(())` once all windows have closed successfully.

[`column!`]: https://docs.rs/kas/latest/kas/widgets/macro.column.html
[`Button`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html
[`Window`]: https://docs.rs/kas/latest/kas/struct.Window.html
[`Window::escapable`]: https://docs.rs/kas/latest/kas/struct.Window.html#method.escapable
[`EventState::exit`]: https://docs.rs/kas/latest/kas/event/struct.EventState.html#method.exit
[`Runner`]: https://docs.rs/kas/latest/kas/runner/struct.Runner.html
[`Runner::run`]: https://docs.rs/kas/latest/kas/runner/struct.Runner.html#method.run
