/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */
use crate::{herr, utils::save_config_value, data::FS};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, SelectView, Dialog}, view::Resizable,
};

type FSView = ResizedView<NamedView<Dialog>>;

pub fn get_fs(value: String) -> FSView {
    let key = "Choose FS";
    let value: usize = value.parse().unwrap();
    let d = Dialog::new()
        .title(key)
        .content(SelectView::new()
            .item("EXT3", 0)
            .item("EXT4", 1)
            .item("ZFS", 2)
            .selected(value)
            .on_submit(move |s, v| 
                herr!(s, save_config_value, FS, v.to_string().as_str(), true))
            .fixed_width(10));
        //.with_name(RAID))
    d.with_name(FS).full_height()
}
