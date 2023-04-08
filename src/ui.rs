use bevy::prelude::*;
use bevy::ecs::system::BoxedSystem;
use bevy::utils::HashMap;

use crate::cli::CliCommandsExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct ButtonHandlerSet;

pub struct UiExtrasPlugin;

impl Plugin for UiExtrasPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            button_run_behaviors
                .in_set(ButtonHandlerSet)
        );
    }
}

/// For disabling some UI elements
#[derive(Component)]
pub struct UiDisabled;

fn button_run_behaviors(
    world: &mut World,
    query: &mut QueryState<
        (Entity, &Interaction, &mut ButtonBehavior),
        (Changed<Interaction>, Without<UiDisabled>)
    >,
) {
    let mut behaviors: HashMap<Entity, Vec<ButtonBehaviorKind>> = Default::default();
    for (entity, interaction, mut behavior) in query.iter_mut(world) {
        if *interaction == Interaction::Clicked {
            let behavior = behavior.bypass_change_detection();
            behaviors.insert(entity, core::mem::take(&mut behavior.actions));
        }
    }
    for (entity, mut actions) in behaviors {
        for action in &mut actions {
            match action {
                ButtonBehaviorKind::Cli(cli) => {
                    world.run_clicommand(cli);
                }
                ButtonBehaviorKind::System(system_opt) => {
                    if let Some(mut system) = system_opt.take() {
                        system.initialize(world);
                        system.run((), world);
                        system.apply_buffers(world);
                        *system_opt = Some(system);
                    }
                }
            }
        }
        let Some(mut entity_mut) = world.get_entity_mut(entity) else { continue; };
        let Some(mut behavior_mut) = entity_mut.get_mut::<ButtonBehavior>() else { continue; };
        behavior_mut.bypass_change_detection().actions = actions;
    }
}

enum ButtonBehaviorKind {
    System(Option<BoxedSystem>),
    Cli(String),
}

#[derive(Component, Default)]
pub struct ButtonBehavior {
    actions: Vec<ButtonBehaviorKind>,
}

impl ButtonBehavior {
    pub fn new() -> ButtonBehavior {
        ButtonBehavior::default()
    }
    pub fn cli(mut self, cli: &str) -> ButtonBehavior {
        self.actions.push(ButtonBehaviorKind::Cli(cli.to_owned()));
        self
    }
    pub fn system<S, Param>(mut self, system: S) -> ButtonBehavior
        where S: IntoSystem<(), (), Param>
    {
        self.actions.push(ButtonBehaviorKind::System(Some(Box::new(IntoSystem::into_system(system)))));
        self
    }
}
