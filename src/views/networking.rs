use std::collections::HashMap;

use crate::{state::GlobalState, herr, error::Result};

use cursive::{
    traits::{Nameable},
    views::{NamedView, ResizedView, SelectView, ListView, EditView}, view::Resizable, Cursive, align::HAlign,
};
use network_interface::{NetworkInterface, NetworkInterfaceConfig};


pub const NETWORKING: &str = "Networking";
pub const ADAPTER: &str = "Adapter";
pub const SUBNET: &str = "Subnet";
pub const GATEWAY: &str = "Gateway";
pub const DNS: &str = "DNS";
type NetworkingView = ResizedView<NamedView<ListView>>;

pub fn get_networking(map: HashMap<String, String>) -> NetworkingView {
    let adapter = map.get(ADAPTER).unwrap();
    let adapter: usize = adapter.parse().unwrap();

    let network_interfaces = NetworkInterface::show().unwrap();
    let mut niv:SelectView<i32> = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();
    let mut i = 0;
    for itf in network_interfaces.iter() {
        niv.add_item(itf.name.clone(), i);
        i += 1;
    }

    let l = ListView::new()
        .child(ADAPTER, niv.selected(adapter).on_submit(move |s, v| herr!(s, set_networking, ADAPTER, v.to_string().as_str())))
        .child(SUBNET, EditView::new().content(map.get(SUBNET).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_networking, SUBNET, v.to_string().as_str())))
        .child(GATEWAY, EditView::new().content(map.get(GATEWAY).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_networking, GATEWAY, v.to_string().as_str())))
        .child(DNS, EditView::new().content(map.get(DNS).unwrap().clone()).on_edit(move |s, v, _| herr!(s, set_networking, DNS, v.to_string().as_str())));
    l.with_name(NETWORKING).full_height()
}

fn set_networking(c: &mut Cursive, key: &str, value: &str) -> Result<()> {
    let mut s: GlobalState = c.take_user_data().unwrap();
    s.data.map.insert(key.to_string(), value.to_string());
    c.set_user_data(s);

    Ok(())
}