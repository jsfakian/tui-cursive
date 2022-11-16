use std::collections::HashMap;

use crate::{state::GlobalState, herr, error::Result};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, SelectView, Dialog, ListView, EditView}, view::Resizable, Cursive,
};


pub const INSTALL_SERVER: &str = "Eve_install_server";
pub const INSTALL_DISK: &str = "Eve_install_disk";
pub const PERSIST_DISK: &str = "Eve_persist_disk";
pub const SOFT_SERIAL: &str = "Eve_soft_serial";
pub const REBOOT_AFTER_INSTALL: &str = "Eve_reboot_after_install";
pub const PAUSE_AFTER_INSTALL: &str = "Eve_pause_after_install";
pub const PAUSE_BEFORE_INSTALL: &str = "Eve_pause_before_install";
pub const ROOT: &str = "Root";
pub const FIND_BOOT: &str = "Find_boot";
pub const CONSOLE: &str = "Console";
pub const CONFIG: &str = "General Config";

type ConfigView = ResizedView<NamedView<ListView>>;

pub fn get_config(map: HashMap<String, String>) -> ConfigView {

    let l = ListView::new()
        .child(INSTALL_SERVER, EditView::new().content(map.get(INSTALL_SERVER).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, INSTALL_SERVER, v.to_string().as_str())))
        .child(INSTALL_DISK, EditView::new().content(map.get(INSTALL_DISK).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, INSTALL_DISK, v.to_string().as_str())))
        .child(PERSIST_DISK, EditView::new().content(map.get(PERSIST_DISK).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, PERSIST_DISK, v.to_string().as_str())))
        .child(SOFT_SERIAL, EditView::new().content(map.get(SOFT_SERIAL).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, SOFT_SERIAL, v.to_string().as_str())))
        .child(REBOOT_AFTER_INSTALL, EditView::new().content(map.get(REBOOT_AFTER_INSTALL).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, REBOOT_AFTER_INSTALL, v.to_string().as_str())))
        .child(PAUSE_AFTER_INSTALL, EditView::new().content(map.get(PAUSE_AFTER_INSTALL).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, PAUSE_AFTER_INSTALL, v.to_string().as_str())))
        .child(PAUSE_BEFORE_INSTALL, EditView::new().content(map.get(PAUSE_BEFORE_INSTALL).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, PAUSE_BEFORE_INSTALL, v.to_string().as_str())))
        .child(ROOT, EditView::new().content(map.get(ROOT).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, ROOT, v.to_string().as_str())))
        .child(FIND_BOOT, EditView::new().content(map.get(FIND_BOOT).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, FIND_BOOT, v.to_string().as_str())))
        .child(CONSOLE, EditView::new().content(map.get(CONSOLE).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_config, CONSOLE, v.to_string().as_str())));
    l.with_name(CONFIG).full_height()
}

fn set_config(c: &mut Cursive, key: &str, value: &str) -> Result<()> {
    let mut s: GlobalState = c.take_user_data().unwrap();
    s.data.map.insert(key.to_string(), value.to_string());
    c.set_user_data(s);

    Ok(())
}