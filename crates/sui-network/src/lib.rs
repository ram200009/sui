// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mysten_network::config::Config;
use std::time::Duration;

pub mod api;

pub use tonic;

pub const DEFAULT_CONNECT_TIMEOUT_SEC: Duration = Duration::from_secs(10);
pub const DEFAULT_REQUEST_TIMEOUT_SEC: Duration = Duration::from_secs(30);

pub fn default_mysten_network_config() -> Config {
    let mut net_config = mysten_network::config::Config::new();
    net_config.connect_timeout = Some(DEFAULT_CONNECT_TIMEOUT_SEC);
    net_config.request_timeout = Some(DEFAULT_REQUEST_TIMEOUT_SEC);
    net_config
}
