/*
 * rbmenu-tui - RBMenu TUI
 * Copyright (C) 2022 DevHyperCoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{io::{Write}, fs::File};

use cursive::{
    views::{LinearLayout, TextView, EditView, Dialog,},
    Cursive
};
use cursive_aligned_view::Alignable;

use crate::{
    state::{CurrentState, GlobalState},
    error::Result,
    utils::{get_state_mut},
    views::{raid::{get_raid, RAID}, ctrl_buttons::buttons, fs_select::{get_fs, FS}, networking::get_networking, config::get_config},
};

use crate::state::Move;

fn new_state(c: &mut Cursive, state: GlobalState) -> Box<(dyn cursive::View + 'static)> {
    match state.current_state {
        CurrentState::FS => {
            let value = state.data.map.get(FS).unwrap().clone();
            return Box::new(get_fs(value));
        }
        CurrentState::Raid => {
            let value = state.data.map.get(RAID).unwrap().clone();
            return Box::new(get_raid(value));
        }
        CurrentState::Networking => {
            let map = state.data.map.clone();
            return Box::new(get_networking(map));
        }
        CurrentState::Config => {
            let map = state.data.map.clone();
            return Box::new(get_config(map));
        }
    };
}

fn navigate(c: &mut Cursive, state: GlobalState) {
    let view = new_state(c, state);
    c.pop_layer();
    c.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(TextView::new("Installer").align_center())
            .child(view)
            .child(buttons()),
    );
}

pub fn execute(c: &mut Cursive, m: Move) -> Result<()> {
    let mut state = get_state_mut(c)?;
    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.txt")?;
    //let _res = file.write(state.current_state.to_string().as_bytes());
    match m {
        Move::Previous => {
            state.current_state = state.current_state.prev();
        }
        Move::Next => {
            state.current_state = state.current_state.next();
        }
    }
    c.set_user_data(state.clone());
    //let _res = file.write(state.current_state.to_string().as_bytes());
    navigate(c, state);

    /*match state.current_state {
        CurrentState::Config => {
            navigate(c, m, state);
        }
        CurrentState::FS => {
        }
        CurrentState::Raid => {
        }
        CurrentState::Networking => todo!(),
    };*/

    //load_bookmarks(c)?;

    Ok(())
}


