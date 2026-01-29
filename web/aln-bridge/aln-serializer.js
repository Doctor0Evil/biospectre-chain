export function toAlnShard(ledgerSlice, meta) {
  const lines = [];
  lines.push(`version ${meta.version}`);
  lines.push(`schema ${meta.schema}`);
  lines.push(`hostid ${meta.hostId}`);
  lines.push(`sessionutc ${meta.sessionUtc}`);
  lines.push("");
  lines.push("entries");

  for (const e of ledgerSlice) {
    lines.push("  - epochidx " + e.epoch_index);
    lines.push("    stage " + e.state.stage);
    lines.push("    sn1 " + e.state.sn1.toFixed(3));
    lines.push("    sn2 " + e.state.sn2.toFixed(3));
    lines.push("    sn3 " + e.state.sn3.toFixed(3));
    lines.push("    sunknown " + e.state.s_unknown.toFixed(3));
    lines.push("    ecoflops " + e.state.eco_flops);
    lines.push("    ecoenergynj " + e.state.eco_energy_nj.toFixed(1));
  }

  return lines.join("\n");
}
