use overworld_components::component::progression::experience::ExperienceSystem;

fn main() {
    let mut system = ExperienceSystem::simple(24, vec![25, 100, 1000, 2500, 10000]);

    for _ in 1..=6 {
        let xp_remaining = system.get_experience_remaining();

        println!(
            "Level: {} (XP: {:5}) - Next level up in: {:4} XP",
            system.get_level(),
            system.get_experience(),
            match xp_remaining {
                Some(xp) => xp.to_string(),
                None => String::from("N/A"),
            }
        );

        if let Some(xp) = xp_remaining {
            system.add_experience(xp)
        }
    }
}
