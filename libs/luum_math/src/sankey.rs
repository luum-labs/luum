use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SankeyNode {
    pub id: String,
    pub label: String,
    pub value: u64,
    pub depth: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SankeyLink {
    pub source: String,
    pub target: String,
    pub value: u64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SankeyGraph {
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
    pub total_flow: u64,
}

pub struct SankeyBuilder {
    source_label: String,
    min_link_value: u64,
    max_depth: u8,
}

impl SankeyBuilder {
    pub fn new(source_label: &str) -> Self {
        Self {
            source_label: source_label.to_string(),
            min_link_value: 0,
            max_depth: 3,
        }
    }

    pub fn with_min_value(mut self, min: u64) -> Self {
        self.min_link_value = min;
        self
    }

    pub fn with_max_depth(mut self, depth: u8) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn build(&self, flows: &[(String, u64, String)]) -> SankeyGraph {
        let mut nodes = Vec::new();
        let mut links = Vec::new();
        let mut total_flow: u64 = 0;
        let mut seen_nodes = std::collections::HashSet::new();

        let source_id = format!("node_{}", self.source_label);
        nodes.push(SankeyNode {
            id: source_id.clone(),
            label: self.source_label.clone(),
            value: 0,
            depth: 0,
        });
        seen_nodes.insert(source_id.clone());

        for (target, value, category) in flows {
            if *value < self.min_link_value {
                continue;
            }

            let target_id = format!("node_{}", target);
            if !seen_nodes.contains(&target_id) {
                nodes.push(SankeyNode {
                    id: target_id.clone(),
                    label: Self::truncate_address(target),
                    value: *value,
                    depth: 1,
                });
                seen_nodes.insert(target_id.clone());
            }

            links.push(SankeyLink {
                source: source_id.clone(),
                target: target_id,
                value: *value,
                category: category.clone(),
            });

            total_flow = total_flow.saturating_add(*value);
        }

        if let Some(src) = nodes.first_mut() {
            src.value = total_flow;
        }

        links.sort_by(|a, b| b.value.cmp(&a.value));

        SankeyGraph {
            nodes,
            links,
            total_flow,
        }
    }

    fn truncate_address(addr: &str) -> String {
        if addr.len() > 10 {
            format!("{}...{}", &addr[..4], &addr[addr.len() - 4..])
        } else {
            addr.to_string()
        }
    }
}
