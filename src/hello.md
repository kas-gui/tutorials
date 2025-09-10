# Hello World!

*Topics: logging, app, run*

![Hello](screenshots/hello.png)

Lets get started with a simple message box.
[Source](https://github.com/kas-gui/tutorials/blob/master/examples/hello.rs).

```rust
# extern crate kas;
use kas::widgets::dialog::MessageBox;

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let window = MessageBox::new("Message").into_window("Hello world");

    kas::app::Default::new(())?.with(window).run()
}
```

```sh
cargo run --example hello
```

## A window, a shell

Next, we construct a [`MessageBox`] widget, then wrap with a [`Window`]:
```rust
# extern crate kas;
# use kas::widgets::dialog::MessageBox;
let window = MessageBox::new("Message")
    .into_window("Hello world");
# let _: kas::Window<()> = window;
```

Finally, we construct a default app, add this window, and run:
```rust
# extern crate kas;
# use kas::widgets::dialog::MessageBox;
# fn main() -> kas::app::Result<()> {
# let window = MessageBox::new("Message").into_window("Hello world");
kas::app::Default::new(())?
    .with(window)
    .run()
# }
```

[`kas::app::Default`] is just a parameterisation of [`kas::app::Application`] which selects a sensible graphics backend and theme.

If you wanted to select your own theme instead, you could do so as follows:
```rust
# extern crate kas;
# use kas::widgets::dialog::MessageBox;
# fn main() -> kas::app::Result<()> {
# let window = MessageBox::new("Message").into_window("Hello world");
let theme = kas::theme::SimpleTheme::new();
kas::app::Default::with_theme(theme)
    .build(())?
    .with(window)
    .run()
# }
```

Or, if you wanted to specify the graphics backend and theme:
```rust
# extern crate kas;
# use kas::widgets::dialog::MessageBox;
# fn main() -> kas::app::Result<()> {
# let window = MessageBox::new("Message").into_window("Hello world");
kas_wgpu::WgpuBuilder::new(())
    .with_theme(kas_wgpu::ShadedTheme::new())
    .build(())?
    .with(window)
    .run()
# }
```

Finally, [`Application::run`] starts our UI. This method runs the event-loop internally, returning `Ok(())` once all windows have closed successfully.

[`MessageBox`]: https://docs.rs/kas/latest/kas/widgets/dialog/struct.MessageBox.html
[`Window`]: https://docs.rs/kas/latest/kas/struct.Window.html
[`kas::app::Default`]: https://docs.rs/kas/latest/kas/app/type.Default.html
[`kas::app::Application`]: https://docs.rs/kas/latest/kas/app/struct.Application.html
[`Application::run`]: https://docs.rs/kas/latest/kas/app/struct.Application.html#method.run
[`winit::event_loop::EventLoop::run`]: https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html#method.run
