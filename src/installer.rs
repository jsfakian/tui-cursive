use cursive::{
    views::{LinearLayout, TextView,},
    Cursive, CursiveExt,
};
use serde_json::Value;

use crate::{
    views::{fs_select::get_fs, ctrl_buttons::buttons,},
    state::{GlobalState, CurrentState,},
    data::Data,
};

use cursive_aligned_view::Alignable;

pub fn run(in_json: Value) {
    let mut c = Cursive::default();
    let data = Data::new(in_json);
    
    let state = GlobalState {
        data,
        current_state: CurrentState::FS,
    };

    c.set_user_data(state.clone());

    c.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(TextView::new("Installer").align_center())
            .child(get_fs("0".to_string()))
            .child(buttons(false)),
    );

    c.run()
}
