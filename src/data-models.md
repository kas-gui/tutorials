# Sync-counter: data models

*Topics: data models and view widgets*

TODO:

-   [`ListView`] and [`ListData`]
-   [`Driver`], including predefined impls
-   [`Filter`] and [`UnsafeFilteredList`]. This rather messy to use (improvable?). The latter should eventually be replaced with a safe variant.
-   [`MatrixView`] and [`MatrixData`]. (Will possibly gain support for row/column labels and be renamed `TableView`.)

For now, see the examples:

-   [`examples/ldata-list-view.rs`](https://github.com/kas-gui/kas/blob/master/examples/data-list-view.rs) uses [`ListView`] with custom [`ListData`] and [`Driver`]
-   [`examples/gallery.rs`](https://github.com/kas-gui/kas/blob/master/examples/gallery.rs#L338)'s `filter_list` uses [`UnsafeFilteredList`] with a custom [`Driver`]. Less code but possibly more complex.
-   [`examples/times-tables.rs`](https://github.com/kas-gui/kas/blob/master/examples/times-tables.rs) uses [`MatrixView`] with custom [`MatrixData`] and [`driver::NavView`]. Probably the easiest example.

[`ListView`]: https://docs.rs/kas/latest/kas/view/struct.ListView.html
[`ListData`]: https://docs.rs/kas/latest/kas/view/trait.ListData.html
[`Driver`]: https://docs.rs/kas/latest/kas/view/trait.Driver.html
[`driver::NavView`]: https://docs.rs/kas/latest/kas/view/driver/struct.NavView.html
[`Filter`]: https://docs.rs/kas/latest/kas/view/filter/trait.Filter.html
[`UnsafeFilteredList`]: https://docs.rs/kas/latest/kas/view/filter/struct.UnsafeFilteredList.html
[`MatrixView`]: https://docs.rs/kas/latest/kas/view/struct.MatrixView.html
[`MatrixData`]: https://docs.rs/kas/latest/kas/view/trait.MatrixData.html
