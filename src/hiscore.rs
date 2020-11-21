use crate::skill::Skill;
use thousands::Separable;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hiscore {
    pub overall: Skill,
    pub attack: Skill,
    pub defence: Skill,
    pub strength: Skill,
    pub hitpoints: Skill,
    pub ranged: Skill,
    pub prayer: Skill,
    pub magic: Skill,
    pub cooking: Skill,
    pub woodcutting: Skill,
    pub fletching: Skill,
    pub fishing: Skill,
    pub firemaking: Skill,
    pub crafting: Skill,
    pub smithing: Skill,
    pub mining: Skill,
    pub herblore: Skill,
    pub agility: Skill,
    pub thieving: Skill,
    pub slayer: Skill,
    pub farming: Skill,
    pub runecraft: Skill,
    pub hunter: Skill,
    pub construction: Skill
}

impl IntoIterator for Hiscore {
    type Item = Skill;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut skills: Vec<Skill> = Vec::new();
        skills.push(self.overall);
        skills.push(self.attack);
        skills.push(self.defence);
        skills.push(self.strength);
        skills.push(self.hitpoints);
        skills.push(self.ranged);
        skills.push(self.prayer);
        skills.push(self.magic);
        skills.push(self.cooking);
        skills.push(self.woodcutting);
        skills.push(self.fletching);
        skills.push(self.fishing);
        skills.push(self.firemaking);
        skills.push(self.crafting);
        skills.push(self.smithing);
        skills.push(self.mining);
        skills.push(self.herblore);
        skills.push(self.agility);
        skills.push(self.thieving);
        skills.push(self.slayer);
        skills.push(self.farming);
        skills.push(self.runecraft);
        skills.push(self.hunter);
        skills.push(self.construction);

        skills.into_iter()
    }
}

// TODO: Add macro to make this less annoying to look at
// TODO: Put skill_names somewhere as a constant
// TODO: Make this a conditional compilation target, this is not necessary for most projects
impl std::fmt::Display for Hiscore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = prettytable::Table::new();

        let skill_names = [
            "Overall",
            "Attack",
            "Defence",
            "Strength",
            "Hitpoints",
            "Ranged",
            "Prayer",
            "Magic",
            "Cooking",
            "Woodcutting",
            "Fletching",
            "Fishing",
            "Firemaking",
            "Crafting",
            "Smithing",
            "Mining",
            "Herblore",
            "Agility",
            "Thieving",
            "Slayer",
            "Farming",
            "Runecraft",
            "Hunter",
            "Construction"
        ];

        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["Name", "Rank", "Level", "XP"]);

        skill_names.iter().zip(self.into_iter()).skip(1).for_each(|(skill_name, skill)| {
            let rank = skill.rank.separate_with_commas();
            let level = skill.level.separate_with_commas();
            let xp = skill.xp.separate_with_commas();
            table.add_row(row![skill_name, rank, level, xp]);
        });
        let rank = self.overall.rank.separate_with_commas();
        let level = self.overall.level.separate_with_commas();
        let xp = self.overall.xp.separate_with_commas();
        table.add_empty_row();
        table.add_row(row!["Overall", rank, level, xp]);

        write!(f, "{}", table.to_string())
    }
}

impl Default for Hiscore {
    fn default() -> Self {
        Hiscore {
            overall: Skill::default(),
            attack: Skill::default(),
            defence: Skill::default(),
            strength: Skill::default(),
            hitpoints: Skill::default(),
            ranged: Skill::default(),
            prayer: Skill::default(),
            magic: Skill::default(),
            cooking: Skill::default(),
            woodcutting: Skill::default(),
            fletching: Skill::default(),
            fishing: Skill::default(),
            firemaking: Skill::default(),
            crafting: Skill::default(),
            smithing: Skill::default(),
            mining: Skill::default(),
            herblore: Skill::default(),
            agility: Skill::default(),
            thieving: Skill::default(),
            slayer: Skill::default(),
            farming: Skill::default(),
            runecraft: Skill::default(),
            hunter: Skill::default(),
            construction: Skill::default()
        }
    }
}