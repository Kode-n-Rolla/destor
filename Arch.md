# DeStor Architecture

DeStor is a verifiable vehicle history protocol for reducing fraud in the used car market.

The core idea: a vehicle has a public digital passport, and trusted actors append signed history records about mileage, repairs, inspections, incidents, insurance reports, and ownership changes.

## Goals

- Make mileage rollback visible or impossible inside the protocol.
- Make vehicle history public and easy to verify by VIN.
- Show who added each record and which organization confirmed it.
- Require multiple trusted signatures for sensitive records.
- Keep the MVP simple enough to build, explain, and demo.

## Non-Goals for MVP

- Full legal identity verification.
- Private records or encrypted history.
- Decentralized governance.
- Full marketplace integration.
- Perfect NFT transfer restrictions.

These can be added later after the base protocol is working.

---

# Ownership Model

There are two related ownership concepts:

## NFT Owner

The wallet that currently owns the NFT asset.

This is different from the minter. The minter creates the asset, but after transfer the current NFT owner is the wallet holding it.

## Vehicle Owner

The `Vehicle.owner` field in the protocol account.

For the MVP, `Vehicle.owner` should be treated as the authoritative owner for protocol permissions.

The intended invariant:

```text
Vehicle.owner should match the current NFT owner.
```

Ownership should change through a protocol instruction, not by manually editing state.

Suggested flow:

```text
transfer_vehicle(new_owner)
- current owner signs
- vehicle.owner changes to new_owner
- owner_count increments
- ownership transfer note is created
- NFT transfer is performed or expected as part of the same flow
```

Important MVP decision:

```text
Protocol permissions are checked against Vehicle.owner.
NFT ownership is used as the public passport/asset identity.
```

Later, NFT transfer rules can be made stricter with Metaplex Core plugins/delegates.

---

# VIN Verification

Do not store raw VIN on-chain in the MVP. Store `vin_hash`.

Frontend flow:

```text
User enters VIN
Frontend normalizes VIN
Frontend hashes normalized VIN
Frontend searches Vehicle by vin_hash
Frontend displays public vehicle history
```

Normalization rule:

```text
trim spaces
uppercase letters
remove formatting separators if needed
```

This prevents different user input formats from producing different hashes for the same vehicle.

---

# Roles

## CoreAdmin

The CoreAdmin manages trusted organizations and members.

Actions:

- initialize protocol
- register manufacturer
- register service center
- register road inspection organization
- register insurance organization
- add organization member
- remove organization member
- set organization threshold
- deactivate organization

For MVP, one admin is acceptable. In production, this can be replaced with a multisig or DAO.

## Manufacturer

Examples: BMW, Mercedes-Benz, Toyota.

Actions:

- mint vehicle NFT/passport
- create the initial Vehicle account
- set initial vehicle metadata
- optionally add manufacturing note

Possible threshold:

```text
3 signatures:
- organization authority
- technical employee
- compliance/registry employee
```

## Service Center

Actions:

- add service records
- confirm repairs
- confirm owner self-maintenance
- record mileage at service time

Possible threshold:

```text
2 signatures:
- service center authority/head
- mechanic/employee
```

## Road Inspection

Actions:

- add accident records
- add traffic/inspection records
- record mileage during inspection
- verify owner (if seller forget transfer NFT to the buyer)

Possible threshold:

```text
2 signatures:
- inspector
- second inspector or insurance representative
```

Additional action:

- verify owner if seller transferred the real vehicle but forgot or refused to transfer the NFT/protocol ownership

This should be treated as an ownership correction flow, not a normal transfer.

Required behavior:

- inspection organization threshold must be satisfied
- new owner is written to Vehicle.owner
- owner_count increments
- OwnershipCorrection note is created
- previous owner cannot silently erase or hide the correction

## Insurance

Actions:

- add insurance incident report
- confirm accident details
- link report URI/hash

Possible threshold:

```text
2 signatures:
- insurance authority/head
- employee who accepted the incident report
```

## Owner

Actions:

- request owner maintenance note
- transfer vehicle ownership

Owner self-maintenance should not be trusted by itself.

For MVP:

```text
Owner note requires:
- owner signature
- service center verifier signature
```

This prevents fake records like "changed oil every week" without trusted confirmation.

---

# Accounts / Data Model

## ProtocolConfig

Stores global protocol settings.

Fields:

```text
admin
bump
```

## Vehicle

Stores current vehicle state.

Fields:

```text
vin_hash
nft_asset
manufacturer
owner
color
manufactured_at
mileage
note_count
owner_count
bump
```

Notes:

