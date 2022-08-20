# Hello World!

![Hello](screenshots/hello.png)

Okay, that's not "hello world". Lets do something *slightly* more fun: a push button.
[Source](https://github.com/kas-gui/tutorials/blob/master/examples/hello.rs).

```rust
use kas::widgets::{TextButton, Window};

fn main() -> Result<(), kas::shell::Error> {
    env_logger::init();

    let content = TextButton::new("Push me");
    let window = Window::new("Hello", content);

    let theme = kas::theme::FlatTheme::new();
    kas::shell::Toolkit::new(theme)?.with(window)?.run()
}
```

```sh
cargo run --example hello
```

Hopefully that is clear enough? Let me explain anyway:

-   our `main` function may fail with the `kas::shell::Error` type; `Toolkit::new`
    and `Toolkit::with` can fail (the `?` "try" operator)
-   we initialise a logger, [`env_logger`] (optional,
    but lets us get useful messages; try setting the environment variable
    `RUST_LOG=kas=info` or `trace` before running the example)
-   we construct a `TextButton` and a `Window` around that
-   we use the `FlatTheme` (with default colours)
-   we initialise the toolkit with our theme, add our window, and run it

Note that `Toolkit::run` does not return. It is in fact a wrapper around
[`winit::event_loop::EventLoop::run`], which does not return.
By default, the program will exit after all windows have closed.

Also note that `kas` is mostly just a wrapper crate. We *could* instead import
widgets from `kas_widgets`, the theme from `kas_theme` and `Toolkit` from
`kas_wgpu`. We *could* even write our own theme and/or shell instead of using
`kas_theme` and/or `kas_wgpu`, though we have little reason to.

## Event handling

You may have noticed that the button used above doesn't *do* anything. Lets fix
that.

If you look at the [`TextButton`] docs, you'll notice that it has an `on_push`
method, allowing a closure to be set as a "push" event handler. This closure
must have the following type:
```ignore
F: Fn(&mut Manager<'_>) -> Option<M> + 'static
```
In other words, it takes a reference to the [`Manager`] and returns an optional
*message* of type `M`. We'll come back to messages later; for now we can just
return `None` — well, we could if type inference worked, but since `None` could
be an option of any type we have to specify that we want `VoidMsg`:
```rust
# use kas::prelude::*;
# use kas::widgets::TextButton;
let content = TextButton::new("Push me").on_push::<VoidMsg, _>(|_| {
    println!("Hello!");
    None
});
```

But let's not just print to the command-line: lets use the [`Manager`] to open
a message dialog!
```rust
# use kas::prelude::*;
# use kas::widgets::{MessageBox, TextButton};
let content = TextButton::new("&Push me").on_push::<VoidMsg, _>(|mgr| {
    let mbox = MessageBox::new("Message", "You pushed the button.");
    mgr.add_window(Box::new(mbox));
    None
});
```

One final note: did you see we put an ampersand in `"&Push me"`? Try holding
`Alt` and pressing `P` when you run the example:

```sh
cargo run --example hello-handler
```

[`env_logger`]: https://docs.rs/env_logger
[`winit::event_loop::EventLoop::run`]: https://docs.rs/winit/0.24/winit/event_loop/struct.EventLoop.html#method.run
[`env_logger::init`]: https://docs.rs/env_logger/0.8/env_logger/fn.init.html
[`kas_theme::ShadedTheme`]: https://docs.rs/kas-theme/latest/kas_theme/struct.ShadedTheme.html
[`kas_wgpu::Toolkit`]: https://docs.rs/kas-wgpu/latest/kas_wgpu/struct.Toolkit.html
[`TextButton`]: https://docs.rs/kas/latest/kas/widget/struct.TextButton.html
[`Manager`]: https://docs.rs/kas/latest/kas/event/struct.Manager.html
