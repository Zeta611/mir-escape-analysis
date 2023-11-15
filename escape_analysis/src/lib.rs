#![feature(rustc_attrs)]

use std::ops::{Deref, DerefMut};

#[cfg_attr(not(test), rustc_diagnostic_item = "EscapeAnalysis")]
pub trait EscapeAnalysis {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(test), rustc_diagnostic_item = "OptBox")]
pub struct OptBox<T: ?Sized>(Box<T>);

impl<T> OptBox<T> {
    pub fn new(t: T) -> Self {
        OptBox(Box::new(t))
    }
}

impl<T: ?Sized> OptBox<T> {
    pub fn from_box(boxed: Box<T>) -> Self {
        OptBox(boxed)
    }
}

impl<T: ?Sized> Deref for OptBox<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ?Sized> DerefMut for OptBox<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ?Sized> AsRef<T> for OptBox<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> AsMut<T> for OptBox<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Clone> Clone for OptBox<T> {
    fn clone(&self) -> Self {
        OptBox(self.0.clone())
    }
}

impl<T: Clone> Clone for OptBox<[T]> {
    fn clone(&self) -> Self {
        let cloned_slice = self.0.to_vec().into_boxed_slice();
        OptBox(cloned_slice)
    }
}

impl<T> From<Vec<T>> for OptBox<[T]> {
    fn from(vec: Vec<T>) -> Self {
        OptBox(vec.into_boxed_slice())
    }
}
