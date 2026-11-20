use glam::{EulerRot, Mat4, Quat, Vec3};

use rbx_dom_weak::types::Variant as DomValue;

use crate::datatypes::types::{CFrame, Vector3};

use super::Instance;

pub const CLASS_NAME: &str = "BasePart";

const PROPERTY_NAME_CFRAME: &str = "CFrame";

/**
    A property of a `BasePart` that is another view of its `CFrame`.

    Roblox saves only the `CFrame`, so these are read from it and written through it.

    ### See Also
    * [`Position`](https://create.roblox.com/docs/reference/engine/classes/BasePart#Position)
      on the Roblox Developer Hub
    * [`Orientation`](https://create.roblox.com/docs/reference/engine/classes/BasePart#Orientation)
      on the Roblox Developer Hub
    * [`Rotation`](https://create.roblox.com/docs/reference/engine/classes/BasePart#Rotation)
      on the Roblox Developer Hub
*/
#[derive(Debug, Clone, Copy)]
pub(super) enum CFrameView {
    Position,
    Orientation,
    Rotation,
}

impl CFrameView {
    pub(super) fn find(this: &Instance, prop_name: &str) -> Option<Self> {
        let view = match prop_name {
            "Position" => Self::Position,
            "Orientation" => Self::Orientation,
            "Rotation" => Self::Rotation,
            _ => return None,
        };
        this.is_a(CLASS_NAME).then_some(view)
    }

    pub(super) fn get(self, this: &Instance) -> Vector3 {
        let cframe = get_cframe(this);
        let rotation = Quat::from_mat4(&cframe);
        Vector3(match self {
            Self::Position => cframe.w_axis.truncate(),
            Self::Orientation => {
                let (ry, rx, rz) = rotation.to_euler(EulerRot::YXZ);
                Vec3::new(rx, ry, rz).map(f32::to_degrees)
            }
            Self::Rotation => {
                let (rx, ry, rz) = rotation.to_euler(EulerRot::XYZ);
                Vec3::new(rx, ry, rz).map(f32::to_degrees)
            }
        })
    }

    pub(super) fn set(self, this: &Instance, value: Vector3) {
        let cframe = get_cframe(this);
        let radians = value.0.map(f32::to_radians);
        let (rotation, position) = match self {
            Self::Position => (cframe, value.0.extend(1.0)),
            Self::Orientation => (
                Mat4::from_euler(EulerRot::YXZ, radians.y, radians.x, radians.z),
                cframe.w_axis,
            ),
            Self::Rotation => (
                Mat4::from_euler(EulerRot::XYZ, radians.x, radians.y, radians.z),
                cframe.w_axis,
            ),
        };
        let updated = Mat4::from_cols(rotation.x_axis, rotation.y_axis, rotation.z_axis, position);
        this.set_property(
            PROPERTY_NAME_CFRAME,
            DomValue::CFrame(CFrame(updated).into()),
        );
    }
}

fn get_cframe(this: &Instance) -> Mat4 {
    match this.get_property(PROPERTY_NAME_CFRAME) {
        Some(DomValue::CFrame(cframe)) => CFrame::from(cframe).0,
        _ => Mat4::IDENTITY,
    }
}
