/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */

use crate::{herr, utils::save_config_value, data::RAID};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, Dialog, SelectView}, view::Resizable,
};

type RaidView = ResizedView<NamedView<Dialog>>;

fn get_raid_index(value: &str) -> usize {
    match value {
        "0" => 0,
        "1" => 1,
        "5" => 2,
        "10" => 3,
        &_ => 0,
    }
}

pub fn get_raid(value: String) -> RaidView {
    let key = "Choose RAID";
    let d = Dialog::new()
        .title(key)
        .content(SelectView::new()
            .item("0", "0")
            .item("1", "1")
            .item("5", "5")
            .item("10", "10")
            .selected(get_raid_index(&value))
            .on_submit(move |s, v| 
                herr!(s, save_config_value, RAID, v, true))
            .fixed_width(10));
    d.with_name(RAID).full_height()
    
    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.json")?;
    //let _res = file.write(value.as_bytes());
}