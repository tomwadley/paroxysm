# Vibrate experiment

Experimental modes for your Lovense vibrator

Modes:

 - `market` - control your vibrator according to inverse stock market momentum
 - `morse` - read, via morse code vibrations, the [Interior Scroll by Carolee Schneemann](http://emuseum.cornell.edu/view/objects/asitem/items$0040:43716)
 - `neo` - track asteroids approaching the Earth via pulses (frequency = distance, intensity = momentum) - data from [CNEOS](https://cneos.jpl.nasa.gov/ca/) 

## Requirements

- A [Lovense](https://www.lovense.com/) device that uses Bluetooth LE (e.g. the Lush but not the Max or Nora)  
- A working BlueZ stack with the `gatttool` command available on your path
- A (free) api-key from [Alpha Vantage](https://www.alphavantage.co/) *(required for `market` only)*
  
## Running

Copy `Config.toml.example` to `Config.toml` and configure each property.

    bd_addr = "A1:B2:C3:D4:E5:F6"
    alphavantage_api_key = "ABCD1234EFGH5678"
    
`bd_addr` is the Bluetooth address of your Lovense device. This is always required. 
You may omit `alphavantage_api_key` if you do not wish to run `market`. 

Then build and run (e.g. via cargo):

    $ cargo run -- morse
    
