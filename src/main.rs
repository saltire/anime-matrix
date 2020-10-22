use core::time::Duration;
use std::thread::sleep;

const VENDOR_ID: u16 = 0x0b05;
const PRODUCT_ID: u16 = 0x193b;

const REQUEST_TYPE: u8 = 0x21;
const REQUEST: u8 = 0x9;
const VALUE: u16 = 0x35e;

const INIT_HEADERS: [[u8; 4]; 3] = [
  [0x5e, 0xc3, 0x01, 0x00],
  [0x5e, 0xc4, 0x01, 0x80],
  [0x5e, 0xc0, 0x04, 0x03],
];
const PANE_HEADERS: [[u8; 7]; 2] = [
  [0x5e, 0xc0, 0x02, 0x01, 0x00, 0x73, 0x02],
  [0x5e, 0xc0, 0x02, 0x74, 0x02, 0x73, 0x02],
];
const FLUSH_HEADER: [u8; 3] = [0x5e, 0xc0, 0x03];

const WIDTHS: [u8; 55] = [33,33,33,33,33,33,33,32,32,31,31,30,30,29,29,28,28,27,27,26,26,25,25,24,24,23,23,22,22,21,21,20,20,19,19,18,18,17,17,16,16,15,15,14,14,13,13,12,12,11,11,10,10,9,9];

fn usb(c: &usize) {
  let mut t = 0;
  for x in 0..55 {
    t += WIDTHS[x];
  }
  println!("{}", t);

  let timeout = Duration::new(5, 0);

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();

    if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
      let mut device_handle = device.open().unwrap(); // sends GET_DESCRIPTOR_FROM_DEVICE x2

      // This device has 1 config, 1 interface, 1 endpoint.
      let config = device.active_config_descriptor().unwrap();
      let interface = config.interfaces().next().unwrap();

      device_handle.claim_interface(interface.number()).unwrap();

      let mut inits = [[0u8; 640]; 3];
      for i in 0..3 {
        for x in 0..INIT_HEADERS[i].len() {
          inits[i][x] = INIT_HEADERS[i][x];
        }
      }

      let mut pane1 = [0u8; 640];
      for x in 0..640 {
        if x < 7 {
          pane1[x] = PANE_HEADERS[0][x];
        }
        else if x % 3 == c % 3 {
          pane1[x] = 255;
        }
      }

      let mut pane2 = [0u8; 640];
      for x in 0..640 {
        if x < 7 {
          pane2[x] = PANE_HEADERS[1][x];
        }
        else if x % 3 == c % 3 {
          pane2[x] = 255;
        }
      }

      let mut flush = [0u8; 640];
      for x in 0..FLUSH_HEADER.len() {
        flush[x] = FLUSH_HEADER[x];
      }

      for buf in [pane1, pane2, flush].iter() {
        // println!("{:?}", buf);
        match device_handle.write_control(REQUEST_TYPE, REQUEST, VALUE, 0, buf, timeout) {
          Ok(size) => println!("{} written", size),
          Err(e) => println!("Error: {}", e)
        }
      }
    }
  }
}

fn main() {
  let tick = Duration::new(1, 0);
  let mut c = 0;
  loop {
    usb(&c);
    c += 1;
    sleep(tick);
  }
}
