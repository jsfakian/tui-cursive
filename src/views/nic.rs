/*
 * installer-tui
 * Copyright (C) 2022 Ioannis Sfakianakis
 */

use crate::{herr, utils::save_config_value, data::NIC};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, Dialog, SelectView}, view::Resizable, align::HAlign,
};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

type NICView = ResizedView<NamedView<Dialog>>;

pub fn get_nic(value: String) -> NICView {
    let title = "Choose NIC";
    let value: usize = value.parse().unwrap();

    let network_interfaces = NetworkInterface::show().unwrap();
    let mut niv:SelectView<i32> = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();
    let mut i = 0;
    for itf in network_interfaces.iter() {
        niv.add_item(itf.name.clone(), i);
        i += 1;
    }

    let d = Dialog::new()
        .title(title)
        .content(niv.selected(value)
            .selected(value)
            .on_submit(move |s, v| 
                herr!(s, save_config_value, NIC, v.to_string().as_str(), true))
            .fixed_width(10));
    d.with_name(NIC).full_height()
}