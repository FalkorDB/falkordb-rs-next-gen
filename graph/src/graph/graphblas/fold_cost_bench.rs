//! Measurements behind the fold policy in [`super::versioned_matrix`].
//!
//! The policy folds a delta into the base at `|delta| ≈ SCALE·sqrt(base_cost ·
//! tx_added)`. That shape follows from one assumption: **every write
//! transaction pays `O(|delta|)`**, because it COW-dups the delta and merges
//! its pending tuples. These benches measure the three costs the model is
//! built on, so the constants stop being guesses:
//!
//!   1. `dup` of a delta — the per-transaction tax, vs assembled nvals.
//!   2. `dup` of a delta holding *pending* (unassembled) tuples, plus the
//!      `wait` that assembles them. The policy assumes both are `O(|delta|)`;
//!      if `dup` of pending tuples is much cheaper than of assembled entries,
//!      the tax is smaller than modelled and deltas can run larger.
//!   3. The fold itself, vs base nvals and nrows — this sets the ceiling on
//!      how often folding can be afforded.
//!
//! Run with:
//!   cargo test --release -p graph fold_cost -- --ignored --nocapture

use std::time::Instant;

use super::matrix::{Dup, Matrix};
use super::test_init::ensure_init;

/// Big-graph shape: square, high capacity, so `nrows` dominates `nvals` for
/// sparse matrices exactly as it does on a real graph's label/adjacency
/// matrices.
const CAP: u64 = 1_000_000;

/// Spread entries over distinct rows so the matrix is genuinely sparse rather
/// than a dense block, which is what a real delta looks like.
fn scatter(n: u64) -> (Vec<u64>, Vec<u64>) {
    let stride = (CAP / n.max(1)).max(1);
    let rows: Vec<u64> = (0..n).map(|i| (i * stride) % CAP).collect();
    let cols: Vec<u64> = (0..n).map(|i| (i * 7 + 3) % CAP).collect();
    (rows, cols)
}

fn assembled(n: u64) -> Matrix<bool> {
    let mut m = Matrix::<bool>::new(CAP, CAP);
    let (rows, cols) = scatter(n);
    m.build(&rows, &cols);
    m.wait();
    m
}

fn pending(n: u64) -> Matrix<bool> {
    let mut m = Matrix::<bool>::new(CAP, CAP);
    let (rows, cols) = scatter(n);
    for i in 0..rows.len() {
        m.set(rows[i], cols[i], true);
    }
    m // deliberately NOT waited: entries sit in the pending list
}

fn time_us<F: FnMut()>(
    reps: u32,
    mut f: F,
) -> f64 {
    let t = Instant::now();
    for _ in 0..reps {
        f();
    }
    t.elapsed().as_secs_f64() * 1e6 / f64::from(reps)
}

#[test]
#[ignore = "measurement, not a correctness check"]
fn fold_cost_dup_vs_nvals() {
    ensure_init();
    println!("\n=== dup cost vs delta size (the per-transaction tax) ===");
    println!(
        "{:>10}  {:>12}  {:>12}  {:>12}",
        "nvals", "assembled us", "pending us", "wait us"
    );
    for &n in &[256u64, 1_024, 4_096, 16_384, 65_536, 262_144, 1_048_576] {
        let reps = if n <= 16_384 { 200 } else { 20 };

        let a = assembled(n);
        let dup_assembled = time_us(reps, || {
            let d = a.dup();
            std::hint::black_box(&d);
        });

        // Rebuild per rep: dup of a pending matrix may assemble it, which
        // would make a reused matrix cheap on every rep after the first.
        let mut dup_pending_total = 0.0;
        let mut wait_total = 0.0;
        let pending_reps = if n <= 16_384 { 20 } else { 5 };
        for _ in 0..pending_reps {
            let p = pending(n);
            let t = Instant::now();
            let d = p.dup();
            dup_pending_total += t.elapsed().as_secs_f64() * 1e6;
            std::hint::black_box(&d);

            let p2 = pending(n);
            let t = Instant::now();
            p2.wait();
            wait_total += t.elapsed().as_secs_f64() * 1e6;
        }

        println!(
            "{:>10}  {:>12.1}  {:>12.1}  {:>12.1}",
            n,
            dup_assembled,
            dup_pending_total / f64::from(pending_reps),
            wait_total / f64::from(pending_reps),
        );
    }
    println!("\nper-entry cost (us per 1k entries), assembled dup:");
    for &n in &[1_024u64, 16_384, 262_144, 1_048_576] {
        let a = assembled(n);
        let us = time_us(if n <= 16_384 { 200 } else { 20 }, || {
            let d = a.dup();
            std::hint::black_box(&d);
        });
        println!("{:>10}  {:>10.3} us/1k", n, us / (n as f64 / 1000.0));
    }
}

