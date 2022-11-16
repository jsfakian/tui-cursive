/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */

use std::collections::HashMap;

use crate::{state::{GlobalState, Move}, herr, error::Result, actions::execute};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, SelectView, Dialog}, view::Resizable, Cursive,
};


pub const FS_LIST: &str = "FS_LIST";
pub const FS: &str = "FS";
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
            .on_submit(move |s, v| herr!(s, set_fs, v.to_string().as_str()))
            .with_name(FS)
            .fixed_width(10));
        //.with_name(RAID))
    d.with_name(FS_LIST).full_height()
}

fn set_fs(c: &mut Cursive, value: &str) -> Result<()> {
    let mut s: GlobalState = c.take_user_data().unwrap();
    s.data.map.insert(FS.to_string(), value.to_string());
    c.set_user_data(s);

    let cb = c.cb_sink().clone();
    cb.send(Box::new(move |s: &mut cursive::Cursive| {
        execute(s, Move::Next);
    })).unwrap();

    Ok(())
}