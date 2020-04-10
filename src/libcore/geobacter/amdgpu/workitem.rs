use crate::geobacter::intrinsics::*;
use super::{DispatchPacket, ensure_amdgpu};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct XAxis;
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct YAxis;
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ZAxis;

pub trait WorkItemAxis {
    fn workitem_id(&self) -> u32;
}
impl WorkItemAxis for Axis {
    #[inline(always)]
    fn workitem_id(&self) -> u32 {
        match self {
            &Axis::X => XAxis.workitem_id(),
            &Axis::Y => YAxis.workitem_id(),
            &Axis::Z => ZAxis.workitem_id(),
        }
    }
}
impl WorkItemAxis for XAxis {
    #[inline(always)]
    fn workitem_id(&self) -> u32 {
        ensure_amdgpu("workitem_x_id");
        unsafe { geobacter_amdgpu_workitem_x_id() as _ }
    }
}
impl WorkItemAxis for YAxis {
    #[inline(always)]
    fn workitem_id(&self) -> u32 {
        ensure_amdgpu("workitem_y_id");
        unsafe { geobacter_amdgpu_workitem_y_id() as _ }
    }
}
impl WorkItemAxis for ZAxis {
    #[inline(always)]
    fn workitem_id(&self) -> u32 {
        ensure_amdgpu("workitem_z_id");
        unsafe { geobacter_amdgpu_workitem_z_id() as _ }
    }
}

pub trait WorkGroupAxis {
    fn workgroup_id(&self) -> u32;
    fn workgroup_size(&self, p: &DispatchPacket) -> u32;
}
impl WorkGroupAxis for Axis {
    #[inline(always)]
    fn workgroup_id(&self) -> u32 {
        match self {
            &Axis::X => XAxis.workgroup_id(),
            &Axis::Y => YAxis.workgroup_id(),
            &Axis::Z => ZAxis.workgroup_id(),
        }
    }
    #[inline(always)]
    fn workgroup_size(&self, p: &DispatchPacket) -> u32 {
        match self {
            &Axis::X => XAxis.workgroup_size(p),
            &Axis::Y => YAxis.workgroup_size(p),
            &Axis::Z => ZAxis.workgroup_size(p),
        }
    }
}
impl WorkGroupAxis for XAxis {
    #[inline(always)]
    fn workgroup_id(&self) -> u32 {
        ensure_amdgpu("workgroup_x_id");
        unsafe { geobacter_amdgpu_workgroup_x_id() as _ }
    }
    #[inline(always)]
    fn workgroup_size(&self, p: &DispatchPacket) -> u32 {
        p.workgroup_size_x as _
    }
}
impl WorkGroupAxis for YAxis {
    #[inline(always)]
    fn workgroup_id(&self) -> u32 {
        ensure_amdgpu("workgroup_y_id");
        unsafe { geobacter_amdgpu_workgroup_y_id() as _ }
    }
    #[inline(always)]
    fn workgroup_size(&self, p: &DispatchPacket) -> u32 {
        p.workgroup_size_y as _
    }
}
impl WorkGroupAxis for ZAxis {
    #[inline(always)]
    fn workgroup_id(&self) -> u32 {
        ensure_amdgpu("workgroup_z_id");
        unsafe { geobacter_amdgpu_workgroup_z_id() as _ }
    }
    #[inline(always)]
    fn workgroup_size(&self, p: &DispatchPacket) -> u32 {
        p.workgroup_size_z as _
    }
}
pub trait GridAxis {
    fn grid_size(&self, p: &DispatchPacket) -> u32;
}
impl GridAxis for Axis {
    #[inline(always)]
    fn grid_size(&self, p: &DispatchPacket) -> u32 {
        match self {
            &Axis::X => XAxis.grid_size(p),
            &Axis::Y => YAxis.grid_size(p),
            &Axis::Z => ZAxis.grid_size(p),
        }
    }
}
impl GridAxis for XAxis {
    #[inline(always)]
    fn grid_size(&self, p: &DispatchPacket) -> u32 {
        p.grid_size_x
    }
}
impl GridAxis for YAxis {
    #[inline(always)]
    fn grid_size(&self, p: &DispatchPacket) -> u32 {
        p.grid_size_y
    }
}
impl GridAxis for ZAxis {
    #[inline(always)]
    fn grid_size(&self, p: &DispatchPacket) -> u32 {
        p.grid_size_z
    }
}

