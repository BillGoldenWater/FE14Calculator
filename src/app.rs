/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use leptos::ev::Event;
use leptos::*;

use fe14_calculator_core::character::{Character, CHARACTERS};
use fe14_calculator_core::class::{Class, CLASSES};
use fe14_calculator_core::stats::Stats;

//noinspection DuplicatedCode
#[component]
pub fn App(cx: Scope) -> impl IntoView {
  let (character, set_character) = create_signal(cx, CHARACTERS.get(0).cloned().unwrap());
  let (last_message, set_last_message) = create_signal(cx, String::new());
  let (enhanced, set_enhanced) = create_signal(cx, false);
  let (doubled, set_doubled) = create_signal(cx, false);

  let cur_character = move || character.get().name;
  let cur_class = move || character.get().cur_attribute.class;

  let avatar = move || format!("public/avatar/{}.webp", character.get().name);

  let lv = move || character.get().get_lv();
  let lv_max = move || character.get().get_max_lv().unwrap();
  let hlv = move || character.get().get_hlv();

  let lvl_up = move |_| {
    set_character.update(|character| {
      let result = character.level_up(enhanced.get(), doubled.get());
      if let Err(err) = result {
        set_last_message.set(err.to_string());
      }
    })
  };
  let reset = move |_| {
    set_character.update(|character| {
      *character = character.clone().init().unwrap();
    })
  };

  let select_character = move |ev: Event| {
    let target_character = event_target_value(&ev);
    set_character.set(Character::find(&target_character).unwrap().clone())
  };
  let select_class = move |ev: Event| {
    let target_class = event_target_value(&ev);
    set_character.update(|character| {
      let result = character.change_class(Class::find(&target_class).unwrap());
      if let Err(err) = result {
        set_last_message.set(err.to_string());
      }
    })
  };
  let use_second_seal = move |_| {
    set_character.update(|character| {
      let result =
        character.change_class(Class::find(&character.cur_attribute.class.clone()).unwrap());
      if let Err(err) = result {
        set_last_message.set(err.to_string());
      }
    })
  };

  let characters = move || {
    CHARACTERS
      .iter()
      .map(|it| it.name.clone())
      .map(|it| view! {cx, <option value=it.clone()>{it}</option>})
      .collect::<Vec<_>>()
  };
  let classes = move || {
    CLASSES
      .iter()
      .map(|it| it.name.clone())
      .map(|it| view! {cx, <option value=it.clone()>{it}</option>})
      .collect::<Vec<_>>()
  };

  let lvlLimitReachedClass = move || {
    if character.get().get_lv() == character.get().get_max_lv().unwrap() {
      "limitReached"
    } else {
      ""
    }
  };

  view! { cx,
    <div class="root">
      <div class="panel">
        <div class="inputItem">
          "角色: "
          <select on:change=select_character prop:value=cur_character>
            {characters}
          </select>
        </div>
        <div class="inputItem">
          "星玉加护: "
          <input type="checkbox" prop:value=move || enhanced.get() on:click=move |_| {set_enhanced.update(|it| *it = !*it)}/>
        </div>
        <div class="inputItem">
          "努力才能: "
          <input type="checkbox" prop:value=move || doubled.get() on:click=move |_| {set_doubled.update(|it| *it = !*it)}/>
        </div>
      </div>
      <div class="panel">
        <div>
          <button on:click=lvl_up>"升级"</button>
          <button on:click=reset>"重置"</button>
        </div>
        <div>
          <div class="statItem">
            <span>"LV: "<span class={lvlLimitReachedClass}>{lv}</span>"/"{lv_max}</span>
          </div>
          <div class="statItem">"隐藏LV: "{hlv}</div>
        </div>
      </div>
      <div class="panel">
        <div class="inputItem">
          "兵种: "
          <select on:change=select_class prop:value=cur_class>
            {classes}
          </select>
        </div>
        <button on:click=use_second_seal>"横转"</button>
      </div>
      <div class="panel">
        <img class="avatar" src=avatar/>
      </div>
      <Stats character=character/>
      {move || if !last_message.get().is_empty() {
        view! {cx,
          <div class="panel panelMsg">
            {last_message}
          </div>
        }
      } else {
        view! { cx,
          <div class="panel panelMsg panelMsgPlaceholder">"message output"</div>
        }
      }}
    </div>
  }
}

#[component]
fn Stats(cx: Scope, #[prop(into)] character: Signal<Character>) -> impl IntoView {
  let values = move || {
    let character = character.get();
    Stats::fields()
      .into_iter()
      .map(|it| {
        (
          it.to_string(),
          character.get_total_stat(it).unwrap(),
          character.get_upper_limit(it).unwrap(),
        )
      })
      .map(|(k, stat, limit)| {
        let limitReachedClass = if stat == limit { "limitReached" } else { "" };

        view! {cx,
          <div class="statItem"><span>{k}": "</span><span class={limitReachedClass}>{format!("{:.2}", stat as f64 / 100.0)}</span></div>
        }
      })
      .collect::<Vec<_>>()
  };

  view! { cx,
    <div class="panel">
      {values}
    </div>
  }
}
