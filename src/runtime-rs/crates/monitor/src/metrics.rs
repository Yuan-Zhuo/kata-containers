// Copyright 2021-2022 Kata Contributors
//
// SPDX-License-Identifier: Apache-2.0
//

extern crate procfs;

use anyhow::{anyhow, Result};
use prometheus::{Encoder, Gauge, IntCounter, Registry, TextEncoder};
use std::sync::Once;

const NAMESPACE_KATA_MONITOR: &str = "kata_monitor";

lazy_static! {

    static ref INIT_REGISTER: Once = Once::new();

    // custom registry
    static ref REGISTRY: Registry = Registry::new();

    // monitor metrics
    static ref MONITOR_SCRAPE_COUNT: IntCounter =
    IntCounter::new(format!("{}_{}",NAMESPACE_KATA_MONITOR,"scrape_count"), "Monitor scrape count").unwrap();

    static ref MONITOR_MAX_FDS: Gauge = Gauge::new(format!("{}_{}", NAMESPACE_KATA_MONITOR, "process_max_fds"), "Open FDs for monitor").unwrap();

    static ref MONITOR_OPEN_FDS: Gauge = Gauge::new(format!("{}_{}", NAMESPACE_KATA_MONITOR, "process_open_fds"), "Open FDs for monitor").unwrap();

    static ref MONITOR_RESIDENT_MEMORY: Gauge = Gauge::new(format!("{}_{}", NAMESPACE_KATA_MONITOR, "process_resident_memory_bytes"), "Resident memory size in bytes for monitor").unwrap();

    // TODO:
    // MONITOR_SCRAPE_FAILED_COUNT & MONITOR_SCRAPE_DURATIONS_HISTOGRAM & MONITOR_RUNNING_SHIM_COUNT

    //  static ref MONITOR_SCRAPE_FAILED_COUNT: IntCounter = IntCounter::new(format!("{}_{}",NAMESPACE_KATA_MONITOR,"scrape_failed_count"), "Monitor scrape failed count").unwrap();

    // static ref MONITOR_SCRAPE_DURATIONS_HISTOGRAM: HistogramVec = HistogramVec::new(HistogramOpts::new(format!("{}_{}", NAMESPACE_KATA_MONITOR, "scrape_durations_histogram_milliseconds"),"Time used to scrape"),&["action"]).unwrap();

    // static ref MONITOR_RUNNING_SHIM_COUNT: Gauge = Gauge::new(format!("{}_{}",NAMESPACE_KATA_MONITOR,"running_shim_count"), "Running shim count(running sandboxes).").unwrap();
}

/// get prometheus metrics
pub async fn get_metrics() -> Result<String> {
    let handle_init = tokio::task::spawn(async move {
        INIT_REGISTER.call_once_force(|_| {
            register_metrics().unwrap();
        });
    });
    if handle_init.await.is_err() {
        return Err(anyhow!("failed to init register"));
    }

    update_metrics()?;

    // gather all metrics and return as a String
    let metric_families = REGISTRY.gather();

    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer)?;

    Ok(String::from_utf8(buffer)?)
}

fn register_metrics() -> Result<()> {
    REGISTRY.register(Box::new(MONITOR_SCRAPE_COUNT.clone()))?;
    REGISTRY.register(Box::new(MONITOR_MAX_FDS.clone()))?;
    REGISTRY.register(Box::new(MONITOR_OPEN_FDS.clone()))?;
    REGISTRY.register(Box::new(MONITOR_RESIDENT_MEMORY.clone()))?;

    // TODO:
    // REGISTRY.register(Box::new(MONITOR_SCRAPE_FAILED_COUNT.clone()))?;
    // REGISTRY.register(Box::new(MONITOR_SCRAPE_DURATIONS_HISTOGRAM.clone()))?;
    // REGISTRY.register(Box::new(MONITOR_RUNNING_SHIM_COUNT.clone()))?;
    Ok(())
}

fn update_metrics() -> Result<()> {
    MONITOR_SCRAPE_COUNT.inc();

    let me = match procfs::process::Process::myself() {
        Ok(p) => p,
        Err(_) => {
            return Ok(());
        }
    };

    if let Ok(fds) = procfs::sys::fs::file_max() {
        MONITOR_MAX_FDS.set(fds as f64);
    }

    if let Ok(fds) = me.fd_count() {
        MONITOR_OPEN_FDS.set(fds as f64);
    }

    if let Ok(statm) = me.statm() {
        MONITOR_RESIDENT_MEMORY.set(statm.resident as f64);
    }

    // TODO:
    // MONITOR_SCRAPE_FAILED_COUNT & MONITOR_SCRAPE_DURATIONS_HISTOGRAM & MONITOR_RUNNING_SHIM_COUNT

    Ok(())
}
