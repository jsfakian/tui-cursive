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
use cursive::{
    views::{LinearLayout, TextView},
    Cursive
};
use cursive_aligned_view::Alignable;

use crate::{
    state::{CurrentState, GlobalState},
    error::Result,
    utils::{get_state_mut},
    views::{
        raid::get_raid, 
        ctrl_buttons::buttons, 
        fs_select::get_fs, 
        networking::get_networking, 
        config::get_config, 
        idev::get_idev, 
        nic::get_nic, 
        pdev::get_pdev
    }, 
    data::{
        NIC, 
        FS, 
        RAID,
    },
};

use crate::state::Move;

fn new_state(state: GlobalState) -> Box<(dyn cursive::View + 'static)> {
    let map = state.data.map.clone();
    match state.current_state {
        CurrentState::FS => {
            return Box::new(get_fs(map.get(FS).unwrap().clone()));
        }
        CurrentState::Raid => {
            return Box::new(get_raid(map.get(RAID).unwrap().clone()));
        }
        CurrentState::NIC => {
            return Box::new(get_nic(map.get(NIC).unwrap().clone()));
        }
        CurrentState::Networking => {
            return Box::new(get_networking(map));
        }
        CurrentState::IDEV => {
            return Box::new(get_idev(map));
        }
        CurrentState::PDEV => {
            return Box::new(get_pdev(map));
        }
        CurrentState::Config => {
            return Box::new(get_config(map));
        }
    };
}

fn navigate(c: &mut Cursive, state: GlobalState) {
    let view = new_state(state);
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


