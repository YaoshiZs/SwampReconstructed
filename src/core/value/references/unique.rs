use std::fmt;
use std::rc::Rc;
use crate::core::value::{Addr, GetAddr};

#[derive(Clone)]
pub struct Unique(Rc<String>);

impl Unique {
    pub fn get_id(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Unique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unique({})", self.0)
    }
}

impl From<&str> for Unique {
    fn from(value: &str) -> Self {
        Self(String::from(value).into())
    }
}
impl From<String> for Unique {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl GetAddr for Unique {
    fn get_addr(&self) -> Addr {
        let ptr = self as *const Unique;
        ptr as Addr
    }
}

// --- --- --- --- --- ---

pub struct GlobalUnique(Option<Unique>);
pub const EMPTY_GLOBAL_UNIQUE: GlobalUnique = GlobalUnique(None);

impl GlobalUnique {
    pub fn init(&mut self, id: &str) {
        self.0 = Some(Unique::from(id));
    }

    pub fn unwrap(&self) -> Unique {
        let value_ref = self.0.as_ref();
        let Some(wrapped) = value_ref else {
            unreachable!()
        };
        wrapped.clone()
    }
}
