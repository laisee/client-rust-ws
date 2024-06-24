# client-rust-ws

[![Cargo Build](https://github.com/laisee/client-rust-ws/actions/workflows/rust.yml/badge.svg)](https://github.com/laisee/client-rust-ws/actions/workflows/rust.yml)
[![Cargo Test](https://github.com/laisee/client-rust-ws/actions/workflows/ci.yml/badge.svg)](https://github.com/laisee/client-rust-ws/actions/workflows/ci.yml)
![Cargo Clippy](https://github.com/laisee/client-rust-ws/actions/workflows/clippy.yml/badge.svg)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
 

## Rust client for Power.Trade WS endpoints

### Content provided (see endpoint links below)

- Balance and Position data (see [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html) for power.trade API docs on balances & positions
- Single Leg RFQs (streaming quotes)
- Multi Leg RFQs (streaming quotes)
   
### Setup

1. Download code from this repo

2. Install Rust using these instructions: 
    https://www.rust-lang.org/learn/get-started OR
    https://doc.rust-lang.org/book/ch01-01-installation.html

3. Cd to the folder containing the code downloaded from repo

4. Run 'build' command -> "cargo build" to update linked libraries and compile code. The version built by default is 'dev'
   
6. To build the 'release' version which is more optimized reduced in size run 'cargo build --release'

7. Before running the code configuration should be added to files (one per env)

    => Running app on Test env requires env file in root folder named ".env.test". 
       Can be created by copying "env.example" sample file and renaming to ".env.test" 
       Once the env file is created fill in values for: Api Key, Api Secret and WS server.

       API key to be used must be one of the REad-Only ones issued under '' dropdown for API type.
       See example of UI fopr generating API Key below with correct settings for the WS Balance/Position API

      ![image](https://github.com/laisee/client-rust-ws/assets/5905130/e5daa2a9-e374-4e7f-aaa6-f916da6da0a9)


    => Running app on Prod env requires a .env file in root folder named ".env.prod". 
       This file can be created by copying "env.example" sample file and and renaming to ".env.prod" 
       Once the env file is created fill in values for: Api Key, Api Secret and WS server 
     
8.  Run the code by this command "cargo run --ENV" where ENV is either 'test' or 'prod'

    e.g. "cargo run test" or "cargo run prod"

9.  Some debug messages are printed at console, but most are copied to a logging file named "app.log" which is in home folder.

10. App runs for XX cycles as it runs a loop sleeping and checking messages. 

Change the 'PT_EPOCH_COUNT' environment value in .env files to make the app run longer or shorter duration. The loop is to be replaced by event-driven code shortly. Check this repo for changes

11. Step 8 above can be replaced by running the generated binary which will be copied to the "target" folder under project root/home folder. 

There will be a "debug" folder generated by running "cargo build" command and a "release" folder created when "cargo build --release" command is run

To run "debug" version: execute command "./target/debug/client-rust-ws --env test" in project root/home folder. The environment can be changed to "production" also

To run "release" version: execute command "./target/release/client-rust-ws --env test" in project root/home folder. The environment can be changed to "production" also

n.b. to view command line options e.g. available settings for environment ("--env") flag run the command without any settings ("./target/debug/client-rust-ws") or with no value added for environment flag ("./target/debug/client-rust-ws --env")

12. Running the Rust WS Client against various power.trade endpoints

The rust client can be configured to listen for any of the three content types below by setting the ennviroment variables named "PT_SERVER" to the URLs below in the relevant configurwtion file for test (".env.test") or production (".env.prod").

N.B. Current Rust client only supports one of the three content types per installed Rust code (under /target/release/ folder). 
Listening for more than one content from list below requires multiple copies of the Rust runtime files and a custom configuration per instance.

### Endpoint Links

#### _Balances & Positions_
see [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html) for power.trade API docs on balance and position data available from the following test & production API endpoints

see [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html#_sample_response) for details on power.trade's balance & positions response message 

| Env | Link | Notes |
|-----|------|---------|
| Test | wss://api.wss.test.power.trade/v1/position_summary |
|Production | wss://api.wss.prod.power.trade/v1/position_summary |

#### _Single Leg RFQs_

| Env | Link | Notes |
|-----|------|---------|
| Test | wss://api.wss.test.power.trade/v1/feeds/?type[]=mbp_snapshot&tradeable_type[]=all_single_leg&mbp_period=1&mbo_period=0 | |
| Production | wss://api.wss.prod.power.trade/v1/feeds/?type[]=mbp_snapshot&tradeable_type[]=all_single_leg&mbp_period=1&mbo_period=0 | |

#### _Multi Leg RFQs_

| Env | Link | Notes |
|-----|------|---------|
| Test| wss://api.wss.test.power.trade/v1/feeds/multi_leg?type[]=cycle,multi_leg_mbp_snapshot&mbp_period=1&mbo_period=0" | |
| Production | wss://api.wss.prod.power.trade/v1/feeds/multi_leg?type[]=cycle,multi_leg_mbp_snapshot&mbp_period=1&mbo_period=0" | |

