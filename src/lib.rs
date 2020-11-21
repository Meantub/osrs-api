#[macro_use] extern crate prettytable;

pub mod skill;
pub mod hiscore;
pub mod gamemode;

use skill::Skill;
use hiscore::Hiscore;
use gamemode::Gamemode;


const HISCORE_URL: &str = "https://secure.runescape.com/m=hiscore_oldschool";
const STATS_URL: &str = "/index_lite.ws?player=";

// TODO: Turn the Option to a Result
/// Gets the hiscore of a specific username in a specific game mode
/// # Examples
///
/// ```
/// use osrs_api::gamemode::Gamemode;
/// let my_hiscore = osrs_api::get_hiscore("meantub".to_string(), Gamemode::Main);
/// println!("{}", my_hiscore.unwrap());
/// ```
pub fn get_hiscore(username: String, gamemode: Gamemode) -> Option<Hiscore> {
    let url_prefix = match gamemode.clone() {
        Gamemode::Main => HISCORE_URL.to_string(),
        _ => {
            let mut hiscore_string: String = HISCORE_URL.to_string();
            hiscore_string.push_str("_");
            hiscore_string.push_str(gamemode.into());
            hiscore_string
        }
    };

    let url = &[url_prefix, STATS_URL.to_string(), username].concat();

    let response = ureq::get(url).call();

    if response.ok() {
        // println!("Success: {}", response.into_string().unwrap());
        let response_string = response.into_string().unwrap();
        // let mut reader = csv::Reader::from_reader(response_string.as_bytes());
        let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(response_string.as_bytes());
        let mut hiscores: [Skill; 24] = [Skill::default(); 24];
        reader.deserialize().take(24).enumerate().for_each(|(index, skill)| {
            let skill: Skill = skill.unwrap();
            hiscores[index] = skill;
        });

        Some(Hiscore {
            overall: hiscores[0],
            attack: hiscores[1],
            defence: hiscores[2],
            strength: hiscores[3],
            hitpoints: hiscores[4],
            ranged: hiscores[5],
            prayer: hiscores[6],
            magic: hiscores[7],
            cooking: hiscores[8],
            woodcutting: hiscores[9],
            fletching: hiscores[10],
            fishing: hiscores[11],
            firemaking: hiscores[12],
            crafting: hiscores[13],
            smithing: hiscores[14],
            mining: hiscores[15],
            herblore: hiscores[16],
            agility: hiscores[17],
            thieving: hiscores[18],
            slayer: hiscores[19],
            farming: hiscores[20],
            runecraft: hiscores[21],
            hunter: hiscores[22],
            construction: hiscores[23]
        })
    } else {
        // println!("Error: {}: {}", response.status(), response.into_string().unwrap());
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hiscore_lynx_titan() {
        assert_eq!(
            get_hiscore("Lynx Titan".to_string(), Gamemode::Main).unwrap(),
            Hiscore {
                overall: (1, 2277, 4_600_000_000f64).into(),
                attack: (15, 99, 200_000_000f64).into(),
                defence: (28, 99, 200_000_000f64).into(),
                strength: (18, 99, 200_000_000f64).into(),
                hitpoints: (7, 99, 200_000_000f64).into(),
                ranged: (8, 99, 200_000_000f64).into(),
                prayer: (11, 99, 200_000_000f64).into(),
                magic: (32, 99, 200_000_000f64).into(),
                cooking: (161, 99, 200_000_000f64).into(),
                woodcutting: (15, 99, 200_000_000f64).into(),
                fletching: (12, 99, 200_000_000f64).into(),
                fishing: (9, 99, 200_000_000f64).into(),
                firemaking: (49, 99, 200_000_000f64).into(),
                crafting: (4, 99, 200_000_000f64).into(),
                smithing: (3, 99, 200_000_000f64).into(),
                mining: (25, 99, 200_000_000f64).into(),
                herblore: (5, 99, 200_000_000f64).into(),
                agility: (24, 99, 200_000_000f64).into(),
                thieving: (12, 99, 200_000_000f64).into(),
                slayer: (2, 99, 200_000_000f64).into(),
                farming: (19, 99, 200_000_000f64).into(),
                runecraft: (7, 99, 200_000_000f64).into(),
                hunter: (4, 99, 200_000_000f64).into(),
                construction: (4, 99, 200_000_000f64).into()
            }
        );

        assert_eq!(get_hiscore("Lynx Titan".to_string(), Gamemode::Main).unwrap().agility, (24, 99, 200_000_000f64).into())
    }
}