- `mileage` is the latest accepted mileage.
- New notes cannot use mileage lower than current vehicle mileage.
- `note_count` is used to derive Note PDA addresses.
- `owner_count` increments on ownership transfer.

## Organization

Represents a trusted organization.

Fields:

```text
role
authority
threshold
active
bump
```

Notes:

- `role` should be an enum, not a free-form string.
- `threshold` defines how many valid member signatures are required.

## Member

Represents a wallet approved under an organization.

Fields:

```text
organization
wallet
active
bump
```

Notes:

- Do not store the whole Organization inside Member.
- Store the organization public key.

## Note

Notes are stored as separate PDA accounts derived from Vehicle.

Fields:

```text
vehicle
note_index
role
note_kind
mileage
timestamp
description
signers
report_uri
report_hash
bump
```

Notes:

- Do not store all notes inside Vehicle.
- Vehicle history is loaded by finding Note accounts for a Vehicle.
- `description` should have a max length.
- `report_uri` should have a max length.
- `report_hash` can verify the off-chain report content.

## Role Enum

Use an enum for roles instead of strings.

Possible variants:

```text
Manufacturer
Service
RoadInspection
Insurance
Owner
```

## Note Kind Enum

Possible variants:

```text
Manufacturing
Service
Inspection
Accident
InsuranceReport
OwnerMaintenance
OwnershipTransfer
```

---

# MVP Instructions

## Protocol Setup

```text
initialize_protocol(admin)
```

## Organization Management

```text
register_organization(role, authority, threshold)
add_organization_member(organization, member)
remove_organization_member(organization, member)
set_organization_threshold(organization, threshold)
deactivate_organization(organization)
```

## Vehicle Lifecycle

```text
mint_vehicle(vin_hash, initial_owner, metadata_uri, initial_mileage)
transfer_vehicle(new_owner)
verify_owner_transfer(...)
```

## History Records

```text
add_service_note(...)
add_inspection_note(...)
add_insurance_note(...)
add_owner_verified_note(...)
```

Optional later:

```text
add_accident_note(...)
add_manufacturing_note(...)
```

---

# Core Invariants

Every note must:

- belong to one Vehicle
- have mileage greater than or equal to current Vehicle mileage
- have the required role
- have enough valid signers
- only use signers that are active members of the required organization
- append immutable history

Every ownership transfer must:

- be signed by the current Vehicle owner
- update Vehicle.owner
- increment owner_count
- create an ownership transfer note

Every organization-controlled action must:

- use an active organization
- match the required role
- satisfy the organization threshold

---

# Development Cycle

The first phase should focus on on-chain functionality only.

Suggested order:

1. Define account models and enums.
2. Implement protocol initialization.
3. Implement organization registration.
4. Implement member management.
5. Implement vehicle creation without NFT CPI first.
6. Implement ownership transfer.
7. Implement note creation and mileage monotonicity.
8. Add NFT integration.
9. Add tests after the Rust logic is understandable.
10. Add frontend VIN lookup and public history page.

Reason:

```text
First learn and stabilize the Rust/Anchor program model.
Then add TypeScript tests.
Then add UI.
Then improve NFT integration.
```

This keeps Rust, TypeScript, frontend, and NFT CPI complexity from all hitting at the same time.

---

# NFT Direction

Preferred direction for a new Solana NFT MVP:

```text
Metaplex Core Asset
```

Why:

- modern NFT standard
- single-account asset model
- lower cost than classic Token Metadata NFTs
- plugin system can help later with transfer/update rules

For the first implementation phase:

```text
Vehicle PDA is the protocol source of truth.
NFT is the public vehicle passport asset.
```

Later:

```text
transfer_vehicle can perform or enforce NFT transfer.
Core plugins/delegates can restrict unauthorized ownership changes.
```

---

# Public Data Policy

Vehicle history is public by design.

Public data examples:

- mileage
- service type
- accident type
- repair summary
- inspection result
- organization
- signer public keys
- report URI/hash

Avoid putting personally identifying data directly on-chain:

- full names
- phone numbers
- home addresses
- government document numbers

---

# Process
[X] Accounts/enums
[X] initialize_protocol
[ ] register_organization
[ ] add/remove member
[ ] mint_vehicle без NFT CPI
[ ] transfer_vehicle
[ ] add_note + mileage monotonicity
[ ] потом NFT
[ ] потом tests
[ ] потом frontend

---

# Future Features

- Metaplex Core transfer restrictions/plugins
- DAO or multisig admin
- organization reputation
- richer report schemas
- image/document storage via Arweave, IPFS, or Shadow Drive
- compressed accounts / ZK compression if account growth becomes expensive
- frontend history timeline
- fraud risk score based on mileage jumps, accidents, and service gaps
