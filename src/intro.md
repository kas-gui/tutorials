# Kas Tutorials

These tutorials concern the [Kas GUI system](https://github.com/kas-gui/kas).
See also the [Kas examples](https://github.com/kas-gui/kas/tree/master/kas-wgpu/examples)
and [7GUIs examples](https://github.com/kas-gui/7guis/).

Further reading can be found on the [Kas blog](https://kas-gui.github.io/blog/).

Please ask questions on the [discussion boards](https://github.com/kas-gui/tutorials/discussions/)
or on the [tutorials issue tracker](https://github.com/kas-gui/tutorials/discussions/1).

## Requirements

It is assumed that you are already familiar with [the Rust language](https://www.rust-lang.org/).
If not, then [Learn Rust](https://www.rust-lang.org/learn)!
You are not expected to master Rust before learning Kas, but this tutorial
series assumes a moderate understanding of the language.

Kas supports both **nightly** and **stable** Rust. Due to the nature of
procedural macros, better diagnostics are available when using **nightly**.

Tutorials use the latest stable release of [Kas](https://github.com/kas-gui/kas),
currently v0.14.

## Examples

All significant examples can be found as working apps in [the example directory](https://github.com/kas-gui/tutorials/tree/master/examples).

To run the examples locally, check out the `tutorials` repository, then run e.g.:
```sh
git clone https://github.com/kas-gui/tutorials.git
cd tutorials
cargo run --example counter
```

## Kas Dependencies

What is `kas`? Here is a heavily-reduced dependency tree:
```plain
kas — Wrapper crate to expose all components under a single API
├── kas-core — Core types, traits and event handling
│   ├── arboard — Clipboard support (optional)
│   ├── async-global-executor — Executor supporting EventState::push_spawn (optional)
│   ├── easy-cast — Numeric type-casting, re-exposed as kas::cast
│   ├── kas-macros (proc-macro) — Macros
│   │   └── impl-tools-lib — Backend used to implement macros
│   ├── kas-text — Font handling, type setting
│   │   ├── ab_glyph — Glyph rastering
│   │   ├── harfbuzz_rs — Shaping (optional)
│   │   ├── pulldown-cmark — Markdown parsing (optional)
│   │   └── rustybuzz — Shaping (optional, default)
│   ├── log — Logging facade
│   ├── serde — Serialization support for persistent configuration (optional)
│   ├── serde_json, serde_yaml, ron — Output formats for configuration (optional)
│   ├── smithay-clipboard — Wayland clipboard support (optional)
│   └── winit — Cross-platform window creation
│   │   └── raw-window-handle — Interoperability for Rust Windowing applications
├── kas-widgets — Standard widget collection
├── kas-resvg — Canvas and Svg widgets
│   ├── resvg — An SVG rendering library
│   └── tiny-skia — Tiny CPU-only Skia subset
├── kas-view — "View widgets" over data models (optional)
└── kas-wgpu — Kas graphics backend over WGPU
    └── wgpu — Rusty WebGPU API wrapper
```


## Licence

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><span property="dct:title">This tutorial, including text but excluding code samples, </span> is licensed under <a href="http://creativecommons.org/licenses/by-sa/4.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC BY-SA 4.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1"><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1"><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/sa.svg?ref=chooser-v1"></a></p> 

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><span property="dct:title">Code samples found within this tutorial</span> are marked with <a href="http://creativecommons.org/publicdomain/zero/1.0?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC0 1.0<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1"><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1"></a></p> 
