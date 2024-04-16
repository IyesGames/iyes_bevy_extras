use bevy::prelude::*;

use bevy::ecs::schedule::ScheduleLabel;

use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, SystemSet)]
pub enum SetStage<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet> {
    /// Anything that runs before and may affect operation
    Prepare(T),
    /// The "Producers", the meat of the logic
    Provide(T),
    /// The "Consumers", anything that depends on / wants whatever Provide does
    Want(T),
    /// Same as Want, but with RCs to run only if anything notable happened
    WantChanged(T),
}

impl<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet> SetStage<T> {
    pub fn configure_sets<M>(
        app: &mut App,
        schedule: impl ScheduleLabel,
        t: T,
        rc_changed: impl Condition<M>,
    ) {
        app.configure_sets(schedule, (
            Self::Prepare(t).before(Self::Provide(t)),
            Self::Want(t).after(Self::Provide(t)),
            Self::WantChanged(t)
                .in_set(Self::Want(t))
                .run_if(rc_changed),
        ));
    }
}

pub trait SetStageAppExt {
    fn configure_stage_set<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
        rc_changed: impl Condition<M>,
    ) -> &mut Self;
}

impl SetStageAppExt for App {
    fn configure_stage_set<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
        rc_changed: impl Condition<M>,
    ) -> &mut Self{
        SetStage::configure_sets(self, schedule, t, rc_changed);
        self
    }
}
