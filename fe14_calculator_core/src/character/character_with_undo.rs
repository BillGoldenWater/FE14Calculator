/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use crate::character::{ChResult, Character};
use crate::class::Class;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharacterWithUndo {
  character: Character,
  operations: Vec<CharacterOperation>,
  redo_operations: Vec<CharacterOperation>,
}

impl CharacterWithUndo {
  pub fn new(character: Character) -> Self {
    Self {
      character,
      operations: vec![],
      redo_operations: vec![],
    }
  }

  pub fn level_up(&mut self, enhanced: bool, doubled: bool) -> ChResult<()> {
    self.get().level_up(enhanced, doubled)?;
    self.do_op(CharacterOperation::LevelUp { enhanced, doubled });
    Ok(())
  }

  pub fn change_class(&mut self, dst_class: &'static Class) -> ChResult<()> {
    self.get().change_class(dst_class)?;
    self.do_op(CharacterOperation::ChangeClass(dst_class));
    Ok(())
  }

  pub fn get(&self) -> Character {
    let mut character = self.character.clone();
    for op in &self.operations {
      match op {
        CharacterOperation::LevelUp { enhanced, doubled } => {
          let _ = character.level_up(*enhanced, *doubled);
        }
        CharacterOperation::ChangeClass(dst_class) => {
          let _ = character.change_class(dst_class);
        }
      }
    }
    character
  }

  pub fn get_operations(&self) -> Vec<CharacterOperationItem> {
    let mut character = self.character.clone();

    let cur_class = Class::find(&character.cur_attribute.class).unwrap();
    let mut operation_items = vec![CharacterOperationItem::LevelUp {
      cur_class,
      cur_lvl: character.get_lv(),
      need: UndoOrRedo::Undo(self.operations.len() as i32),
      is_max: false,
    }];

    let mut do_operation = |op: &CharacterOperation, need: UndoOrRedo| match op {
      CharacterOperation::LevelUp { enhanced, doubled } => {
        let cur_class = Class::find(&character.cur_attribute.class).unwrap();
        let _ = character.level_up(*enhanced, *doubled);
        operation_items.push(CharacterOperationItem::LevelUp {
          need,

          cur_class,
          cur_lvl: character.get_lv(),
          is_max: character.get_lv() == character.get_max_lv().unwrap(),
        })
      }
      CharacterOperation::ChangeClass(dst_class) => {
        let prev_class = Class::find(&character.cur_attribute.class).unwrap();
        let prev_lv = character.get_lv();
        let prev_is_max = prev_lv == character.get_max_lv().unwrap();
        let _ = character.change_class(dst_class);
        let dst_lv = character.get_lv();
        operation_items.push(CharacterOperationItem::ChangeClass {
          need,

          prev_class,
          prev_lv,
          prev_is_max,
          dst_class,
          dst_lv,
        })
      }
    };

    let mut undo_count = (self.operations.len() as i32) - 1;
    for op in &self.operations {
      do_operation(op, UndoOrRedo::Undo(undo_count));
      undo_count -= 1;
    }

    for op in self.redo_operations.iter().rev() {
      do_operation(op, UndoOrRedo::Redo(undo_count.abs()));
      undo_count -= 1;
    }

    operation_items
  }

  fn do_op(&mut self, op: CharacterOperation) {
    self.operations.push(op);
    self.redo_operations.clear();
  }

  pub fn undo(&mut self) {
    if let Some(op) = self.operations.pop() {
      self.redo_operations.push(op)
    }
  }

  pub fn redo(&mut self) {
    if let Some(op) = self.redo_operations.pop() {
      self.operations.push(op);
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum CharacterOperation {
  LevelUp { enhanced: bool, doubled: bool },
  ChangeClass(&'static Class),
}

#[derive(Debug)]
pub enum CharacterOperationItem {
  LevelUp {
    need: UndoOrRedo,
    cur_class: &'static Class,
    cur_lvl: i32,
    is_max: bool,
  },
  ChangeClass {
    need: UndoOrRedo,
    prev_class: &'static Class,
    prev_lv: i32,
    prev_is_max: bool,
    dst_class: &'static Class,
    dst_lv: i32,
  },
}

#[derive(Debug)]
pub enum UndoOrRedo {
  Undo(i32),
  Redo(i32),
}
