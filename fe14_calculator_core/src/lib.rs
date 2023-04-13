/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

pub mod attribute;
pub mod character;
pub mod class;
pub mod class_type;
pub mod stats;

#[cfg(test)]
mod tests {
  use crate::character::CHARACTERS;

  #[test]
  fn main() {
    println!(
      "{}",
      CHARACTERS
        .iter()
        .map(|it| it.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    );
  }
}
