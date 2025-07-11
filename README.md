# client-rust-ws

[![Rust[1.74.1, stable]](https://github.com/laisee/client-rust-ws/actions/workflows/rust.yml/badge.svg)](https://github.com/laisee/client-rust-ws/actions/workflows/rust.yml)
[![Cargo Test](https://github.com/laisee/client-rust-ws/actions/workflows/ci.yml/badge.svg)](https://github.com/laisee/client-rust-ws/actions/workflows/ci.yml)
![Cargo Clippy](https://github.com/laisee/client-rust-ws/actions/workflows/clippy.yml/badge.svg)[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)![MSRV](https://img.shields.io/badge/MSRV-1.74.1-orange)
 
## Rust client for Power.Trade WS endpoints (v0.1.7)

### Power.Trade WebSocket API documents

See [here](https://docs.api.power.trade/#Introduction) for WS API documents

### Power.Trade Authentication

See [here](https://docs.api.power.trade/#Authentication) for an explanation of the WS API authentication process @ power.trade.

### Content provided (see endpoint links below)

- Balance and Position data

  See [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html) for power.trade API docs on balances & positions

- Single Leg Orders/RFQs (streaming quotes)

  See [here](https://docs.api.power.trade/#_processing_display_order_added) for power.trade API docs on streaming data.

  The WS message format can be found at
      [new order](https://docs.api.power.trade/#display_order_added) [order deleted](https://docs.api.power.trade/#display_order_added) [order executed](https://docs.api.power.trade/#display_order_added) [order update](https://docs.api.power.trade/#display_order_added)
   
- Multi Leg orders/RFQs (streaming quotes)

  See [here](https://docs.api.power.trade/#multi_leg_display_order_added) for power.trade API docs on streaming data.

  The WS message format for multi-leg orders can be found at:
          [new order](https://docs.api.power.trade/#multi_leg_display_order_added) [order deleted](https://docs.api.power.trade/#multi_leg_display_order_deleted) [order executed](https://docs.api.power.trade/#multi_leg_display_order_executed) [order updated](https://docs.api.power.trade/#multi_leg_display_order_updated)
   
### Sample Requests

   See [this page](https://power-trade.github.io/api-docs-source/ws_feeds.html#Market_Feeds_Per_Symbol_Channels_Sample_Requests) for sample data feed requests
  
### Setup

1. Download code from this repo

2. Install Rust using instructions [here](https://www.rust-lang.org/learn/get-started) OR [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

3. Cd to the folder containing the code downloaded from repo

4. Run 'build' command:
   ```
   cargo build
   ```
   to update linked libraries and compile code. The version built by default is 'dev'
   
6. To build the 'release' version which is more optimized reduced in size run this command
   ```
   cargo build --release
   ```
8. Before running the code configuration should be added to files (one per env)

    => Running app on Development env requires env file in root folder named ".env.dev". 
       Can be created by copying "env.example" sample file and renaming to ".env.dev" to setup dev env  

       ```
       cp .env.example .env.dev
       ```

    => Running app on Test env requires env file in root folder named ".env.test". 
       Can be created by copying "env.example" sample file and renaming to ".env.test" to setup test env  

       ```
       cp .env.example .env.test
       ```
   
       Once the env file is created add values for:
       * PT_API_KEY - API Key for authentication
       * PT_API_SECRET - API Secret for authentication
       * PT_SERVER_URL - WebSocket server address & port
       * PT_EPOCH_COUNT - Number of cycles the app will run
       * PT_WS_SLEEP - Sleep duration between message checks (in seconds)

       API key to be used must be one of the Read-Only ones issued under '' dropdown for API type.
       See example of UI fopr generating API Key below with correct settings for the WS Balance/Position API

      ![image](https://github.com/laisee/client-rust-ws/assets/5905130/e5daa2a9-e374-4e7f-aaa6-f916da6da0a9)


    => Running app on Prod env requires a .env file in root folder named ".env.prod". 
       This file can be created by copying "env.example" sample file and and renaming to ".env.prod"
   
       ```
       cp .env.example .env.prod
       ```
   
       Once the env file is created add the same values as mentioned above.
     
10.  Run the code by this command "cargo run -- --env ENV" where ENV is either 'development', 'test' or 'production'

     ```
     cargo run -- --env development
     ```
     or
     ```
     cargo run -- --env test
     ```
     or
     ```
     cargo run -- --env production
     ```

12.  Some debug messages are printed at console, but most are copied to a logging file named "app.log" which is located in the root folder.

13. App runs for the number of cycles specified in PT_EPOCH_COUNT environment variable as it runs a loop sleeping and checking messages. 

Change the 'PT_EPOCH_COUNT' environment value in .env files to make the app run longer or shorter duration.

11. Step 8 above can be replaced by running the generated binary which will be copied to the "target" folder under project root/home folder. 

There will be a "debug" folder generated by running "cargo build" command and a "release" folder created when "cargo build --release" command is run

To run "debug" version execute command
```
./target/debug/client-rust-ws --env test
```
in the project root/home folder. The environment can be changed to "development" or "production" also as follows
```
./target/debug/client-rust-ws --env development
```
or
```
./target/debug/client-rust-ws --env production
```

To run "release" version: execute command 
```
./target/release/client-rust-ws --env test
``` 
in project root/home folder. The environment can be changed to "development" or "production" also.

n.b. to view command line options e.g. available settings for environment ("--env") flag run the command without any settings ("./target/debug/client-rust-ws") or with no value added for environment flag ("./target/debug/client-rust-ws --env")

12. Running the Rust WS Client against various power.trade endpoints

The rust client can be configured to listen for any of the three content types below by setting the environment variable named "PT_SERVER_URL" to the URLs below in the relevant configuration file for development (".env.dev"), test (".env.test") or production (".env.prod").

N.B. Current Rust client only supports one of the three content types per installed Rust code (under /target/release/ folder). 
Listening for more than one set of content from list below requires multiple copies of the Rust runtime files and a custom configuration per instance.

### Endpoint Links

#### _Balances & Positions_
see [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html) for power.trade API docs on balance and position data available from the following test & production API endpoints

see [here](https://power-trade.github.io/api-docs-source/ws_position_summary.html#_sample_response) for details on power.trade's balance & positions response message 

| Env | Link | Notes |
|-----|------|---------|
| Test | wss://api.wss.test.power.trade/v1/position_summary | both balances & positions returned in one response |
|Production | wss://api.wss.prod.power.trade/v1/position_summary | both balances & positions returned in one response |

#### _Single Leg Orders/RFQs_

n.b. to customize or filter  the data feed content see parameters documented at [this page](https://power-trade.github.io/api-docs-source/ws_feeds.html#Market_Feeds_Connection_Parameters) 

| Env | Link | Notes |
|-----|------|---------|
| Test | wss://api.wss.test.power.trade/v1/feeds/?type[]=mbp_snapshot&tradeable_type[]=all_single_leg&mbp_period=1&mbo_period=0 | link returns all tradeable types with full price book snapshot sent but no order book snapshot|
| Production | wss://api.wss.prod.power.trade/v1/feeds/?type[]=mbp_snapshot&tradeable_type[]=all_single_leg&mbp_period=1&mbo_period=0 | link returns all tradeable types with full price book snapshot sent but no order book snapshot |

#### _Multi Leg orders/RFQs_

n.b. to customize or filter  the data feed content see parameters documented at [this page](https://power-trade.github.io/api-docs-source/ws_feeds.html#Market_Feeds_Connection_Parameters) 

| Env | Link | Notes |
|-----|------|---------|
| Test| wss://api.wss.test.power.trade/v1/feeds/multi_leg?type[]=all_multi_leg,multi_leg_mbp_snapshot&mbp_period=1&mbo_period=0 | all multi-leg showing price book snapshot but not order book snapshot. add "market_id[]=none" to retireve RFQs  |
| Production | wss://api.wss.prod.power.trade/v1/feeds/multi_leg?type[]=all_multi_leg,multi_leg_mbp_snapshot&mbp_period=1&mbo_period=0 | all multi-leg showing price book snapshot but not order book snapshot. add "market_id[]=none" to retireve RFQs|

