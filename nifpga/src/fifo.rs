use fehler::throws;
use std::marker::PhantomData;

use crate::datatype::Type;
use crate::elements::{ReadElements, WriteElements};
use crate::error::{NifpgaError, ToResult};
use crate::session::Session;

pub struct ReadFifo<'a, T: Type> {
    pub(crate) handle: u32,
    pub(crate) session: &'a Session,
    phantom: PhantomData<T>,
}

impl<'a, T: Type> ReadFifo<'a, T> {
    #[throws(NifpgaError)]
    pub fn open(session: &'a Session, fifo: u32, depth: usize) -> (Self, usize) {
        let mut actual_depth = 0;
        unsafe {
            nifpga_sys::configure_fifo2(session.handle, fifo, depth, &mut actual_depth)
                .to_result()?;
            nifpga_sys::start_fifo(session.handle, fifo).to_result()?;
        };
        (
            ReadFifo {
                handle: fifo,
                session,
                phantom: PhantomData,
            },
            actual_depth,
        )
    }

    #[throws(NifpgaError)]
    pub fn read(&self, data: &mut [T], timeout: u32) {
        T::read_fifo(
            self.session.handle,
            self.handle,
            data,
            timeout,
        )?
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire_elements<'b>(
        &'b self,
        num_elements: usize,
        timeout: u32,
    ) -> (ReadElements<'b, 'a, T>, usize, usize) {
        let (ptr, elements_acquired, elements_remaining) =
            T::acquire_fifo_read_elements(self.session.handle, self.handle, num_elements, timeout)?;
        (
            ReadElements::from_raw(ptr, elements_acquired, &self),
            elements_acquired,
            elements_remaining,
        )
    }
}

impl<T: Type> Drop for ReadFifo<'_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::stop_fifo(self.session.handle, self.handle)
                .to_result()
                .unwrap();
        };
    }
}

pub struct WriteFifo<'a, T: Type> {
    pub(crate) handle: u32,
    pub(crate) session: &'a Session,
    phantom: PhantomData<T>,
}

impl<'a, T: Type> WriteFifo<'a, T> {
    #[throws(NifpgaError)]
    pub fn open(session: &'a Session, fifo: u32, depth: usize) -> (Self, usize) {
        let mut actual_depth = 0;
        unsafe {
            nifpga_sys::configure_fifo2(session.handle, fifo, depth, &mut actual_depth)
                .to_result()?;
            nifpga_sys::start_fifo(session.handle, fifo).to_result()?;
        };
        (
            WriteFifo {
                handle: fifo,
                session,
                phantom: PhantomData,
            },
            actual_depth,
        )
    }

    #[throws(NifpgaError)]
    pub fn write(&self, data: &[T], timeout: u32) -> usize {
        T::write_fifo(self.session.handle, self.handle, data, timeout)?
    }

    #[throws(NifpgaError)]
    pub unsafe fn acquire_elements<'b>(
        &'b self,
        num_elements: usize,
        timeout: u32,
    ) -> (WriteElements<'b, 'a, T>, usize, usize) {
        WriteElements::acquire(&self, num_elements, timeout)?
    }
}

impl<T: Type> Drop for WriteFifo<'_, T> {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::stop_fifo(self.session.handle, self.handle)
                .to_result()
                .unwrap();
        };
    }
}