#[test]
#[ignore = "measurement, not a correctness check"]
fn fold_cost_fold_vs_base() {
    ensure_init();
    println!("\n=== fold cost (eWiseAdd base<-delta) vs base size ===");
    println!(
        "{:>12}  {:>10}  {:>12}  {:>14}",
        "base nvals", "delta", "fold us", "us per 1k base"
    );
    for &b in &[1_024u64, 16_384, 262_144, 1_048_576] {
        for &d in &[256u64, 4_096, 65_536] {
            if d > b {
                continue;
            }
            let delta = assembled(d);
            let reps = if b <= 16_384 { 50 } else { 10 };
            let base = assembled(b);
            let mut total = 0.0;
            for _ in 0..reps {
                // Mirror VersionedMatrix::flush: build the folded base into a
                // fresh matrix (under MVCC the base is always shared, so an
                // in-place fold would deep-copy it first).
                let t = Instant::now();
                let mut new_m = Matrix::<bool>::new(CAP, CAP);
                new_m.element_wise_add(None, Some(&base), Some(&delta), None);
                new_m.wait();
                total += t.elapsed().as_secs_f64() * 1e6;
                std::hint::black_box(&new_m);
            }
            let us = total / f64::from(reps);
            println!(
                "{:>12}  {:>10}  {:>12.1}  {:>14.3}",
                b,
                d,
                us,
                us / (b as f64 / 1000.0)
            );
        }
    }
}

#[test]
#[ignore = "measurement, not a correctness check"]
fn fold_cost_empty_matrix_floor() {
    ensure_init();
    // What does an empty-but-high-capacity matrix cost to dup and fold? This
    // is the `O(nrows)` term the policy's `base_cost = nvals + nrows` models.
    println!("\n=== capacity-only cost (nvals = 0) ===");
    println!("{:>12}  {:>10}  {:>10}", "nrows", "dup us", "fold us");
    for &cap in &[1_024u64, 65_536, 1_048_576, 16_777_216] {
        let empty = Matrix::<bool>::new(cap, cap);
        let dup_us = time_us(50, || {
            let d = empty.dup();
            std::hint::black_box(&d);
        });
        let mut delta = Matrix::<bool>::new(cap, cap);
        delta.build(&[0, cap / 2], &[1, cap / 2]);
        delta.wait();
        let base = Matrix::<bool>::new(cap, cap);
        let mut total = 0.0;
        for _ in 0..20 {
            let t = Instant::now();
            let mut new_m = Matrix::<bool>::new(cap, cap);
            new_m.element_wise_add(None, Some(&base), Some(&delta), None);
            new_m.wait();
            total += t.elapsed().as_secs_f64() * 1e6;
            std::hint::black_box(&new_m);
        }
        println!("{:>12}  {:>10.2}  {:>10.2}", cap, dup_us, total / 20.0);
    }
}

#[test]
#[ignore = "measurement, not a correctness check"]
fn fold_cost_write_cycle() {
    ensure_init();
    // The actual per-transaction tax the policy models as `O(|delta|)`: take a
    // delta that already holds D entries, COW-dup it (new MVCC version), add
    // `t` more, then materialize. If this is flat in D the sqrt term is
    // unnecessary; if it grows with D the term is justified.
    println!("\n=== write-cycle cost vs existing delta size ===");
    println!(
        "{:>10}  {:>4}  {:>10}  {:>10}  {:>10}  {:>10}",
        "delta D", "t", "dup us", "set us", "wait us", "total us"
    );
    for &d in &[0u64, 1_024, 4_096, 16_384, 65_536, 262_144] {
        for &t in &[1u64, 100] {
            let reps: u32 = if d <= 16_384 { 30 } else { 10 };
            let (mut dup_us, mut set_us, mut wait_us) = (0.0, 0.0, 0.0);
            for r in 0..reps {
                let base_delta = assembled(d);

                let t0 = Instant::now();
                let mut v = base_delta.dup();
                dup_us += t0.elapsed().as_secs_f64() * 1e6;

                let t1 = Instant::now();
                for k in 0..t {
                    // Rows past the existing delta so these are new entries.
                    let row = (d + k + u64::from(r) * 1_000) % CAP;
                    v.set(row, (row * 13 + 1) % CAP, true);
                }
                set_us += t1.elapsed().as_secs_f64() * 1e6;

                let t2 = Instant::now();
                v.wait();
                wait_us += t2.elapsed().as_secs_f64() * 1e6;
                std::hint::black_box(&v);
            }
            let r = f64::from(reps);
            println!(
                "{:>10}  {:>4}  {:>10.1}  {:>10.1}  {:>10.1}  {:>10.1}",
                d,
                t,
                dup_us / r,
                set_us / r,
                wait_us / r,
                (dup_us + set_us + wait_us) / r
            );
        }
    }
}
