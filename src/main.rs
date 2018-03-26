

// extern crate blurz;


use std::result::Result;
use std::time::Duration;
use std::thread;
use std::collections::HashMap;

// use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
// use blurz::bluetooth_device::BluetoothDevice as Device;
// use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
    
    #[derive(Debug, PartialEq, Eq)]
    pub enum TagError {
        UnknownManufacturerId
    }

#[derive(Debug)]
struct Tag {
    id: u16,
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
       
       let tag = Tag  {
        id: 0,
        humidity: 0.0,
        temperature: -100.00,
        pressure: 50000,
        acceleration: Acceleration{
            x: 0,
            y: 0,
            z: 0
        },
        battery_voltage: 3300
       };
       Ok(tag)
   }
}

// fn discover_tags() -> Result<Tag, Err>{
    
//     get_tag_data()
    
// }


#[test]
fn parse_packet() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(1177, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(packet.len(), 1);
    let tag_data = Tag::new(packet).unwrap();
    assert_eq!(tag_data.id, 3);
    // assert_eq!(tag_data.humidity, 86 as f64);
    // assert_eq!(tag_data.temperature, 5.31 as f64);
    // assert_eq!(tag_data.pressure, 99159);
    // assert_eq!(tag_data.battery_voltage, 2911);
        
} 

#[test]
fn invalid_manufacturer_id() {
    let mut packet: HashMap<u16, Vec<u8>> = HashMap::new();
    packet.insert(0x123, vec![3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]);
    assert_eq!(Tag::new(packet).is_err(), true);
    //TODO:
    //assert_eq!(Tag::new(packet), TagError::UnknownManufacturerId);
}