use crate::{key_codes::KeyCode, NUM_COLS, NUM_ROWS};

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [ KeyCode:: Escape, KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::D ],
    [ KeyCode::E ,KeyCode::F ,KeyCode::G ,KeyCode::H ,KeyCode::I ],
    [ KeyCode::J ,KeyCode::K ,KeyCode::L ,KeyCode::M ,KeyCode::N ],
    [ KeyCode::O ,KeyCode::P ,KeyCode::Q ,KeyCode::R ,KeyCode::S ],
    [ KeyCode::T ,KeyCode::U ,KeyCode::V ,KeyCode::W ,KeyCode::X ]
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KeyCode; NUM_ROWS]; NUM_COLS] = [
    [ KeyCode:: Escape, KeyCode::A, KeyCode::B, KeyCode::C, KeyCode::D ],
    [ KeyCode::E ,KeyCode::F ,KeyCode::G ,KeyCode::H ,KeyCode::I ],
    [ KeyCode::J ,KeyCode::K ,KeyCode::L ,KeyCode::M ,KeyCode::N ],
    [ KeyCode::O ,KeyCode::P ,KeyCode::Q ,KeyCode::R ,KeyCode::S ],
    [ KeyCode::T ,KeyCode::U ,KeyCode::V ,KeyCode::W ,KeyCode::X ]
];
