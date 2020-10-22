extern crate hidapi;

use hidapi::HidApi;

use core::time::Duration;
use std::thread::sleep;

// device guid 861d861a-44da-44e7-bfe2-2b6cf922715e
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

fn usb(c: &usize) {
  let timeout = Duration::new(5, 0);

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();

    // println!("Bus {:03} Device {:03} Port {:03} ID {:04x}:{:04x}",
    //   device.bus_number(),
    //   device.address(),
    //   device.port_number(),
    //   device_desc.vendor_id(),
    //   device_desc.product_id());

    if device_desc.vendor_id() == VENDOR_ID && device_desc.product_id() == PRODUCT_ID {
      let mut device_handle = device.open().unwrap(); // sends GET_DESCRIPTOR_FROM_DEVICE x2

      // device_handle.reset().unwrap();

      // This device has 1 config, 1 interface, 1 endpoint.
      let config = device.active_config_descriptor().unwrap();
      let interface = config.interfaces().next().unwrap();
      // let interface_desc = interface.descriptors().next().unwrap();
      // let endpoint = interface_desc.endpoint_descriptors().next().unwrap();

      device_handle.claim_interface(interface.number()).unwrap();

      // device_handle.reset().unwrap();

      // let has_kernel = device_handle.kernel_driver_active(interface.number()).unwrap_or(false);
      // println!("Kernel? {}", has_kernel);

      // let mut header = [0u8; 640];
      // header[0] = 0x5e;
      // let header_bytes = "ASUS Tech.Inc.".as_bytes();
      // for x in 0..header_bytes.len() {
      //   header[x + 1] = header_bytes[x];
      // }

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
        // match device_handle.write_bulk(endpoint.number(), buf, timeout) {
          Ok(size) => println!("{} written", size),
          Err(e) => println!("Error: {}", e)
        }
      }
    }

    // match device.open() {
    //   Err(e) => {
    //     println!("  Device handle error: {}", e);
    //   }
    //   Ok(device_handle) => {
    //     let lang = device_handle.read_languages(timeout).unwrap()[0];
    //     let manu = device_handle.read_manufacturer_string_ascii(&device_desc).unwrap();
    //     let prod = device_handle.read_product_string_ascii(&device_desc).unwrap();

    //     match device_handle.read_serial_number_string_ascii(&device_desc) {
    //       Err(e) => {
    //         println!("  Serial error: {}", e);
    //       }
    //       Ok(serial) => {
    //         println!("  Serial: {}", serial);
    //       }
    //     }
    //     println!("  Product: {} - {}", manu, prod);

    //     for x in 0..device_desc.num_configurations() {
    //       match device.config_descriptor(x) {
    //         Err(e) => {
    //           println!("    Config Error {}", e);
    //         }
    //         Ok(config) => {
    //           println!("    Config {}", x);
    //           for interface in config.interfaces() {
    //             println!("      Interface {}", interface.number());
    //             for interface_desc in interface.descriptors() {
    //               let inter_str = device_handle.read_interface_string(lang, &interface_desc, timeout).unwrap_or("No string".to_string());
    //               println!("        Interface Descriptor {:x} - {:x} - {}",
    //                 interface_desc.class_code(),
    //                 interface_desc.sub_class_code(),
    //                 inter_str);

    //               for endpoint in interface_desc.endpoint_descriptors() {
    //                 println!("          Endpoint {}", endpoint.address());
    //               }
    //             }
    //           }
    //         }
    //       }
    //     }
    //   }
    // }

    // for x in 0..device_desc.num_configurations() {
    //   match device.config_descriptor(x) {
    //     Err(e) => {
    //       println!("    Config Error {}", e);
    //     }
    //     Ok(config) => {
    //       println!("    Config {}", x);
    //       for interface in config.interfaces() {
    //         println!("      Interface {}", interface.number());
    //         for interface_desc in interface.descriptors() {
    //           println!("        Interface Descriptor {:x} - {:x}", interface_desc.class_code(), interface_desc.sub_class_code());
    //           for endpoint in interface_desc.endpoint_descriptors() {
    //             println!("          Endpoint {}", endpoint.address());
    //           }
    //         }
    //       }
    //     }
    //   }
    // }

    // match device.active_config_descriptor() {
    //   Ok(config) => {
    //     for interface in config.interfaces() {
    //       for interface_desc in interface.descriptors() {
    //         println!("{}", interface_desc.interface_number());
    //         for endpoint in interface_desc.endpoint_descriptors() {
    //           println!("{}", endpoint.address());
    //         }
    //       }
    //     }
    //   }
    //   Err(e) => {
    //     println!("    Interface Error {}", e);
    //   }
    // }
  }
}

fn hid() {
  // let manager = hid::init().unwrap();
  // for device in manager.devices() {
  //   println!("{} {} {} {}",
  //     device.product_id(),
  //     device.product_string().unwrap(),
  //     device.manufacturer_string().unwrap(),
  //     device.serial_number().unwrap());
  // }

  match HidApi::new() {
    Ok(api) => {
      for device_info in api.device_list() {
        println!("{:04x}:{:04x} {} - {} ({})",
          device_info.vendor_id(),
          device_info.product_id(),
          device_info.manufacturer_string().unwrap_or("(manufacturer)"),
          device_info.product_string().unwrap_or("(product)"),
          device_info.interface_number());

        if device_info.vendor_id() == VENDOR_ID && device_info.product_id() == PRODUCT_ID {
          let device = device_info.open_device(&api).unwrap();
          let manu = device.get_manufacturer_string().unwrap().unwrap();
          let prod = device.get_product_string().unwrap().unwrap();
          println!("{} - {}", manu, prod);

          let mut report = [0u8; 641];
          report[0] = 0x5e;
          for x in 0..640 {
            if x % 5 == 1 {
              report[x + 1] = 255;
            }
          }

          match device.send_feature_report(&report) {
            Ok(()) => {},
            Err(e) => println!("{:?}", e),
          }
        }
      }
    },
    Err(e) => {
      eprintln!("Error: {}", e);
    },
  }
}

fn main() {
  let tick = Duration::new(1, 0);
  let mut c = 0;
  loop {
    usb(&c);
    // hid(&c);
    c += 1;
    sleep(tick);
  }
  // usb();
  // hid();
}
