# rust_game_huarong_dao
This is a game written in Rust.  
The game has many names, such as:
1. Huarong Dao
2. Number Puzzle
3. Klotski

## Play the game
You can play an online WASM version here:   
[huarong_dao](https://windysha.github.io/rust_game_huarong_dao/)

or, you can clone this project and run it:
```
git clone https://github.com/WindySha/rust_game_huarong_dao.git
cargo run --release
```

## Build

### Mac/Win/Linux
```
$ cargo build --release
or
$ cargo run --release
```

### Web
First, install toolchains:
```
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
```
Then, compile wasm:
```
cargo build --target wasm32-unknown-unknown --release
```
At last, generate js and wasm binding:
```
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/rust_game_huarong_dao.wasm
```

## ScreenShot
Home Page:

<img src="https://github.com/WindySha/rust_game_huarong_dao/blob/master/screenshot/01.png" width="580" height="300">

Game Page:

<img src="https://github.com/WindySha/rust_game_huarong_dao/blob/master/screenshot/02.png" width="580" height="300">
<img src="https://github.com/WindySha/rust_game_huarong_dao/blob/master/screenshot/03.png" width="580" height="300">


## Reference 
Rust Game Engine: [bracket-lib](https://github.com/amethyst/bracket-lib/tree/master)
