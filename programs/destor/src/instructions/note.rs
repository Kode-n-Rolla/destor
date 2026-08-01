use anchor_lang::prelude::*;

use crate::{
    constant::{INSURANCE_NOTE_SIGNERS, MANUFACTURER_NOTE_SIGNERS, MAX_DESCRIPTION_LENGTH, MAX_NOTE_SIGNERS, MAX_REPORT_HASH_LENGTH, MAX_REPORT_URI_LENGTH, MEMBER_SEED, NOTE_SEED, ORGANIZATION_SEED, OWNER_NOTE_SIGNERS, ROAD_INSPECTION_NOTE_SIGNERS, SERVICE_HUB_NOTE_SIGNERS, VEHICLE_SEED}, error::DeStorError, events::{AddedNote, SignedNote}, state::{Member, Note, Organization, Vehicle}, types::{
        NoteKind, Role, Status,
    },
};

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct AddOrganizationNote<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,

    #[account(
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.key().as_ref()],
        bump,
        has_one = wallet,
    )]
    pub member: Account<'info, Member>,

    #[account(
        init,
        payer = wallet,
        space = Note::INIT_SPACE,
        seeds = [NOTE_SEED, vehicle.key().as_ref(), &vehicle.next_note_index.to_le_bytes()],
        bump
    )]
    pub note: Account<'info, Note>,

    #[account(
        mut,
        seeds = [VEHICLE_SEED, vin_hash.as_ref()],
        bump,
    )]
    pub vehicle: Account<'info, Vehicle>,

    pub system_program: Program<'info, System>, 
}

pub fn add_organization_note(
    ctx: Context<AddOrganizationNote>,
    vin_hash: [u8; 32],
    description: String,
    mileage: u64,
    report_uri: String,
    report_hash: String,
) -> Result<()> {
    require_eq!(ctx.accounts.organization.key(), ctx.accounts.member.organization, DeStorError::InvalidMember);
    if ctx.accounts.vehicle.vin_hash != vin_hash {
        return err!(DeStorError::InvalidVin);
    }
    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);
    require!(ctx.accounts.member.active, DeStorError::MemberIsNotActive);
    require_gt!(mileage, ctx.accounts.vehicle.mileage, DeStorError::InvalidMileage);
    require_gte!(MAX_DESCRIPTION_LENGTH, description.len(),DeStorError::DescriptionToLong);
    require_gte!(MAX_REPORT_URI_LENGTH, report_uri.len(), DeStorError::ReportUriToLong);
    require_gte!(MAX_REPORT_HASH_LENGTH, report_hash.len(), DeStorError::ReportHashToLong);
    require_eq!(
        ctx.accounts.member.organization,
        ctx.accounts.organization.key(),
        DeStorError::InvalidMember
    );

    let vehicle = &mut ctx.accounts.vehicle;
    let note = &mut ctx.accounts.note;
    let current_time = Clock::get()?.unix_timestamp;
    let required_signers = match ctx.accounts.organization.role {
        Role::Manufacturer => MANUFACTURER_NOTE_SIGNERS,
        Role::ServiceHub => SERVICE_HUB_NOTE_SIGNERS,
        Role::RoadInspection => ROAD_INSPECTION_NOTE_SIGNERS,
        Role::Insurance => INSURANCE_NOTE_SIGNERS,
        Role::Owner => OWNER_NOTE_SIGNERS,
    };

    note.status = Status::Pending;
    note.vehicle = vehicle.key();
    note.note_index = vehicle.next_note_index;
    note.role = ctx.accounts.organization.role;
    note.note_kind = match ctx.accounts.organization.role {
        Role::Manufacturer => NoteKind::Manufacturing,
        Role::ServiceHub => NoteKind::Service,
        Role::RoadInspection => NoteKind::Accident,
        Role::Insurance => NoteKind::InsuranceReport,
        Role::Owner => NoteKind::OwnerMaintenance,
    };
    note.mileage = mileage;
    note.timestamp = current_time;
    note.description = description;

    note.signers = Vec::with_capacity(required_signers);
    note.signers.push(ctx.accounts.wallet.key());

    note.report_uri = report_uri;
    note.report_hash = report_hash;
    note.required_signers = required_signers as u8;
    note.bump = ctx.bumps.note;

    vehicle.mileage = mileage;
    vehicle.next_note_index += 1;

    emit!(AddedNote {
        signer: ctx.accounts.wallet.key(),
        vehicle_pda: vehicle.key(),
        note_pda: note.key(),
        note_index: note.note_index,
        role: ctx.accounts.organization.role,
        note_kind: note.note_kind,
        timestamp: current_time,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct AddOwnerNote<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [VEHICLE_SEED, vin_hash.as_ref()],
        bump,
        has_one = owner,
    )]
    pub vehicle: Account<'info, Vehicle>,

    #[account(
        init,
        payer = owner,
        space = Note::INIT_SPACE,
        seeds = [NOTE_SEED, vehicle.key().as_ref(), &vehicle.next_note_index.to_le_bytes()],
        bump,
    )]
    pub note: Account<'info, Note>,
    pub system_program: Program<'info, System>, 
}

