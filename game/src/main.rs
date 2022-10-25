use engine::prelude::*;

struct Name(&'static str);

struct Health(u32);

fn main(){

    let mut world = World::new();

    let ent1 = world.add_entity();
    let ent2 = world.add_entity();

    world.add_component_to_entity(ent1, Name("Hanz"));
    world.add_component_to_entity(ent2, Name("Barry"));

    world.add_component_to_entity(ent1, Health(50));
    world.add_component_to_entity(ent2, Health(20));

    'main: loop{

        let mut health_vec = world.borrow_component_vec::<Health>();
        let mut name_vec =world.borrow_component_vec::<Name>();
        
        let zip = health_vec.iter_mut().zip(name_vec.iter_mut());

        for (health, name) in zip.map(|(health, name)|{(health.as_mut().unwrap(), name.as_mut().unwrap())}
        ){

            if health.0 <= 0 {
                println!("{} is dead!", name.0);
                break 'main;
            } else{
                println!("{} has {} hp", name.0, health.0);
                health.0 -= 10;
            }
 
            println!();
        }
    };


    
}