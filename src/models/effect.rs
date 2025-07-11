use serde::{Deserialize, Serialize};

/// Available effects on the device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Effect {
    BrickBreaker,
    PingPong,
    Rainbow,
    ColorWaves,
    TheaterChase,
    Fireworks,
    Matrix,
    SwirlOut,
    SwirlIn,
    Pacifica,
    TwinkleFox,
    PlasmaCloud,
    LookingEyes,
    Ripple,
    Fire,
    HeartBeat,
    Fade,
    None,
}

impl Effect {
    pub fn as_str(&self) -> &'static str {
        match self {
            Effect::BrickBreaker => "BrickBreaker",
            Effect::PingPong => "PingPong",
            Effect::Rainbow => "Rainbow",
            Effect::ColorWaves => "ColorWaves",
            Effect::TheaterChase => "TheaterChase",
            Effect::Fireworks => "Fireworks",
            Effect::Matrix => "Matrix",
            Effect::SwirlOut => "SwirlOut",
            Effect::SwirlIn => "SwirlIn",
            Effect::Pacifica => "Pacifica",
            Effect::TwinkleFox => "TwinkleFox",
            Effect::PlasmaCloud => "PlasmaCloud",
            Effect::LookingEyes => "LookingEyes",
            Effect::Ripple => "Ripple",
            Effect::Fire => "Fire",
            Effect::HeartBeat => "HeartBeat",
            Effect::Fade => "Fade",
            Effect::None => "None",
        }
    }
}

impl std::fmt::Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for Effect {
    fn from(s: String) -> Self {
        match s.as_str() {
            "BrickBreaker" => Effect::BrickBreaker,
            "PingPong" => Effect::PingPong,
            "Rainbow" => Effect::Rainbow,
            "ColorWaves" => Effect::ColorWaves,
            "TheaterChase" => Effect::TheaterChase,
            "Fireworks" => Effect::Fireworks,
            "Matrix" => Effect::Matrix,
            "SwirlOut" => Effect::SwirlOut,
            "SwirlIn" => Effect::SwirlIn,
            "Pacifica" => Effect::Pacifica,
            "TwinkleFox" => Effect::TwinkleFox,
            "PlasmaCloud" => Effect::PlasmaCloud,
            "LookingEyes" => Effect::LookingEyes,
            "Ripple" => Effect::Ripple,
            "Fire" => Effect::Fire,
            "HeartBeat" => Effect::HeartBeat,
            "Fade" => Effect::Fade,
            _ => Effect::None,
        }
    }
}

/// Available transition effects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Transition {
    Slide,
    Dim,
    Zoom,
    Rotate,
    Pixelize,
    Curtain,
    Ripple,
    Blink,
    Reload,
    Fade,
    None,
}

impl Transition {
    pub fn as_str(&self) -> &'static str {
        match self {
            Transition::Slide => "Slide",
            Transition::Dim => "Dim",
            Transition::Zoom => "Zoom",
            Transition::Rotate => "Rotate",
            Transition::Pixelize => "Pixelize",
            Transition::Curtain => "Curtain",
            Transition::Ripple => "Ripple",
            Transition::Blink => "Blink",
            Transition::Reload => "Reload",
            Transition::Fade => "Fade",
            Transition::None => "None",
        }
    }
}

impl std::fmt::Display for Transition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for Transition {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Slide" => Transition::Slide,
            "Dim" => Transition::Dim,
            "Zoom" => Transition::Zoom,
            "Rotate" => Transition::Rotate,
            "Pixelize" => Transition::Pixelize,
            "Curtain" => Transition::Curtain,
            "Ripple" => Transition::Ripple,
            "Blink" => Transition::Blink,
            "Reload" => Transition::Reload,
            "Fade" => Transition::Fade,
            _ => Transition::None,
        }
    }
}