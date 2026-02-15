/// Graph Value Calculation
///
/// From the paper, Graph Value measures a producer's contribution to the network.
/// Formula: GV = W^x̄ · x^(1-x̄) · r
///
/// Where:
/// - W = total edge weight for producer (sum of all transaction fees)
/// - x̄ = normalized EC score (0 to 1, where 1 = highest EC in graph)
/// - x = raw EC score
/// - r = reputation score (from peer reviews)
///
/// Intuition:
/// - High EC (x̄ → 1): GV ≈ W · r (volume and reputation matter most)
/// - Low EC (x̄ → 0): GV ≈ x · r (connectivity matters most)
/// - This balances between rewarding volume vs connectivity

/// Calculate Graph Value for a single producer
///
/// Returns: W^x̄ · x^(1-x̄) · r
pub fn graph_value(total_weight: f64, normalized_ec: f64, raw_ec: f64, reputation: f64) -> f64 {
    // Handle edge cases
    if total_weight <= 0.0 || raw_ec <= 0.0 || reputation <= 0.0 {
        return 0.0;
    }

    // W^x̄ - weight contribution (higher x̄ = more weight influence)
    let weight_term = total_weight.powf(normalized_ec);

    // x^(1-x̄) - EC contribution (lower x̄ = more EC influence)
    let ec_term = raw_ec.powf(1.0 - normalized_ec);

    // Final graph value
    weight_term * ec_term * reputation
}

/// Calculate total edge weight for a node in the graph
///
/// W_u = Σ w(u,v) for all neighbors v
pub fn total_weight<const N: usize>(weights: &[[f64; N]; N], node: usize) -> f64 {
    weights[node].iter().sum()
}

/// Calculate Graph Value for all producers in the graph
///
/// Input:
/// - weights: adjacency matrix
/// - ec: raw EC scores
/// - normalized_ec: normalized EC scores (0 to 1)
/// - reputations: reputation score for each node
/// - producer_indices: which indices are producers
///
/// Output: Graph Value for each producer
pub fn graph_values<const N: usize>(
    weights: &[[f64; N]; N],
    ec: &[f64; N],
    normalized_ec: &[f64; N],
    reputations: &[f64; N],
    producer_indices: &[usize],
) -> Vec<(usize, f64)> {
    producer_indices
        .iter()
        .map(|&i| {
            let w = total_weight(weights, i);
            let gv = graph_value(w, normalized_ec[i], ec[i], reputations[i]);
            (i, gv)
        })
        .collect()
}

/// Normalize Graph Values to sum to 1.0 (for reward distribution)
///
/// Returns the fraction of total rewards each producer should receive
pub fn normalize_graph_values(gvs: &[(usize, f64)]) -> Vec<(usize, f64)> {
    let total: f64 = gvs.iter().map(|(_, gv)| gv).sum();

    if total < 1e-15 {
        return gvs.iter().map(|&(i, _)| (i, 0.0)).collect();
    }

    gvs.iter().map(|&(i, gv)| (i, gv / total)).collect()
}

/// Bare graph value change after a transaction
///
/// ΔG_u = (W_u + ΔW_u)^x̄ · (x_u + Δx_u)^(1-x̄) · (r_u + Δr_u) - W_u^x̄ · x_u^(1-x̄) · r_u
pub fn bare_graph_value_change(
    old_weight: f64,
    delta_weight: f64,
    old_ec: f64,
    delta_ec: f64,
    normalized_ec: f64,
    old_reputation: f64,
    delta_reputation: f64,
) -> f64 {
    let old_gv = graph_value(old_weight, normalized_ec, old_ec, old_reputation);
    let new_gv = graph_value(
        old_weight + delta_weight,
        normalized_ec,
        old_ec + delta_ec,
        old_reputation + delta_reputation,
    );
    new_gv - old_gv
}

/// Calculate the performance multiplier α_u
///
/// α_u = Max(0, 1 + 2 * ((ΔG_u/ΔW_u) - avg) / (|ΔG_u/ΔW_u| + |avg|))
pub fn performance_multiplier(delta_g: f64, delta_w: f64, average_ratio: f64) -> f64 {
    if delta_w.abs() < 1e-15 {
        return 0.0;
    }

    let user_ratio = delta_g / delta_w;
    let numerator = user_ratio - average_ratio;
    let denominator = user_ratio.abs() + average_ratio.abs();

    if denominator < 1e-15 {
        return 1.0;
    }

    let b_u = 2.0 * numerator / denominator;
    (1.0 + b_u).max(0.0)
}
