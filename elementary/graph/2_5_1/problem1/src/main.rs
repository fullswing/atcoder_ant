#[warn(unused_imports)]
use std::io::*;
use std::str::FromStr;
use std::cmp;
 
#[warn(unused_variables)]
 
fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}
 
fn dfs(v: usize, c: i64, graph: &Vec<Vec<usize>>, color: &mut Vec<i64>) -> bool {
    color[v] = c;
    //println!("{:?}", color);
    for i in 0..graph[v].len() {
        if color[graph[v][i]] == c {
            return false;
        }
        if color[graph[v][i]] == 0 && !dfs(graph[v][i], -c, &graph, color) {
            return false;
        }
    }
    return true;
}
 
fn solve(graph: &mut Vec<Vec<usize>>, n: usize, m: usize) {
    let mut color = vec![0; n+1];
    color[1] = 1;
    let bipartite: bool = dfs(1, 1, &graph, &mut color);
    if bipartite {
        let mut k = 0;
        for i in 0..color.len() {
            if color[i] == 1 {
                k += 1;
            }
        }
        let n = n as i64;
        let m = m as i64;
        println!("{:?}", k * (n - k) - m);
    } else {
        println!("{}", n * (n-1) / 2 - m);
    }
}
 
fn main() {
    let n = read::<usize>(); // 頂点数
    let m = read::<usize>(); // 辺の数
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for _ in 0..m {
        let a = read::<usize>();
        let b = read::<usize>();
        graph[a].push(b);
        graph[b].push(a);
    }
    solve(&mut graph, n, m);
}
