/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use serde::Deserialize;

use crate::stats::Stats;

#[derive(Deserialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Attribute {
  pub class: String,
  #[serde(rename = "LV")]
  pub lv: i32,
  #[serde(rename = "HLV")]
  pub hlv: i32,
  pub stats: Stats,
}
