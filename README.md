# freundchen

A little buddy, written in [Rust](https://www.rust-lang.org), using [guiver](https://github.com/kud1ing/guiver).

<img width="912" alt="Bildschirmfoto 2022-10-11 um 14 53 50" src="https://user-images.githubusercontent.com/391975/195096506-0ec82534-bd92-4879-8174-2e803776fa64.png">

Since guiver is a fairly new project itself, not everything may work smoothly.

Currently only a mood tracker is provided.
It's possible to add other helpful functionality, like:
* a habit tracker alongside the mood tracker, so that correlations can be identified
* ways to question thoughts (inspired by CBT, The Work etc.)
* the possibility to store and display inspiring quotes

All your data stays with you. Nothing is shared with anyone. This is not a commercial product and will never be.


## Backlog

* [ ] comment on the adjusted mood?
* [ ] add a habit tracker alongside the mood tracker
* [ ] add views
  * `current_view`
  * `dashboard_view`
  * [ ] add a header/toolbar to switch between views
* [ ] add a way to configure a username the user would like to be addressed with
* [ ] persist the data encrypted
* [ ] Ctrl+C handler should call `ApplicationState::save_and_quit()`
  * [ ] how?
    * call `ApplicationState::handle_event(Event::RequestClose)` somehow?
* [ ] publish Crate
  * this is blocked by a release of guiver, which is blocked by a a release of Piet
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
