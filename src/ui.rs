use bevy::prelude::*;
use bevy::ecs::system::BoxedSystem;
use bevy::utils::HashMap;

use crate::cli::CliCommandsExt;
use crate::system::IntoChainOptionalSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ClickHandlerSet;

pub struct UiExtrasPlugin;

impl Plugin for UiExtrasPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            onclick_run_behaviors
                .in_set(ClickHandlerSet)
        );
    }
}

/// For disabling some UI elements
#[derive(Component)]
pub struct UiDisabled;

fn onclick_run_behaviors(
    world: &mut World,
    query: &mut QueryState<
        (Entity, &Interaction, &mut ClickBehavior),
        (Changed<Interaction>, Without<UiDisabled>)
    >,
) {
    let mut behaviors: HashMap<Entity, Vec<ClickBehaviorKind>> = Default::default();
    for (entity, interaction, mut behavior) in query.iter_mut(world) {
        if *interaction == Interaction::Clicked {
            let behavior = behavior.bypass_change_detection();
            behaviors.insert(entity, core::mem::take(&mut behavior.actions));
        }
    }
    for (entity, mut actions) in behaviors {
        for action in &mut actions {
            match action {
                ClickBehaviorKind::Cli(cli) => {
                    world.run_clicommand(cli);
                }
                ClickBehaviorKind::System(initted, system_opt) => {
                    if let Some(mut system) = system_opt.take() {
                        if !*initted {
                            system.initialize(world);
                            *initted = true;
                        }
                        system.run((), world);
                        system.apply_buffers(world);
                        *system_opt = Some(system);
                    }
                }
                ClickBehaviorKind::EntitySystem(initted, system_opt) => {
                    if let Some(mut system) = system_opt.take() {
                        if !*initted {
                            system.initialize(world);
                            *initted = true;
                        }
                        system.run(entity, world);
                        system.apply_buffers(world);
                        *system_opt = Some(system);
                    }
                }
            }
        }
        let Some(mut entity_mut) = world.get_entity_mut(entity) else { continue; };
        let Some(mut behavior_mut) = entity_mut.get_mut::<ClickBehavior>() else { continue; };
        behavior_mut.bypass_change_detection().actions = actions;
    }
}

enum ClickBehaviorKind {
    System(bool, Option<Box<dyn System<In = (), Out = ()>>>),
    EntitySystem(bool, Option<Box<dyn System<In = Entity, Out = ()>>>),
    Cli(String),
}

#[derive(Component, Default)]
pub struct ClickBehavior {
    actions: Vec<ClickBehaviorKind>,
}

impl ClickBehavior {
    pub fn new() -> ClickBehavior {
        ClickBehavior::default()
    }
    pub fn system<S, Param>(mut self, system: S) -> ClickBehavior
        where S: IntoSystem<(), (), Param>
    {
        self.actions.push(ClickBehaviorKind::System(false, Some(Box::new(IntoSystem::into_system(system)))));
        self
    }
    pub fn entity_system<S, Param>(mut self, system: S) -> ClickBehavior
        where S: IntoSystem<Entity, (), Param>
    {
        self.actions.push(ClickBehaviorKind::EntitySystem(false, Some(Box::new(IntoSystem::into_system(system)))));
        self
    }
    pub fn cli(mut self, cli: &str) -> ClickBehavior {
        self.actions.push(ClickBehaviorKind::Cli(cli.to_owned()));
        self
    }
}
