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

  pub fn change_class(&mut self, dst_class: &Class) -> ChResult<()> {
    self.get().change_class(dst_class)?;
    self.do_op(CharacterOperation::ChangeClass(dst_class.clone()));
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
  ChangeClass(Class),
}
