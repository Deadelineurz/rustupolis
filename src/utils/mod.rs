pub mod interruptible_sleep;
pub mod pair;

use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use crate::engine::core::{Engine, LockableEngine};
use crate::threads::sidebar::SideBarMessage;

#[macro_export]
macro_rules! send_to_side_bar_auto {
    (r, $engine:expr, $($arg: expr);+, $log_type: expr, $log_color: expr) => {
        crate::utils::send_to_side_bar_read($engine, ((vec![$(std::boxed::Box::new($arg),)+]), $log_type, $log_color));
    };
    (w, $engine:expr, $($arg: expr);+, $log_type: expr, $log_color: expr) => {
        crate::utils::send_to_side_bar_write($engine, ((vec![$(std::boxed::Box::new($arg),)+]), $log_type, $log_color));
    };
    (e, $engine:ident, $($arg: expr);+, $log_type: expr, $log_color: expr) => {
        lock_read!($engine |> s);
        crate::utils::send_to_side_bar_read(&s, ((vec![$(std::boxed::Box::new($arg),)+]), $log_type, $log_color));
        lock_unlock!(s);
    };
}

#[macro_export]
macro_rules! lock_read {
    ($lock: ident |> $lock_name: ident) => {
        //log::debug!("Reading {} into {}", stringify!($lock), stringify!($lock_name));
        let $lock_name = $lock.read().unwrap();
        //log::debug!("Lock {} acquired", stringify!($lock_name))
    }
}

#[macro_export]
macro_rules! lock_write {
    ($lock: ident |> $lock_name: ident) => {
        //log::debug!("Writing {} into {}", stringify!($lock), stringify!($lock_name));
        let mut $lock_name = $lock.write().unwrap();
        //log::debug!("Lock {} acquired", stringify!($lock_name))
    }
}

#[macro_export]
macro_rules! lock_unlock {
    ($lock: ident) => {
        //log::debug!("Unlocking {}", stringify!($lock));
        drop($lock);
    };
}

#[inline]
pub fn send_to_side_bar_read(engine: &RwLockReadGuard<Engine>, msg: SideBarMessage) {
    let _ = engine.side_bar_tx.send(msg);
}

#[inline]
pub fn send_to_side_bar_write(engine: &RwLockWriteGuard<Engine>, msg: SideBarMessage) {
    let _ = engine.side_bar_tx.send(msg);
}