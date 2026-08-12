# Roadmap

## Milestone 1
- [X] Accounts/enums
- [X] initialize_protocol
- [X] register_organization
- [X] add/remove member
- [X] `deactivate_organization`, `set_organization_threshold`, authority transfer
- [X] mint_vehicle без NFT CPI или все-таки NFT?
- [X] transfer_vehicle
- [X] testing

## Milestone 2
- [X] add `verify_owner_transfer` function
- [X] testing
- [X] add re-activate member funcion
- [X] testing
- [X] add `add_note` (for organization and owner)
- [ ] testing
- [X] add `sign_note` functionality
- [ ] testing
- [ ] add `change_color` function
- [ ] add adding note to transfer functions (`transfer_vehicle`, `verify_owner_transfer`, ` change_color`)
- [ ] testing

## Milestone 3
- [ ] implement `enum` role for `Member` (`?`)
- [ ] use derived `bump`, where needed for optimization
- [ ] add reject note functionality
- [ ] add verify vehicle mileage with `protocol.admin` permission
- [ ] implement NFT
- [ ] testing

## Milestone 4
- [ ] Add docstrings for function, structs, etc. (`///`)
- [ ] need or not `pending_authority` as `Some()`?
- [ ] implement restiction for frontrun protocol initialization
- [ ] Security testing
- [ ] Deploy demo with vehicles

# Future Features

## Upgrade itself
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
