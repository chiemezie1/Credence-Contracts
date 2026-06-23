# Delegation Summary View

The `get_delegation_summary` view function provides a comprehensive summary of a delegation's state for indexers and off-chain tools.

## Entrypoint

```rust
pub fn get_delegation_summary(
    e: Env,
    owner: Address,
    delegate: Address,
    delegation_type: DelegationType,
) -> DelegationSummary
```

## `DelegationSummary` Struct

| Field | Type | Description |
|-------|------|-------------|
| `is_valid` | `bool` | `true` if the delegation is NOT revoked AND the current ledger timestamp is less than `expires_at`. |
| `time_to_expiry` | `u64` | The remaining lifetime of the delegation in seconds (`expires_at - now`). Returns `0` if expired. |
| `delegation_type` | `DelegationType` | The type of delegation (`Attestation` or `Management`). |
| `revoked_at` | `u64` | The ledger timestamp at which the delegation was revoked. `0` means the delegation has not been revoked (not-revoked sentinel). |
| `scheme` | `u32` | The signature scheme used to authorise delegation creation: `0` = Ed25519 (default), `1` = Secp256r1, `2` = MLDSA44. |

## Field Semantics

### `revoked_at`

Recorded by `mark_delegation_revoked` at the moment any revoke entry point completes:

- `revoke_delegation` — owner revokes via direct auth
- `revoke_attestation` — attester revokes via direct auth
- `execute_delegated_revoke` — relayer revokes via signed payload
- `execute_delegated_revoke_attest` — relayer revokes attestation via signed payload

A value of `0` is the not-revoked sentinel. The very first revoke call sets this field; subsequent calls to revoke the same delegation are rejected with `AlreadyRevoked` (#502) before reaching the write, so the first recorded timestamp is preserved.

### `scheme`

Recorded by `store_delegation` at creation time:

- **Direct auth path** (`delegate`): always `0` (Ed25519). The direct-auth path does not carry a payload, so no scheme identifier is available.
- **Relayer path** (`execute_delegated_delegate`): stored from `payload.scheme`. The payload producer sets this when building the off-chain signature.

Scheme values match the `SchemeTag` constants in `verifier.rs`:

| Value | Scheme |
|-------|--------|
| `0` | Ed25519 (default) |
| `1` | Secp256r1 |
| `2` | MLDSA44 |

## Legacy Entry Defaults

The `revoked_at` and `scheme` fields were added in v2. Pre-v2 entries stored on-chain lack these fields. Their documented defaults are:

| Field | Legacy default | Meaning |
|-------|---------------|---------|
| `revoked_at` | `0` | Not-revoked sentinel (consistent with a never-revoked entry) |
| `scheme` | `0` | Ed25519 (consistent with the only scheme available at contract deployment) |

A live-upgrade migration must read pre-v2 entries as `LegacyDelegation` (the 5-field type documented in `lib.rs`), set both new fields to `0`, and re-persist them as `Delegation` before the contract's read paths are used. All new writes since v2 go through the full `Delegation` struct.

## Auditor Notes

- `revoked_at = 0` is **not** ambiguous with a real ledger timestamp of `0` in practice: Soroban ledger timestamps start at genesis and are always positive by the time a contract is deployed and used.
- Indexers should treat `is_valid = false && revoked_at = 0` as "expired but not revoked" and `is_valid = false && revoked_at > 0` as "explicitly revoked".
- This is a read-only view. It performs no `require_auth` and mutates nothing — safe to expose publicly.

## Usage for Indexers

Indexers should use this view to track the validity and remaining lifetime of delegations without needing to implement the expiration logic locally. The `revoked_at` and `scheme` fields provide the audit trail data that dispute tooling requires.
