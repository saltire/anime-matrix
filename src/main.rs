use core::time::Duration;
use std::thread::sleep;

use rusb::{DeviceHandle, GlobalContext};

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

// 55 rows, 1215 total
const ROW_WIDTHS: [usize; 55] = [33,33,33,33,33,33,33,32,32,31,31,30,30,29,29,28,28,27,27,26,26,25,25,24,24,23,23,22,22,21,21,20,20,19,19,18,18,17,17,16,16,15,15,14,14,13,13,12,12,11,11,10,10,9,9];
const PANE1_ROW_INDICES: [[usize; 2]; 21] = [
  [7, 39], // 1 short on right side
  [41, 73],
  [76, 108],
  [109, 141],
  [144, 176],
  [177, 209],
  [211, 243],
  [244, 275],
  [277, 308],
  [309, 339],
  [341, 371],
  [372, 401],
  [403, 432],
  [433, 461],
  [463, 491],
  [492, 519],
  [521, 548],
  [549, 575],
  [577, 603],
  [604, 629],
  [631, 633], // first 3 pixels of row
];
const PANE2_ROW_INDICES: [[usize; 2]; 35] = [
  [7, 29], // starts at 4th pixel of last row of pane 1
  [30, 54],
  [56, 80],
  [81, 104],
  [106, 129],
  [130, 152],
  [154, 176],
  [177, 198],
  [200, 221],
  [222, 242],
  [244, 264],
  [265, 284],
  [286, 305],
  [306, 324],
  [326, 344],
  [345, 362],
  [364, 381],
  [382, 398],
  [400, 416],
  [417, 432],
  [434, 449],
  [450, 464],
  [466, 480],
  [481, 494],
  [496, 509],
  [510, 522],
  [524, 536],
  [537, 548],
  [550, 561],
  [562, 572],
  [574, 584],
  [585, 594],
  [596, 605],
  [606, 614],
  [616, 625],
];
fn send(buf: &[u8], device_handle: &DeviceHandle<GlobalContext>, timeout: &Duration) {
  match device_handle.write_control(REQUEST_TYPE, REQUEST, VALUE, 0, buf, *timeout) {
    // Ok(size) => println!("{} written", size),
    Ok(_) => {},
    Err(e) => println!("Error: {}", e)
  }
}

fn usb() {
  let tick = Duration::new(0, 500 * 1000000);
  let timeout = Duration::new(5, 0);

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();

    if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
      let mut device_handle = device.open().unwrap(); // sends GET_DESCRIPTOR_FROM_DEVICE x2

      // This device has 1 config, 1 interface, 1 endpoint.
      let config = device.active_config_descriptor().unwrap();
      let interface = config.interfaces().next().unwrap();

      device_handle.claim_interface(interface.number()).unwrap();

      // let mut inits = [[0u8; 640]; 3];
      // for i in 0..3 {
      //   for x in 0..INIT_HEADERS[i].len() {
      //     inits[i][x] = INIT_HEADERS[i][x];
      //   }
      // }

      let mut p = 0;
      loop {
        let mut pane1 = [0u8; 640];
        for x in 0..7 {
          pane1[x] = PANE_HEADERS[0][x];
        }
        for r in 0..PANE1_ROW_INDICES.len() {
          for x in PANE1_ROW_INDICES[r][0]..(PANE1_ROW_INDICES[r][1] + 1) {
            pane1[x] = if r % 3 == 0 { 255 } else { 32 };
          }
        }

        // second pane starts at row 19, col 3 (0-indexed)
        let mut pane2 = [0u8; 640];
        for x in 0..7 {
          pane2[x] = PANE_HEADERS[1][x];
        }
        for r in 0..PANE2_ROW_INDICES.len() {
          for x in PANE2_ROW_INDICES[r][0]..(PANE2_ROW_INDICES[r][1] + 1) {
            pane2[x] = if r % 3 == 1 { 255 } else { 32 };
          }
        }

        let mut flush = [0u8; 640];
        for x in 0..FLUSH_HEADER.len() {
          flush[x] = FLUSH_HEADER[x];
        }

        for buf in [pane1, pane2, flush].iter() {
          // println!("{:?}", buf);
          send(buf, &device_handle, &timeout);
        }

        p = (p + 1) % 640;
        sleep(tick);
      }
    }
  }
}

fn main() {
  usb();
}
