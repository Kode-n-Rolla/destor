# Roadmap

## Milestone 1
- [X] Accounts/enums
- [X] initialize_protocol
- [X] register_organization
- [X] add/remove member
- [X] deactivate_organization, set_organization_threshold, authority transfer
- [X] mint_vehicle без NFT CPI или все-таки NFT?
- [X] transfer_vehicle
- [ ] testing

## Milestone 2
- [ ] add_note functionality
- [ ] testing

## Milestone 3
- [ ] implement NFT
- [ ] testing


# Future Features

## Upgrade itself
- Add `enum` role for `Member`
- ZK Compress (compressed accounts / ZK compression if account growth becomes expensive)

## Protocol
- Metaplex Core transfer restrictions/plugins `?`
- DAO or multisig admin `?`
- image/document storage via Arweave, IPFS, or Shadow Drive

## Frontend
- frontend
    - history timeline
    - vin input as string, backend hash it and pass to the function
    - auto change from unix time to user readable date
- fraud risk score based on mileage jumps, accidents, and service gaps
