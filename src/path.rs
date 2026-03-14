use std::{collections::HashSet, hash::Hash};

pub fn dfs<Pos: Clone + Hash + Eq>(
    starts: &[Pos],
    reachable: impl FnMut(Pos) -> Vec<Pos>,
) -> impl Iterator<Item = Pos> {
    Dfs {
        stack: starts.to_vec(),
        visited: starts.iter().cloned().collect::<HashSet<_>>(),
        reachable,
        to_emit: starts.to_vec(),
    }
}

struct Dfs<Pos: Clone + Hash + Eq, R: FnMut(Pos) -> Vec<Pos>> {
    stack: Vec<Pos>,
    visited: HashSet<Pos>,
    reachable: R,
    to_emit: Vec<Pos>,
}

impl<Pos: Clone + Hash + Eq, R: FnMut(Pos) -> Vec<Pos>> Iterator for Dfs<Pos, R> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(p) = self.to_emit.pop() {
                return Some(p);
            }
            if let Some(p) = self.stack.pop() {
                let mut reachable = (self.reachable)(p);
                reachable.retain(|p| !self.visited.contains(p));
                self.visited.extend(reachable.iter().cloned());
                self.stack.extend(reachable.iter().cloned());
                self.to_emit.extend(reachable);
            } else {
                return None;
            }
        }
    }
}

pub fn bfs_paths<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>>(
    starts: &[Pos],
    maxdist: usize,
    reachable: impl FnMut(Pos) -> T,
) -> impl Iterator<Item = Vec<Pos>> {
    BfsPaths {
        periphery: starts.iter().map(|p| vec![p.clone()]).collect(),
        new_periphery: vec![],
        visited: starts.iter().cloned().collect::<HashSet<_>>(),
        reachable,
        to_emit: starts.iter().map(|p| vec![p.clone()]).collect(),
        maxdist,
    }
}

struct BfsPaths<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>, R: FnMut(Pos) -> T> {
    periphery: Vec<Vec<Pos>>,
    new_periphery: Vec<Vec<Pos>>,
    visited: HashSet<Pos>,
    reachable: R,
    to_emit: Vec<Vec<Pos>>,
    maxdist: usize,
}

impl<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>, R: FnMut(Pos) -> T> Iterator
    for BfsPaths<Pos, T, R>
{
    type Item = Vec<Pos>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(path) = self.to_emit.pop() {
                return Some(path);
            }
            if let Some(mut path) = self.periphery.pop() {
                let reachable = (self.reachable)(path.last().unwrap().clone()).into_iter();
                for pos in reachable.into_iter() {
                    if !self.visited.contains(&pos) {
                        self.visited.insert(pos.clone());
                        path.push(pos);
                        self.to_emit.push(path.clone());
                        if path.len() < self.maxdist {
                            self.new_periphery.push(path.clone());
                        }
                        path.pop();
                    }
                }
            } else if !self.new_periphery.is_empty() {
                std::mem::swap(&mut self.periphery, &mut self.new_periphery);
            } else {
                return None;
            }
        }
    }
}

pub fn bfs_dist<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>>(
    starts: &[Pos],
    maxdist: usize,
    reachable: impl FnMut(Pos) -> T,
) -> impl Iterator<Item = (usize, Pos)> {
    BfsDist {
        periphery: starts.iter().cloned().map(|p| (0, p)).collect(),
        new_periphery: vec![],
        visited: starts.iter().cloned().collect::<HashSet<Pos>>(),
        reachable,
        to_emit: starts.iter().cloned().map(|p| (0, p)).collect(),
        maxdist,
    }
}

struct BfsDist<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>, R: FnMut(Pos) -> T> {
    periphery: Vec<(usize, Pos)>,
    new_periphery: Vec<(usize, Pos)>,
    visited: HashSet<Pos>,
    reachable: R,
    to_emit: Vec<(usize, Pos)>,
    maxdist: usize,
}

impl<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>, R: FnMut(Pos) -> T> Iterator
    for BfsDist<Pos, T, R>
{
    type Item = (usize, Pos);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(path) = self.to_emit.pop() {
                return Some(path);
            }
            if let Some((dist, node)) = self.periphery.pop() {
                let reachable = (self.reachable)(node.clone()).into_iter();
                for pos in reachable.into_iter() {
                    if !self.visited.contains(&pos) {
                        self.visited.insert(pos.clone());
                        self.to_emit.push((dist + 1, pos.clone()));
                        if dist + 1 < self.maxdist {
                            self.new_periphery.push((dist + 1, pos.clone()));
                        }
                    }
                }
            } else if !self.new_periphery.is_empty() {
                std::mem::swap(&mut self.periphery, &mut self.new_periphery);
            } else {
                return None;
            }
        }
    }
}

pub fn bfs<Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos>>(
    starts: &[Pos],
    maxdist: usize,
    reachable: impl FnMut(Pos) -> T,
) -> impl Iterator<Item = Pos> {
    bfs_dist(starts, maxdist, reachable).map(|(_, p)| p)
}

pub fn build_dijkstra_map<'a, Pos: Clone + Hash + Eq, T: IntoIterator<Item = Pos> + 'a>(
    starts: &'a [Pos],
    maxdist: usize,
    reachable: impl FnMut(Pos) -> T + 'a,
) -> impl Iterator<Item = (Pos, usize)> + 'a {
    starts
        .iter()
        .map(|p| (p.clone(), 0))
        .chain(bfs_dist(starts, maxdist, reachable).map(|(dist, pos)| (pos, dist)))
}
