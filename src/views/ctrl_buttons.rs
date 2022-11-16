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
    traits::{Nameable},
    views::{Button, NamedView, OnEventView, ResizedView, ListView}, view::Resizable,
};

use crate::{actions::execute, herr};

use crate::state::Move;

pub const BUTTONS: &str = "Buttons";

//type CmdLine = OnEventView<ResizedView<NamedView<ListView>>>;
type CmdLine = OnEventView<ResizedView<NamedView<ListView>>>;

pub fn buttons() -> CmdLine {
    OnEventView::new(
        ListView::new()
            .child("Navigation", Button::new_raw("previous", |c| herr!(c, execute, Move::Previous)))
            .child("", Button::new_raw("next", |c| herr!(c, execute, Move::Next)))
            .with_name(BUTTONS)
            .full_width(),
    )
}
