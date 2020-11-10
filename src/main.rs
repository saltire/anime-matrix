#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;


use core::time::Duration;
// use std::thread::sleep;

use rusb::{DeviceHandle, GlobalContext};

mod server;

const VENDOR_ID: u16 = 0x0b05;
const PRODUCT_ID: u16 = 0x193b;

const REQUEST_TYPE: u8 = 0x21;
const REQUEST: u8 = 0x9;
const VALUE: u16 = 0x35e;

// "ASUS Tech.Inc."
const INIT_HEADER: [u8; 15] = [0x5e, 0x41, 0x53, 0x55, 0x53, 0x20, 0x54, 0x65, 0x63, 0x68, 0x2e, 0x49, 0x6e, 0x63, 0x2e];
// const INIT_01: [u8; 2] = [0x5e, 0xc2];
// const INIT_02: [u8; 2] = [0x5e, 0xc0];

// turn on
const INIT1: [u8; 4] = [0x5e, 0xc3, 0x01, 0x00];
const INIT2: [u8; 4] = [0x5e, 0xc4, 0x01, 0x80];
const INIT3: [u8; 4] = [0x5e, 0xc0, 0x04, 0x03];
// turn off
// const INIT1: [u8; 4] = [0x5e, 0xc3, 0x01, 0x80];
// const INIT2: [u8; 4] = [0x5e, 0xc4, 0x01, 0x80];
// const INIT3: [u8; 4] = [0x5e, 0xc0, 0x04, 0x00];

const PANE1_HEADER: [u8; 7] = [0x5e, 0xc0, 0x02, 0x01, 0x00, 0x73, 0x02];
const PANE2_HEADER: [u8; 7] = [0x5e, 0xc0, 0x02, 0x74, 0x02, 0x73, 0x02];

const FLUSH: [u8; 3] = [0x5e, 0xc0, 0x03];

// 55 rows, 1215 total
const PANE1_ROWS: [(usize, usize); 21] = [
  (8, 32), // 1 short on right side
  (41, 33),
  (76, 33),
  (109, 33),
  (144, 33),
  (177, 33),
  (211, 33),
  (244, 32),
  (277, 32),
  (309, 31),
  (341, 31),
  (372, 30),
  (403, 30),
  (433, 29),
  (463, 29),
  (492, 28),
  (521, 28),
  (549, 27),
  (577, 27),
  (604, 26),
  (631, 3), // first 3 pixels of row
];
const PANE2_ROWS: [(usize, usize); 35] = [
  (7, 23), // starts at 4th pixel of last row of pane 1
  (30, 25),
  (56, 25),
  (81, 24),
  (106, 24),
  (130, 23),
  (154, 23),
  (177, 22),
  (200, 22),
  (222, 21),
  (244, 21),
  (265, 20),
  (286, 20),
  (306, 19),
  (326, 19),
  (345, 18),
  (364, 18),
  (382, 17),
  (400, 17),
  (417, 16),
  (434, 16),
  (450, 15),
  (466, 15),
  (481, 14),
  (496, 14),
  (510, 13),
  (524, 13),
  (537, 12),
  (550, 12),
  (562, 11),
  (574, 11),
  (585, 10),
  (596, 10),
  (606, 9),
  (616, 9),
];
const PANE2_Y_OFFSET: usize = 20;
const PANE2_X_OFFSET: usize = 3;

fn send(buf: &[u8], device_handle: &DeviceHandle<GlobalContext>, timeout: &Duration) {
  let mut buffer = [0u8; 640];
  for x in 0..buf.len() {
    buffer[x] = buf[x];
  }
  // println!("{:02X?}", buffer);

  match device_handle.write_control(REQUEST_TYPE, REQUEST, VALUE, 0, &buffer, *timeout) {
    Ok(_) => {},
    Err(e) => println!("Error: {}", e)
  }
}

fn send_pixels(pixels: Vec<Vec<u8>>, device_handle: &DeviceHandle<GlobalContext>, timeout: &Duration) {
  let mut pane1 = [0u8; 640];
  for x in 0..7 {
    pane1[x] = PANE1_HEADER[x];
  }
  for (r, (index, width)) in PANE1_ROWS.iter().enumerate() {
    for x in 0..*width {
      pane1[index + x] = pixels[r][x];
    }
  }

  let mut pane2 = [0u8; 640];
  for x in 0..7 {
    pane2[x] = PANE2_HEADER[x];
  }
  for (r, (index, width)) in PANE2_ROWS.iter().enumerate() {
    let x_offset = if r == 0 { PANE2_X_OFFSET } else { 0 };
    for x in 0..*width {
      pane2[index + x] = pixels[r + PANE2_Y_OFFSET][x + x_offset];
    }
  }

  send(&pane1, &device_handle, &timeout);
  send(&pane2, &device_handle, &timeout);
  send(&FLUSH, &device_handle, &timeout);
}

fn usb() {
  // let tick = Duration::new(0, 500 * 1000000);
  let timeout = Duration::new(5, 0);

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();

    if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
      let mut device_handle = device.open().unwrap(); // sends GET_DESCRIPTOR_FROM_DEVICE x2

      // This device has 1 config, 1 interface, 1 endpoint.
      let config = device.active_config_descriptor().unwrap();
      let interface = config.interfaces().next().unwrap();

      // device_handle.reset().unwrap();

      device_handle.claim_interface(interface.number()).unwrap();

      send(&INIT_HEADER, &device_handle, &timeout);
      // send(&INIT_01, &device_handle, &timeout);
      // send(&INIT_02, &device_handle, &timeout);

      for buf in [INIT1, INIT2, INIT3].iter() {
        send(buf, &device_handle, &timeout);
      }

      let mut widths = vec![];
      for (_index, width) in PANE1_ROWS.iter() {
        widths.push(*width);
      }
      for (r, (_index, width)) in PANE2_ROWS.iter().enumerate() {
        let last_width = if r == 0 { widths.pop().unwrap() } else { 0 };
        widths.push(last_width + width);
      }
      let mut pixels = vec![];
      for width in widths {
        let mut row = vec![0u8; width];
        row[0] = 255;
        row[width - 1] = 255;
        pixels.push(row);
      }

      send_pixels(pixels, &device_handle, &timeout);
    }
  }
}

fn main() {
  usb();

  server::start();
}
