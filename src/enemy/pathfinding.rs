use std::{cmp::Reverse, collections::HashMap};

use bevy::prelude::*;
use priority_queue::PriorityQueue;

use crate::prelude::*;

pub fn a_star(start: &IVec2, dest: &IVec2, grid: &Res<Grid>) -> Option<Vec<IVec2>> {
    // trivial case
    if start == dest {
        return Some(Vec::new());
    }

    // info!("starting a star search {:?} {:?}", start, dest);
    let mut search: PriorityQueue<IVec2, Reverse<i32>> = PriorityQueue::new();
    search.push_decrease(*start, Reverse(0));

    let mut costs: HashMap<IVec2, i32> = HashMap::new();
    let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();

    costs.insert(*start, 0);

    while !search.is_empty() {
        let (cur_pos, _cur_cost) = search.pop().unwrap();
        // info!("searching {:?}", cur_pos);

        // done
        if cur_pos == *dest {
            let mut pos = dest;
            let mut path: Vec<IVec2> = Vec::new();
            loop {
                let prev_pos = came_from.get(pos).unwrap();
                if prev_pos == start {
                    break;
                }
                path.push(*prev_pos);
                pos = prev_pos;
            }
            path.reverse();
            // info!("path {:?}", path);
            return Some(path);
        }

        // insert potential search cells
        for next_pos in tiles_around(&cur_pos, grid).iter() {
            let new_cost = costs.get(&cur_pos).unwrap() + 1;
            if costs.get(next_pos).unwrap_or(&std::i32::MAX) > &new_cost {
                search.push_decrease(*next_pos, Reverse(new_cost + heuristic(next_pos, dest)));
                costs.insert(*next_pos, new_cost);
                came_from.insert(*next_pos, cur_pos);
            }
        }
    }

    None
}

fn tiles_around(pos: &IVec2, grid: &Res<Grid>) -> Vec<IVec2> {
    use rand::{seq::SliceRandom, thread_rng};

    let mut dirs = [
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(0, 1),
        IVec2::new(0, -1),
    ];

    dirs.shuffle(&mut thread_rng());

    dirs.into_iter()
        .map(|d| *pos + d)
        .filter(|pos| grid.bounds_check(pos) && !grid.contains_at(pos, CellType::Wall).unwrap())
        .collect()
}

fn heuristic(cur: &IVec2, dest: &IVec2) -> i32 {
    // manhatten distance
    (cur.x - dest.x).abs() + (cur.y - dest.y).abs()
}
