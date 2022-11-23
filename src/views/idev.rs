/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */

use std::collections::HashMap;

use crate::{
    data::{INSTALL_DISK, PERSIST_DISK},
    herr,
    utils::{get_block_devices, save_config_value},
};

use cursive::{
    align::HAlign,
    traits::Nameable,
    view::Resizable,
    views::{Dialog, NamedView, ResizedView, SelectView},
};

type IDEVView = ResizedView<NamedView<Dialog>>;

pub fn get_idev(map: HashMap<String, String>) -> IDEVView {
    let title = "Choose installation disk";
    let idev = map.get(INSTALL_DISK).unwrap().clone();
    let pdev = map.get(PERSIST_DISK).unwrap().clone();
    let mut selected: usize = 0;

    let mut bv: SelectView<String> = SelectView::new().h_align(HAlign::Center).autojump();
    let mut i = 0;

    let devices = get_block_devices();
    for d in devices.unwrap() {
        if d == idev {
            selected = i;
        }
        if !d.contains(&pdev) {
            bv.add_item(d.clone(), d.clone());
            i += 1;
        }
    }

    let d = Dialog::new().title(title).content(
        bv.selected(selected)
            .on_submit(move |s, v| herr!(s, save_config_value, INSTALL_DISK, v, true))
            .fixed_width(10),
    );
    d.with_name(INSTALL_DISK).full_height()

    //let mut file = File::create("/Users/jsfakian/Documents/src/tui-cursive/debug.json")?;
    //let _res = file.write(value.as_bytes());
}
