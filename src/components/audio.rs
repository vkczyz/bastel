use crate::components::Component;

#[allow(dead_code)]
pub struct AudioComponent {
    pub muted: bool,
    pub bgm: Option<String>,
    pub sfx: Option<String>,
}

impl AudioComponent {
    pub fn new() -> Component {
        Component::Audio(
            AudioComponent {
                muted: false,
                bgm: None,
                sfx: None,
            }
        )
    }
}