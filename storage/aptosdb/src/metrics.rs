// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_metrics::{
    register_histogram_vec, register_int_counter, register_int_gauge, register_int_gauge_vec,
    HistogramVec, IntCounter, IntGauge, IntGaugeVec,
};
use once_cell::sync::Lazy;

pub static APTOS_STORAGE_LEDGER: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_storage_ledger",
        // metric description
        "Aptos storage ledger counters",
        // metric labels (dimensions)
        &["type"]
    )
    .unwrap()
});

pub static APTOS_STORAGE_COMMITTED_TXNS: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "aptos_storage_committed_txns",
        "Aptos storage committed transactions"
    )
    .unwrap()
});

pub static APTOS_STORAGE_LATEST_TXN_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_latest_transaction_version",
        "Aptos storage latest transaction version"
    )
    .unwrap()
});

pub static APTOS_STORAGE_LEDGER_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_ledger_version",
        "Version in the latest saved ledger info."
    )
    .unwrap()
});

pub static APTOS_STORAGE_NEXT_BLOCK_EPOCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_next_block_epoch",
        "ledger_info.next_block_epoch() for the latest saved ledger info."
    )
    .unwrap()
});

pub static APTOS_STORAGE_LATEST_ACCOUNT_COUNT: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_storage_latest_account_count",
        "Total number of account in the StateDB at the latest version."
    )
    .unwrap()
});

pub static APTOS_STORAGE_PRUNE_WINDOW: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("aptos_storage_prune_window", "Aptos storage prune window").unwrap()
});

/// DB pruner least readable versions
pub static APTOS_PRUNER_LEAST_READABLE_VERSION: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_pruner_least_readable_version",
        // metric description
        "Aptos pruner least readable state version",
        // metric labels (dimensions)
        &["pruner_name",]
    )
    .unwrap()
});

pub static APTOS_STORAGE_API_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "aptos_storage_api_latency_seconds",
        // metric description
        "Aptos storage api latency in seconds",
        // metric labels (dimensions)
        &["api_name", "result"]
    )
    .unwrap()
});

pub static APTOS_STORAGE_OTHER_TIMERS_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        // metric name
        "aptos_storage_other_timers_seconds",
        // metric description
        "Various timers below public API level.",
        // metric labels (dimensions)
        &["name"]
    )
    .unwrap()
});

/// Rocksdb metrics
pub static APTOS_STORAGE_ROCKSDB_PROPERTIES: Lazy<IntGaugeVec> = Lazy::new(|| {
    register_int_gauge_vec!(
        // metric name
        "aptos_rocksdb_properties",
        // metric description
        "rocksdb integer properties",
        // metric labels (dimensions)
        &["cf_name", "property_name",]
    )
    .unwrap()
});

// Backup progress gauges:

pub(crate) static BACKUP_EPOCH_ENDING_EPOCH: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_epoch_ending_epoch",
        "Current epoch returned in an epoch ending backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_TXN_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_transaction_version",
        "Current version returned in a transaction backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_STATE_SNAPSHOT_VERSION: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_state_snapshot_version",
        "Version of requested state snapshot backup."
    )
    .unwrap()
});

pub(crate) static BACKUP_STATE_SNAPSHOT_LEAF_IDX: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(
        "aptos_backup_handler_state_snapshot_leaf_index",
        "Index of current leaf index returned in a state snapshot backup."
    )
    .unwrap()
});
