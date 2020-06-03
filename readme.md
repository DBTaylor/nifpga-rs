nifpga-rs
===

Interface to NI FPGAs via the C API

Usage
---

Any project using this library must also link NiFpga.c

The API this library provides uses control addresses and FIFO numbers. To create an API specific to your FPGA, use [nifpga-apigen](https://github.com/dbtaylor/nifpga-apigen).

An example project can be found here: https://github.com/dbtaylor/rio-rs-ex

```rust
extern crate nifpga;

use nifpga::{NifpgaError, Session};

fn main() -> Result<(), NifpgaError>{
    //open the session
    //it will be closed when it goes out of scope
    let session = Session::open(
        "/home/admin/fpga.lvbitx",
        "0DA1668CDC2C6C492F1437AE6DC2151E",//signature from generated header
        "RIO0",
        true, //run on open
        true //reset on close
    )?;

    //reserve an IRQ context and wait on IRQ 0
    //the context will be unreserved when it goes out of scope
    session.reserve_irq_context()?.wait_on_irqs(1, 1000)?;
    
    //acknowledge IRQ 0
    session.acknowledge_irqs(1)?;

    //open a target-to-host FIFO
    //this configures the FIFO and starts it
    //it will be stopped when it goes out of scope
    let (reader, _) = session.open_read_fifo::<f32>(0, 100)?;
    
    //open a host-to-target FIFO
    let (writer, _) = session.open_write_fifo::<f32>(1, 100)?;
    
    //write 2 values to the FIFO
    writer.write(&[5.0, 5.0], 1000)?;

    unsafe{
        //acquire 4 elements in the host write buffer
        //these elements will not be sent to the target until they go out of scope
        //this method is unsafe because elements must be dropped in the order they are acquired
        let( elements, _, _) = writer.acquire_elements(4, 1000)?;
        elements.slice.iter_mut().for_each(|el| {*el = 1.0});
    }
    
    //read 3 elements from the FIFO
    let mut read_buff = [0.0; 3];
    reader.read(&mut read_buff, 1000)?;
    println!("{:?}", read_buff);
    unsafe{
        //acquire 3 elements in the host read buffer
        //this section of the buffer will not be available for the FIFO until these elements are dropped
        //this method is unsafe because elements must be dropped in the order they are acquired
        let(elements, _, _) = reader.acquire_elements(3, 1000)?;
        println!("{:?}", elements.slice)
    }
    
    //write to control x18004
    session.write::<f32>(0x18004, 5.0)?;
    //read from indicator x1800C
    println!("{:?}", session.read::<f32>(0x1800C)?);

    //write contents of array to control x18000
    let array = [5.0, 5.0];
    session.write_array::<f32>(0x18000, &array)?;

    //read indicator 0x18008 into array
    let mut array = [0.0; 2];
    session.read_array::<f32>(0x18008, &mut array)?;
    println!("{:?}", array);

    Ok(())
}
```