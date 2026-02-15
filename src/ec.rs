/// Compute Eigenvector Centrality using power iteration
///
/// For bipartite graphs, standard power iteration oscillates because
/// the adjacency matrix has eigenvalues that come in +/- pairs.
///
/// Fix: Use A² (squared adjacency matrix) which has only positive eigenvalues.
/// The eigenvector of A² corresponding to λ_max² is the same as for A.

/// Compute A² (matrix squared)
fn square_matrix<const N: usize>(matrix: &[[f64; N]; N]) -> [[f64; N]; N] {
    let mut result = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                result[i][j] += matrix[i][k] * matrix[k][j];
            }
        }
    }
    result
}

/// Compute Eigenvector Centrality using power iteration on A²
///
/// Input: symmetric adjacency matrix where A[i][j] = edge weight between nodes i and j
/// Output: EC score for each node
pub fn power_iteration<const N: usize>(matrix: &[[f64; N]; N]) -> [f64; N] {
    let matrix_squared = square_matrix(matrix);
    let mut x = [1.0; N];

    for _ in 0..1000 {
        let mut x_new = [0.0; N];
        for i in 0..N {
            for j in 0..N {
                x_new[i] += matrix_squared[i][j] * x[j];
            }
        }

        let norm: f64 = x_new.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm < 1e-15 {
            break;
        }

        for i in 0..N {
            x_new[i] /= norm;
        }

        let diff: f64 = x
            .iter()
            .zip(x_new.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        x = x_new;

        if diff < 1e-10 {
            break;
        }
    }

    x.map(|v| v.abs())
}

/// Normalize EC scores: x̄_u = x_u / x_max
///
/// Returns values between 0 and 1, where 1 = highest EC in the graph
pub fn normalize_ec<const N: usize>(ec: &[f64; N]) -> [f64; N] {
    let x_max = ec.iter().cloned().fold(0.0_f64, f64::max);
    if x_max < 1e-15 {
        return *ec;
    }
    ec.map(|v| v / x_max)
}
