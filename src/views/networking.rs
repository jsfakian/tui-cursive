use std::collections::HashMap;

use crate::{
    data::{DNS, GATEWAY, NETWORKING, SUBNET},
    herr,
    utils::save_config_value,
};

use cursive::{
    traits::Nameable,
    view::Resizable,
    views::{EditView, ListView, NamedView, ResizedView},
};

type NetworkingView = ResizedView<NamedView<ListView>>;

pub fn get_networking(map: HashMap<String, String>) -> NetworkingView {
    let l = ListView::new()
        .child(
            SUBNET,
            EditView::new()
                .content(map.get(SUBNET).unwrap().clone())
                .on_edit(move |s, v, _| {
                    herr!(s, save_config_value, SUBNET, v.to_string().as_str(), false)
                }),
        )
        .child(
            GATEWAY,
            EditView::new()
                .content(map.get(GATEWAY).unwrap().clone())
                .on_edit(move |s, v, _| {
                    herr!(s, save_config_value, GATEWAY, v.to_string().as_str(), false)
                }),
        )
        .child(
            DNS,
            EditView::new()
                .content(map.get(DNS).unwrap().clone())
                .on_edit(move |s, v, _| {
                    herr!(s, save_config_value, DNS, v.to_string().as_str(), false)
                }),
        );
    l.with_name(NETWORKING).full_height()
}
