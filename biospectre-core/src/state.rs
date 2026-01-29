#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StageKind {
    Wake,
    N1,
    N2,
    N3,
    Rem,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct EpochState {
    pub stage: StageKind,
    pub sn1: f32,
    pub sn2: f32,
    pub sn3: f32,
    pub s_unknown: f32,
    pub eco_flops: u64,
    pub eco_energy_nj: f32,
}
