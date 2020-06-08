use crate::graphics::data::TileSheet;

// Tiles should be drawn in a particular order to get the correct Z-index cheaply.
//
// NOTE: Instead of using tile sheet for ordering, we should instead use the
// component type for this purpose. It would be a more flexible way to do it
//
// Until then: gotta make sure that there is no overlap between these orders.
// Otherwise, there is no use in doing this at all!

pub const GAME_PLAY_DRAW_ORDER: [TileSheet; 5] = [
    TileSheet::Cloud,
    TileSheet::Platform,
    TileSheet::Enemy,
    TileSheet::Character,
    TileSheet::Number,
];

pub const INSTRUCTIONS_DRAW_ORDER: [TileSheet; 2] = [TileSheet::Letter, TileSheet::Icon];
