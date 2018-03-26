extern crate blurz;

use std::error::Error;
use std::time::Duration;
use std::thread;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;

#[derive(Debug)]
struct Tag {
    id: u16,
    humidity: f64, 
    temperature: f64,
    pressure: u16,
    acceleration: Acceleration,
    battery_voltage: u16;
}

#[derive(Debug)]
struct Acceleration {
    x: i16,
    y: i16;
    z: i16;
}

fn discover_tags -> Result(Tag, Error){
    
    get_tag_data()
    
}

fn get_tag_data(data: HashMap<u16, Vec<u8>>) -> Result<Tag, Err> {
    unimplemented!();
}