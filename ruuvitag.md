# Ruuvitag majakka

Manufacrur data palauttaa ID:n ja byte arrayn varsinaisesta mainostetusta datasta (payload).
>dict ManufacturerData [readonly, optional]
>
>Manufacturer specific advertisement data. Keys are 16 bits Manufacturer ID followed >by its byte array value.
>
Alla esimerkki ruuvitagin datasta, ulkoanturi (25.3.2018 ~12:00):

```bash
Manufacturer Data: Ok({1177: [3, 172, 5, 31, 192, 7, 2, 215, 2, 223, 255, 247, 11, 95]})
```

Ja Ruuvitagin [pakettimääritykse](https://github.com/ruuvi/ruuvi-sensor-protocols#data-format-3-protocol-specification) avulla voimme auki kirjoittaa paketin:

1177 --> ID

|arvo   |   merkitys                                                            |
|-------|-----------------------------------------------------------------------|
| 3     | paketin speksin version                                               |
| 172   |kosteusprosentti 172 / 2 = 86%                                         |
| 5     |lämpötila, kokonaislukuosio                                            |
| 31    |lämpötila, desimaaliosio 31 / 100 = 0.31 eli 5.31 Celsius astetta      |
| 192   |paine MSB 0xC0                                                         |
| 7     |paine LSB 0x07 0xC007 = 49159 joten 50000Pa + 49159Pa = 99159Pa        |
| 2     |kiihtyvyys x-akseli MSB                                                |
| 215   |kiihtyvyys x-akseli LSB                                                |
| 2     |kiihtyvyys y-akseli MSB                                                |
| 223   |kiihtyvyy y-akseli LSB                                                 |
| 255   |kiihtyvyys z-akseli MSB                                                |
| 247   |kiihtyys z-akseli LSB                                                  |
|  11   |patterin jännite MSB 0x0B                                              |
|  95   |patterin jännite LSB 0x5F --> 0x0B5F -> 2911 eli 2.911V                |

Alla kolmosversion speksi githubista:
>## Data Format 3 Protocol Specification
>The data is decoded from "Manufacturer Specific Data" -field, for more details please check [this article](http://www.argenox.com/a-ble-advertising-primer/) out.
>Manufacturer ID is 0x0499.
>The actual data payload is:
>
>Offset | Allowed values | Description
>-----|:-----:|-----------
> 0 | 3 | Data format definition (3 = current sensor readings)
> 1 | `0 ... 200` | Humidity (one lsb is 0.5%, e.g. 128 is 64%)
> 2 | `-127 ... 127, signed` | Temperature (MSB is sign, next 7 bits are decimal value)
> 3 | `0 ... 99` | Temperature (fraction, 1/100.)
> 4 - 5| `0 ... 65535` | Pressure (Most Significant Byte first, value - 50kPa)
> 6-7 | `-32767 ... 32767, signed`  | Acceleration-X (Most Significant Byte first)
> 8 - 9 | `-32767 ... 32767, signed`  | Acceleration-Y (Most Significant Byte first)
> 10 - 11| `-32767 ... 32767, signed`  | Acceleration-Z (Most Significant Byte first)
> 12 - 13| `0 ... 65535` | Battery voltage (millivolts). MSB First
>

[Bluetooth Low Energy - Kuinka mainostetaan ympäristöön](http://www.argenox.com/a-ble-advertising-primer/)

[Rust bluetooth crate - blurz](https://crates.io/crates/blurz)