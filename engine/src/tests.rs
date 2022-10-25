mod tests{
    #[allow(unused_imports)]
    use engine_internal::prelude::*;

    #[test]
    fn remove_entity(){
        let mut world = World::new();
        let entity1 = world.add_entity();
        let _entity2 = world.add_entity();

        world.destroy_entity(entity1);
        let entity3 = world.add_entity();
        let entity4 = world.add_entity();

        assert!(entity3 == 0, "wrong entity ID assigned to #3");
        assert!(entity4 == 2, "wrong entity ID assigned to #4");
    }
}