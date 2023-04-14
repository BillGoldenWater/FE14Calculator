/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

mod app;

use app::*;
use leptos::*;

fn main() {
  mount_to_body(|cx| {
    view! { cx,
        <App/>
    }
  })
}
