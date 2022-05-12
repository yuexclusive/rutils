#![allow(dead_code)]

use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex, Once};

#[derive(Debug)]
struct Config {
    db_connection_str: String,
}

fn singleton() -> &'static Mutex<Config> {
    static mut CONF: MaybeUninit<Mutex<Config>> = MaybeUninit::uninit();
    static ONCE: Once = Once::new();

    ONCE.call_once(|| unsafe {
        CONF.as_mut_ptr().write(Mutex::new(Config {
            db_connection_str: "test config".to_string(),
        }));
    });

    unsafe { &*CONF.as_ptr() }
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, cell::RefCell, ops::Deref, rc::Rc};

    use super::*;
    #[test]
    fn test_singleton() {
        let c1 = singleton();
        let c2 = singleton();

        assert_eq!(format!("{:p}", c1), format!("{:p}", c2));
    }
}
