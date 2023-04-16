/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use once_cell::sync::Lazy;
use regex::Regex;

static CLASS_NAME_ONLY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("（.*?）").unwrap());

pub fn get_pure_name(name: &str) -> String {
  CLASS_NAME_ONLY_REGEX.replace(name, "").to_string()
}
