use crate::prelude::*;

#[system]
#[read_component(IntendsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &IntendsToAttack)>::query();
    let defenders: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.defender))
        .collect();

    defenders.iter().for_each(|(message, defender)| {
        
        
        let is_player = ecs
            .entry_ref(*defender)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(health) = ecs
            .entry_mut(*defender)
            .unwrap()
            .get_component_mut::<Health>()
        {
            debug!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*defender);
            }
            debug!("Health after attack: {}", health.current);
        }
        commands.remove(*message);
    });
}
