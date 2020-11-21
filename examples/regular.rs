use osrs_api::gamemode::Gamemode;

fn main() {
    println!("{}", osrs_api::get_hiscore("meantub".to_string(), Gamemode::Main).unwrap());
}