/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use crate::class_type::ClassType;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::stats::Stats;

pub static CLASSES: Lazy<Vec<Class>> = Lazy::new(|| {
  serde_json::from_str::<Vec<Class>>(include_str!("class.json")).expect("failed to load class")
});

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Class {
  pub name: String,
  pub r#type: ClassType,
  pub correctness: Stats,
  pub growth_rate: Stats,
  pub upper_limit: Stats,
}

impl Class {
  pub fn find(class_name: &str) -> Option<&Class> {
    CLASSES.iter().find(|it| it.name.eq(&class_name))
  }
}
