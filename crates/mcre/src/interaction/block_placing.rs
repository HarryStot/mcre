use crate::chunk::{Chunk, ChunkComponent, loader::ChunkLoaderConfig, world_pos_to_chunk_pos};
use crate::chunk_map::ChunkMap;
use crate::interaction::raycasting::{BlockRaycastHit, raycast_block_data};
use crate::textures::BlockTextures;
use crate::ui::hotbar::Hotbar;
use bevy::prelude::*;
use mcre_core::{Block, Direction};

#[derive(Message, Clone, Event)]
pub struct BlockPlaceMessage(pub BlockRaycastHit);

pub fn handle_block_placing_input(
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<&Transform, With<Camera>>,
    chunk_map: Res<ChunkMap>,
    chunks_query: Query<&ChunkComponent>,
    chunks: Res<Assets<Chunk>>,
    mut place_event_writer: MessageWriter<BlockPlaceMessage>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }

    let ray_origin = camera_transform.translation;
    let ray_direction = camera_transform.forward();

    if let Some(hit) = raycast_block_data(
        ray_origin,
        *ray_direction,
        &chunk_map,
        &chunks_query,
        &chunks,
    ) {
        place_event_writer.write(BlockPlaceMessage(hit));
    }
}

pub fn apply_block_placing(
    mut events: MessageReader<BlockPlaceMessage>,
    mut chunk_map: ResMut<ChunkMap>,
    mut chunks_query: Query<(&ChunkComponent, &mut Mesh3d)>,
    mut chunks: ResMut<Assets<Chunk>>,
    config: Res<ChunkLoaderConfig>,
    textures: Res<BlockTextures>,
    mut meshes: ResMut<Assets<Mesh>>,
    hotbar: Res<Hotbar>,
    mut commands: Commands,
) {
    for event in events.read() {
        let hit: &BlockRaycastHit = &event.0;

        let place_world_pos = match hit.face {
            Direction::North => hit.block_pos + IVec3::new(0, 0, -1),
            Direction::South => hit.block_pos + IVec3::new(0, 0, 1),
            Direction::East => hit.block_pos + IVec3::new(1, 0, 0),
            Direction::West => hit.block_pos + IVec3::new(-1, 0, 0),
            Direction::Up => hit.block_pos + IVec3::new(0, 1, 0),
            Direction::Down => hit.block_pos + IVec3::new(0, -1, 0),
        };

        let (chunk_world_pos, local_pos) =
            world_pos_to_chunk_pos(place_world_pos, &config.chunk_size);

        let block_to_place = hotbar.get_selected_block();

        let chunk_entity = if let Some(&existing_entity) = chunk_map.0.get(&chunk_world_pos) {
            existing_entity
        } else {
            // Create a new empty chunk
            let mut new_chunk = Chunk::empty(config.chunk_size, chunk_world_pos);
            new_chunk.set_block(local_pos, block_to_place);

            let chunk_handle = chunks.add(new_chunk.clone());
            let mesh = meshes.add(new_chunk.generate_mesh(&textures));

            let entity = commands
                .spawn((
                    ChunkComponent(chunk_handle.clone()),
                    new_chunk.transform(),
                    MeshMaterial3d(textures.texture().unwrap().clone()),
                    Mesh3d(mesh),
                ))
                .id();

            chunk_map.0.insert(chunk_world_pos, entity);

            // Skip the rest since we already placed the block during chunk creation
            continue;
        };

        // Update existing chunk
        if let Ok((chunk_component, mut mesh_handle)) = chunks_query.get_mut(chunk_entity) {
            if let Some(chunk) = chunks.get_mut(&chunk_component.0) {
                let can_place = chunk
                    .get(local_pos)
                    .map_or(true, |block_state| block_state.block() == Block::AIR);

                if can_place {
                    chunk.set_block(local_pos, block_to_place);
                    mesh_handle.0 = chunk.regenerate_mesh(&textures, &mut meshes);
                }
            }
        }
    }
}
