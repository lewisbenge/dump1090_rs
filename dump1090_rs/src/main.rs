mod sdrconfig;
use std::alloc::alloc_zeroed;
use std::{thread, alloc::Layout};
use std::net::{IpAddr, TcpListener};
use adsb_deku::Frame;
use adsb_deku::deku::DekuContainerRead;
use libdump1090_rs::utils;
use num_complex::{Complex, ComplexFloat};
use rtlsdr_rs::{error::Result, RtlSdr,DEFAULT_BUF_LENGTH};

// main will exit as 0 for success, 1 on error
fn main() -> Result<()> {
   
    let mut device = RtlSdr::open(0)?;
    let gains = device.get_tuner_gains()?;
    println!("Max available gain is: {}", gains.last().unwrap_or(&0));

    let gain = *gains.last().unwrap_or(&0);
    device.set_tuner_gain(rtlsdr_rs::TunerGain::Manual(gain));
    println!("Setting gain to: {}", gain);
    device.set_center_freq(1_090_000_000);
    device.set_sample_rate(2_400_000);
    device.reset_buffer();

    loop {
        let mut buf: Box<[u8; DEFAULT_BUF_LENGTH]> = alloc_buf();
        // try and read from sdr device
        match device.read_sync(&mut *buf) {
            Ok(len) => {
                //utils::save_test_data(&buf[..len]);
                // demodulate new data
                let buf = &buf[..len];              
                let outbuf = utils::to_mag(buf);
                let resulting_data = libdump1090_rs::demod_2400::demodulate2400(&outbuf).unwrap();

                // send new data to connected clients
                if !resulting_data.is_empty() {
                 
                    for data in &resulting_data {
                        let hex = hex::encode(data);  
                        println!("{}", &hex[..hex.len() - 1]);
                        let bytes = hex::decode(&hex).unwrap();
                        // decode
                        match Frame::from_bytes((&bytes, 0)) {
                            Ok((_, frame)) => {        
                                println!("{frame}");                                
                            },
                            Err(e) => {
                               println!("Error: {}", e);
                            },
                        }
                    }
                    

            
                }
            } Err(err) => {
                // handle error
                println!("Error: {}", err)
            }
        }  
        
    }
}

 /// Allocate a buffer on the heap
 fn alloc_buf<T>() -> Box<T> {
    let layout: Layout = Layout::new::<T>();
    // TODO move to using safe code once we can allocate an array directly on the heap.
    unsafe {
        let ptr = alloc_zeroed(layout) as *mut T;
        Box::from_raw(ptr)
    }
}
