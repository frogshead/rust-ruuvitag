

extern crate blurz;

use std::error::Error;
use std::result::Result;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
    
    #[derive(Debug, PartialEq, Eq)]
    pub enum TagError {
        UnknownManufacturerId
    }

#[derive(Debug)]
struct Tag {
    id: u8,
    humidity: f64, 
    temperature: f64,
    pressure: u32,
    acceleration: Acceleration,
    battery_voltage: u16
}

#[derive(Debug)]
struct Acceleration {
    x: i16,
    y: i16,
    z: i16
}

impl Tag {
   pub fn new(data: HashMap<u16, Vec<u8>>) -> Result<Tag, TagError> {
       if !data.contains_key(&0x0499){
           return Err(TagError::UnknownManufacturerId);
       }
       
       let values = data.get(&0x0499).unwrap();
       let tag = Tag  {
        id: values[0],
        humidity: (values[1] / 2) as f64,
        temperature: (values[2] as f64) + ((values[3] as f64 * 0.01)),
        pressure: (((values[4] as u32) << 8) | values[5] as u32)  + 50000,
        acceleration: Acceleration{
            x: (((values[6] as i16) << 8) | values[7] as i16),
            y: (((values[8] as i16) << 8) | values[9] as i16),
            z: (((values[10] as i16) << 8) | values[11] as i16)
        },
        battery_voltage: (((values[12] as u16) << 8) | values[13] as u16)
       };
       Ok(tag)
   }
}

fn discover_tags() -> Result<Tag, Box<Error>>{
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
            let vendor_data = device.get_manufacturer_data();
            if !vendor_data.contains_key(&0x0499){
                let mut tag = Tag::new(vendor_data);
            }
            try!(adapter.remove_device(device.get_id()));
        }
        try!(session.stop_discovery());
        Ok(tag)
    }
    
}

fn fn main() {
    match discover_tags() {
        Ok(tag) => println!("Temperature: {:?}", tag.temperature),
        Err(e) => println!("Error {:?}", e);,
    }
}


#[test]
fn parse_packet() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(1177, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(packet.len(), 1);
    let tag_data = Tag::new(packet).unwrap();
    assert_eq!(tag_data.id, 3);
    assert_eq!(tag_data.humidity, 86 as f64);
    assert_eq!(tag_data.temperature, 5.31 as f64);
    assert_eq!(tag_data.pressure, 99159);
    assert_eq!(tag_data.acceleration.x, 0x2d7);
    assert_eq!(tag_data.acceleration.y, 0x2df);
    assert_eq!(tag_data.acceleration.z, 0xfff7);
    assert_eq!(tag_data.battery_voltage, 2911);
        
} 

#[test]
fn invalid_manufacturer_id() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(0x123, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(Tag::new(packet).is_err(), true);
    //TODO:
    //assert_eq!(Tag::new(packet), TagError::UnknownManufacturerId);
}