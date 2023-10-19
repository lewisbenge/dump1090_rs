// crate
// third-party
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_complex::Complex;

use crate::MagnitudeBuffer;

pub fn save_test_data(data: &[Complex<i16>]) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let name = format!("test_{now}.iq");
    let mut file = std::fs::File::create(name).unwrap();

    for d in data {
        file.write_i16::<LittleEndian>(d.im).unwrap();
        file.write_i16::<LittleEndian>(d.re).unwrap();
    }
}

#[must_use]
pub fn read_test_data(filepath: &str) -> [Complex<i16>; 0x20000] {
    let mut file = std::fs::File::open(filepath).unwrap();
    let mut r_buf = [Complex::new(0, 0); 0x20000];

    let mut i = 0;
    loop {
        let im = file.read_i16::<LittleEndian>().unwrap();
        let re = file.read_i16::<LittleEndian>().unwrap();
        r_buf[i] = Complex::new(re, im);

        i += 1;
        if i == 0x20000 {
            break;
        }
    }

    r_buf
}

#[must_use]
pub fn to_mag(data: &[u8]) -> MagnitudeBuffer {
    
    //populate lookup table
    let mut maglut = vec![0u16; 129 * 129];
    for i in 0..=128 {
        for q in 0..=128 {
            let mag = ((i * i + q * q) as f32).sqrt() * 360.0;
            maglut[i * 129 + q] = mag.round() as u16;
        }
    }
    
    let mut outbuf = MagnitudeBuffer::default();
    for (j, chunk) in data.chunks_exact(2).enumerate() {
        let i = (chunk[0] as i32) - 127;
        let q = (chunk[1] as i32) - 127;

        let i = if i < 0 { -i } else { i } as usize;
        let q = if q < 0 { -q } else { q } as usize;


        outbuf.push(maglut[i * 129 + q]);
    }
    outbuf
}
/* *
#[must_use]
pub fn to_mag(data: &[Complex<i16>]) -> MagnitudeBuffer {
    let mut outbuf = MagnitudeBuffer::default();
    for b in data {
        // TODO: lookup table
        let i = b.im;
        let q = b.re;

        let fi = f32::from(i) / (1 << 15) as f32;
        let fq = f32::from(q) / (1 << 15) as f32;

        let mag_sqr = fi.mul_add(fi, fq * fq);
        let mag = f32::sqrt(mag_sqr);
        outbuf.push(mag.mul_add(f32::from(u16::MAX), 0.5) as u16);
    }
    outbuf
}
*/