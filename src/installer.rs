use cursive::{
    views::{LinearLayout, TextView,},
    Cursive, CursiveExt,
};
use serde_json::Value;

use crate::{
    views::{ctrl_buttons::buttons, fs_select::get_fs},
    state::{GlobalState, CurrentState, Move,},
    data::{Data, INTERACTIVE_MODE, LABELS, FS},
    actions::{execute},
};

use cursive_aligned_view::Alignable;

pub fn config(in_json: Value) {
    let mut c = Cursive::default();
    let data = Data::new(in_json);
    if data.map.get(INTERACTIVE_MODE).unwrap() == "false" {
        return;
    }
    
    let state = GlobalState {
        data,
        current_state: CurrentState::FS,
    };

    c.set_user_data(state.clone());

    c.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(TextView::new("Installer").align_center())
            .child(get_fs(state.data.map.get(FS).unwrap().clone()))
            .child(buttons(false))
    );

    let map = state.data.map.clone();
    for label in LABELS.iter() {
        let v = map.get(label.clone()).unwrap();
        if v != "" {
            let _res = execute(&mut c, Move::Next);
        } else {
            break;
        }
    }

    c.run()
}

pub fn run() {
    println!("Installing EVE");
}
