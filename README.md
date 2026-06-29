# parking_pass

## Project Title
parking_pass

## Project Description
parking_pass is a Soroban smart contract that turns real-estate parking spots into
on-chain, time-bound reservation passes. Building / lot managers register their spots,
drivers reserve a spot for a date-time window, and every cancellation, release or
no-show is recorded immutably on the Stellar ledger — eliminating double-bookings,
phone-in disputes and paper logs that plague today's residential and commercial
parking operations.

## Project Vision
Our vision is to become the default reservation rail for every paid and permit-only
parking surface in the real-estate sector — from condominium visitor bays and office
podiums to shopping-mall lots and event-day overflow parking. By anchoring spot
ownership, reservation rights and behavioural reputation (no-shows) to a single
public ledger, parking_pass aims to unlock a frictionless secondary market for
unused spot-hours and to let property managers monetise idle real-estate inventory
24/7 with zero reconciliation overhead.

## Key Features
- **Manager-controlled spot registry** — `register_spot` lets a verified manager
  publish a spot with `spot_id` and `location`, becoming its sole on-chain owner.
- **Time-windowed user reservations** — `reserve` mints a unique reservation id
  for a `[start, end)` window after `require_auth()` on the driver's address.
- **Driver self-service cancellation** — `cancel` lets the original user release
  their hold; only they can do it, enforced on-chain.
- **Manager override & no-show flagging** — `release` lets the spot's manager
  free a hold with a reason `Symbol`, and `mark_no_show` records driver
  reliability data tied to the reservation.
- **Public status oracle** — `get_status`, `get_reservation` and `get_spot`
  expose read-only views that any frontend, gate-arm or audit tool can poll
  without paying for state changes.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** real_estate dApp — see `contracts/parking_pass/src/lib.rs` for the full parking_pass business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CBPSLC42SOAF2OAI22M3TGKFJUPD4EH3Y74MAMGXIVCF2TA2I5FTX3U6`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/7fae5a4a467d6694963f04200e73ec0d1b10ab67fa4dfdec5d49690c56d4d56d`

## Future Scope
- **Payments rail integration** — escrow USDC or XLM per reservation and auto-pay
  the spot manager after a successful park-out window.
- **Dynamic pricing oracle** — feed congestion / event data to surge-price hot
  spots while keeping resident permits at a fixed rate.
- **Reputation NFT for drivers** — aggregate `no_show` counters into a
  transferable on-chain score that influences future booking priority.
- **Secondary market for spot-hours** — allow holders to resell or sublet
  unused windows of an active reservation peer-to-peer.
- **IoT gate-arm integration** — let LPR cameras and gate controllers verify a
  driver's reservation status by calling `get_status` directly from the edge.
- **Mainnet launch with KYC'd manager onboarding** — pair the contract with an
  off-chain manager-verification flow to roll out to commercial real-estate
  partners.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `parking_pass` (real_estate)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
