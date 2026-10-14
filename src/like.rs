use crate::formatting::Pokemon;

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let n = a.len();
    let m = b.len();

    let mut matrix = vec![vec![0usize; m + 1]; n + 1];

    for i in 0..=n {
        matrix[i][0] = i;
    }
    for j in 0..=m {
        matrix[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            let delete = matrix[i - 1][j] + 1;
            let insert = matrix[i][j - 1] + 1;
            let substitute = matrix[i - 1][j - 1] + cost;
            matrix[i][j] = delete.min(insert).min(substitute);
        }
    }

    matrix[n][m]
}

pub fn closest<'a>(query: &str, list: &'a [Pokemon]) -> Option<&'a Pokemon> {
    let query = query.to_lowercase();
    list.iter()
        .min_by_key(|p| levenshtein(&query, &p.name.to_lowercase()))
}
