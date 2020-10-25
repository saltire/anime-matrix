use core::time::Duration;
// use std::thread::sleep;

use rusb::{DeviceHandle, GlobalContext};

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
const PANE1_ROW_INDICES: [[usize; 2]; 21] = [
  [8, 39], // 1 short on right side
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
  [616, 624],
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
    // Ok(size) => println!("{} written", size),
    Ok(_) => {},
    Err(e) => println!("Error: {}", e)
  }
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
      for indices in PANE1_ROW_INDICES.iter() {
        widths.push(indices[1] - indices[0] + 1);
      }
      for (r, indices) in PANE2_ROW_INDICES.iter().enumerate() {
        if r == 0 {
          let p1_last_width = widths.pop().unwrap();
          widths.push(p1_last_width + indices[1] - indices[0] + 1);
        }
        else {
          widths.push(indices[1] - indices[0] + 1);
        }
      }
      let mut pixels = vec![];
      for width in widths {
        let mut row = vec![0u8; width];
        row[0] = 255;
        row[width - 1] = 255;
        pixels.push(row);
      }

      let mut pane1 = [0u8; 640];
      for x in 0..7 {
        pane1[x] = PANE1_HEADER[x];
      }
      for (r, indices) in PANE1_ROW_INDICES.iter().enumerate() {
        for (i, x) in (indices[0]..(indices[1] + 1)).enumerate() {
          pane1[x] = pixels[r][i];
        }
      }

      let mut pane2 = [0u8; 640];
      for x in 0..7 {
        pane2[x] = PANE2_HEADER[x];
      }
      for (r, indices) in PANE2_ROW_INDICES.iter().enumerate() {
        let x_offset = if r == 0 { PANE2_X_OFFSET } else { 0 };
        for (i, x) in (indices[0]..(indices[1] + 1)).enumerate() {
          pane2[x] = pixels[r + PANE2_Y_OFFSET][i + x_offset];
        }
      }

      for buf in [pane1, pane2].iter() {
        send(buf, &device_handle, &timeout);
      }

      send(&FLUSH, &device_handle, &timeout);
    }
  }
}

fn main() {
  usb();
}
