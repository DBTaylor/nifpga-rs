extern crate libc;
use libc::{c_char, c_void, size_t};

#[cfg_attr(feature = "static", link(name = "NiFpga", kind = "static"))]
#[cfg_attr(not(feature = "static"), link(name = "NiFpga"))]
extern "C" {
    #[link_name = "NiFpga_Initialize"]
    pub fn initialize() -> i32;

    #[link_name = "NiFpga_Open"]
    pub fn open(
        bitfile: *const c_char,
        signature: *const c_char,
        resource: *const c_char,
        attribute: u32,
        session: *mut u32,
    ) -> i32;

    #[link_name = "NiFpga_Close"]
    pub fn close(session: u32, attribute: u32) -> i32;

    #[link_name = "NiFpga_Finalize"]
    pub fn finalize() -> i32;

    #[link_name = "NiFpga_Run"]
    pub fn run(session: u32) -> i32;

    #[link_name = "NiFpga_Abort"]
    pub fn abort(session: u32) -> i32;

    #[link_name = "NiFpga_Reset"]
    pub fn reset(session: u32) -> i32;

    #[link_name = "NiFpga_Download"]
    pub fn download(session: u32) -> i32;

    #[link_name = "NiFpga_ReadI8"]
    pub fn read_i8(session: u32, indicator: u32, value: *mut i8) -> i32;

    #[link_name = "NiFpga_WriteI8"]
    pub fn write_i8(session: u32, control: u32, value: i8) -> i32;

    #[link_name = "NiFpga_ReadArrayI8"]
    pub fn read_array_i8(session: u32, indicator: u32, array: *mut i8, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayI8"]
    pub fn write_array_i8(session: u32, indicator: u32, array: *const i8, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadU8"]
    pub fn read_u8(session: u32, indicator: u32, value: *mut u8) -> i32;

    #[link_name = "NiFpga_WriteU8"]
    pub fn write_u8(session: u32, control: u32, value: u8) -> i32;

    #[link_name = "NiFpga_ReadArrayU8"]
    pub fn read_array_u8(session: u32, indicator: u32, array: *mut u8, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayU8"]
    pub fn write_array_u8(session: u32, indicator: u32, array: *const u8, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadI16"]
    pub fn read_i16(session: u32, indicator: u32, value: *mut i16) -> i32;

    #[link_name = "NiFpga_WriteI16"]
    pub fn write_i16(session: u32, control: u32, value: i16) -> i32;

    #[link_name = "NiFpga_ReadArrayI16"]
    pub fn read_array_i16(session: u32, indicator: u32, array: *mut i16, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayI16"]
    pub fn write_array_i16(session: u32, indicator: u32, array: *const i16, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadU16"]
    pub fn read_u16(session: u32, indicator: u32, value: *mut u16) -> i32;

    #[link_name = "NiFpga_WriteU16"]
    pub fn write_u16(session: u32, control: u32, value: u16) -> i32;

    #[link_name = "NiFpga_ReadArrayU16"]
    pub fn read_array_u16(session: u32, indicator: u32, array: *mut u16, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayU16"]
    pub fn write_array_u16(session: u32, indicator: u32, array: *const u16, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadI32"]
    pub fn read_i32(session: u32, indicator: u32, value: *mut i32) -> i32;

    #[link_name = "NiFpga_WriteI32"]
    pub fn write_i32(session: u32, control: u32, value: i32) -> i32;

    #[link_name = "NiFpga_ReadArrayI32"]
    pub fn read_array_i32(session: u32, indicator: u32, array: *mut i32, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayI32"]
    pub fn write_array_i32(session: u32, indicator: u32, array: *const i32, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadU32"]
    pub fn read_u32(session: u32, indicator: u32, value: *mut u32) -> i32;

    #[link_name = "NiFpga_WriteU32"]
    pub fn write_u32(session: u32, control: u32, value: u32) -> i32;

    #[link_name = "NiFpga_ReadArrayU32"]
    pub fn read_array_u32(session: u32, indicator: u32, array: *mut u32, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayU32"]
    pub fn write_array_u32(session: u32, indicator: u32, array: *const u32, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadI64"]
    pub fn read_i64(session: u32, indicator: u32, value: *mut i64) -> i32;

    #[link_name = "NiFpga_WriteI64"]
    pub fn write_i64(session: u32, control: u32, value: i64) -> i32;

    #[link_name = "NiFpga_ReadArrayI64"]
    pub fn read_array_i64(session: u32, indicator: u32, array: *mut i64, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayI64"]
    pub fn write_array_i64(session: u32, indicator: u32, array: *const i64, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadU64"]
    pub fn read_u64(session: u32, indicator: u32, value: *mut u64) -> i32;

    #[link_name = "NiFpga_WriteU64"]
    pub fn write_u64(session: u32, control: u32, value: u64) -> i32;

    #[link_name = "NiFpga_ReadArrayU64"]
    pub fn read_array_u64(session: u32, indicator: u32, array: *mut u64, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayU64"]
    pub fn write_array_u64(session: u32, indicator: u32, array: *const u64, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadSgl"]
    pub fn read_f32(session: u32, indicator: u32, value: *mut f32) -> i32;

    #[link_name = "NiFpga_WriteSgl"]
    pub fn write_f32(session: u32, control: u32, value: f32) -> i32;

    #[link_name = "NiFpga_ReadArraySgl"]
    pub fn read_array_f32(session: u32, indicator: u32, array: *mut f32, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArraySgl"]
    pub fn write_array_f32(session: u32, indicator: u32, array: *const f32, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadDbl"]
    pub fn read_f64(session: u32, indicator: u32, value: *mut f64) -> i32;

    #[link_name = "NiFpga_WriteDbl"]
    pub fn write_f64(session: u32, control: u32, value: f64) -> i32;

    #[link_name = "NiFpga_ReadArrayDbl"]
    pub fn read_array_f64(session: u32, indicator: u32, array: *mut f64, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayDbl"]
    pub fn write_array_f64(session: u32, indicator: u32, array: *const f64, size: size_t) -> i32;

    #[link_name = "NiFpga_ReadBool"]
    pub fn read_bool(session: u32, indicator: u32, value: *mut bool) -> i32;

    #[link_name = "NiFpga_WriteBool"]
    pub fn write_bool(session: u32, control: u32, value: bool) -> i32;

    #[link_name = "NiFpga_ReadArrayBool"]
    pub fn read_array_bool(session: u32, indicator: u32, array: *mut bool, size: size_t) -> i32;

    #[link_name = "NiFpga_WriteArrayBool"]
    pub fn write_array_bool(session: u32, indicator: u32, array: *const bool, size: size_t) -> i32;

    #[link_name = "NiFpga_ReserveIrqContext"]
    pub fn reserve_irq_context(session: u32, context: *mut *const c_void) -> i32;

    #[link_name = "NiFpga_UnreserveIrqContext"]
    pub fn unreserve_irq_context(session: u32, context: *const c_void) -> i32;

    #[link_name = "NiFpga_WaitOnIrqs"]
    pub fn wait_on_irqs(
        session: u32,
        context: *const c_void,
        irqs: u32,
        timeout: u32,
        irqsAsserted: *mut u32,
        timedOut: *mut u8,
    ) -> i32;
    
    #[link_name = "NiFpga_AcknowledgeIrqs"]
    pub fn acknowledge_irqs(session: u32, irqs: u32) -> i32;

    #[link_name = "NiFpga_ConfigureFifo"]
    pub fn configure_fifo(session: u32, fifo: u32, depth: size_t) -> i32;

    #[link_name = "NiFpga_ConfigureFifo2"]
    pub fn configure_fifo2(
        session: u32,
        fifo: u32,
        requestedDepth: size_t,
        actualDepth: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_StartFifo"]
    pub fn start_fifo(session: u32, fifo: u32) -> i32;

    #[link_name = "NiFpga_StopFifo"]
    pub fn stop_fifo(session: u32, fifo: u32) -> i32;

    #[link_name = "NiFpga_ReadFifoI8"]
    pub fn read_fifo_i8(
        session: u32,
        fifo: u32,
        data: *mut i8,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoI8"]
    pub fn write_fifo_i8(
        session: u32,
        fifo: u32,
        data: *const i8,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsI8"]
    pub fn acquire_fifo_read_elements_i8(
        session: u32,
        fifo: u32,
        elements: *mut *const i8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsI8"]
    pub fn acquire_fifo_write_elements_i8(
        session: u32,
        fifo: u32,
        elements: *mut *mut i8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoU8"]
    pub fn read_fifo_u8(
        session: u32,
        fifo: u32,
        data: *mut u8,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoU8"]
    pub fn write_fifo_u8(
        session: u32,
        fifo: u32,
        data: *const u8,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsU8"]
    pub fn acquire_fifo_read_elements_u8(
        session: u32,
        fifo: u32,
        elements: *mut *const u8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsU8"]
    pub fn acquire_fifo_write_elements_u8(
        session: u32,
        fifo: u32,
        elements: *mut *mut u8,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoI16"]
    pub fn read_fifo_i16(
        session: u32,
        fifo: u32,
        data: *mut i16,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoI16"]
    pub fn write_fifo_i16(
        session: u32,
        fifo: u32,
        data: *const i16,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsI16"]
    pub fn acquire_fifo_read_elements_i16(
        session: u32,
        fifo: u32,
        elements: *mut *const i16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsI16"]
    pub fn acquire_fifo_write_elements_i16(
        session: u32,
        fifo: u32,
        elements: *mut *mut i16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoU16"]
    pub fn read_fifo_u16(
        session: u32,
        fifo: u32,
        data: *mut u16,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoU16"]
    pub fn write_fifo_u16(
        session: u32,
        fifo: u32,
        data: *const u16,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsU16"]
    pub fn acquire_fifo_read_elements_u16(
        session: u32,
        fifo: u32,
        elements: *mut *const u16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsU16"]
    pub fn acquire_fifo_write_elements_u16(
        session: u32,
        fifo: u32,
        elements: *mut *mut u16,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoI32"]
    pub fn read_fifo_i32(
        session: u32,
        fifo: u32,
        data: *mut i32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoI32"]
    pub fn write_fifo_i32(
        session: u32,
        fifo: u32,
        data: *const i32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsI32"]
    pub fn acquire_fifo_read_elements_i32(
        session: u32,
        fifo: u32,
        elements: *mut *const i32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsI32"]
    pub fn acquire_fifo_write_elements_i32(
        session: u32,
        fifo: u32,
        elements: *mut *mut i32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoU32"]
    pub fn read_fifo_u32(
        session: u32,
        fifo: u32,
        data: *mut u32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoU32"]
    pub fn write_fifo_u32(
        session: u32,
        fifo: u32,
        data: *const u32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsU32"]
    pub fn acquire_fifo_read_elements_u32(
        session: u32,
        fifo: u32,
        elements: *mut *const u32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsU32"]
    pub fn acquire_fifo_write_elements_u32(
        session: u32,
        fifo: u32,
        elements: *mut *mut u32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoI64"]
    pub fn read_fifo_i64(
        session: u32,
        fifo: u32,
        data: *mut i64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoI64"]
    pub fn write_fifo_i64(
        session: u32,
        fifo: u32,
        data: *const i64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsI64"]
    pub fn acquire_fifo_read_elements_i64(
        session: u32,
        fifo: u32,
        elements: *mut *const i64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsI64"]
    pub fn acquire_fifo_write_elements_i64(
        session: u32,
        fifo: u32,
        elements: *mut *mut i64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoU64"]
    pub fn read_fifo_u64(
        session: u32,
        fifo: u32,
        data: *mut u64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoU64"]
    pub fn write_fifo_u64(
        session: u32,
        fifo: u32,
        data: *const u64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsU64"]
    pub fn acquire_fifo_read_elements_u64(
        session: u32,
        fifo: u32,
        elements: *mut *const u64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsU64"]
    pub fn acquire_fifo_write_elements_u64(
        session: u32,
        fifo: u32,
        elements: *mut *mut u64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoSgl"]
    pub fn read_fifo_f32(
        session: u32,
        fifo: u32,
        data: *mut f32,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoSgl"]
    pub fn write_fifo_f32(
        session: u32,
        fifo: u32,
        data: *const f32,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsSgl"]
    pub fn acquire_fifo_read_elements_f32(
        session: u32,
        fifo: u32,
        elements: *mut *const f32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsSgl"]
    pub fn acquire_fifo_write_elements_f32(
        session: u32,
        fifo: u32,
        elements: *mut *mut f32,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoDbl"]
    pub fn read_fifo_f64(
        session: u32,
        fifo: u32,
        data: *mut f64,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoDbl"]
    pub fn write_fifo_f64(
        session: u32,
        fifo: u32,
        data: *const f64,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsDbl"]
    pub fn acquire_fifo_read_elements_f64(
        session: u32,
        fifo: u32,
        elements: *mut *const f64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsDbl"]
    pub fn acquire_fifo_write_elements_f64(
        session: u32,
        fifo: u32,
        elements: *mut *mut f64,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReadFifoBool"]
    pub fn read_fifo_bool(
        session: u32,
        fifo: u32,
        data: *mut bool,
        numberOfElements: size_t,
        timeout: u32,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_WriteFifoBool"]
    pub fn write_fifo_bool(
        session: u32,
        fifo: u32,
        data: *const bool,
        numberOfElements: size_t,
        timeout: u32,
        emptyElementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoReadElementsBool"]
    pub fn acquire_fifo_read_elements_bool(
        session: u32,
        fifo: u32,
        elements: *mut *const bool,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_AcquireFifoWriteElementsBool"]
    pub fn acquire_fifo_write_elements_bool(
        session: u32,
        fifo: u32,
        elements: *mut *mut bool,
        elementsRequested: size_t,
        timeout: u32,
        elementsAcquired: *mut size_t,
        elementsRemaining: *mut size_t,
    ) -> i32;

    #[link_name = "NiFpga_ReleaseFifoElements"]
    pub fn release_fifo_elements(session: u32, fifo: u32, elements: size_t) -> i32;

    #[link_name = "NiFpga_GetPeerToPeerFifoEndpoint"]
    pub fn get_p2p_fifo_endpoint(session: u32, fifo: u32, endpoint: *mut u32) -> i32;

    #[link_name = "NiFpga_GetBitfileContents"]
    pub fn get_bitfile_contents(session: u32, contents: *mut *const c_char) -> i32;

    #[link_name = "NiFpga_ClientFunctionCall"]
    pub fn client_function_call(
        session: u32,
        group: u32,
        functionId: u32,
        inBuffer: *const c_void,
        inBufferSize: size_t,
        outBuffer: *mut c_void,
        outBufferSize: size_t,
    ) -> i32;
}
