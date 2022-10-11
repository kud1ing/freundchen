# freundchen

A little buddy.

<img width="869" alt="Bildschirmfoto 2022-10-11 um 13 48 38" src="https://user-images.githubusercontent.com/391975/195082455-417e3ead-badc-4b59-b0a8-161b77b74fec.png">

Currently only a mood tracker is provided.
It's possible to add other helpful functionality, like:
* a habit tracker alongside the mood tracker, so that correlations can be identified
* ways to question thoughts (inspired by CBT, The Work etc.)
* the possibility to store and display inspiring quotes

All your data stays with you. Nothing is shared with anyone. This is not a commercial product and will never be.

freundchen is written in [Rust](https://www.rust-lang.org) and uses [guiver](https://github.com/kud1ing/guiver).
Since guiver is a fairly new project itself, not everything may work smoothly.

## Backlog

* [ ] publish Crate
* [ ] add views
  * `current_view`
  * `dashboard_view`
* [ ] Ctrl+C handler should call `ApplicationState::save_and_quit()` somehow
  * call `ApplicationState::handle_event(Event::RequestClose)` somehow?
* [ ] `MoodWidget`: use a more beautiful color palette
* [ ] persist the data encrypted
* [ ] add a header with the different views
* [ ] add a way to configure a username the user would like to be addressed with
* [ ] add internationalization


## License

Licensed under either of
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
