#![allow(dead_code)]

use core::cell::{Ref, RefCell, RefMut};
use cortex_m::{
    self,
    interrupt::{free, CriticalSection, Mutex},
};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Error {
    AlreadyInitialized,
    NotInitialized,
}

pub struct MutableMutex<T>(Mutex<RefCell<Option<T>>>);
impl<'r, T> MutableMutex<T> {
    pub const fn new() -> Self {
        Self(Mutex::new(RefCell::new(None)))
    }
    pub fn borrow(&'r self, cs: &'r CriticalSection) -> Ref<'r, Option<T>> {
        self.0.borrow(cs).borrow()
    }
    pub fn borrow_mut(&'r self, cs: &'r CriticalSection) -> RefMut<'r, Option<T>> {
        self.0.borrow(cs).borrow_mut()
    }
    pub fn initialize(&self, cs: &CriticalSection, data: T) -> Result<(), Error> {
        if self.borrow(cs).is_none() {
            *self.borrow_mut(cs) = Some(data);
            Ok(())
        } else {
            Err(Error::AlreadyInitialized)
        }
    }
    pub fn replace_with<F: Fn(T) -> T>(&self, cs: &CriticalSection, f: F) -> Result<(), Error> {
        let t = self.borrow_mut(cs).take().ok_or(Error::NotInitialized)?;
        *self.borrow_mut(cs) = Some(f(t));
        Ok(())
    }
    pub fn modify<F: Fn(&mut T)>(&self, cs: &CriticalSection, f: F) -> Result<(), Error> {
        self.replace_with(cs, |mut v| {
            f(&mut v);
            v
        })
    }
    pub fn deinitialize(&self, cs: &CriticalSection) -> Result<(), Error>
    where
        T: Drop,
    {
        if self.borrow(cs).is_some() {
            drop(self.borrow_mut(cs).take().unwrap());
            Ok(())
        } else {
            Err(Error::NotInitialized)
        }
    }
    pub fn critical_initialize(&self, data: T) -> Result<(), Error> {
        free(|cs| self.initialize(cs, data))
    }
    pub fn critical_replace_with<F: Fn(T) -> T>(&self, f: F) -> Result<(), Error> {
        free(|cs| self.replace_with(cs, f))
    }
    pub fn critical_modify<F: Fn(&mut T)>(&self, f: F) -> Result<(), Error> {
        free(|cs| self.modify(cs, f))
    }
    pub fn critical_deinitialize<F: Fn(&mut T)>(&self) -> Result<(), Error>
    where
        T: Drop,
    {
        free(|cs| self.deinitialize(cs))
    }
}
