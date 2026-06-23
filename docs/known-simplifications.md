# Known Simplifications

## Current Limitations

1. **No appeal mechanism** — Decisions are final once finalized
2. **No timelocks** — Disputes resolve immediately upon quorum
3. **Single dispute per identity per epoch** — Cannot have overlappingtes
4. **No appeal bond** — Appeals are free (not yet implemented)
5. **Manual dispute initiation** — No automated trigger
6. **No graceful shutdown** — Cannot pause arbitration
7. **No fee split** — All slashed XLM goes to protocol
8. **No arbitrator rotation** — Same arbitrators in all disputes

## Resolved

✅ **#9**: Admin-set arbitrator weights → Now derived from credence_bond balance
