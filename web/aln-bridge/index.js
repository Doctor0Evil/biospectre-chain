import { createBrainTokenDecision } from "./native/biospectre_brain_tokens.js";

export function decideWorkload(request) {
  // request: { hostId, epoch, tokens, workload }
  const decision = createBrainTokenDecision({
    host_id: request.hostId,
    epoch_state: request.epoch,
    token_bundle: request.tokens,
    workload_request: request.workload,
  });

  return {
    allowed: decision.allowed,
    reason: decision.reason,
    updatedTokens: decision.updated_tokens,
  };
}
