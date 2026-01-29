use crate::tokens::BiophysicalTokenBundle;
use biospectre_core::state::EpochState;

#[derive(Debug, Clone)]
pub struct LedgerEntry {
    pub epoch_index: u32,
    pub host_id: String,
    pub state: EpochState,
    pub tokens_before: BiophysicalTokenBundle,
    pub tokens_after: BiophysicalTokenBundle,
    pub workload_id: Option<String>,
}

#[derive(Debug, Default)]
pub struct BiophysicalLedger {
    pub entries: Vec<LedgerEntry>,
}

impl BiophysicalLedger {
    pub fn append(
        &mut self,
        epoch_index: u32,
        host_id: &str,
        state: EpochState,
        before: BiophysicalTokenBundle,
        after: BiophysicalTokenBundle,
        workload_id: Option<String>,
    ) {
        self.entries.push(LedgerEntry {
            epoch_index,
            host_id: host_id.to_string(),
            state,
            tokens_before: before,
            tokens_after: after,
            workload_id,
        });
    }
}
