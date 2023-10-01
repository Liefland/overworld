#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "better_enums",
    derive(strum_macros::EnumString, strum_macros::EnumIter, Display)
)]
/// Source: https://www.dndbeyond.com/races
pub enum Race {
    Dragonborn,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    Halfling,
    HalfOrc,
    Human,
    Tiefling,

    #[cfg(feature = "expansion_motm")]
    Aarakocra,
    #[cfg(feature = "expansion_motm")]
    Aasimar,
    #[cfg(feature = "expansion_motm")]
    AirGenasi,
    #[cfg(feature = "expansion_motm")]
    Bugbear,
    #[cfg(any(feature = "expansion_motm", feature = "expansion_guildmasters"))]
    Centaur,
    #[cfg(any(feature = "expansion_motm", feature = "expansion_eberron"))]
    Changeling,
    #[cfg(feature = "expansion_motm")]
    DeepGnome,
    #[cfg(feature = "expansion_motm")]
    Duergar,
    #[cfg(feature = "expansion_motm")]
    EarthGenasi,
    #[cfg(feature = "expansion_motm")]
    Ealdrin,
    #[cfg(feature = "expansion_motm")]
    Fairy,
    #[cfg(feature = "expansion_motm")]
    Firbolg,
    #[cfg(feature = "expansion_motm")]
    FireGenasi,
    #[cfg(feature = "expansion_motm")]
    Githyanki,
    #[cfg(feature = "expansion_motm")]
    Githzerai,
    #[cfg(feature = "expansion_motm")]
    Goblin,
    #[cfg(feature = "expansion_motm")]
    Goliath,
    #[cfg(feature = "expansion_motm")]
    Harengon,
    #[cfg(feature = "expansion_motm")]
    Hobgoblin,
    #[cfg(feature = "expansion_motm")]
    Kenku,
    #[cfg(feature = "expansion_motm")]
    Kobold,
    #[cfg(feature = "expansion_motm")]
    Lizardfolk,
    #[cfg(any(feature = "expansion_motm", feature = "expansion_guildmasters"))]
    Minotaur,
    #[cfg(feature = "expansion_motm")]
    Orc,
    #[cfg(any(feature = "expansion_motm", feature = "expansion_mythic_odysseys"))]
    Satyr,
    #[cfg(feature = "expansion_motm")]
    SeaElf,
    #[cfg(feature = "expansion_motm")]
    ShadarKai,
    #[cfg(any(feature = "expansion_motm", feature = "expansion_eberron"))]
    Shifter,
    #[cfg(feature = "expansion_motm")]
    Tabaxi,
    #[cfg(feature = "expansion_motm")]
    Tortle,
    #[cfg(feature = "expansion_motm")]
    Triton,
    #[cfg(feature = "expansion_motm")]
    WaterGenasi,
    #[cfg(feature = "expansion_motm")]
    Yuanti,

    #[cfg(feature = "expansion_dragonlance")]
    Kender,

    #[cfg(feature = "expansion_spelljammer")]
    AstralElf,
    #[cfg(feature = "expansion_spelljammer")]
    Autognome,
    #[cfg(feature = "expansion_spelljammer")]
    Giff,
    #[cfg(feature = "expansion_spelljammer")]
    Hadozee,
    #[cfg(feature = "expansion_spelljammer")]
    Plasmoid,
    #[cfg(feature = "expansion_spelljammer")]
    ThriKreen,

    #[cfg(feature = "expansion_strixhaven")]
    Owlin,

    #[cfg(feature = "expansion_van_richten")]
    Lineages,

    #[cfg(feature = "expansion_mythic_odysseys")]
    Leonin,

    #[cfg(feature = "expansion_eberron")]
    Kalashtar,
    #[cfg(feature = "expansion_eberron")]
    ShifterEberron,
    #[cfg(feature = "expansion_eberron")]
    Warforged,
    #[cfg(feature = "expansion_eberron")]
    Verdan,

    #[cfg(feature = "expansion_guildmasters")]
    Loxodon,
    #[cfg(feature = "expansion_guildmasters")]
    SimicHybrid,
    #[cfg(feature = "expansion_guildmasters")]
    Vedalken,

    #[cfg(feature = "expansion_sword_coast")]
    FeralTiefling,

    #[cfg(feature = "expansion_locathah")]
    Locathah,

    #[cfg(feature = "expansion_grung")]
    Grung,
}

mod tests {
    #![allow(unused_imports)]

    use crate::race::Race;
    #[cfg(feature = "better_enums")]
    use crate::strum::IntoEnumIterator;

    #[cfg(feature = "better_enums")]
    #[test]
    fn strum_integration_works() {
        assert_eq!(Race::Dragonborn.to_string(), "Dragonborn");
    }

    #[cfg(feature = "better_enums")]
    #[test]
    fn strum_iterate_races() {
        assert!(Race::iter().count() > 0);
    }
}
