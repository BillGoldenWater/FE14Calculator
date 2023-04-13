/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use std::fmt::{Debug, Display, Formatter};

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::attribute::Attribute;
use crate::class::{Class, CLASSES};
use crate::class_type::ClassType;
use crate::stats::Stats;

pub static CHARACTERS: Lazy<Vec<Character>> = Lazy::new(|| {
  serde_json::from_str::<Vec<Character>>(include_str!("character.json"))
    .expect("failed to load character")
    .into_iter()
    .map(Character::init)
    .collect::<ChResult<Vec<_>>>()
    .expect("failed to initialize characters")
});

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Character {
  pub name: String,
  pub growth_rate: Stats,
  pub correctness: Stats,
  pub init_attribute: Attribute,
  #[serde(default)]
  pub cur_attribute: Attribute,
}
impl Character {
  pub fn find(character_name: &str) -> Option<&Character> {
    CHARACTERS.iter().find(|it| it.name.eq(&character_name))
  }

  fn cur_stat(&mut self, stat: &str) -> &mut i32 {
    self.cur_attribute.stats.get_mut(stat)
  }

  fn limit_stats(&mut self, stat: &str) -> ChResult<()> {
    *self.cur_stat(stat) =
      self.get_upper_limit(stat)?.min(self.get_total_stat(stat)?) - self.get_cor_stat(stat)?;

    Ok(())
  }

  pub fn init(mut self) -> ChResult<Self> {
    self.cur_attribute = self.init_attribute.clone();
    for stat in Stats::fields() {
      *self.cur_stat(stat) += self.growth_rate.get(stat);
      self.limit_stats(stat)?;
    }

    Ok(self)
  }

  fn get_current_class(&self) -> ChResult<Class> {
    CLASSES
      .iter()
      .find(|it| it.name.eq(&self.cur_attribute.class))
      .cloned()
      .ok_or(CharacterError::InvalidCurrentClass(
        self.cur_attribute.class.clone(),
      ))
  }

  pub fn get_max_lv(&self) -> ChResult<i32> {
    Ok(if self.get_current_class()?.r#type == ClassType::Special {
      40
    } else {
      20
    })
  }

  pub fn get_upper_limit(&self, stat: &str) -> ChResult<i32> {
    Ok(self.get_current_class()?.upper_limit.get(stat) + self.correctness.get(stat))
  }

  fn get_cor_stat(&self, stat: &str) -> ChResult<i32> {
    Ok(self.get_current_class()?.correctness.get(stat))
  }

  pub fn get_total_stat(&self, stat: &str) -> ChResult<i32> {
    Ok(
      (self.cur_attribute.stats.get(stat) + self.get_current_class()?.correctness.get(stat))
        .min(self.get_upper_limit(stat)?),
    )
  }

  pub fn get_total_stats(&self) -> ChResult<Stats> {
    let mut stats = Stats::default();
    for field in Stats::fields() {
      *stats.get_mut(field) = self.get_total_stat(field)?
    }
    Ok(stats)
  }

  fn calculate_growth(&self, stat: &str, enhanced: bool, doubled: bool) -> ChResult<i32> {
    let doubled_index = if doubled { 2 } else { 1 };
    let enhanced_index = if enhanced { 1 } else { 0 };

    Ok(
      self.growth_rate.get(stat)
        + self.get_current_class()?.growth_rate.get(stat) * doubled_index
        + 15 * enhanced_index,
    )
  }

  pub fn level_up(&mut self, enhanced: bool, doubled: bool) -> ChResult<()> {
    if self.get_lv() >= self.get_max_lv()? {
      return Err(CharacterError::MaxLevelReached(self.get_max_lv()?));
    }

    self.cur_attribute.lv += 1;
    for stat in Stats::fields() {
      if self.get_upper_limit(stat)? <= self.get_total_stat(stat)? {
        continue;
      }

      *self.cur_stat(stat) += self.calculate_growth(stat, enhanced, doubled)?;
      self.limit_stats(stat)?;
    }

    Ok(())
  }

  pub fn get_lv(&self) -> i32 {
    self.cur_attribute.lv
  }

  pub fn get_hlv(&self) -> i32 {
    self.cur_attribute.hlv
  }

  fn get_total_lv(&self) -> i32 {
    self.get_lv() + self.get_hlv()
  }

  pub fn change_class(&mut self, dst_class: &Class) -> ChResult<()> {
    let cur_cls = self.get_current_class()?;

    if cur_cls.eq(dst_class) && self.get_max_lv()? > self.get_lv() {
      return Err(CharacterError::UseSecondSealWhenNotMaxLvl {
        current: self.get_lv(),
        required: self.get_max_lv()?,
      });
    }
    if dst_class.r#type == ClassType::Advanced {
      if cur_cls.r#type == ClassType::Special && self.get_lv() < 21 {
        return Err(CharacterError::ChangeToAdvWhenSpecialAndLvlNotMeet(
          self.get_lv(),
        ));
      }
      if cur_cls.r#type == ClassType::Basic && self.get_lv() < 10 {
        return Err(CharacterError::ChangeToAdvWhenBasicAndLvlNotMeet(
          self.get_lv(),
        ));
      }
    }

    let mut lv = 1;

    if dst_class.r#type == ClassType::Special {
      if cur_cls.r#type == ClassType::Advanced {
        lv = 21;
      } else if cur_cls.r#type == ClassType::Special && self.get_lv() >= 21 {
        lv = 21
      }
    }

    self.cur_attribute.class = dst_class.name.clone();
    self.cur_attribute.hlv = (self.get_total_lv() - lv).max(0);
    self.cur_attribute.lv = lv;

    Ok(())
  }
}

impl Display for Character {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "name: {}", self.name)?;
    writeln!(f, "level: {}", self.get_lv())?;
    writeln!(f, "hidden_level: {}", self.get_hlv())?;
    writeln!(f, "class: {}", self.cur_attribute.class)?;

    writeln!(
      f,
      "{:#?}",
      self.get_total_stats().map_err(|_| std::fmt::Error)?
    )
  }
}

type ChResult<T> = Result<T, CharacterError>;

#[derive(thiserror::Error, Debug)]
pub enum CharacterError {
  #[error("invalid current class {0}")]
  InvalidCurrentClass(String),
  #[error("已达到最大等级 {0}")]
  MaxLevelReached(i32),
  #[error("不能在未满级时横转, 当前等级: {current}, 需要: {required}")]
  UseSecondSealWhenNotMaxLvl { current: i32, required: i32 },
  #[error("不能在等级未达到 21 级时从特殊兵种转换至高级兵种, 当前等级 {0}")]
  ChangeToAdvWhenSpecialAndLvlNotMeet(i32),
  #[error("不能在等级未达到 10 级时从基础兵种转换至高级兵种, 当前等级 {0}")]
  ChangeToAdvWhenBasicAndLvlNotMeet(i32),
}
