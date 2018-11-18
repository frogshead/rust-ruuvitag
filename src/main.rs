extern crate blurz;
extern crate clap;
extern crate ruuvitag;

use ruuvitag::Tag;

use std::error::Error;
use std::result::Result;
use std::time::Duration;
use std::thread;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;

use clap::{Arg, App};

fn discover_tags() -> Result<Tag, Box<Error>> {
    let adapter: Adapter = try!(Adapter::init());
    try!(adapter.set_powered(true));
    loop {
        let session = try!(DiscoverySession::create_session(adapter.get_id()));
        thread::sleep(Duration::from_millis(200));
        try!(session.start_discovery());
        thread::sleep(Duration::from_millis(800));
        let devices = try!(adapter.get_device_list());

        
        'device_loop: for d in devices {
            let device = Device::new(d.clone());
            match device.get_manufacturer_data(){  
                Err(_e) => println!("No manufacturer data"),
                Ok(vendor_data) => {
                if vendor_data.contains_key(&0x0499) {
                let mut tag = Tag::new(vendor_data).unwrap();
                tag.mac = Some(device.get_address().unwrap());
                println!("Temperature {:?}", tag.temperature);
                println!("Humidity {:?}", tag.humidity);
                println!("Pressure {:?}", tag.pressure);
                println!("Battery Voltage {:?}", tag.battery_voltage);
                println!("Accelaration x:{:?} y:{:?} z:{:?}", tag.acceleration.x, tag.acceleration.y, tag.acceleration.z);
                println!("MAC Address: {:?}", tag.mac.unwrap());
                println!("===============================================");
                
            }
                }}

            try!(adapter.remove_device(device.get_id()));
        }
        try!(session.stop_discovery());
    }
}

fn main() {

    let _matches = App::new("App to read RuuviTag advertisements")
                            .version("0.1.0")
                            .author("Mikko Viitamäki")
                            .about("Reads RuuviTag advartisement packets. Main idea was to use RPi3 as a gateway and send results over the net.")
                            .arg(Arg::with_name("url")
                                .short("u")
                                .long("url")
                                .value_name("url")
                                .help("Send Tag data over network to given url")
                                .takes_value(true))
                            .get_matches();
    match discover_tags() {
        Ok(tag) => println!("Temperature: {:?}", tag),
        Err(e) => println!("Error {:?}", e),
    };
}
