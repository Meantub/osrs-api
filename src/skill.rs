use serde::Deserialize;
use thousands::Separable;

// FIXME: f64 is not entirely accurate since
// 32-bit integer with one decimal point is actually
// how the xp values are stored
// so when we do calculations it is actually off
// by a decent margin due to the accuracy being better
type XP = f64;
type Level = u64;
type Rank = u64;

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub struct Skill {
    pub rank: Rank,
    pub level: Level,
    pub xp: XP
}

impl Skill {
    pub fn xp_till_200_mill(self) -> XP {
        200_000_000f64 - self.xp
    }

    pub fn xp_at_level(level: Level) -> XP {
        let prev_level_sum: f64 = (1..level).into_iter()
            .map(|n: u64| n as f64 + 300.0 * 2f64.powf(n as f64/7.0))
            .sum();

        (1.0/4.0) * prev_level_sum
    }

    pub fn xp_till_level(self, level: Level) -> XP {
        if level < self.level {
            panic!("Your level is higher than the provided level")
        }

        Self::xp_at_level(level) - self.xp
    }
}

impl Into<Skill> for (Rank, Level, XP) {
    fn into(self) -> Skill {
        Skill {
            rank: self.0,
            level: self.1,
            xp: self.2
        }
    }
}

// TODO: Make this a conditional compilation target
impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = self.rank.separate_with_commas();
        let level = self.level.separate_with_commas();
        let xp = self.xp.separate_with_commas();

        write!(f, "({}, {}, {})", rank, level, xp)
    }
}



impl Default for Skill {
    fn default() -> Self {
        Skill {
            rank: 0,
            level: 0,
            xp: 0f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // FIXME: Currently these tests fail because the precision
    // is too high
    #[test]
    fn test_xp_at_level() {
        assert_eq!(Skill::xp_at_level(50), 101_333f64);
    }

    #[test]
    fn test_xp_till_200_mill() {
        let skill_1 = Skill {
            rank: 20_000,
            level: 33,
            xp: 19_000f64
        };

        assert_eq!(skill_1.xp_till_200_mill(), 200_000_000f64 - skill_1.xp);
    }

    #[test]
    fn test_xp_till_level() {
        let skill_1 = Skill {
            rank: 20_000,
            level: 33,
            xp: 19_000f64
        };

        assert_eq!(skill_1.xp_till_level(50), 101_333f64 - 19_000f64);
    }
}

