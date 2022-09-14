// use std::io::{stdin, stdout, Read, Write};

use ch347_rs;

fn main() {
    ch347_rs::set_notify_callback(0, "VID_1A86&PID_55D\0", |status| {
        println!("[notify_callback] {:?}", status);
    });

    let a = ch347_rs::enum_device();
    // println!("enum_device len:{:?} :", a.len());
    for (k, v) in a.iter().enumerate() {
        println!(
            "#{} => {:?} {:?}\r\n\tdevice_id:{}\r\n\tdevice_path:{}\r\n\trpoduct_string:{}\r\n\tget_manufacturer_string:{}\r\n\tfunc_desc_str:{}",
            k,
            v.get_func_type(),
            v.get_usb_class(),
            v.get_device_id(),
            v.get_device_path(),
            v.get_rpoduct_string(),
            v.get_manufacturer_string(),
            v.get_func_desc_str(),
        )
        // println!("#{} => {} ", k, v)
    }

    let a = ch347_rs::enum_uart_device();
    // println!("enum_uart_device len:{:?} :", a.len());
    for (k, v) in a.iter().enumerate() {
        println!(
            "#{} => {:?} {:?}\r\n\tdevice_id:{}\r\n\tdevice_path:{}\r\n\trpoduct_string:{}\r\n\tget_manufacturer_string:{}\r\n\tfunc_desc_str:{}",
            k,
            v.get_func_type(),
            v.get_usb_class(),
            v.get_device_id(),
            v.get_device_path(),
            v.get_rpoduct_string(),
            v.get_manufacturer_string(),
            v.get_func_desc_str(),
        );
        // println!("#{} => {} ", k, v)
    }

    // let mut stdout = stdout();
    // stdout.write(b"Press Enter to continue...").unwrap();
    // stdout.flush().unwrap();
    // stdin().read(&mut [0]).unwrap();

    // println!("Exit");
}
