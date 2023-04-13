/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use std::io::{stdin, stdout, Write};

use fe14_calculator_core::character;
use fe14_calculator_core::character::Character;
use fe14_calculator_core::class::Class;

fn main() -> Result<(), Error> {
  loop {
    let input = read_input("please enter character name")?;
    if input.eq("exit") {
      break;
    }

    let character = Character::find(&input).cloned();
    let mut character = match character {
      None => {
        println!("unknown character {input}");
        continue;
      }
      Some(character) => character,
    };

    loop {
      println!("\n\n{}", character);
      let input = read_input(
        r#""up" to level up by one
"change" to change character class
"reset" to reset character
"exit" to exit current character
please enter command"#,
      )?;

      match input.as_str() {
        "exit" => break,
        "reset" => {
          character = character.init()?;
        }
        "up" => {
          let result = character.level_up(false, false);
          if let Err(err) = result {
            println!("{}", err);
          } else {
            println!("success")
          }
        }
        "change" => loop {
          let input = read_input("please enter class name")?;

          if input == "exit" {
            break;
          }

          let class = Class::find(&input);

          if let Some(class) = class {
            let result = character.change_class(class);
            if let Err(err) = result {
              println!("{}", err);
            } else {
              println!("success");
            }
            break;
          } else {
            println!("unknown class {input}")
          }
        },
        _ => {
          println!("unknown command {input}");
        }
      }
    }
  }
  Ok(())
}

fn read_input(prompt: &str) -> std::io::Result<String> {
  print!("{prompt}: ");
  stdout().flush()?;
  let mut input = String::new();
  stdin().read_line(&mut input)?;

  Ok(input.trim_end_matches('\n').to_string())
}

#[derive(thiserror::Error, Debug)]
enum Error {
  #[error("io error: \n{0}")]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Character(#[from] character::CharacterError),
}
