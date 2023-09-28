use overworld_components::component::resource::{GameResource, Resource};

fn main() {
    let resource: GameResource = GameResource::new("Money".to_string(), 100);

    println!("Resource name: {}", resource.name());
    println!("Resource value: {}", resource.value());
    println!("Resource display: {}", resource);
}
