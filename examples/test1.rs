extern crate owa5x_sys;
use owa5x_sys as owa;
use std::os::raw::{c_int, c_uchar};

pub fn main() {
    println!("Starting RTU");
    unsafe {
        let rtu = owa5x_sys::RTUControl_Initialize(std::ptr::null_mut()) as u32;
        println!("rtu: {}", rtu);
        let rtu_start = owa::RTUControl_Start();
        println!("rtu start: {}", rtu_start);

        let io_init = owa::IO_Initialize();
        let io_start = owa::IO_Start();

        loop {
            let mut result: c_int = 0;
            owa::ANAGIO_GetAnalogIn(0, &mut result);
            println!("analog in 0: {}", result);
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    }
}
