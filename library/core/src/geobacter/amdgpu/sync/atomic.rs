use crate::geobacter::intrinsics::*;
use crate::sync::atomic::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Scope {
    WorkItem,
    SubGroup,
    WorkGroup,
    Device,
    System,
}

/// XXX "work_item"??
#[inline(always)]
fn atomic_work_item_fence(order: Ordering, scope: Scope) {
    match (scope, order) {
        (Scope::WorkItem, Ordering::Release) => unsafe {
            atomic_scoped_fence_singlethread_rel()
        },
        (Scope::WorkItem, Ordering::Acquire) => unsafe {
            atomic_scoped_fence_singlethread_acq()
        },
        (Scope::WorkItem, Ordering::AcqRel) => unsafe {
            atomic_scoped_fence_singlethread_acqrel()
        },
        (Scope::WorkItem, Ordering::SeqCst) => unsafe {
            atomic_scoped_fence_singlethread_seqcst()
        },
        (Scope::WorkItem, _) => {
            // non-exhaustive?? Okay...
        }

        (Scope::SubGroup, Ordering::Release) => unsafe {
            atomic_scoped_fence_wavefront_rel()
        },
        (Scope::SubGroup, Ordering::Acquire) => unsafe {
            atomic_scoped_fence_wavefront_acq()
        },
        (Scope::SubGroup, Ordering::AcqRel) => unsafe {
            atomic_scoped_fence_wavefront_acqrel()
        },
        (Scope::SubGroup, Ordering::SeqCst) => unsafe {
            atomic_scoped_fence_wavefront_seqcst()
        },
        (Scope::SubGroup, _) => { },

        (Scope::WorkGroup, Ordering::Release) => unsafe {
            atomic_scoped_fence_workgroup_rel()
        },
        (Scope::WorkGroup, Ordering::Acquire) => unsafe {
            atomic_scoped_fence_workgroup_acq()
        },
        (Scope::WorkGroup, Ordering::AcqRel) => unsafe {
            atomic_scoped_fence_workgroup_acqrel()
        },
        (Scope::WorkGroup, Ordering::SeqCst) => unsafe {
            atomic_scoped_fence_workgroup_seqcst()
        },
        (Scope::WorkGroup, _) => { },

        (Scope::Device, Ordering::Release) => unsafe {
            atomic_scoped_fence_agent_rel()
        },
        (Scope::Device, Ordering::Acquire) => unsafe {
            atomic_scoped_fence_agent_acq()
        },
        (Scope::Device, Ordering::AcqRel) => unsafe {
            atomic_scoped_fence_agent_acqrel()
        },
        (Scope::Device, Ordering::SeqCst) => unsafe {
            atomic_scoped_fence_agent_seqcst()
        },
        (Scope::Device, _) => { },

        (_, Ordering::Relaxed) => {
            // don't panic
        },
        (Scope::System, order) => {
            crate::sync::atomic::fence(order)
        },
    }
}

#[inline(always)]
pub fn work_group_barrier(scope: Scope, acquire: Ordering,
                          release: Ordering)
{
    atomic_work_item_fence(acquire, scope);
    super::workgroup_barrier();
    atomic_work_item_fence(release, scope)
}
#[inline(always)]
pub fn work_group_rel_acq_barrier(scope: Scope) {
    work_group_barrier(scope, Ordering::Release,
                       Ordering::Acquire);
}
