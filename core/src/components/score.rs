use specs::Component;
use specs::VecStorage;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Score {
    pub position: Position,
}

pub enum Position {
    One,
    Ten,
    Hundred,
    Thousand,
    TenThousand,
    HundredThousand,
}
