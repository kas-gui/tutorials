# Sync-counter: data models

*Topics: simple data models and view widgets*

## Data models

In the previous example, `Counter` has these two fields:
```rust,ignore
struct Counter {
    // ...
    #[widget] display: Label<String>,
    count: i32,
}
```
But, given that `display` is always supposed to show the value of `count`,
can we combine them? Yes!

First, we need a data item. This should implement the [`SingleData`] trait
(or rather, its base [`SharedData`]). The latter trait looks a little complex, but for
a single data item `Key = ()` and `data.contains_key(&())` will always return
true. `data.get_cloned(&())` simply returns a clone of the data item.
The only unexpected complexity is the synchronisation mechanism.

We could implement a suitable type ourselves, but [`SharedRc`] does what we
want. We use `SharedRc<i32>` as our data type.

### Data views

On its own, using a data model isn't useful, but it allows us to use view
widgets. [`SingleView`] is a "view controller" which constructs and updates a
"view widget" over its data model using a [`Driver`]. The default driver,
[`driver::View`], suffices in this case.

We can adapt the `Counter` example as follows:
```rust,ignore
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
            self.display.update_value(mgr, |count| count + incr);
        }
    }
}
```

Is this change worthwhile on its own? That's debatable: it uses more complex
types to reduce the risk of de-synchronised data. Its real utility is when that
data might be accessed from multiple locations.

### Multiple windows

We can very easily adapt the above to use multiple instances of the same
(synchronised) counter:
```rust,ignore
    let counter = Counter::new(0);
    kas::shell::DefaultShell::new(theme)?
        .with(counter.clone())?
        .with(counter)?
        .run()
```

[Full code for this example](https://github.com/kas-gui/tutorials/blob/master/examples/sync-counter.rs).

## Drivers

Why should we implement our own `Counter` widget when a [`Spinner`] would do the
same job better?

Above, we use [`driver::View`] which constructs a [`Label`] widget over our
`i32` datum. We instead use [`driver::Spinner`].

Unlike our `Counter` [`SingleView`] does not support the [`Window`] trait, thus
we wrap with [`dialog::Window`].

The [complete code for this example](https://github.com/kas-gui/tutorials/blob/master/examples/sync-spinner.rs) is listed below:

```rust
use kas::model::SharedRc;
use kas::view::{driver, SingleView};
use kas::widgets::dialog::Window;

fn main() -> kas::shell::Result<()> {
    env_logger::init();

    let driver = driver::Spinner::new(i32::MIN..=i32::MAX, 1);
    let c1 = SingleView::new_with_driver(driver, SharedRc::new(0));
    let c2 = SingleView::new_with_driver(driver, c1.data().clone());

    let theme = kas::theme::ShadedTheme::new().with_font_size(24.0);
    kas::shell::DefaultShell::new(theme)?
        .with(Window::new("Counter 1", c1))?
        .with(Window::new("Counter 2", c2))?
        .run()
}
```

[`Spinner`]: https://docs.rs/kas/latest/kas/widgets/struct.Spinner.html
[`SingleData`]: https://docs.rs/kas/latest/kas/model/trait.SingleData.html
[`SharedData`]: https://docs.rs/kas/latest/kas/model/trait.SharedData.html
[`SharedRc`]: https://docs.rs/kas/latest/kas/model/struct.SharedRc.html
[`SingleView`]: https://docs.rs/kas/latest/kas/view/struct.SingleView.html
[`Driver`]: https://docs.rs/kas/latest/kas/view/trait.Driver.html
[`driver::Spinner`]: https://docs.rs/kas/latest/kas/view/driver/struct.Spinner.html
[`driver::View`]: https://docs.rs/kas/latest/kas/view/driver/struct.View.html
[`Label`]: https://docs.rs/kas/latest/kas/widgets/struct.Label.html
[`Window`]: https://docs.rs/kas/latest/kas/trait.Window.html
[`dialog::Window`]: https://docs.rs/kas/latest/kas/widgets/dialog/struct.Window.html
