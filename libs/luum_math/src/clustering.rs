use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverCluster {
    pub address: String,
    pub tx_count: u32,
    pub total_amount: u64,
    pub category: ClusterCategory,
    pub risk_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClusterCategory {
    ApiProvider,
    Oracle,
    Exchange,
    Compute,
    DataFeed,
    Unknown,
}

pub struct ClusterEngine {
    threshold_amount: u64,
    threshold_frequency: u32,
    min_cluster_size: usize,
}

impl ClusterEngine {
    pub fn new(threshold_amount: u64, threshold_frequency: u32) -> Self {
        Self {
            threshold_amount,
            threshold_frequency,
            min_cluster_size: 2,
        }
    }

    pub fn with_min_size(mut self, size: usize) -> Self {
        self.min_cluster_size = size;
        self
    }

    pub fn cluster(&self, transactions: &[(String, u64, u32)]) -> Vec<ReceiverCluster> {
        let mut grouped: HashMap<String, (u64, u32)> = HashMap::new();

        for (addr, amount, count) in transactions {
            let entry = grouped.entry(addr.clone()).or_insert((0, 0));
            entry.0 = entry.0.saturating_add(*amount);
            entry.1 = entry.1.saturating_add(*count);
        }

        let mut clusters: Vec<ReceiverCluster> = grouped
            .into_iter()
            .filter(|(_, (amt, cnt))| {
                *amt >= self.threshold_amount || *cnt >= self.threshold_frequency
            })
            .map(|(addr, (total_amount, tx_count))| {
                let category = Self::categorize(tx_count, total_amount);
                let risk_score = Self::compute_risk(tx_count, total_amount);
                ReceiverCluster {
                    address: addr,
                    tx_count,
                    total_amount,
                    category,
                    risk_score,
                }
            })
            .collect();

        clusters.sort_by(|a, b| b.total_amount.cmp(&a.total_amount));
        clusters
    }

    fn categorize(tx_count: u32, total_amount: u64) -> ClusterCategory {
        let avg = if tx_count > 0 {
            total_amount / tx_count as u64
        } else {
            0
        };

        if tx_count > 1000 && avg < 10_000 {
            ClusterCategory::ApiProvider
        } else if tx_count > 500 && avg < 50_000 {
            ClusterCategory::Oracle
        } else if tx_count < 10 && total_amount > 1_000_000 {
            ClusterCategory::Exchange
        } else if tx_count > 200 {
            ClusterCategory::Compute
        } else {
            ClusterCategory::Unknown
        }
    }

    fn compute_risk(tx_count: u32, total_amount: u64) -> f64 {
        let freq_factor = (tx_count as f64).ln().min(5.0) / 5.0;
        let amount_factor = (total_amount as f64).ln().min(15.0) / 15.0;
        ((freq_factor * 0.4 + amount_factor * 0.6) * 100.0).round()
    }
}
