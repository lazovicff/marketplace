/// Reputation Score
///
/// From the paper, Section 6.4:
/// - Users start with a minimum reputation r_min (not zero, to avoid zeroing graph values)
/// - After each transaction, buyer and producer mutually review each other
/// - Reviews are weighted by the reviewer's Graph Value
///
/// Update formula for producer u after transacting with buyer v:
///   r_u = (N_u * r_u + G_v * r_vu) / (N_u + 1)
///
/// Where:
/// - N_u = number of transactions producer has completed before this one
/// - G_v = graph value of the buyer giving the review
/// - r_vu = the rating buyer v gives to producer u (in range r_min to r_max)
///
/// Key insight: reviews from high graph-value users have more impact

/// Default minimum reputation (must be > 0 to avoid zeroing graph values)
pub const R_MIN: f64 = 0.1;

/// Default maximum reputation
pub const R_MAX: f64 = 5.0;

/// Clamp a rating to valid range
pub fn clamp_rating(rating: f64) -> f64 {
    rating.clamp(R_MIN, R_MAX)
}

/// Update reputation after a transaction
///
/// Arguments:
/// - current_reputation: user's current reputation score
/// - num_transactions: number of transactions completed before this one
/// - reviewer_graph_value: graph value of the user giving the review
/// - rating: the rating given (will be clamped to r_min..r_max)
///
/// Returns: new reputation score
pub fn update_reputation(
    current_reputation: f64,
    num_transactions: u64,
    reviewer_graph_value: f64,
    rating: f64,
) -> f64 {
    let rating = clamp_rating(rating);
    let n = num_transactions as f64;

    // r_u = (N_u * r_u + G_v * r_vu) / (N_u + 1)
    let numerator = n * current_reputation + reviewer_graph_value * rating;
    let denominator = n + 1.0;

    if denominator < 1e-15 {
        return current_reputation;
    }

    numerator / denominator
}

/// Mutual reputation update after a transaction
///
/// Both producer and buyer update each other's reputation
///
/// Returns: (new_producer_reputation, new_buyer_reputation)
pub fn mutual_update(
    producer_rep: f64,
    producer_tx_count: u64,
    producer_graph_value: f64,
    producer_rates_buyer: f64,
    buyer_rep: f64,
    buyer_tx_count: u64,
    buyer_graph_value: f64,
    buyer_rates_producer: f64,
) -> (f64, f64) {
    let new_producer_rep = update_reputation(
        producer_rep,
        producer_tx_count,
        buyer_graph_value,
        buyer_rates_producer,
    );

    let new_buyer_rep = update_reputation(
        buyer_rep,
        buyer_tx_count,
        producer_graph_value,
        producer_rates_buyer,
    );

    (new_producer_rep, new_buyer_rep)
}
