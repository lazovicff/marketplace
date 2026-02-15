mod ec;
mod graph;
mod reputation;

fn main() {
    // Peers 0 and 1 have higher weights which results in higher graph values
    let adjacency_matrix: [[f64; 5]; 5] = [
        [0.0, 20.0, 2.0, 0.0, 0.0],
        [20.0, 0.0, 1.5, 3.0, 0.0],
        [2.0, 1.5, 0.0, 0.0, 2.5],
        [0.0, 3.0, 0.0, 0.0, 1.0],
        [0.0, 2.0, 2.5, 1.0, 0.0],
    ];

    let ec = ec::power_iteration(&adjacency_matrix);
    let norm_ec = ec::normalize_ec(&ec);
    println!(
        "norm ec: [{:.6}, {:.6}, {:.6}, {:.6}, {:.6}]",
        norm_ec[0], norm_ec[1], norm_ec[2], norm_ec[3], norm_ec[4]
    );

    // Track all users
    let mut reputations = [reputation::R_MIN; 5];
    let mut tx_counts = [0u64; 5];

    // Helper to compute graph value for a user
    let compute_gv = |user: usize, reps: &[f64; 5]| -> f64 {
        let w = graph::total_weight(&adjacency_matrix, user);
        graph::graph_value(w, norm_ec[user], ec[user], reps[user])
    };

    // Track user 4 (producer)
    let producer = 4;
    println!("\n--- User {} initial state ---", producer);
    println!(
        "weight: {:.6}",
        graph::total_weight(&adjacency_matrix, producer)
    );
    println!("ec: {:.6}, norm_ec: {:.6}", ec[producer], norm_ec[producer]);
    println!("reputation: {:.6}", reputations[producer]);
    println!("graph value: {:.6}", compute_gv(producer, &reputations));

    // Transaction 1: buyer 1 rates producer 0
    println!("\n--- Transaction 1: buyer 2 rates producer 4 with 5.0 ---");
    let buyer = 2;
    let rating = 5.0;
    let buyer_gv = compute_gv(buyer, &reputations);
    let old_rep = reputations[producer];
    let old_gv = compute_gv(producer, &reputations);
    reputations[producer] =
        reputation::update_reputation(reputations[producer], tx_counts[producer], buyer_gv, rating);
    tx_counts[producer] += 1;
    println!("buyer {} graph value: {:.6}", buyer, buyer_gv);
    println!(
        "reputation: {:.6} -> {:.6} (Δ = {:.6})",
        old_rep,
        reputations[producer],
        reputations[producer] - old_rep
    );
    println!(
        "graph value: {:.6} -> {:.6}",
        old_gv,
        compute_gv(producer, &reputations)
    );

    // Transaction 2: buyer 1 rates producer 0
    println!("\n--- Transaction 2: buyer 3 rates producer 4 with 5.0 ---");
    let buyer = 3;
    let rating = 5.0;
    let buyer_gv = compute_gv(buyer, &reputations);
    let old_rep = reputations[producer];
    let old_gv = compute_gv(producer, &reputations);
    reputations[producer] =
        reputation::update_reputation(reputations[producer], tx_counts[producer], buyer_gv, rating);
    tx_counts[producer] += 1;
    println!("buyer {} graph value: {:.6}", buyer, buyer_gv);
    println!(
        "reputation: {:.6} -> {:.6} (Δ = {:.6})",
        old_rep,
        reputations[producer],
        reputations[producer] - old_rep
    );
    println!(
        "graph value: {:.6} -> {:.6}",
        old_gv,
        compute_gv(producer, &reputations)
    );

    // Transaction 3: buyer 1 rates producer 4
    println!("\n--- Transaction 3: buyer 1 rates producer 4 with 5.0 ---");
    let buyer = 1;
    let rating = 5.0;
    let buyer_gv = compute_gv(buyer, &reputations);
    let old_rep = reputations[producer];
    let old_gv = compute_gv(producer, &reputations);
    reputations[producer] =
        reputation::update_reputation(reputations[producer], tx_counts[producer], buyer_gv, rating);
    tx_counts[producer] += 1;
    println!("buyer {} graph value: {:.6}", buyer, buyer_gv);
    println!(
        "reputation: {:.6} -> {:.6} (Δ = {:.6})",
        old_rep,
        reputations[producer],
        reputations[producer] - old_rep
    );
    println!(
        "graph value: {:.6} -> {:.6}",
        old_gv,
        compute_gv(producer, &reputations)
    );
}

// EXECUTION LOGS: ------------------------------------------------------------
//
// norm ec: [0.991025, 1.000000, 0.184878, 0.152143, 0.127192]

// --- User 4 initial state ---
// weight: 5.500000
// ec: 0.088713, norm_ec: 0.127192
// reputation: 0.100000
// graph value: 0.014996

// --- Transaction 1: buyer 2 rates producer 4 with 5.0 ---
// buyer 2 graph value: 0.026227
// reputation: 0.100000 -> 0.131133 (Δ = 0.031133)
// graph value: 0.014996 -> 0.019664

// --- Transaction 2: buyer 3 rates producer 4 with 5.0 ---
// buyer 3 graph value: 0.018433
// reputation: 0.131133 -> 0.111649 (Δ = -0.019483)
// graph value: 0.019664 -> 0.016742

// --- Transaction 3: buyer 1 rates producer 4 with 5.0 ---
// buyer 1 graph value: 2.450000
// reputation: 0.111649 -> 4.157766 (Δ = 4.046117) <--- Big change in reputation of peer 4
// graph value: 0.016742 -> 0.623482
