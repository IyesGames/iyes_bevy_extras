use bevy::{ecs::system::{ReadOnlySystemParam, StaticSystemParam, SystemParam}, prelude::*, tasks::{block_on, poll_once, Task}};
#[cfg(feature = "iyes_progress")]
use iyes_progress::prelude::*;

pub trait AppTaskHelperExt {
    fn add_async_task_helper<T: TaskHelper>(&mut self);
    fn add_async_task_helper_to_state<T: TaskHelper, S: States>(&mut self, state: S);
    #[cfg(feature = "iyes_progress")]
    fn add_async_task_helper_progress<T: TaskHelper, S: States>(&mut self, state: S);
}

impl AppTaskHelperExt for App {
    fn add_async_task_helper<T: TaskHelper>(&mut self) {
        self.add_systems(Update, (
            handle_start_tasks::<T>
                .run_if(rc_handle_start_tasks::<T>),
            handle_task_outputs::<T>
                .run_if(any_with_component::<AwaitingTask<T>>),
        ));
    }
    fn add_async_task_helper_to_state<T: TaskHelper, S: States>(&mut self, state: S) {
        self.add_systems(OnEnter(state.clone()), reset_task_helper_state::<T>);
        self.add_systems(Update, (
            handle_start_tasks::<T>
                .run_if(rc_handle_start_tasks::<T>),
            handle_task_outputs::<T>
                .run_if(any_with_component::<AwaitingTask<T>>),
        ).run_if(in_state(state.clone())));
    }
    #[cfg(feature = "iyes_progress")]
    fn add_async_task_helper_progress<T: TaskHelper, S: States>(&mut self, state: S) {
        self.add_systems(OnEnter(state.clone()), reset_task_helper_state::<T>);
        self.add_systems(Update, (
            handle_start_tasks::<T>
                .run_if(rc_handle_start_tasks::<T>),
            handle_task_outputs_progress::<T>
                .track_progress(),
        ).run_if(in_state(state.clone())));
    }
}

pub trait TaskHelper: Send + Sync + Sized + 'static {
    type Output: Send + Sync + Sized + 'static;
    type InputRcParam: ReadOnlySystemParam + 'static;
    type InputParam: SystemParam + 'static;
    type OutputParam: SystemParam + 'static;

    fn rc_start_tasks(
        &self,
        _param: &<Self::InputRcParam as SystemParam>::Item<'_, '_>,
    ) -> bool {
        true
    }

    fn start_tasks(
        &mut self,
        param: &mut <Self::InputParam as SystemParam>::Item<'_, '_>,
    ) -> impl Iterator<Item = Task<Self::Output>> + Send + Sync + 'static;

    fn handle_output(
        &mut self,
        param: &mut <Self::OutputParam as SystemParam>::Item<'_, '_>,
        out: Self::Output
    );
}

#[derive(Resource)]
pub struct TaskHelperState<T: TaskHelper>{
    t: T,
    #[cfg(feature = "iyes_progress")]
    progress: Progress,
}

#[derive(Component)]
struct AwaitingTask<T: TaskHelper> {
    task: Task<T::Output>,
}

#[allow(private_interfaces)]
pub fn reset_task_helper_state<T: TaskHelper>(
    mut commands: Commands,
    q_task: Query<Entity, With<AwaitingTask<T>>>,
    #[cfg(feature = "iyes_progress")]
    mut res: ResMut<TaskHelperState<T>>,
) {
    q_task.iter().for_each(|e| commands.entity(e).despawn());
    #[cfg(feature = "iyes_progress")]
    {
        res.progress.done = 0;
        res.progress.total = 0;
    }
}

fn rc_handle_start_tasks<T: TaskHelper>(
    res: Res<TaskHelperState<T>>,
    ssp: StaticSystemParam<T::InputRcParam>,
) -> bool {
    let ssp = ssp.into_inner();
    res.t.rc_start_tasks(&ssp)
}

#[allow(private_interfaces)]
pub fn handle_start_tasks<T: TaskHelper>(
    mut commands: Commands,
    mut res: ResMut<TaskHelperState<T>>,
    ssp: StaticSystemParam<T::InputParam>,
) {
    let mut ssp = ssp.into_inner();
    for task in res.t.start_tasks(&mut ssp) {
        commands.spawn(AwaitingTask::<T> { task });
        #[cfg(feature = "iyes_progress")]
        {
            res.progress.total += 1;
        }
    }
}

#[allow(private_interfaces)]
pub fn handle_task_outputs<T: TaskHelper>(
    mut commands: Commands,
    mut res: ResMut<TaskHelperState<T>>,
    mut q_task: Query<(Entity, &mut AwaitingTask<T>)>,
    ssp: StaticSystemParam<T::OutputParam>,
) {
    let mut ssp = ssp.into_inner();
    for (e, mut t) in &mut q_task {
        if let Some(out) = block_on(poll_once(&mut t.task)) {
            res.t.handle_output(&mut ssp, out);
            commands.entity(e).despawn();
        }
    }
}

#[cfg(feature = "iyes_progress")]
#[allow(private_interfaces)]
pub fn handle_task_outputs_progress<T: TaskHelper>(
    mut commands: Commands,
    mut res: ResMut<TaskHelperState<T>>,
    mut q_task: Query<(Entity, &mut AwaitingTask<T>)>,
    ssp: StaticSystemParam<T::OutputParam>,
) -> Progress {
    let mut ssp = ssp.into_inner();
    for (e, mut t) in &mut q_task {
        if let Some(out) = block_on(poll_once(&mut t.task)) {
            res.t.handle_output(&mut ssp, out);
            commands.entity(e).despawn();
            #[cfg(feature = "iyes_progress")]
            {
                res.progress.done += 1;
            }
        }
    }
    Progress {
        done: res.progress.done,
        total: res.progress.total.max(1),
    }
}
