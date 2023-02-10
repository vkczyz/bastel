use crate::components::Component;
use roxmltree;

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

    pub fn from_xml(data: roxmltree::Node) -> Component {
        let mut muted = false;
        let mut bgm = None;

        data.children()
            .filter(|n| n.is_element())
            .map(|n| {
                match n.tag_name().name() {
                    "muted" => muted = true,
                    "bgm" => match n.attribute("bgm") {
                        Some(b) => bgm = Some(String::from(b)),
                        _ => (),
                    },
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