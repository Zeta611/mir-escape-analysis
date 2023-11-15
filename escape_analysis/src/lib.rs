#![feature(rustc_attrs)]

use std::ops::{Deref, DerefMut};

#[cfg_attr(not(test), rustc_diagnostic_item = "EscapeAnalysis")]
pub trait EscapeAnalysis {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(test), rustc_diagnostic_item = "OptBox")]
pub struct OptBox<T>(Box<T>);

impl<T> OptBox<T> {
    pub fn new(t: T) -> Self {
        OptBox(Box::new(t))
    }
}

impl<T> Deref for OptBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OptBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
