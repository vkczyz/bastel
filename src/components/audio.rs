use crate::components::Component;
use roxmltree;

#[allow(dead_code)]
#[derive(PartialEq)]
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

    pub fn from_xml(data: roxmltree::Node) -> Component {
        let mut muted = false;
        let mut bgm = None;

        data.attributes()
            .map(|a| {
                match a.name() {
                    "muted" => muted = true,
                    "bgm" => bgm = Some(String::from(a.value())),
                    _ => (),
                }
            }
        ).for_each(drop);

        Component::Audio(
            AudioComponent {
                muted,
                bgm,
                sfx: None,
            }
        )
    }
}