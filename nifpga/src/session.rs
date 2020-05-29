extern crate libc;

use datatype::Type;
use fehler::throws;
use std::ffi::CString;
use std::sync::Once;

use crate::datatype;
use crate::error::{NifpgaError, ToResult};
use crate::fifo::{ReadFifo, WriteFifo};
use crate::irq::Context;

#[throws(NifpgaError)]
fn initialize() {
    extern "C" fn cleanup() {
        unsafe {
            nifpga_sys::finalize().to_result().unwrap();
        }
    }

    static mut STATUS: i32 = 0;
    static INIT: Once = Once::new();
    unsafe {
        INIT.call_once(|| {
            STATUS = nifpga_sys::initialize();
            assert_eq!(libc::atexit(cleanup), 0);
        });
        STATUS
    }
    .to_result()?
}

pub struct Session {
    pub(crate) handle: u32,
    reset_on_close: bool,
}

impl Session {
    #[throws(NifpgaError)]
    pub fn open(
        bitfile: &str,
        signature: &str,
        resource: &str,
        run: bool,
        reset_on_close: bool,
    ) -> Self {
        initialize()?;
        let mut handle = 0;
        unsafe {
            nifpga_sys::open(
                CString::new(bitfile).unwrap().as_ptr(),
                CString::new(signature).unwrap().as_ptr(),
                CString::new(resource).unwrap().as_ptr(),
                if run { 0 } else { 1 },
                &mut handle,
            )
            .to_result()?
        }
        Session {
            handle,
            reset_on_close,
        }
    }

    #[throws(NifpgaError)]
    pub fn read<T: Type>(&self, indicator: u32) -> T {
        T::read(self.handle, indicator)?
    }

    #[throws(NifpgaError)]
    pub fn write<T: Type>(&self, control: u32, value: T) {
        T::write(self.handle, control, value)?
    }

    #[throws(NifpgaError)]
    pub fn read_array<T: Type>(&self, indicator: u32, slice: &mut [T]) {
        T::read_array(self.handle, indicator, slice)?
    }

    #[throws(NifpgaError)]
    pub fn write_array<T: Type>(&self, control: u32, slice: &[T]) {
        T::write_array(self.handle, control, slice)?
    }

    #[throws(NifpgaError)]
    pub fn open_read_fifo<'a, T: Type>(
        &'a self,
        fifo: u32,
        depth: usize,
    ) -> (ReadFifo<'a, T>, usize) {
        ReadFifo::<'a, T>::open(&self, fifo, depth)?
    }

    #[throws(NifpgaError)]
    pub fn open_write_fifo<'a, T: Type>(
        &'a self,
        fifo: u32,
        depth: usize,
    ) -> (WriteFifo<'a, T>, usize) {
        WriteFifo::<'a, T>::open(&self, fifo, depth)?
    }

    #[throws(NifpgaError)]
    pub fn run(&self) {
        unsafe { nifpga_sys::run(self.handle).to_result()? }
    }

    #[throws(NifpgaError)]
    pub fn abort(&self) {
        unsafe { nifpga_sys::abort(self.handle).to_result()? }
    }

    #[throws(NifpgaError)]
    pub fn reset(&self) {
        unsafe { nifpga_sys::reset(self.handle).to_result()? }
    }

    #[throws(NifpgaError)]
    pub fn download(&self) {
        unsafe { nifpga_sys::download(self.handle).to_result()? }
    }

    #[throws(NifpgaError)]
    pub fn acknowledge_irqs(&self, irqs: u32) {
        unsafe { nifpga_sys::acknowledge_irqs(self.handle, irqs).to_result()? }
    }

    #[throws(NifpgaError)]
    pub fn reserve_irq_context(&self) -> Context {
        Context::reserve(&self)?
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::close(self.handle, if self.reset_on_close { 0 } else { 1 })
                .to_result()
                .unwrap()
        };
    }
}
