# Receipt Bot

A project to create a telegram bot which can read the albanian receipt QR codes.
These codes point to a dynamic website which needs to be scraped using
(WebDriver) and got the datapoints which are then stored in a database. The
whole thing is written in Rust.

# Dependencies
To set up the project you need either [`geckodriver`](https://github.com/mozilla/geckodriver/releases) or the
[`chromedriver`](https://chromedriver.chromium.org/), which needs be run before
starting the app.
It also needs a MYSQL database.

# Building

Like other Rust project, you only need to run:
- `cargo b`           -> for debug
- `cargo b --release` -> for release
