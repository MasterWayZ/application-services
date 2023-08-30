/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![allow(unknown_lints)]
#![warn(rust_2018_idioms)]

pub use as_ohttp_client;
pub use autofill;
pub use crashtest;
pub use error_support;
pub use fxa_client;
pub use logins;
pub use nimbus;
pub use places;
pub use push;
// TODO: Drop this dependency once firefox-ios switches to using `rust_log_forwarder` for log
// forwarding.
pub use rc_log_ffi;
pub use remote_settings;
pub use rust_log_forwarder;
pub use sync15;
pub use sync_manager;
pub use tabs;
pub use viaduct_reqwest;
