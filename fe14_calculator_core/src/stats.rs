/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone, Eq, PartialEq)]
pub struct Stats {
    #[serde(rename = "HP")]
    pub hp: i32,
    #[serde(rename = "力量")]
    pub strength: i32,
    #[serde(rename = "魔力")]
    pub magic: i32,
    #[serde(rename = "技巧")]
    pub skill: i32,
    #[serde(rename = "速度")]
    pub speed: i32,
    #[serde(rename = "幸运")]
    pub luck: i32,
    #[serde(rename = "防守")]
    pub def: i32,
    #[serde(rename = "魔防")]
    pub res: i32,
    #[serde(rename = "体格")]
    pub build: i32,
}

impl Stats {
    pub fn fields() -> [&'static str; 9] {
        [
            "hp", "strength", "magic", "skill", "speed", "luck", "def", "res", "build",
        ]
    }

    pub fn get(&self, name: &str) -> i32 {
        match name {
            "hp" => self.hp,
            "strength" => self.strength,
            "magic" => self.magic,
            "skill" => self.skill,
            "speed" => self.speed,
            "luck" => self.luck,
            "def" => self.def,
            "res" => self.res,
            "build" => self.build,

            _ => {
                unreachable!("unexpected field {name}")
            }
        }
    }

    pub fn get_mut(&mut self, name: &str) -> &mut i32 {
        match name {
            "hp" => &mut self.hp,
            "strength" => &mut self.strength,
            "magic" => &mut self.magic,
            "skill" => &mut self.skill,
            "speed" => &mut self.speed,
            "luck" => &mut self.luck,
            "def" => &mut self.def,
            "res" => &mut self.res,
            "build" => &mut self.build,

            _ => {
                unreachable!("unexpected field {name}")
            }
        }
    }
}
