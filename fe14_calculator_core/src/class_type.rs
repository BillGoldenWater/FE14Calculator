/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum ClassType {
  #[serde(rename = "基础")]
  Basic,
  #[serde(rename = "高级")]
  Advanced,
  #[serde(rename = "特殊")]
  Special,
}