pub fn add_owner_note(
    ctx: Context<AddOwnerNote>,
    vin_hash: [u8; 32],
    description: String,
    mileage: u64,
    report_uri: String,
    report_hash: String,
) -> Result<()> {
    require_gt!(mileage, ctx.accounts.vehicle.mileage, DeStorError::InvalidMileage);
    require_gte!(MAX_DESCRIPTION_LENGTH, description.len(), DeStorError::DescriptionToLong);
    require_gte!(MAX_REPORT_URI_LENGTH, report_uri.len(), DeStorError::ReportUriToLong);
    require_gte!(MAX_REPORT_HASH_LENGTH, report_hash.len(), DeStorError::ReportHashToLong);

    if ctx.accounts.vehicle.vin_hash != vin_hash {
        return err!(DeStorError::InvalidVin);
    }

    let vehicle = &mut ctx.accounts.vehicle;
    let note = &mut ctx.accounts.note;
    let current_time = Clock::get()?.unix_timestamp;

    note.status = Status::Pending;
    note.vehicle = vehicle.key();
    note.note_index = vehicle.next_note_index;
    note.role = Role::Owner;
    note.note_kind = NoteKind::OwnerMaintenance;
    note.mileage = mileage;
    note.timestamp = current_time;
    note.description = description;

    note.signers = Vec::with_capacity(OWNER_NOTE_SIGNERS);
    note.signers.push(ctx.accounts.owner.key());

    note.report_uri = report_uri;
    note.report_hash = report_hash;
    note.required_signers = OWNER_NOTE_SIGNERS as u8;
    note.bump = ctx.bumps.note;

    vehicle.next_note_index += 1;

    emit!(AddedNote {
        signer: ctx.accounts.owner.key(),
        vehicle_pda: vehicle.key(),
        note_pda: note.key(),
        note_index: note.note_index,
        role: Role::Owner,
        note_kind: note.note_kind,
        timestamp: current_time,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(vin_hash: [u8; 32])]
pub struct SignNote<'info> {
    pub wallet: Signer<'info>,

    #[account(
        seeds = [ORGANIZATION_SEED, organization.organization_id.as_ref()],
        bump,
    )]
    pub organization: Account<'info, Organization>,

    #[account(
        seeds = [MEMBER_SEED, organization.key().as_ref(), wallet.key().as_ref()],
        bump,
        has_one = wallet,
    )]
    pub member: Account<'info, Member>,
    
    #[account(
        mut,
        seeds = [NOTE_SEED, vehicle.key().as_ref(), &note.note_index.to_le_bytes()],
        bump,
    )]
    pub note: Account<'info, Note>,
    
    #[account(
        mut,
        seeds = [VEHICLE_SEED, vin_hash.as_ref()],
        bump,
    )]
    pub vehicle: Account<'info, Vehicle>,
}

pub fn sign_note(ctx: Context<SignNote>, vin_hash: [u8; 32], mileage: u64) -> Result<()> {
    if ctx.accounts.vehicle.vin_hash != vin_hash {
        return err!(DeStorError::InvalidVin);
    }

    if ctx.accounts.note.status == Status::Approved || ctx.accounts.note.status == Status::Rejected {
        return err!(DeStorError::InvalidNote);
    }

    require!(ctx.accounts.organization.active, DeStorError::OrganizationNotActive);
    require!(ctx.accounts.member.active, DeStorError::MemberIsNotActive);
    require_gt!(mileage, ctx.accounts.note.mileage, DeStorError::InvalidMileage);
    require_eq!(ctx.accounts.vehicle.key(), ctx.accounts.note.vehicle, DeStorError::InvalidVehicleOrNote);
    require_eq!(ctx.accounts.note.signers.contains(&ctx.accounts.wallet.key()), false, DeStorError::MemberAlreadySigner);
    require_eq!(
        ctx.accounts.member.organization,
        ctx.accounts.organization.key(),
        DeStorError::InvalidMember
    );

    require!(
        ctx.accounts.note.signers.len() < MAX_NOTE_SIGNERS,
        DeStorError::InvalidNote
    );

    let note = &mut ctx.accounts.note;
    let vehilce = &mut ctx.accounts.vehicle;
    let current_time = Clock::get()?.unix_timestamp;

    note.signers.push(ctx.accounts.wallet.key());
    note.mileage = mileage;

    if note.signers.len() >= note.required_signers as usize {
        note.status = Status::Approved;
        vehilce.mileage = mileage;
    }

    emit!(SignedNote{
        signer: ctx.accounts.wallet.key(),
        vehicle_pda: vehilce.key(),
        note_pda: note.key(),
        note_index: note.note_index,
        organization_pda: ctx.accounts.organization.key(),
        role: ctx.accounts.organization.role,
        timestamp: current_time,
    });

    Ok(())
}

pub struct RejectNote {
    // @todo
}

pub fn reject_note() -> Result<()> {

    // event, lir.rs handler
    Ok(())
}
