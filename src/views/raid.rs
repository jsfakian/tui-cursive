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


pub fn get_raid(value: String) -> RaidView {
    let key = "Choose RAID";
    let value: usize = value.parse().unwrap();
    let d = Dialog::new()
        .title(key)
        .content(SelectView::new()
            .item("0", 0)
            .item("1", 1)
            .item("5", 2)
            .item("10", 3)
            .selected(value)
            .on_submit(move |s, v| 
                herr!(s, save_config_value, RAID, v.to_string().as_str(), true))
            .fixed_width(10));
    d.with_name(RAID).full_height()
    
    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.json")?;
    //let _res = file.write(value.as_bytes());
}