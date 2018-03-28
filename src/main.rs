extern crate blurz;
extern crate ruuvitag;

use ruuvitag::ruuvitag::Tag;

use std::error::Error;
use std::result::Result;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;

fn discover_tags() -> Result<Tag, Box<Error>> {
    let adapter: Adapter = try!(Adapter::init());
    try!(adapter.set_powered(true));
    loop {
        let session = try!(DiscoverySession::create_session(adapter.get_id()));
        thread::sleep(Duration::from_millis(200));
        try!(session.start_discovery());
        thread::sleep(Duration::from_millis(800));
        let devices = try!(adapter.get_device_list());

        println!("{} device(s) found", devices.len());
        'device_loop: for d in devices {
            let device = Device::new(d.clone());
            let vendor_data = device.get_manufacturer_data().unwrap();
              if vendor_data.contains_key(&0x0499){
                    println!("vendor data {:?}", vendor_data);
                    let tag = Tag::new(vendor).unwrap();
                  //let mut tag = r::ruuvitag::Tag::new(vendor_data).unwrap();
                 println!("Temperature {:?}", tag.temperature);
                  println!("Humidity {:?}", tag.humidity);

 }

            try!(adapter.remove_device(device.get_id()));
        }
        try!(session.stop_discovery());
    }
}

fn main() {
    match discover_tags() {
        Ok(tag) => println!("Temperature: {:?}", tag),
        Err(e) => println!("Error {:?}", e),
    };
}
