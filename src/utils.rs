use cursive::Cursive;

use crate::{error::Error, state::GlobalState};

pub fn get_state_mut(c: &mut Cursive) -> Result<GlobalState, Error> {
    match c.take_user_data::<GlobalState>() {
        Some(data) => {
            c.set_user_data(data.clone());
            Ok(data)
        }
        None => Err(Error::NoState),
    }
}

#[macro_export]
macro_rules! herr {
    ($c:expr,$f:expr) => {{
        if let Err(e) = $f($c) {
            e.show_dialog($c);
            return
        }
    }};
    ($c:expr,$f:expr,$($args:expr),*) => {{
        if let Err(e) = $f($c,$($args),*) {
            e.show_dialog($c);
            return
        }
    }};
}

#[macro_export]
macro_rules! herrcl {
    ($f:expr) => {{
        |c| {
            use crate::herr;
            herr!(c,$f);
        }
    }};
    ($f:expr,$($args:expr),*) => {{
        move |c| {
            use crate::herr;
            herr!(c,$f,$($args),*);
        }
    }};
}
