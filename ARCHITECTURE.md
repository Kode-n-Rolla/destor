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

---

# VIN Verification
Protocol does not store raw VIN on-chain. Store `vin_hash`.

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


# Roles

## CoreAdmin
The CoreAdmin manages trusted organizations at the registry level.

Actions:
- initialize protocol
- register organization by role:
    - manufacturer
    - service center
    - road inspection organization
    - insurance organization
- deactivate organization

For MVP, one admin is acceptable. In production, this can be replaced with a multisig or DAO.

## Organization Authority
Each registered organization has its own authority wallet.

Actions:
- add organization member
- remove organization member
- set organization threshold
- change organization authority (via 2 steps)

This keeps responsibilities separated:
```text
CoreAdmin decides which organizations are trusted.
Organization authority decides which wallets belong to that organization.
```

## Manufacturer
Examples: BMW, Cadillac, Chevrolet, Toyota, Mitsubishi

Actions:
- mint vehicle NFT/passport
- create the initial Vehicle account
- set initial vehicle metadata
- add manufacturing note (optionally)

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

Possible threshold:

```text
2 signatures:
- service center authority/head
- mechanic/employee
```

## Road Inspection
Actions:
- add accident records
- add traffic/inspection records (optionally)
- verify owner (if seller forget transfer NFT to the buyer)

Possible threshold:
```text
2 signatures:
- inspector
- second inspector or insurance representative
```

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
manufactured_at
model
owner
color
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
pending_authority
organization_id
threshold
active
bump
```

Notes:
- `role` should be an enum, not a free-form string.
- `threshold` defines how many valid member signatures are required.
- `authority` manages members for this organization.
- CoreAdmin registers or deactivates the organization, but does not manage every member in normal operation.

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
- Member accounts are created by the organization authority, not by CoreAdmin.
- A member is unique by `(organization, wallet)`.

## Note

Notes are stored as separate PDA accounts derived from Vehicle.
Store details about case, current mileage

Fields:
```text
status
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

## Role Enum for Organization

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


# Project Structure
```
protocol.rs
- initialize_protocol

organization.rs
- register_organization
- deactivate_organization
- set_organization_threshold
- request_authority_transfer
- accept_authority_transfer

member.rs
- add_organization_member
- remove_organization_member

vehicle.rs
- mint_vehicle
- assign_initial_owner
- transfer_vehicle
- verify_owner_transfer

note.rs
- add_service_note
- add_inspection_note
- add_insurance_note
- add_owner_verified_note
```


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