use overworld::progression::experience::ExperienceSystem;

fn main() {
    let mut progression = ExperienceSystem::simple(0, vec![100, 300, 500]);

    progression.add_experience(100);

    println!(
        "You are level {}, next level in {} experience",
        progression.get_level(),
        progression.get_next_milestone().unwrap_or(0)
    );
}
