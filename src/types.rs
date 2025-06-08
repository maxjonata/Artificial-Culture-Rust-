use strum_macros::EnumIter;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum PhysiologicalNeeds {
    Hunger,
    Thirst,
    Fatigue,
    Sleep,
    Comfort,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum BodyPart {
    Head,
    Torso,
    Arms,
    Legs,
    Hands,
    Feet,
    Neck,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum InjuryType {
    Concussion,
    Cut,
    Pain,
    Bruise,
}
