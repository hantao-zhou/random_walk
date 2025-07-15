use rand::Rng;
use std::collections::{HashMap, VecDeque};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WalkResult {
    x: Vec<f32>,
    y: Vec<f32>,
}

#[wasm_bindgen]
impl WalkResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            x: Vec::new(),
            y: Vec::new(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(self.x.as_slice())
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> js_sys::Float32Array {
        js_sys::Float32Array::from(self.y.as_slice())
    }
}

#[wasm_bindgen]
pub fn random_walk(steps: usize) -> WalkResult {
    let mut rng = rand::thread_rng();
    let mut x = Vec::with_capacity(steps + 1);
    let mut y = Vec::with_capacity(steps + 1);
    let mut cx = 0f32;
    let mut cy = 0f32;
    x.push(cx);
    y.push(cy);
    for _ in 0..steps {
        let dir = rng.gen_range(0..4);
        match dir {
            0 => cx += 1.0,
            1 => cx -= 1.0,
            2 => cy += 1.0,
            _ => cy -= 1.0,
        }
        x.push(cx);
        y.push(cy);
    }
    WalkResult { x, y }
}

// helper to canonicalize an edge between two lattice points
fn edge_key(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

/// Edge reinforced random walk with simple parameters.
/// `strength` is the reinforcement weight applied per visit once `delay` steps
/// have been taken. `memory` controls how many recent traversals are remembered.
/// `memory = 0` means infinite memory.
#[wasm_bindgen]
pub fn erw_walk(steps: usize, strength: f64, delay: usize, memory: usize) -> WalkResult {
    let mut rng = rand::thread_rng();
    let mut x = Vec::with_capacity(steps + 1);
    let mut y = Vec::with_capacity(steps + 1);
    let mut cx = 0i32;
    let mut cy = 0i32;
    x.push(cx as f32);
    y.push(cy as f32);

    let mut counts: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
    let mut history: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();

    for step in 0..steps {
        let neighbors = [
            (cx + 1, cy),
            (cx - 1, cy),
            (cx, cy + 1),
            (cx, cy - 1),
        ];
        let mut weights = [0.0f64; 4];
        let mut total = 0.0f64;
        for (i, nb) in neighbors.iter().enumerate() {
            let key = edge_key((cx, cy), *nb);
            let count = *counts.get(&key).unwrap_or(&0) as f64;
            let w = if step >= delay {
                1.0 + strength * count
            } else {
                1.0
            };
            weights[i] = w;
            total += w;
        }
        let mut choice = 0;
        let mut acc = 0.0f64;
        let r = rng.gen_range(0.0..total);
        for (i, w) in weights.iter().enumerate() {
            acc += *w;
            if r <= acc {
                choice = i;
                break;
            }
        }
        let (nx, ny) = neighbors[choice];
        let key = edge_key((cx, cy), (nx, ny));
        *counts.entry(key).or_insert(0) += 1;
        history.push_back(key);
        if memory > 0 && history.len() > memory {
            if let Some(old) = history.pop_front() {
                if let Some(v) = counts.get_mut(&old) {
                    *v = v.saturating_sub(1);
                }
            }
        }
        cx = nx;
        cy = ny;
        x.push(cx as f32);
        y.push(cy as f32);
    }
    WalkResult { x, y }
}

/// Monte Carlo estimate of return time for the edge reinforced walk.
#[wasm_bindgen]
pub fn monte_carlo_erw_et(
    trials: usize,
    steps: usize,
    strength: f64,
    delay: usize,
    memory: usize,
) -> f64 {
    let mut total = 0u64;
    for _ in 0..trials {
        let mut rng = rand::thread_rng();
        let mut counts: HashMap<((i32, i32), (i32, i32)), u32> = HashMap::new();
        let mut history: VecDeque<((i32, i32), (i32, i32))> = VecDeque::new();
        let mut cx = 0i32;
        let mut cy = 0i32;
        for step_idx in 1..=steps {
            let neighbors = [
                (cx + 1, cy),
                (cx - 1, cy),
                (cx, cy + 1),
                (cx, cy - 1),
            ];
            let mut weights = [0.0f64; 4];
            let mut tot = 0.0f64;
            for (i, nb) in neighbors.iter().enumerate() {
                let key = edge_key((cx, cy), *nb);
                let count = *counts.get(&key).unwrap_or(&0) as f64;
                let w = if step_idx > delay { 1.0 + strength * count } else { 1.0 };
                weights[i] = w;
                tot += w;
            }
            let r = rng.gen_range(0.0..tot);
            let mut acc = 0.0f64;
            let mut choice = 0;
            for (i, w) in weights.iter().enumerate() {
                acc += *w;
                if r <= acc {
                    choice = i;
                    break;
                }
            }
            let (nx, ny) = neighbors[choice];
            let key = edge_key((cx, cy), (nx, ny));
            *counts.entry(key).or_insert(0) += 1;
            history.push_back(key);
            if memory > 0 && history.len() > memory {
                if let Some(old) = history.pop_front() {
                    if let Some(v) = counts.get_mut(&old) {
                        *v = v.saturating_sub(1);
                    }
                }
            }
            cx = nx;
            cy = ny;
            if cx == 0 && cy == 0 {
                total += step_idx as u64;
                break;
            }
            if step_idx == steps {
                total += steps as u64;
            }
        }
    }
    total as f64 / trials as f64
}

// simple monte carlo simulation to estimate hitting time back to origin
#[wasm_bindgen]
pub fn monte_carlo_et(trials: usize, steps: usize) -> f64 {
    let mut total = 0u64;
    for _ in 0..trials {
        let mut rng = rand::thread_rng();
        let mut cx = 0i32;
        let mut cy = 0i32;
        for step in 1..=steps {
            let dir = rng.gen_range(0..4);
            match dir {
                0 => cx += 1,
                1 => cx -= 1,
                2 => cy += 1,
                _ => cy -= 1,
            }
            if cx == 0 && cy == 0 {
                total += step as u64;
                break;
            }
            if step == steps {
                total += steps as u64;
            }
        }
    }
    total as f64 / trials as f64
}
