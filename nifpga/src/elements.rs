use crate::datatype::Type;
use crate::error::{NifpgaError, ToResult};
use crate::fifo::{ReadFifo, WriteFifo};

use fehler::throws;

pub struct WriteElements<'a, 'b, T: Type>
where
    'a: 'b,
{
    pub slice: &'a mut [T],
    fifo: &'a WriteFifo<'b, T>,
}

impl<'a, T: Type> WriteElements<'a, '_, T> {
    pub fn from_raw(data: *mut T, len: usize, fifo: &'a WriteFifo<T>) -> Self {
        unsafe {
            WriteElements {
                slice: std::slice::from_raw_parts_mut(data, len),
                fifo,
            }
        }
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire<'b>(
        fifo: &'b WriteFifo<T>,
        num_elements: usize,
        timeout: u32,
    ) -> (WriteElements<'a, 'b, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) = T::acquire_fifo_write_elements(
            fifo.session.handle,
            fifo.handle,
            num_elements,
            timeout,
        )?;
        (
            WriteElements {
                slice: std::slice::from_raw_parts_mut(ptr, elements_acquired),
                fifo,
            },
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for WriteElements<'_, '_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::release_fifo_elements(
                self.fifo.session.handle,
                self.fifo.handle,
                self.slice.len(),
            )
            .to_result()
            .unwrap()
        }
    }
}

pub struct ReadElements<'a, 'b, T: Type>
where
    'a: 'b,
{
    pub slice: &'a [T],
    fifo: &'a ReadFifo<'b, T>,
}

impl<'a, T: Type> ReadElements<'a, '_, T> {
    pub fn from_raw(data: *const T, len: usize, fifo: &'a ReadFifo<T>) -> Self {
        unsafe {
            ReadElements {
                slice: std::slice::from_raw_parts(data, len),
                fifo,
            }
        }
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire<'b>(
        fifo: &'b ReadFifo<T>,
        num_elements: usize,
        timeout: u32,
    ) -> (ReadElements<'a, 'b, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) =
            T::acquire_fifo_read_elements(fifo.session.handle, fifo.handle, num_elements, timeout)?;
        (
            ReadElements {
                slice: std::slice::from_raw_parts(ptr, elements_acquired),
                fifo,
            },
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for ReadElements<'_, '_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::release_fifo_elements(
                self.fifo.session.handle,
                self.fifo.handle,
                self.slice.len(),
            )
            .to_result()
            .unwrap()
        }
    }
}
