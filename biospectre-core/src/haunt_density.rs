use crate::state::EpochState;

#[derive(Debug, Clone, Copy)]
pub struct HauntDensityToken {
    pub rare_n3_rate: f32,   // fraction of epochs with sn3 >= 2.5
    pub rare_q_rate: f32,    // fraction with s_unknown >= 0.3
}

impl HauntDensityToken {
    pub fn from_night(epochs: &[EpochState]) -> Self {
        let total = epochs.len().max(1) as f32;
        let rare_n3 = epochs
            .iter()
            .filter(|e| matches!(e.stage, crate::state::StageKind::N3) && e.sn3 >= 2.5)
            .count() as f32;
        let rare_q = epochs
            .iter()
            .filter(|e| e.s_unknown >= 0.3)
            .count() as f32;

        Self {
            rare_n3_rate: rare_n3 / total,
            rare_q_rate: rare_q / total,
        }
    }
}
