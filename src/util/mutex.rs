use core::cell::UnsafeCell;

pub trait Mutex {
    type Data;
    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R;
}

/*
    [Single Thead Mutex]
    Mutex that only supports synchronous actions
    See https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/06_drivers_gpio_uart/src/synchronization.rs
*/
pub struct SyncMutex<T: ?Sized> {
    data: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Sync for SyncMutex<T> {}

impl<T> SyncMutex<T> {
    pub const fn new(data: T) -> Self {
        SyncMutex {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> Mutex for &SyncMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }
}
