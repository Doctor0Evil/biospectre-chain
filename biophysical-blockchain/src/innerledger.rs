use crate::evolution_turns::{DailyTurnState, MAX_DAILY_TURNS_INNER};

#[derive(Clone, Debug)]
pub struct InnerLedger {
    pub state: BioTokenState,
    pub env: HostEnvelope,
    pub daily_turns: DailyTurnState,
    // existing fields...
}

#[derive(thiserror::Error, Debug)]
pub enum InnerLedgerError {
    #[error("evolution turn limit reached for this UTC day")]
    EvolutionTurnLimitReached,
    // existing variants...
}

// Inside your evolution path, before committing an evolution/mutation adjustment:

impl InnerLedger {
    pub fn system_apply_evolution(
        &mut self,
        idheader: IdentityHeader,
        required_k: f32,
        adj: SystemAdjustment,
        timestamp_utc: &str,
        lf_series: LifeforceBandSeries,
        eco_profile: EcoBandProfile,
        wave_curve: SafetyCurveWave,
        is_evolution: bool,
    ) -> Result<LedgerEvent, InnerLedgerError> {
        // identity + consent already validated earlier...

        if is_evolution {
            let ok = self
                .daily_turns
                .try_consume_turn(MAX_DAILY_TURNS_INNER);
            if !ok {
                return Err(InnerLedgerError::EvolutionTurnLimitReached);
            }
        }

        // Then call existing guarded mutation path.
        // (Pseudo-call; adjust to your actual function name)
        let _ = crate::lifeforce::apply_lifeforce_guarded_adjustment(
            &mut self.state,
            &self.env,
            adj,
            lf_series,
            eco_profile,
            wave_curve,
        )?;
        // build LedgerEvent as you already do...
        // ...
        #![allow(unreachable_code)]
        Err(InnerLedgerError::EvolutionTurnLimitReached) // placeholder
    }
}