#[inline(always)]
pub fn workitem_ids() -> [u32; 3] {
    [
        XAxis.workitem_id(),
        YAxis.workitem_id(),
        ZAxis.workitem_id(),
    ]
}
#[inline(always)]
pub fn workgroup_ids() -> [u32; 3] {
    [
        XAxis.workgroup_id(),
        YAxis.workgroup_id(),
        ZAxis.workgroup_id(),
    ]
}

impl DispatchPacket {
    #[inline(always)]
    pub fn workgroup_sizes(&self) -> [u32; 3] {
        [
            XAxis.workgroup_size(self),
            YAxis.workgroup_size(self),
            ZAxis.workgroup_size(self),
        ]
    }
    #[inline(always)]
    pub fn grid_sizes(&self) -> [u32; 3] {
        [
            XAxis.grid_size(self),
            YAxis.grid_size(self),
            ZAxis.grid_size(self),
        ]
    }
    #[inline(always)]
    pub fn global_linear_id(&self) -> usize {
        let [l0, l1, l2] = workitem_ids();
        let [g0, g1, g2] = workgroup_ids();
        let [s0, s1, s2] = self.workgroup_sizes();
        let [n0, n1, _n2] = self.grid_sizes();

        let n0 = n0 as usize;
        let n1 = n1 as usize;

        let i0 = (g0 * s0 + l0) as usize;
        let i1 = (g1 * s1 + l1) as usize;
        let i2 = (g2 * s2 + l2) as usize;
        (i2 * n1 + i1) * n0 + i0
    }
    #[inline(always)]
    pub fn global_id_x(&self) -> u32 {
        self.global_id(XAxis)
    }
    #[inline(always)]
    pub fn global_id_y(&self) -> u32 {
        self.global_id(YAxis)
    }
    #[inline(always)]
    pub fn global_id_z(&self) -> u32 {
        self.global_id(ZAxis)
    }
    #[inline(always)]
    pub fn global_id<T>(&self, axis: T) -> u32
        where T: WorkItemAxis + WorkGroupAxis,
    {
        let l = axis.workitem_id();
        let g = axis.workgroup_id();
        let s = axis.workgroup_size(self);
        g * s + l
    }
    #[inline(always)]
    pub fn global_ids(&self) -> (u32, u32, u32) {
        (self.global_id_x(), self.global_id_y(), self.global_id_z())
    }
}

pub trait ReadFirstLane {
    unsafe fn read_first_lane(self) -> Self;
}
macro_rules! read_first_lane_sprim {
  ($($prim:ty, )*) => {$(
    impl ReadFirstLane for $prim {
      unsafe fn read_first_lane(self) -> Self {
        geobacter_amdgpu_readfirstlane(self as _) as _
      }
    }
  )*}
}
read_first_lane_sprim!(i8, u8, i16, u16, i32, u32, );

#[cfg(target_pointer_width = "32")]
read_first_lane_sprim!(usize, isize, );

macro_rules! read_first_lane_x64 {
  ($($prim:ty, )*) => {$(
    impl ReadFirstLane for $prim {
      unsafe fn read_first_lane(self) -> Self {
        let h1 = (self & (0xffffffff << 32)) >> 32;
        let h2 = self & 0xffffffff;

        let h1 = geobacter_amdgpu_readfirstlane(h1 as _) as Self;
        let h2 = geobacter_amdgpu_readfirstlane(h2 as _) as Self;

        (h1 << 32) | h2
      }
    }
  )*}
}
read_first_lane_x64!(i64, u64, );
#[cfg(target_pointer_width = "64")]
read_first_lane_x64!(usize, isize, );
