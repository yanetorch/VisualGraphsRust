use crate::algorithm::dfs::dfs;

// Система предлагает пользователю задать граф матрицей смежности (или матрицей инци-
// дентности или списками смежности) и затем показывает пользователю степень каждой вершины,
// число компонент связности, является ли граф эйлеровым (полуэйлеровым), является ли граф
// двудольным (полным двудольным)

pub fn number_of_components_graph(v: &Vec<Vec<usize>>) -> usize {
    let mut cnt = 0;

    let mut visits = vec![false; v.len()];

    for i in 0..v.len() {
        if !visits[i] {
            cnt += 1;
            dfs(i, i, v, &mut visits);
        }
    }

    cnt
}

pub fn is_bipartite(v: &Vec<Vec<usize>>) -> bool {
    let mut ok = true;
    let mut color = vec![ 0; v.len() ];
    dfs_color(0, 1, &mut color, v, &mut ok);

    for i in 0..v.len() {
        if color[i] == 0 {
            dfs_color(i, 1, &mut color, v, &mut ok);
        }
        if !ok {
            break;
        }
    }

    ok
}

fn color_invert(c: u8) -> u8 {
    match c {
        1 => 2,
        _ => 1,
    }
}

fn dfs_color(v: usize, c: u8, color: &mut Vec<u8>, g: &Vec<Vec<usize>>, ok: &mut bool) {
    if *ok == false {
        return;
    }
    color[v] = c;
    for &u in &g[v] {
        if color[u] == 0 {
            dfs_color(u, color_invert(c), color, g, ok);
        } else if color[u] == c {
            *ok &= false;
        }
    }
}