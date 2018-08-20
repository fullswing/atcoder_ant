#[warn(unused_imports)]
use std::io::*;
use std::str::FromStr;
use std::collections::HashMap;
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

fn dfs(v: usize, c: i64, graph: &Vec<Vec<usize>>, color: &mut HashMap<usize, i64>) -> bool {
    //color[v] = c;
    color.insert(v, c);
    //println!("{:?}", color);
    for i in 0..graph[v].len() {
        //if color[graph[v][i]] == c {
        if color.contains_key(&graph[v][i]) && color[&graph[v][i]] == c {
            return false;
        }
        //if color[graph[v][i]] == 0 && !dfs(graph[v][i], -c, &graph, color) {
        if !color.contains_key(&graph[v][i]) && !dfs(graph[v][i], -c, &graph, color) {
            return false;
        }
    }
    return true;
}

fn check(color: & Vec<i64>) -> usize {
    for i in 1..color.len() {
        if color[i] == 0 {
            return i;
        }
    }
    return 0;
}

fn solve(graph: & Vec<Vec<usize>>, n: usize) {
    //let mut color = vec![0; n+1];
    let mut color = HashMap::new();
    //color[1] = 1;
    for v in 1..n+1 {
        //println!("{}", v);
        if !color.contains_key(&v) {
            let bipartite = dfs(v, 1, &graph, &mut color);
            if !bipartite {
                println!("-1");
                return;
            } 
        }
    }
    let mut k = 0;
    //println!("hoge");
    for i in 1..n+1 {
        if color[&i] == 1 {
            k += 1;
        }
    }
    let m = cmp::max(k, n - k);
    println!("{}", m);
}

fn main() {
    let n = read::<usize>(); // 頂点数
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n+1];
    for i in 1..n+1 {
        let a = read::<usize>();
        graph[i].push(a);
        graph[a].push(i);
    }
    solve(&graph, n);
}