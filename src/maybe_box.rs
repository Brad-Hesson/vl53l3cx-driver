pub use implementation::MaybeBox;

#[cfg(not(feature = "std"))]
mod implementation {
    use core::ops::{Deref, DerefMut};
    pub struct MaybeBox<T>(T);
    impl<T> MaybeBox<T> {
        pub fn new(t: T) -> Self {
            Self(t)
        }
        pub fn as_mut(&mut self) -> &mut T {
            &mut self.0
        }
    }
    impl<T> Deref for MaybeBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for MaybeBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

#[cfg(feature = "std")]
mod implementation {
    pub type MaybeBox<T> = Box<T>;
}
