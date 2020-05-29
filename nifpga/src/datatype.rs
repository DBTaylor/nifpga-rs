use crate::error;

use error::{NifpgaError, ToResult};
use fehler::throws;
use nifpga_type_macro::datatype;

pub trait Type: Sized {
    fn read(session: u32, indicator: u32) -> Result<Self, NifpgaError>;

    fn write(session: u32, control: u32, value: Self) -> Result<(), NifpgaError>;

    fn read_array(session: u32, indicator: u32, slice: &mut [Self]) -> Result<(), NifpgaError>;

    fn write_array(session: u32, control: u32, slice: &[Self]) -> Result<(), NifpgaError>;

    fn write_fifo(
        session: u32,
        fifo: u32,
        data: &[Self],
        timeout: u32,
    ) -> Result<usize, NifpgaError>;

    fn read_fifo(
        session: u32,
        fifo: u32,
        data: &mut Vec<Self>,
        num_elements: usize,
        timeout: u32,
    ) -> Result<usize, NifpgaError>;

    unsafe fn acquire_fifo_read_elements<'a>(
        session: u32,
        fifo: u32,
        num_elements: usize,
        timeout: u32,
    ) -> Result<(*const Self, usize, usize), NifpgaError>;

    unsafe fn acquire_fifo_write_elements<'a>(
        session: u32,
        fifo: u32,
        num_elements: usize,
        timeout: u32,
    ) -> Result<(*mut Self, usize, usize), NifpgaError>;
}

#[datatype(i8 u8 i16 u16 i32 u32 i64 u64 f64 bool)]
impl Type for f32 {
    #[throws(NifpgaError)]
    fn read(session: u32, indicator: u32) -> Self {
        let mut value = Default::default();
        unsafe {
            nifpga_sys::read_f32(session, indicator, &mut value).to_result()?;
        };
        value
    }

    #[throws(NifpgaError)]
    fn write(session: u32, control: u32, value: Self) {
        unsafe {
            nifpga_sys::write_f32(session, control, value).to_result()?;
        }
    }

    #[throws(NifpgaError)]
    fn read_array(session: u32, indicator: u32, slice: &mut [Self]) {
        unsafe {
            nifpga_sys::read_array_f32(session, indicator, slice.as_mut_ptr(), slice.len())
                .to_result()?;
        }
    }

    #[throws(NifpgaError)]
    fn write_array(session: u32, indicator: u32, slice: &[Self]) {
        unsafe {
            nifpga_sys::write_array_f32(session, indicator, slice.as_ptr(), slice.len())
                .to_result()?;
        }
    }

    #[throws(NifpgaError)]
    fn write_fifo(session: u32, fifo: u32, data: &[Self], timeout: u32) -> usize {
        let mut empty_elements_remaining = 0;
        unsafe {
            nifpga_sys::write_fifo_f32(
                session,
                fifo,
                data.as_ptr(),
                data.len(),
                timeout,
                &mut empty_elements_remaining,
            )
            .to_result()?
        }
        empty_elements_remaining
    }

    #[throws(NifpgaError)]
    fn read_fifo(
        session: u32,
        fifo: u32,
        data: &mut Vec<Self>,
        num_elements: usize,
        timeout: u32,
    ) -> usize {
        data.reserve(num_elements);
        let mut elements_remaining = 0;
        let len = data.len();
        unsafe {
            data.set_len(len + num_elements);
            nifpga_sys::read_fifo_f32(
                session,
                fifo,
                data.as_mut_ptr().offset(len as isize),
                num_elements,
                timeout,
                &mut elements_remaining,
            )
            .to_result()?
        };
        elements_remaining
    }

    #[throws(NifpgaError)]
    unsafe fn acquire_fifo_read_elements(
        session: u32,
        fifo: u32,
        num_elements: usize,
        timeout: u32,
    ) -> (*const Self, usize, usize) {
        let mut elements_acquired = 0;
        let mut elements_remaining = 0;
        let mut ptr = std::ptr::null();
        nifpga_sys::acquire_fifo_read_elements_f32(
            session,
            fifo,
            &mut ptr,
            num_elements,
            timeout,
            &mut elements_acquired,
            &mut elements_remaining,
        )
        .to_result()?;
        (ptr, elements_acquired, elements_remaining)
    }

    #[throws(NifpgaError)]
    unsafe fn acquire_fifo_write_elements(
        session: u32,
        fifo: u32,
        num_elements: usize,
        timeout: u32,
    ) -> (*mut Self, usize, usize) {
        let mut elements_acquired = 0;
        let mut elements_remaining = 0;
        let mut ptr = std::ptr::null_mut();
        nifpga_sys::acquire_fifo_write_elements_f32(
            session,
            fifo,
            &mut ptr,
            num_elements,
            timeout,
            &mut elements_acquired,
            &mut elements_remaining,
        )
        .to_result()?;
        (ptr, elements_acquired, elements_remaining)
    }
}
