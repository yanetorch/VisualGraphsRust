//! Алгоритм обхода в глубину

pub fn dfs(cur: usize, p: usize, v: &Vec<Vec<usize>>, visits: &mut Vec<bool>) {
    visits[cur] = true;
    for &i in &v[cur] {
        if i == p || visits[i] {
            continue;
        }
        dfs(i, cur, v, visits);
    }
}
