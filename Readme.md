# stock-vibrate

Adjusts the vibration of your Lovense Vibrator with the momentum of the stock market.

## Requirements

- A [Lovense](https://www.lovense.com/) device that uses Bluetooth LE (e.g. the Lush but not the Max or Nora)  
- An api-key for [Alpha Vantage](https://www.alphavantage.co/) (it's free)
- A working BlueZ stack with the `gatttool` command available on your path
  
## Running

Copy `Config.toml.example` to `Config.toml` and configure each property.

    bd_addr = "A1:B2:C3:D4:E5:F6"
    alphavantage_api_key = "ABCD1234EFGH5678"
    
`bd_addr` is the Bluetooth address of your Lovense device.

Then, start running with:

    $ cargo run
