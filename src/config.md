# Configuration

*Topics: themes and UI configuration*

We won't build anything new this chapter. Instead, we'll take a moment to discuss configuration.


## Themes

Kas supports theme abstraction: widgets, for the most part, don't precisely determine their sizes or handle the minutae of drawing.

Theming is abstracted and exposed to widgets through two interfaces:

-   [`SizeCx`] supplies widgets with size information
-   [`DrawCx`] is used to draw widget elements

Kas currently provides three theme implementations (along with one meta-implementation):

-   [`kas::theme::SimpleTheme`] prioritises simplicity without loss of functionality.
-   [`kas::theme::FlatTheme`] extends `SimpleTheme`, putting more effort into styling while using no complex drawing techniques (well, if one doesn't count fonts).
-   [`kas_wgpu::ShadedTheme`] extends `FlatTheme` using shaded drawing for bevelled widget borders. The resulting styling is rather opinionated, bordering on a tech demo (it could further be adapted to e.g. use the mouse pointer as a light source instead of assuming a fixed light position, though it would quickly become apparent that the theme lacks true shadows).
-   [`kas::theme::MultiTheme`] supports run-time switching between pre-loaded themes. It is used by the [Gallery example].


## Configuration

Previously we adjusted the font size before the UI was started:
```rust
# use kas::prelude::*;
# fn main() -> kas::runner::Result<()> {
    let theme = kas::theme::SimpleTheme::new();
    let mut app = kas::runner::Runner::with_theme(theme).build(())?;
    let _ = app.config_mut().font.set_size(24.0);
    # Ok(())
# }
```

Various aspects of fonts, themes, event handling and shortcuts may be adjusted here; see the [`Config`] struct. The above snippet adjusts the default configuration before the UI is started using [`Runner::config_mut`]. The returned [`Action`] is discarded (`let _ =`) since the UI has not yet been started.

Configuration may also be adjusted through config files if the Kas feature `toml` (or another serialization format) and the environment variable `KAS_CONFIG` is set (e.g. `KAS_CONFIG="path/to/config.toml"`). See [`ReadWriteFactory`] documentation.

Run-time configuration of a sub-set of configurable items is possible using the [`EventConfig`] widget or by sending [`EventConfigMsg`]. Code may read configuration through [`EventState::config`] and adjust it using [`WindowConfig::update_base`], though this has some limitations; in particular fonts are not re-selected and new widget sizes are not fully realized without manual resizing of the window.


[`Runner::config_mut`]: https://docs.rs/kas/latest/kas/runner/struct.Runner.html#method.config_mut
[`Action`]: https://docs.rs/kas/latest/kas/struct.Action.html
[`EventState::config`]: https://docs.rs/kas/latest/kas/event/struct.EventState.html#method.config
[`WindowConfig::update_base`]: https://docs.rs/kas/latest/kas/config/struct.WindowConfig.html#method.update_base
[`ReadWriteFactory`]: https://docs.rs/kas/latest/kas/config/struct.ReadWriteFactory.html
[`Builder::with_config`]: https://docs.rs/kas/latest/kas/runner/struct.Builder.html#method.with_config
[Gallery example]: https://github.com/kas-gui/kas/tree/master/examples#gallery
[`Config`]: https://docs.rs/kas/latest/kas/config/struct.Config.html
[`SizeCx`]: https://docs.rs/kas/latest/kas/theme/struct.SizeCx.html
[`DrawCx`]: https://docs.rs/kas/latest/kas/theme/struct.DrawCx.html
[`kas::theme::SimpleTheme`]: https://docs.rs/kas/latest/kas/theme/struct.SimpleTheme.html
[`kas::theme::FlatTheme`]: https://docs.rs/kas/latest/kas/theme/struct.FlatTheme.html
[`kas::theme::MultiTheme`]: https://docs.rs/kas/latest/kas/theme/struct.MultiTheme.html
[`kas_wgpu::ShadedTheme`]: https://docs.rs/kas-wgpu/latest/kas_wgpu/struct.ShadedTheme.html
[`EventConfig`]: https://docs.rs/kas/latest/kas/widgets/struct.EventConfig.html
[`EventConfigMsg`]: https://docs.rs/kas/latest/kas/config/enum.EventConfigMsg.html
