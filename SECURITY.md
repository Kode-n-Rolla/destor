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
- update `Vehicle.owner`
- increment `owner_count`
- create an ownership transfer note

Every organization-controlled action must:
- use an active organization
- match the required role
- satisfy the organization threshold

Every member-management action must:
- be signed by the organization authority
- use an active organization
- create or update a Member PDA derived from organization and wallet
- not require CoreAdmin during normal operation

