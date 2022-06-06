use bevy::prelude::*;
use bevy::ecs::query::Access;
use bevy::ecs::component::ComponentId;
use bevy::ecs::archetype::ArchetypeComponentId;
use std::borrow::Cow;

pub mod prelude {
    pub use crate::despawn_with;
    pub use crate::despawn_with_recursive;
    pub use crate::remove_resource;
    pub use crate::remove_from_all;
    pub use crate::remove_from_all_with;
}

#[cfg(feature = "bevy_ui")]
pub mod ui;
#[cfg(feature = "2d")]
pub mod d2;

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with_recursive<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}

/// Remove a resource using Commands
pub fn remove_resource<T: Send + Sync + 'static>(
    mut cmd: Commands,
) {
    cmd.remove_resource::<T>();
}

/// Remove a component type from all entities that have it
pub fn remove_from_all<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}

/// Remove a component type from any entities with some other component
pub fn remove_from_all_with<T: Component, W: Component>(
    mut cmd: Commands,
    q: Query<Entity, (With<T>, With<W>)>,
) {
    for e in q.iter() {
        cmd.entity(e).remove::<T>();
    }
}

pub struct ChainResultSystem<SystemIn, SystemOk, SystemErr> {
    system_in: SystemIn,
    system_ok: SystemOk,
    system_err: SystemErr,
    name: Cow<'static, str>,
    component_access: Access<ComponentId>,
    archetype_component_access: Access<ArchetypeComponentId>,
}

impl<T, E, O, SystemIn: System<Out = Result<T, E>>, SystemOk: System<In = T, Out = O>, SystemErr: System<In = E, Out = O>> System for ChainResultSystem<SystemIn, SystemOk, SystemErr> {
    type In = SystemIn::In;
    type Out = O;

    fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn archetype_component_access(&self) -> &Access<ArchetypeComponentId> {
        &self.archetype_component_access
    }

    fn component_access(&self) -> &Access<ComponentId> {
        &self.component_access
    }

    fn is_send(&self) -> bool {
        self.system_in.is_send() && self.system_ok.is_send() && self.system_err.is_send()
    }

    unsafe fn run_unsafe(&mut self, input: Self::In, world: &World) -> Self::Out {
        match self.system_in.run_unsafe(input, world) {
            Ok(t) => self.system_ok.run_unsafe(t, world),
            Err(e) => self.system_err.run_unsafe(e, world),
        }
    }

    fn apply_buffers(&mut self, world: &mut World) {
        self.system_in.apply_buffers(world);
        self.system_ok.apply_buffers(world);
        self.system_err.apply_buffers(world);
    }

    fn initialize(&mut self, world: &mut World) {
        self.system_in.initialize(world);
        self.system_ok.initialize(world);
        self.system_err.initialize(world);
        self.component_access
            .extend(self.system_in.component_access());
        self.component_access
            .extend(self.system_ok.component_access());
        self.component_access
            .extend(self.system_err.component_access());
    }

    fn update_archetype_component_access(&mut self, world: &World) {
        self.system_in.update_archetype_component_access(world);
        self.system_ok.update_archetype_component_access(world);
        self.system_err.update_archetype_component_access(world);

        self.archetype_component_access
            .extend(self.system_in.archetype_component_access());
        self.archetype_component_access
            .extend(self.system_ok.archetype_component_access());
        self.archetype_component_access
            .extend(self.system_err.archetype_component_access());
    }

    fn check_change_tick(&mut self, change_tick: u32) {
        self.system_in.check_change_tick(change_tick);
        self.system_ok.check_change_tick(change_tick);
        self.system_err.check_change_tick(change_tick);
    }
}

pub trait IntoChainResultSystem<T, E, Out, SysOk, SysErr, ParamIn, ParamOk, ParamErr>:
    IntoSystem<(), Result<T, E>, ParamIn> + Sized
where
    SysOk: IntoSystem<T, Out, ParamOk>,
    SysErr: IntoSystem<E, Out, ParamErr>,
{
    fn chain_result(self, ok: SysOk, err: SysErr) -> ChainResultSystem<Self::System, SysOk::System, SysErr::System>;
}

impl<T, E, Out, SysIn, SysOk, SysErr, ParamIn, ParamOk, ParamErr>
    IntoChainResultSystem<T, E, Out, SysOk, SysErr, ParamIn, ParamOk, ParamErr> for SysIn
where
    SysIn: IntoSystem<(), Result<T, E>, ParamIn>,
    SysOk: IntoSystem<T, Out, ParamOk>,
    SysErr: IntoSystem<E, Out, ParamErr>,
{
    fn chain_result(self, ok: SysOk, err: SysErr) -> ChainResultSystem<SysIn::System, SysOk::System, SysErr::System> {
        let system_in = IntoSystem::into_system(self);
        let system_ok = IntoSystem::into_system(ok);
        let system_err = IntoSystem::into_system(err);

        ChainResultSystem {
            name: Cow::Owned(format!("ChainResult({} -> {} / {})", system_in.name(), system_ok.name(), system_err.name())),
            system_in,
            system_ok,
            system_err,
            archetype_component_access: Default::default(),
            component_access: Default::default(),
        }
    }
}

