extern crate nifpga_sys;

use fehler::throws;
use libc::c_void;

use crate::error::{NifpgaError, ToResult};
use crate::session::Session;

pub struct Context<'a> {
    context: *const c_void,
    session: &'a Session,
}

impl Context<'_> {
    #[throws(NifpgaError)]
    pub fn wait_on_irqs(&self, irqs: u32, timeout: u32) -> (u32, bool) {
        let mut irqs_asserted: u32 = Default::default();
        let mut timed_out: u8 = Default::default();
        unsafe {
            nifpga_sys::wait_on_irqs(
                self.session.handle,
                self.context,
                irqs,
                timeout,
                &mut irqs_asserted,
                &mut timed_out,
            )
            .to_result()?
        }
        (irqs_asserted, timed_out > 0)
    }

    #[throws(NifpgaError)]
    pub fn reserve(session: &Session) -> Context {
        let mut context = std::ptr::null();
        unsafe { nifpga_sys::reserve_irq_context(session.handle, &mut context).to_result()? }
        Context { context, session }
    }
}

impl Drop for Context<'_> {
    fn drop(&mut self) {
        unsafe {
            nifpga_sys::reserve_irq_context(self.session.handle, &mut self.context)
                .to_result()
                .unwrap()
        }
    }
}
