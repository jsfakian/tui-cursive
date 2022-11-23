/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */
use crate::{data::FS, herr, utils::save_config_value};

use cursive::{
    traits::Nameable,
    view::Resizable,
    views::{Dialog, NamedView, ResizedView, SelectView},
};

type FSView = ResizedView<NamedView<Dialog>>;

fn get_fs_index(value: &str) -> usize {
    match value {
        "EXT3" => 0,
        "EXT4" => 1,
        "ZFS" => 2,
        &_ => 0,
    }
}

pub fn get_fs(value: String) -> FSView {
    let key = "Choose FS";
    let d = Dialog::new().title(key).content(
        SelectView::new()
            .item("EXT3", "EXT3")
            .item("EXT4", "EXT4")
            .item("ZFS", "ZFS")
            .selected(get_fs_index(&value))
            .on_submit(move |s, v| herr!(s, save_config_value, FS, v, true))
            .fixed_width(10),
    );
    //.with_name(RAID))
    d.with_name(FS).full_height()
}
