use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

/// Abstraction for organizing system dependencies around system sets.
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

pub trait SetStageAppExt {
    fn configure_stage_set<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
        rc_changed: impl Condition<M>,
    ) -> &mut Self;
    fn configure_stage_set_no_rc<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
    ) -> &mut Self;
}

impl SetStageAppExt for App {
    fn configure_stage_set<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet, M>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
        rc_changed: impl Condition<M>,
    ) -> &mut Self {
        self.configure_sets(
            schedule,
            (
                SetStage::Prepare(t).before(SetStage::Provide(t)),
                SetStage::Want(t).after(SetStage::Provide(t)),
                SetStage::WantChanged(t)
                    .in_set(SetStage::Want(t))
                    .run_if(rc_changed),
            ),
        );
        self
    }

    fn configure_stage_set_no_rc<T: Debug + PartialEq + Eq + Clone + Copy + Hash + SystemSet>(
        &mut self,
        schedule: impl ScheduleLabel,
        t: T,
    ) -> &mut Self {
        self.configure_sets(
            schedule,
            (
                SetStage::Prepare(t).before(SetStage::Provide(t)),
                SetStage::Want(t).after(SetStage::Provide(t)),
                SetStage::WantChanged(t).in_set(SetStage::Want(t)),
            ),
        );
        self
    }
}
