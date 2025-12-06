use crate::chunk::{Chunk, math::pos::ChunkPosition};
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct ChunkMap(pub HashMap<ChunkPosition, Entity>);

pub fn update_chunk_map_system(
    mut chunk_map: ResMut<ChunkMap>,
    query: Query<(Entity, &crate::chunk::ChunkComponent)>,
    chunks: Res<Assets<Chunk>>,
) {
    for (entity, chunk_component) in query.iter() {
        if let Some(chunk) = chunks.get(&chunk_component.0) {
            if !chunk_map.0.contains_key(&chunk.loc) {
                chunk_map.0.insert(chunk.loc, entity);
                // info!("Added chunk at {:?} to ChunkMap", chunk.loc);
            }
        }
    }
}
