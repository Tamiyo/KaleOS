use core::marker::PhantomData;
use core::ops::Deref;

pub struct MMIOWrapper<T> {
    base_addr: usize,
    phantom: PhantomData<T>,
}

impl<T> MMIOWrapper<T> {
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            phantom: PhantomData,
        }
    }

    fn ptr(&self) -> *const T {
        self.base_addr as *const _
    }
}

impl<T> Deref for MMIOWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}
