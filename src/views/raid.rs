/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */

use crate::{state::{GlobalState, Move}, herr, error::Result, data::Data, actions::execute};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, Dialog, SelectView}, view::Resizable, Cursive,
};

pub const RAID: &str = "RAID";
pub const RAIDLIST: &str = "RaidList";
type RaidView = ResizedView<NamedView<Dialog>>;


pub fn get_raid(value: String) -> RaidView {
    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.txt").unwrap();
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
            .on_submit(move |s, v| herr!(s, set_raid, v.to_string().as_str()))
            .with_name(RAID)
            .fixed_width(10));
    d.with_name(RAIDLIST).full_height()
    //*c.data.map.entry("Find_boot".to_string()).or_insert("".to_string());

    //l.set_on_select(|c, f| herr!(c, set_config, Field { label: key, value: value }));

    //l.with_name(RAIDLIST).full_height()
}

fn set_raid(c: &mut Cursive, value: &str) -> Result<()> {
    //let map = &mut c.user_data::<crate::data::Data>().unwrap().map;
    //*map.entry(key.to_string()).or_insert("".to_string()) = value.to_string();

    let mut s: GlobalState = c.take_user_data().unwrap();
    s.data.map.insert(RAID.to_string(), value.to_string());
    c.set_user_data(s);

    let cb = c.cb_sink().clone();
    cb.send(Box::new(move |s: &mut cursive::Cursive| {
        execute(s, Move::Next);
    })).unwrap();

    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.json")?;
    //let _res = file.write(value.as_bytes());
    //serde_json::to_writer(file, map)?;

    Ok(())
}