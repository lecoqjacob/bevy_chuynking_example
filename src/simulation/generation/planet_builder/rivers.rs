use super::*;
use bevy::utils::HashSet;
use bracket_random::prelude::RandomNumberGenerator;

pub fn run_rivers(planet: &mut Planet) {
    let mut rng = RandomNumberGenerator::seeded(planet.rng_seed);

    let n_rivers = WORLD_WIDTH / 2;
    let mut used_starts: HashSet<usize> = HashSet::new();
    let mut used_steps: HashSet<usize> = HashSet::new();

    let mut rivers = Vec::new();
    for _ in 0..n_rivers {
        let mut river = River::new();
        let mut start_ok = false;
        while !start_ok {
            river.start = IVec2::new(
                rng.roll_dice(1, WORLD_WIDTH as i32 - 1),
                rng.roll_dice(1, WORLD_HEIGHT as i32 - 1),
            );
            let pidx = planet_idx(river.start.x as usize, river.start.y as usize);
            if (planet.landblocks[pidx].btype == BiomeType::Mountains
                || planet.landblocks[pidx].btype == BiomeType::Hills)
                && !used_starts.contains(&pidx)
            {
                start_ok = true;
            }
        }
        used_starts.insert(planet_idx(river.start.x as usize, river.start.y as usize));

        let mut done = false;
        let mut x = river.start.x;
        let mut y = river.start.y;

        while !done {
            let mut candidates: Vec<(u32, usize)> = Vec::new();
            candidate(&used_starts, &used_steps, x - 1, y, planet, &mut candidates);
            candidate(&used_starts, &used_steps, x + 1, y, planet, &mut candidates);
            candidate(&used_starts, &used_steps, x, y - 1, planet, &mut candidates);
            candidate(&used_starts, &used_steps, x, y + 1, planet, &mut candidates);
            if candidates.is_empty() {
                done = true;
            } else {
                candidates.sort_by(|(h, _), (h2, _)| h.cmp(h2));
                used_steps.insert(candidates[0].1);
                let sx = candidates[0].1 % WORLD_WIDTH;
                let sy = candidates[0].1 / WORLD_WIDTH;
                river.steps.push(RiverStep { pos: IVec2::new(sx as i32, sy as i32) });
                x = sx as i32;
                y = sy as i32;
            }
        }
        rivers.push(river);
    }

    planet.rivers = rivers;
}

fn candidate(
    used_starts: &HashSet<usize>,
    used_steps: &HashSet<usize>,
    x: i32,
    y: i32,
    planet: &Planet,
    candidates: &mut Vec<(u32, usize)>,
) {
    if x < 0 || x > WORLD_WIDTH as i32 - 1 || y < 0 || y > WORLD_HEIGHT as i32 - 1 {
        return;
    }
    let pidx = planet_idx(x as usize, y as usize);
    if used_starts.contains(&pidx) {
        return;
    }
    if used_steps.contains(&pidx) {
        return;
    }

    if planet.landblocks[pidx].btype == BiomeType::Water {
        return;
    }

    candidates.push((planet.landblocks[pidx].height, pidx));
}
