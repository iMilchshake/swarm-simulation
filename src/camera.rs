use glam::Vec2;
use macroquad::camera::Camera2D;
use macroquad::input::mouse_position;
use macroquad::math::{Rect, Vec2 as MqVec2};
use macroquad::window::{screen_height, screen_width};

const ZOOM_FACTOR: f32 = 0.9;

/// A zoomable and pannable camera for viewing a fixed-size map.
pub struct MapCamera {
    offset: Vec2,
    zoom: f32,
    map_size: Vec2,
    viewport: Option<Viewport>,
    cam: Option<Camera2D>,
}

struct Viewport {
    size: Vec2,
    ratio: Vec2,
    x_offset: f32,
    y_offset: f32,
}

impl MapCamera {
    /// Create a new camera for a map of the given size.
    pub fn new(map_width: f32, map_height: f32) -> Self {
        MapCamera {
            offset: Vec2::ZERO,
            zoom: 1.0,
            map_size: Vec2::new(map_width, map_height),
            viewport: None,
            cam: None,
        }
    }

    /// Update the map size.
    pub fn set_map_size(&mut self, width: f32, height: f32) {
        self.map_size = Vec2::new(width, height);
    }

    /// Set a custom viewport (useful when you have UI panels).
    /// x_offset: left edge of viewport
    /// y_offset: top edge of viewport (for top menu bars)
    /// width: viewport width
    /// height: viewport height
    pub fn set_viewport(&mut self, x_offset: f32, y_offset: f32, width: f32, height: f32) {
        let size = Vec2::new(width, height);
        self.viewport = Some(Viewport {
            size,
            ratio: Vec2::new(width / screen_width(), height / screen_height()),
            x_offset,
            y_offset,
        });
    }

    /// Clear the custom viewport (use fullscreen).
    pub fn clear_viewport(&mut self) {
        self.viewport = None;
    }

    /// Reset camera transformations (zoom and pan).
    pub fn reset(&mut self) {
        self.offset = Vec2::ZERO;
        self.zoom = 1.0;
    }

    /// Get current zoom level.
    pub fn zoom_level(&self) -> f32 {
        self.zoom
    }

    /// Zoom in by the constant zoom factor.
    pub fn zoom_in(&mut self) {
        self.zoom /= ZOOM_FACTOR;
    }

    /// Zoom out by the constant zoom factor.
    pub fn zoom_out(&mut self) {
        self.zoom *= ZOOM_FACTOR;
    }

    /// Zoom in or out by a factor (> 1.0 zooms in, < 1.0 zooms out).
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom *= factor;
    }

    /// Shift the camera by a delta in screen-normalized coordinates [-1, +1].
    /// If a viewport is used, it will be scaled accordingly.
    pub fn shift(&mut self, local_shift: Vec2) {
        let viewport_ratio = self
            .viewport
            .as_ref()
            .map(|v| v.ratio)
            .unwrap_or(Vec2::ONE);
        let scaled_shift = local_shift / viewport_ratio;
        self.offset += scaled_shift / self.zoom;
    }

    /// Shift the camera by a delta in pixels.
    pub fn shift_pixels(&mut self, pixel_delta: Vec2) {
        let screen_size = self
            .viewport
            .as_ref()
            .map(|v| v.size)
            .unwrap_or(Vec2::new(screen_width(), screen_height()));
        // Convert pixel delta to normalized [-1, +1] range
        let normalized = pixel_delta * 2.0 / screen_size;
        self.shift(normalized);
    }

    /// Apply the camera transformation to macroquad.
    /// Call this before drawing map content.
    pub fn apply(&mut self) {
        let viewport = self
            .viewport
            .as_ref()
            .map(|v| v.size)
            .unwrap_or(Vec2::new(screen_width(), screen_height()));

        // Calculate aspect ratio
        let viewport_ratio = viewport.x / viewport.y;
        let map_ratio = self.map_size.x / self.map_size.y;
        let (cam_width, cam_height) = if viewport_ratio > map_ratio {
            (
                self.map_size.x * viewport_ratio / map_ratio,
                self.map_size.y,
            )
        } else {
            (
                self.map_size.x,
                self.map_size.y * map_ratio / viewport_ratio,
            )
        };

        // Create camera rect
        let mut cam = Camera2D::from_display_rect(Rect::new(0.0, 0.0, cam_width, cam_height));

        // Apply user transformations
        cam.target = MqVec2::new(
            (self.offset.x / cam.zoom.x) + (cam_width / 2.),
            (-self.offset.y / cam.zoom.y) + (cam_height / 2.),
        );
        cam.zoom *= self.zoom;
        cam.zoom.y *= -1.0; // Flip Y axis for macroquad 0.4 camera consistency

        // Set viewport if defined
        if let Some(vp) = &self.viewport {
            cam.viewport = Some((
                vp.x_offset as i32,
                vp.y_offset as i32,
                vp.size.x as i32,
                vp.size.y as i32,
            ));
        }

        macroquad::camera::set_camera(&cam);
        self.cam = Some(cam);
    }

    /// Reset to default camera (no transformations).
    /// Call this after drawing map content if you need to draw UI.
    pub fn reset_camera(&self) {
        macroquad::camera::set_default_camera();
    }

    /// Get the underlying macroquad Camera2D (panics if apply() wasn't called).
    pub fn get_camera(&self) -> &Camera2D {
        self.cam.as_ref().expect("camera not applied yet")
    }

    /// Convert screen mouse position to map coordinates.
    pub fn screen_to_map(&self, screen_pos: Vec2) -> Vec2 {
        let cam = self.cam.as_ref().expect("camera not applied yet");
        let world = cam.screen_to_world(MqVec2::new(screen_pos.x, screen_pos.y));
        Vec2::new(world.x, world.y)
    }

    /// Get the current mouse position in map coordinates.
    pub fn mouse_map_pos(&self) -> Vec2 {
        let (mouse_x, mouse_y) = mouse_position();
        self.screen_to_map(Vec2::new(mouse_x, mouse_y))
    }
}
