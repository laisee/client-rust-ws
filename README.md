# client-rust-ws
Rust client for Power.Trade WS endpoint for Balance/Position data

Setup

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


    => Running app on Prod env requires env file in root folder named ".env.test". 
       Can be created by copying "env.example" sample file and and renaming to ".env.prod" 
       Once the env file is created fill in values for: Api Key, Api Secret and WS server
     
8.  Run the code by this command "cargo run --ENV" where ENV is either 'test' or 'prod'

    e.g. "cargo run test" or "cargo run prod"

9.  Some debug messages are printed at console, but most are copied to a logging file named "app.log" which is in home folder.

10. App runs for XX cycles as it runs a loop sleeping and checking messages. 

Change the 'PT_EPOCH_COUNT' environment value in .env files to make the app run longer or shorter duration. The loop is to be replaced by event-driven code shortly. Check this repo for changes.    
