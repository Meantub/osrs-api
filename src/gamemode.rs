#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Gamemode {
    Main,
    Ironman,
    Hardcore,
    Ultimate,
    Deadman,
    Seasonal,
    Tournament
}

impl Into<Gamemode> for &str {
 fn into(self) -> Gamemode {
    match self {
        "" => Gamemode::Main,
        "ironman" => Gamemode::Ironman,
        "hardcore_ironman" => Gamemode::Hardcore,
        "ultimate" => Gamemode::Ultimate,
        "deadman" => Gamemode::Deadman,
        "seasonal" => Gamemode::Seasonal,
        "tournament" => Gamemode::Tournament,
        _ => Gamemode::Main
    }
 }
}

impl From<Gamemode> for &str {
 fn from(g: Gamemode) -> Self {
    match g {
        Gamemode::Main => "",
        Gamemode::Ironman => "ironman",
        Gamemode::Hardcore => "hardcore_ironman",
        Gamemode::Ultimate => "ultimate",
        Gamemode::Deadman => "deadman",
        Gamemode::Seasonal => "seasonal",
        Gamemode::Tournament => "tournament"
    }
 }
}