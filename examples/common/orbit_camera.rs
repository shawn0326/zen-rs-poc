use dolly::prelude::*;
use glam::{Mat4, Quat, Vec2, Vec3};
use winit::event::{MouseButton, MouseScrollDelta, WindowEvent};
use winit::window::Window;

/// A renderer-agnostic orbit camera controller built on dolly and winit.
/// - Drive with mouse: Left-drag orbit, Middle/Right-drag pan, Wheel zoom.
/// - Call `update` per mouse event; when state changes, your callback is invoked with (view, eye, target).
/// - No dependency on your renderer types.
pub struct OrbitController {
    // Target point the camera orbits around
    focus: Vec3,
    // Spherical rotation around focus (radians)
    yaw: f32,
    pitch: f32,
    // Distance from focus
    radius: f32,

    // Input state
    last_cursor: Option<Vec2>,
    orbiting: bool,
    panning: bool,

    // Tunables
    pub rotate_sensitivity: f32, // radians per pixel
    pub pan_sensitivity: f32,    // world units per pixel at radius=1, scales with radius & fov
    pub zoom_sensitivity: f32,   // scale factor per wheel unit

    // Dolly camera rig to smooth (optional usage)
    rig: CameraRig,
}

impl OrbitController {
    pub fn new(focus: Vec3, radius: f32, yaw_deg: f32, pitch_deg: f32) -> Self {
        let yaw = yaw_deg.to_radians();
        let pitch = pitch_deg.to_radians();
        // Pass glam::Vec3 directly; with glam "mint" feature enabled it implements Into<mint::Point3<f32>> / Into<mint::Vector3<f32>>.
        let rig = CameraRig::builder()
            .with(Position::new(focus))
            .with(
                YawPitch::new()
                    .yaw_degrees(yaw_deg)
                    .pitch_degrees(pitch_deg),
            )
            .with(Arm::new(Vec3::new(0.0, 0.0, radius)))
            .with(LookAt::new(focus))
            .with(Smooth::new_position_rotation(1.0, 1.0))
            .build();

        Self {
            focus,
            yaw,
            pitch,
            radius: radius.max(0.01),
            last_cursor: None,
            orbiting: false,
            panning: false,
            rotate_sensitivity: 0.005,
            pan_sensitivity: 1.0,
            zoom_sensitivity: 0.1,
            rig,
        }
    }

    fn clamp_pitch(&self, pitch: f32) -> f32 {
        let limit = std::f32::consts::FRAC_PI_2 - 0.01;
        pitch.clamp(-limit, limit)
    }

    fn basis_from_angles(yaw: f32, pitch: f32) -> (Vec3, Vec3, Vec3) {
        let rot = Quat::from_euler(glam::EulerRot::YXZ, yaw, pitch, 0.0);
        let fwd = rot * Vec3::Z; // camera looks along +Z arm (eye = focus + +Z*radius), adjust to your convention
        let right = rot * Vec3::X;
        let up = rot * Vec3::Y;
        (right, up, fwd)
    }

    /// Process one winit WindowEvent. If state changed, invoke `apply(view, eye, target)`.
    /// - `viewport` in pixels (width, height)
    /// - `fovy` in radians
    /// - `dt` seconds for smoothing
    pub fn update<F>(
        &mut self,
        _window: &Window,
        event: &WindowEvent,
        viewport: Vec2,
        fovy: f32,
        dt: f32,
        mut apply: F,
    ) where
        F: FnMut(Mat4, Vec3, Vec3),
    {
        let mut changed = false;
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let pos = Vec2::new(position.x as f32, position.y as f32);
                if let Some(last) = self.last_cursor {
                    let delta = pos - last;
                    if self.orbiting {
                        self.yaw -= delta.x * self.rotate_sensitivity;
                        self.pitch =
                            self.clamp_pitch(self.pitch - delta.y * self.rotate_sensitivity);
                        changed = true;
                    } else if self.panning {
                        // screen pixels to world units scaling: proportional to radius and tan(fov/2)
                        let pixels_to_world = (self.radius * (fovy * 0.5).tan()) * 2.0 / viewport.y;
                        let (right, up, _) = Self::basis_from_angles(self.yaw, self.pitch);
                        self.focus += (-delta.x * pixels_to_world * self.pan_sensitivity) * right
                            + (delta.y * pixels_to_world * self.pan_sensitivity) * up;
                        changed = true;
                    }
                }
                self.last_cursor = Some(pos);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = *state == winit::event::ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        self.orbiting = pressed;
                        changed = false; // state toggle only
                    }
                    MouseButton::Middle | MouseButton::Right => {
                        self.panning = pressed;
                        changed = false;
                    }
                    _ => {}
                }
                // Capture the cursor position on press to avoid a large initial jump
                // On press we rely on last_cursor already set by prior CursorMoved; no direct cursor query in winit 0.30.
                // If no movement happened yet, first movement will establish reference.
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let scroll = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => (p.y as f32) / 50.0, // heuristic: 50px ~ 1 line
                };
                // Exponential zoom feels nicer
                let factor = 1.0 - scroll * self.zoom_sensitivity;
                self.radius = (self.radius * factor).max(0.01);
                changed = true;
            }
            WindowEvent::Focused(false) => {
                self.orbiting = false;
                self.panning = false;
            }
            _ => {}
        }

        // Update dolly rig even if not changed to keep smoothing consistent; but only apply output if changed
        self.rig.driver_mut::<Position>().position = self.focus.into();
        {
            let yaw_pitch = self.rig.driver_mut::<YawPitch>();
            // Prefer direct field assignment to avoid move issues on dolly 0.6
            yaw_pitch.yaw_degrees = self.yaw.to_degrees();
            yaw_pitch.pitch_degrees = self.pitch.to_degrees();
        }
        self.rig.driver_mut::<Arm>().offset = Vec3::new(0.0, 0.0, self.radius).into();
        self.rig.driver_mut::<LookAt>().target = self.focus.into();
        self.rig.update(dt);

        if changed {
            // Compose view
            let rot = Quat::from_euler(glam::EulerRot::YXZ, self.yaw, self.pitch, 0.0);
            let eye = self.focus + rot * Vec3::new(0.0, 0.0, self.radius);
            let view = Mat4::look_at_rh(eye, self.focus, Vec3::Y);
            apply(view, eye, self.focus);
        }
    }
}
