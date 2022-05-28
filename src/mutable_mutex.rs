#![allow(dead_code)]

use core::cell::{RefCell, RefMut};
use cortex_m::{
    self,
    interrupt::{free, CriticalSection, Mutex},
};

#[derive(Debug, Clone)]
pub struct AlreadyInitialized;
#[derive(Debug, Clone)]
pub struct NotInitialized;

pub struct MutableMutex<T>(Mutex<RefCell<Option<T>>>);
impl<'r, T> MutableMutex<T> {
    pub const fn new() -> Self {
        Self(Mutex::new(RefCell::new(None)))
    }
    pub fn borrow_mut(&'r self, cs: &'r CriticalSection) -> RefMut<'r, Option<T>> {
        self.0.borrow(cs).borrow_mut()
    }
    pub fn initialize(&self, cs: &CriticalSection, data: T) -> Result<(), AlreadyInitialized> {
        if self.0.borrow(cs).borrow().is_some() {
            return Err(AlreadyInitialized);
        }
        *self.borrow_mut(cs) = Some(data);
        Ok(())
    }
    pub fn critical_initialize(&self, data: T) -> Result<(), AlreadyInitialized> {
        free(|cs| self.initialize(cs, data))
    }
    pub fn replace_with<F: Fn(T) -> T>(
        &self,
        cs: &CriticalSection,
        f: F,
    ) -> Result<(), NotInitialized> {
        let t = self
            .borrow_mut(cs)
            .take()
            .ok_or(())
            .map_err(|_| NotInitialized)?;
        *self.borrow_mut(cs) = Some(f(t));
        Ok(())
    }
    pub fn critical_replace_with<F: Fn(T) -> T>(&self, f: F) -> Result<(), NotInitialized> {
        free(|cs| self.replace_with(cs, f))
    }
    pub fn modify<F: Fn(&mut T)>(&self, cs: &CriticalSection, f: F) -> Result<(), NotInitialized> {
        let mut t = self
            .borrow_mut(cs)
            .take()
            .ok_or(())
            .map_err(|_| NotInitialized)?;
        f(&mut t);
        *self.borrow_mut(cs) = Some(t);
        Ok(())
    }
    pub fn critical_modify<F: Fn(&mut T)>(&self, f: F) -> Result<(), NotInitialized> {
        free(|cs| self.modify(cs, f))
    }
    pub fn deinitialize(&self, cs: &CriticalSection) -> Result<(), NotInitialized>
    where
        T: Drop,
    {
        if self.0.borrow(cs).borrow().is_none() {
            return Err(NotInitialized);
        }
        let mut opt = self.0.borrow(cs).borrow_mut();
        drop(opt.take().unwrap());
        Ok(())
    }
}
