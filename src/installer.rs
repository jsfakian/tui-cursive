use cursive::{
    views::{LinearLayout, TextView,},
    Cursive, CursiveExt,
};

use crate::{
    views::{fs_select::get_fs, ctrl_buttons::buttons,},
    state::{GlobalState, CurrentState,},
    data::Data,
};

use cursive_aligned_view::Alignable;

pub fn run() {
    let mut c = Cursive::default();
    let data = Data::new();
    
    let state = GlobalState {
        data,
        current_state: CurrentState::FS,
    };

    c.set_user_data(state.clone());

    c.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(TextView::new("Installer").align_center())
            .child(get_fs("0".to_string()))
            .child(buttons()),
    );

    c.run()
}
