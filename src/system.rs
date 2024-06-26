use bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell;
use bevy::prelude::*;
use bevy::ecs::query::{Access, QueryFilter};
use bevy::ecs::component::{ComponentId, Tick};
use bevy::ecs::archetype::ArchetypeComponentId;
use bevy::utils::intern::Interned;
use std::borrow::Cow;

pub fn any_filter<F: QueryFilter>(
    q: Query<(), F>,
) -> bool {
    !q.is_empty()
}

pub fn none_filter<F: QueryFilter>(
    q: Query<(), F>,
) -> bool {
    q.is_empty()
}

pub fn any_added_component<T: Component>(
    q: Query<(), Added<T>>,
) -> bool {
    !q.is_empty()
}

pub fn any_changed_component<T: Component>(
    q: Query<(), Changed<T>>,
) -> bool {
    !q.is_empty()
}

/// Similar to Bevy's PipedSystem, but with diverging paths for Ok/Err Results
///
/// The main system will run, returning a `Result`, and then one of two other systems
/// will be run, depending on whether the result was Ok or Err. The data returned
/// in the respective enum variant will be piped as an input.
///
/// The output value from this system is to be returned from either of the two
/// secondary systems, whichever runs.
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

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
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

    fn is_exclusive(&self) -> bool {
        self.system_in.is_send() || self.system_ok.is_send() || self.system_err.is_send()
    }

    fn has_deferred(&self) -> bool {
        self.system_in.has_deferred() || self.system_ok.has_deferred() || self.system_err.has_deferred()
    }

    unsafe fn run_unsafe(&mut self, input: Self::In, world: UnsafeWorldCell) -> Self::Out {
        match self.system_in.run_unsafe(input, world) {
            Ok(t) => self.system_ok.run_unsafe(t, world),
            Err(e) => self.system_err.run_unsafe(e, world),
        }
    }

    fn run(&mut self, input: Self::In, world: &mut World) -> Self::Out {
        match self.system_in.run(input, world) {
            Ok(t) => self.system_ok.run(t, world),
            Err(e) => self.system_err.run(e, world),
        }
    }

    fn apply_deferred(&mut self, world: &mut World) {
        self.system_in.apply_deferred(world);
        self.system_ok.apply_deferred(world);
        self.system_err.apply_deferred(world);
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

    fn update_archetype_component_access(&mut self, world: UnsafeWorldCell) {
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

    fn check_change_tick(&mut self, change_tick: Tick) {
        self.system_in.check_change_tick(change_tick);
        self.system_ok.check_change_tick(change_tick);
        self.system_err.check_change_tick(change_tick);
    }

    fn get_last_run(&self) -> Tick {
        self.system_in.get_last_run()
    }

    fn set_last_run(&mut self, last_run: Tick) {
        self.system_in.set_last_run(last_run);
        self.system_ok.set_last_run(last_run);
        self.system_err.set_last_run(last_run);
    }

    fn default_system_sets(&self) -> Vec<Interned<dyn SystemSet>> {
        let mut default_sets = self.system_in.default_system_sets();
        default_sets.append(&mut self.system_ok.default_system_sets());
        default_sets.append(&mut self.system_err.default_system_sets());
        default_sets
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

/// Similar to Bevy's PipedSystem, but the second system is optional
///
/// The main system will run, returning an `Option`, and then, if the
/// value is Some, the secondary system will run. The value will
/// be piped in as an input.
///
/// The output value of this system is produced by the secondary system,
/// or the default value if the primary system returns None.
pub struct ChainOptionalSystem<SystemIn, SystemSome> {
    system_in: SystemIn,
    system_some: SystemSome,
    name: Cow<'static, str>,
    component_access: Access<ComponentId>,
    archetype_component_access: Access<ArchetypeComponentId>,
}

impl<T, O: Default, SystemIn: System<Out = Option<T>>, SystemSome: System<In = T, Out = O>> System for ChainOptionalSystem<SystemIn, SystemSome> {
    type In = SystemIn::In;
    type Out = O;

    fn name(&self) -> Cow<'static, str> {
        self.name.clone()
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    fn archetype_component_access(&self) -> &Access<ArchetypeComponentId> {
        &self.archetype_component_access
    }

    fn component_access(&self) -> &Access<ComponentId> {
        &self.component_access
    }

    fn is_send(&self) -> bool {
        self.system_in.is_send() && self.system_some.is_send()
    }

    fn is_exclusive(&self) -> bool {
        self.system_in.is_send() || self.system_some.is_send()
    }

    fn has_deferred(&self) -> bool {
        self.system_in.has_deferred() || self.system_some.has_deferred()
    }

    unsafe fn run_unsafe(&mut self, input: Self::In, world: UnsafeWorldCell) -> Self::Out {
        if let Some(t) = self.system_in.run_unsafe(input, world) {
            self.system_some.run_unsafe(t, world)
        } else {
            O::default()
        }
    }

    fn apply_deferred(&mut self, world: &mut World) {
        self.system_in.apply_deferred(world);
        self.system_some.apply_deferred(world);
    }

    fn initialize(&mut self, world: &mut World) {
        self.system_in.initialize(world);
        self.system_some.initialize(world);
        self.component_access
            .extend(self.system_in.component_access());
        self.component_access
            .extend(self.system_some.component_access());
    }

    fn update_archetype_component_access(&mut self, world: UnsafeWorldCell) {
        self.system_in.update_archetype_component_access(world);
        self.system_some.update_archetype_component_access(world);
        self.archetype_component_access
            .extend(self.system_in.archetype_component_access());
        self.archetype_component_access
            .extend(self.system_some.archetype_component_access());
    }

    fn check_change_tick(&mut self, change_tick: Tick) {
        self.system_in.check_change_tick(change_tick);
        self.system_some.check_change_tick(change_tick);
    }

    fn get_last_run(&self) -> Tick {
        self.system_in.get_last_run()
    }

    fn set_last_run(&mut self, last_run: Tick) {
        self.system_in.set_last_run(last_run);
        self.system_some.set_last_run(last_run);
    }

    fn default_system_sets(&self) -> Vec<Interned<dyn SystemSet>> {
        let mut default_sets = self.system_in.default_system_sets();
        default_sets.append(&mut self.system_some.default_system_sets());
        default_sets
    }
}

pub trait IntoChainOptionalSystem<T, Out, SysSome, ParamIn, ParamSome>:
    IntoSystem<(), Option<T>, ParamIn> + Sized
where
    SysSome: IntoSystem<T, Out, ParamSome>,
{
    fn chain_optional(self, system: SysSome) -> ChainOptionalSystem<Self::System, SysSome::System>;
}

impl<T, Out, SysIn, SysSome, ParamIn, ParamSome>
    IntoChainOptionalSystem<T, Out, SysSome, ParamIn, ParamSome> for SysIn
where
    SysIn: IntoSystem<(), Option<T>, ParamIn>,
    SysSome: IntoSystem<T, Out, ParamSome>,
{
    fn chain_optional(self, system: SysSome) -> ChainOptionalSystem<SysIn::System, SysSome::System> {
        let system_in = IntoSystem::into_system(self);
        let system_some = IntoSystem::into_system(system);

        ChainOptionalSystem {
            name: Cow::Owned(format!("ChainOptional({} -> {})", system_in.name(), system_some.name())),
            system_in,
            system_some,
            archetype_component_access: Default::default(),
            component_access: Default::default(),
        }
    }
}
