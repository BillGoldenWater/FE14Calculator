/*
 *  Copyright 2023 Golden_Water, 火纹梅戚
 *  SPDX-License-Identifier: AGPL-3.0-only
 */

use leptos::ev::Event;
use leptos::*;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

use fe14_calculator_core::character::character_with_undo::CharacterWithUndo;
use fe14_calculator_core::character::{Character, CHARACTERS};
use fe14_calculator_core::class::{Class, CLASSES};
use fe14_calculator_core::stats::Stats;

//noinspection DuplicatedCode
#[component]
pub fn App(cx: Scope) -> impl IntoView {
  let (character, set_character) = create_signal(
    cx,
    CharacterWithUndo::new(CHARACTERS.get(0).cloned().unwrap()),
  );
  let (last_message, set_last_message) = create_signal(cx, String::new());
  let (enhanced, set_enhanced) = create_signal(cx, false);
  let (doubled, set_doubled) = create_signal(cx, false);

  let msg_box_ref = create_node_ref::<html::Div>(cx);
  let update_msg_box = move |msg: String| {
    set_last_message.set(msg);
    if let Some(msg_box) = msg_box_ref.get() {
      msg_box.scroll_into_view_with_scroll_into_view_options(
        ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth),
      );
    }
  };

  let character = move || character.get().get();

  let cur_character = move || character().name;
  let cur_class = move || character().cur_attribute.class;

  let avatar = move || format!("public/avatar/{}.webp", character().name);

  let lv = move || character().get_lv();
  let lv_max = move || character().get_max_lv().unwrap();
  let hlv = move || character().get_hlv();

  let lvl_up = move |_| {
    set_character.update(|character| {
      let result = character.level_up(enhanced.get(), doubled.get());
      if let Err(err) = result {
        update_msg_box(err.to_string());
      }
    })
  };
  let reset = move |_| {
    set_character.update(|character| {
      if let Some(window) = web_sys::window() {
        if !window
          .confirm_with_message("确定重置吗, 此操作不可撤销")
          .unwrap_or(false)
        {
          return;
        }
      }
      *character = CharacterWithUndo::new(character.get().init().unwrap());
    })
  };
  let undo = move |_| {
    set_character.update(|character| {
      character.undo();
    })
  };
  let redo = move |_| {
    set_character.update(|character| {
      character.redo();
    })
  };

  let select_character = move |ev: Event| {
    let target_character = event_target_value(&ev);
    set_character.set(CharacterWithUndo::new(
      Character::find(&target_character).unwrap().clone(),
    ))
  };
  let select_class = move |ev: Event| {
    let target_class = event_target_value(&ev);
    set_character.update(|character| {
      let result = character.change_class(Class::find(&target_class).unwrap());
      if let Err(err) = result {
        update_msg_box(err.to_string());
      }
    })
  };
  let use_second_seal = move |_| {
    set_character.update(|character| {
      let result =
        character.change_class(Class::find(&character.get().cur_attribute.class).unwrap());
      if let Err(err) = result {
        update_msg_box(err.to_string());
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
    if character().get_lv() == character().get_max_lv().unwrap() {
      "limitReached"
    } else {
      ""
    }
  };

  view! { cx,
    <div class="root">
      <div class="panel">
        <div class="inputItem horizontalBox">
          <img class="avatar" src=avatar/>
          <select on:change=select_character prop:value=cur_character>
            {characters}
          </select>
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
        <div>
          <div class="statItem">
            <span>"LV: "<span class={lvlLimitReachedClass}>{lv}</span><span class="maximumLabel">"/"{lv_max}</span></span>
          </div>
          <div class="horizontalBox">
            <div class="statItem">"隐藏LV: "{hlv}</div>
            <div class="statItem">"总LV: "{move || lv() + hlv()}</div>
          </div>
        </div>
      </div>
      <div class="panel horizontalBox">
        <div class="horizontalBox">
          <div class="verticalBox">
            <button on:click=lvl_up>"升级"</button>
            <button on:click=reset>"重置"</button>
          </div>
          <div class="verticalBox">
            <button on:click=undo>"撤销"</button>
            <button on:click=redo>"重做"</button>
          </div>
        </div>
        <div>
          <div class="inputItem">
            "星玉加护: "
            <input type="checkbox" prop:value=move || enhanced.get() on:click=move |_| {set_enhanced.update(|it| *it = !*it)}/>
          </div>
          <div class="inputItem">
            "努力才能: "
            <input type="checkbox" prop:value=move || doubled.get() on:click=move |_| {set_doubled.update(|it| *it = !*it)}/>
          </div>
        </div>
      </div>
      <Stats character=Signal::derive(cx, character)/>
      {move || if !last_message.get().is_empty() {
        view! {cx,
          <div _ref=msg_box_ref class="panel panelMsg">
            {last_message}
          </div>
        }
      } else {
        view! { cx,
          <div _ref=msg_box_ref class="panel panelMsg panelMsgPlaceholder">"message output"</div>
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
          Stats::get_l10n(it),
          character.get_total_stat(it).unwrap(),
          character.get_upper_limit(it).unwrap(),
        )
      })
      .map(|(k, stat, limit)| {
        let limitReachedClass = if stat == limit { "limitReached" } else { "" };

        view! {cx,
          <div class="statItem">
            <span>{k}": "</span>
            <span>
              <span class={limitReachedClass}>{format!("{:.2}", stat as f64 / 100.0)}</span>
              <span class="maximumLabel">"/"{limit / 100}</span>
            </span>
          </div>
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
