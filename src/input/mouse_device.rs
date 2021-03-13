use autopilot::mouse;
use autopilot::screen::size as screen_size;

use tracing::warn;

use crate::input::device::InputDevice;
use crate::protocol::Button;
use crate::protocol::PointerEvent;
use crate::protocol::PointerEventType;

#[cfg(target_os = "linux")]
use crate::x11helper::Capturable;

pub struct Mouse {
    #[cfg(target_os = "linux")]
    capture: Capturable,
}

#[cfg(target_os = "linux")]
impl Mouse {
    pub fn new(capture: Capturable) -> Self {
        Self { capture }
    }
}

#[cfg(not(target_os = "linux"))]
impl Mouse {
    pub fn new() -> Self {
        Self {}
    }
}

impl InputDevice for Mouse {
    fn send_event(&mut self, event: &PointerEvent) {
        if !event.is_primary {
            return;
        }
        #[cfg(target_os = "linux")]
        {
            if let Err(err) = self.capture.before_input() {
                warn!("Failed to activate window, sending no input ({})", err);
                return;
            }
            let geometry = self.capture.geometry();
            if let Err(err) = geometry {
                warn!("Failed to get window geometry, sending no input ({})", err);
                return;
            }
            let geometry = geometry.unwrap();
            if let Err(err) = mouse::move_to(autopilot::geometry::Point::new(
                (event.x * geometry.width + geometry.x) * screen_size().width,
                (event.y * geometry.height + geometry.y) * screen_size().height,
            )) {
                warn!("Could not move mouse: {}", err);
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            if let Err(err) = mouse::move_to(autopilot::geometry::Point::new(
                event.x * screen_size().width,
                event.y * screen_size().height,
            )) {
                warn!("Could not move mouse: {}", err);
            }
        }
        match event.event_type {
            PointerEventType::DOWN => mouse::toggle(mouse::Button::Left, true),
            PointerEventType::UP | PointerEventType::CANCEL => {
                mouse::toggle(mouse::Button::Left, false)
            }
            _ => (),
        }
    }
}
