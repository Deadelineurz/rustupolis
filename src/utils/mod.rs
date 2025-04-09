pub mod interruptible_sleep;

use crate::engine::core::LockableEngine;
use crate::threads::sidebar::SideBarMessage;

#[macro_export]
macro_rules! send_to_side_bar_auto {
    ($engine:expr, $($arg: expr);+, $log_type: expr, $log_color: expr) => {
        crate::utils::send_to_side_bar($engine, ((vec![$(std::boxed::Box::new($arg),)+]), $log_type, $log_color))
    };
}

#[inline]
pub fn send_to_side_bar(engine: &LockableEngine, msg: SideBarMessage) {
    match engine.read() {
        Ok(x) => {
            let _ = x.side_bar_tx.send(msg);
        }

        _ => {}
    }
}