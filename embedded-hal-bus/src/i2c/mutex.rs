use embedded_hal::i2c::{ErrorType, I2c};
use std::sync::Mutex;

/// `std` `Mutex`-based shared bus [`I2c`] implementation.
///
/// Sharing is implemented with an `std` [`Mutex`](std::sync::Mutex). It allows a single bus across multiple threads,
/// with finer-grained locking than [`CriticalSectionDevice`](super::CriticalSectionDevice). The downside is that
/// it is only available in `std` targets.
pub struct MutexDevice<'a, T> {
    bus: &'a Mutex<T>,
}

impl<'a, T> MutexDevice<'a, T> {
    /// Create a new `MutexDevice`
    pub fn new(bus: &'a Mutex<T>) -> Self {
        Self { bus }
    }
}

impl<'a, T> ErrorType for MutexDevice<'a, T>
where
    T: I2c,
{
    type Error = T::Error;
}

impl<'a, T> I2c for MutexDevice<'a, T>
where
    T: I2c,
{
    fn read(&mut self, address: u8, read: &mut [u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.read(address, read)
    }

    fn write(&mut self, address: u8, write: &[u8]) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.write(address, write)
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.write_read(address, write, read)
    }

    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let bus = &mut *self.bus.lock().unwrap();
        bus.transaction(address, operations)
    }
}
