//! This is an unofficial wrapper around [Countly](https://count.ly/)'s JavaScript SDK based on
//! [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen). It allows Rust developers to use
//! Countly from the wasm32-unknown-unknown target running in a browser (or compatible environment).
//!
//! ## What is Countly?
//! 
//! Countly is an innovative, real-time, open source product analytics application. It collects data from mobile phones,
//! desktop apps (Mac OS X & Windows), web apps and visualizes this information to analyze application usage and end-user behavior.
//! 
//! ## What is Countly Web Analytics?
//! Countly is an open source, enterprise grade analytics platform built for all types of devices and platforms, including web.
//! When you add an application and select “web”, Countly, unlike several other cross-device analytics platforms, automatically
//! arranges and redesigns its user interface to emphasize web analytics related menus and widgets. Countly Community Edition
//! source code is available for free via GitHub, and Countly Enterprise Edition, which is available by subscription, can be
//! purchased from Countly.
//! 
//! # Installation
//! 
//! Note that you need to add the countly JavaScript SDK as a module to wasm-bindgen (via webpack or a similar module management
//! system). If you're using npm, you can add it using
//! 
//! ```
//! npm install --save countly-sdk-web
//! ```
//! 
//! If there is any confusion about the usage, take a look at [the JavaScript SDK documentation](https://support.count.ly/hc/en-us/articles/360037441932-Web-analytics-JavaScript-). This crate is just wrapping the functionality with little changes (except having some type safety).

pub mod countly_sys;
mod config;
pub use config::Config;

mod countly;
pub use countly::{Countly, Value, UserDetails};

mod gdpr;
pub use gdpr::ConsentFeatures;