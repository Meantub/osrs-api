# OSRS API
An Oldschool Runescape API wrapper

## Description
This is a wrapper for the Hiscores API in Oldschool Runescape. It lets you access information for any of the gamemodes.

Currently it only shows the skills but if you would like you can submit a pull request to add the boss kill stats and stuff like that.

## Usage
Check out the examples folder, there is only one method as of right now:
```rust
use osrs_api::gamemode::Gamemode;

fn main() {
    println!("{}", osrs_api::get_hiscore("meantub".to_string(), Gamemode::Main).unwrap());
}
```

## NOTE
There are XP calculations that are being worked on, apparently the data structure that holds the experience in the game uses only 1 decimal place, and by default the floats in Rust have a lot more decimal points of precision. So probably needs a custom data type of some sort. It's still very close to what the normal values are.

## TODO
* Experience precision 32 bit integer with 1 decimal point
* Conditional compilation with features for `std::fmt::Display` traits
* Make a constants module
* Include more of the activities in the game
* Maybe RS3, don't know much about it
* More testing
* Grand Exchange/Item APIs
* Maybe add a single import to make things easier `osrs_api::prelude::*`