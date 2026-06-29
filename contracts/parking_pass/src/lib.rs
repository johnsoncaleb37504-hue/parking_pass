#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

/// Status codes for a reservation lifecycle.
const STATUS_NONE: u32 = 0;
const STATUS_ACTIVE: u32 = 1;
const STATUS_CANCELLED: u32 = 2;
const STATUS_RELEASED: u32 = 3;
const STATUS_NO_SHOW: u32 = 4;

/// Storage key for the next reservation id counter.
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");
/// Storage key for the contract admin (deployer).
const ADMIN: Symbol = symbol_short!("ADMIN");

/// A parking spot registered by a building / lot manager.
#[contracttype]
#[derive(Clone)]
pub struct Spot {
    pub manager: Address,
    pub spot_id: Symbol,
    pub location: Symbol,
    pub active: bool,
}

/// A reservation that a user holds for a spot in a given window.
#[contracttype]
#[derive(Clone)]
pub struct Reservation {
    pub id: u64,
    pub user: Address,
    pub spot_id: Symbol,
    pub start: u64,
    pub end: u64,
    pub status: u32,
    pub no_show: bool,
    pub reason: Symbol,
}

/// Persistent storage keys.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Spot(Symbol),
    Reservation(u64),
}

#[contract]
pub struct ParkingPass;

#[contractimpl]
impl ParkingPass {
    /// Initialize the contract by recording the deploying admin address.
    /// Must be called once before any other state-changing function.
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&ADMIN) {
            panic!("already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&NEXT_ID, &1u64);
    }

    /// Register a new parking spot identified by `spot_id` at a `location`.
    /// Only the calling `manager` (who must authorize) becomes owner of the spot.
    pub fn register_spot(env: Env, manager: Address, spot_id: Symbol, location: Symbol) {
        manager.require_auth();

        let key = DataKey::Spot(spot_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("spot already registered");
        }

        let spot = Spot {
            manager: manager.clone(),
            spot_id: spot_id.clone(),
            location,
            active: true,
        };
        env.storage().persistent().set(&key, &spot);
    }

    /// Reserve an active spot for `user` during the window `[start, end)`.
    /// Returns the freshly minted reservation id.
    pub fn reserve(env: Env, user: Address, spot_id: Symbol, start: u64, end: u64) -> u64 {
        user.require_auth();

        if end <= start {
            panic!("invalid window");
        }

        let spot_key = DataKey::Spot(spot_id.clone());
        let spot: Spot = env
            .storage()
            .persistent()
            .get(&spot_key)
            .unwrap_or_else(|| panic!("spot not found"));

        if !spot.active {
            panic!("spot inactive");
        }

        let mut next_id: u64 = env.storage().instance().get(&NEXT_ID).unwrap_or(1u64);
        let reservation_id = next_id;
        next_id += 1;
        env.storage().instance().set(&NEXT_ID, &next_id);

        let reservation = Reservation {
            id: reservation_id,
            user: user.clone(),
            spot_id,
            start,
            end,
            status: STATUS_ACTIVE,
            no_show: false,
            reason: symbol_short!("NONE"),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Reservation(reservation_id), &reservation);

        reservation_id
    }

    /// User-initiated cancellation of their own active reservation.
    pub fn cancel(env: Env, user: Address, reservation_id: u64) {
        user.require_auth();

        let key = DataKey::Reservation(reservation_id);
        let mut reservation: Reservation = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("reservation not found"));

        if reservation.user != user {
            panic!("not your reservation");
        }
        if reservation.status != STATUS_ACTIVE {
            panic!("not active");
        }

        reservation.status = STATUS_CANCELLED;
        reservation.reason = symbol_short!("USR_CXL");
        env.storage().persistent().set(&key, &reservation);
    }

    /// Manager-initiated release of a held reservation with an attached `reason`.
    /// Only the manager of the underlying spot may release it.
    pub fn release(env: Env, manager: Address, reservation_id: u64, reason: Symbol) {
        manager.require_auth();

        let key = DataKey::Reservation(reservation_id);
        let mut reservation: Reservation = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("reservation not found"));

        let spot: Spot = env
            .storage()
            .persistent()
            .get(&DataKey::Spot(reservation.spot_id.clone()))
            .unwrap_or_else(|| panic!("spot missing"));

        if spot.manager != manager {
            panic!("not spot manager");
        }
        if reservation.status != STATUS_ACTIVE {
            panic!("not active");
        }

        reservation.status = STATUS_RELEASED;
        reservation.reason = reason;
        env.storage().persistent().set(&key, &reservation);
    }

    /// Manager flags a reservation as a no-show (driver never arrived).
    pub fn mark_no_show(env: Env, manager: Address, reservation_id: u64) {
        manager.require_auth();

        let key = DataKey::Reservation(reservation_id);
        let mut reservation: Reservation = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("reservation not found"));

        let spot: Spot = env
            .storage()
            .persistent()
            .get(&DataKey::Spot(reservation.spot_id.clone()))
            .unwrap_or_else(|| panic!("spot missing"));

        if spot.manager != manager {
            panic!("not spot manager");
        }
        if reservation.status != STATUS_ACTIVE {
            panic!("not active");
        }

        reservation.no_show = true;
        reservation.status = STATUS_NO_SHOW;
        reservation.reason = symbol_short!("NO_SHOW");
        env.storage().persistent().set(&key, &reservation);
    }

    /// Read-only view: returns the numeric status of a reservation
    /// (0 = none, 1 = active, 2 = cancelled, 3 = released, 4 = no-show).
    pub fn get_status(env: Env, reservation_id: u64) -> u32 {
        let key = DataKey::Reservation(reservation_id);
        let maybe: Option<Reservation> = env.storage().persistent().get(&key);
        match maybe {
            Some(r) => r.status,
            None => STATUS_NONE,
        }
    }

    /// Read-only view: fetches the full reservation record.
    pub fn get_reservation(env: Env, reservation_id: u64) -> Reservation {
        env.storage()
            .persistent()
            .get(&DataKey::Reservation(reservation_id))
            .unwrap_or_else(|| panic!("reservation not found"))
    }

    /// Read-only view: fetches the registered spot metadata.
    pub fn get_spot(env: Env, spot_id: Symbol) -> Spot {
        env.storage()
            .persistent()
            .get(&DataKey::Spot(spot_id))
            .unwrap_or_else(|| panic!("spot not found"))
    }
}
