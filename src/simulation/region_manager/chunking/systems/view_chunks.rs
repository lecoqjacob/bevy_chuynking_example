use crate::prelude::*;
use bevy::utils::FloatOrd;

/// Run criteria for the [`update_view_chunks`] system
pub fn should_update_view_chunks(
    view_distance: &Res<ChunkLoadRadius>,
    player_pos: &Res<CurrentLocalPlayerChunk>,
) -> bool {
    player_pos.is_changed()
        || view_distance.is_changed()
        || player_pos.is_added()
        || view_distance.is_added()
}

/// Checks for the loaded chunks around the player and schedules loading of new chunks in sight
pub fn update_view_chunks(
    view_radius: Res<ChunkLoadRadius>,
    chunk_entities: Res<ChunkEntities>,
    player_pos: Res<CurrentLocalPlayerChunk>,
    mut chunk_command_queue: ResMut<ChunkCommandQueue>,
) {
    if !should_update_view_chunks(&view_radius, &player_pos) {
        return;
    }

    // quick n dirty circular chunk loading.
    //perf: optimize this.
    for x in -view_radius.horizontal..=view_radius.horizontal {
        for y in -view_radius.vertical..=view_radius.vertical {
            if (x < 0 || y < 0) || x.pow(2) + y.pow(2) >= view_radius.horizontal.pow(2) {
                continue;
            }

            let chunk_key = {
                let mut pos: IVec2 = player_pos.chunk_min
                    + IVec2::new(x * CHUNK_SIZE as i32, y * CHUNK_SIZE as i32);
                pos.x = pos.x.max(0);
                pos
            };

            let loc: ChunkLocation = chunk_key.into();
            println!("chunk key: {loc:?}");
            if chunk_entities.entity(loc).is_none()
                && !chunk_command_queue.create.contains(&loc)
                && !chunk_command_queue.destroy.contains(&loc)
            {
                chunk_command_queue.create.push(loc);
            }
        }
    }

    // quick n dirty circular chunk !loading.
    for loaded_chunk in chunk_entities.0.keys() {
        let delta: IVec2 = loaded_chunk.as_ivec2() - player_pos.chunk_min;
        if delta.x.pow(2) >= view_radius.horizontal.pow(2) * (CHUNK_SIZE as i32).pow(2)
            || delta.y.pow(2) >= view_radius.vertical.pow(2) * (CHUNK_SIZE as i32).pow(2)
        {
            chunk_command_queue.destroy.push(*loaded_chunk);
        }
    }

    // load chunks starting from the player position
    chunk_command_queue.create.sort_unstable_by_key(|key| {
        FloatOrd(key.as_vec2().distance(player_pos.chunk_min.as_vec2()))
    });
}
