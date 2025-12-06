use super::components::*;
use crate::textures::BlockTextures;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn handle_hotbar_keyboard(mut hotbar: ResMut<Hotbar>, keyboard: Res<ButtonInput<KeyCode>>) {
    for i in 0..9 {
        let key = match i {
            0 => KeyCode::Digit1,
            1 => KeyCode::Digit2,
            2 => KeyCode::Digit3,
            3 => KeyCode::Digit4,
            4 => KeyCode::Digit5,
            5 => KeyCode::Digit6,
            6 => KeyCode::Digit7,
            7 => KeyCode::Digit8,
            8 => KeyCode::Digit9,
            _ => continue,
        };
        if keyboard.just_pressed(key) {
            hotbar.select_slot(i);
        }
    }
}

pub fn handle_hotbar_scroll(
    mut hotbar: ResMut<Hotbar>,
    mut scroll_events: MessageReader<MouseWheel>,
) {
    for event in scroll_events.read() {
        if event.y > 0.0 {
            // Scroll up = previous slot
            hotbar.select_previous();
        } else if event.y < 0.0 {
            // Scroll down = next slot
            hotbar.select_next();
        }
    }
}

pub fn spawn_hotbar(mut commands: Commands, textures: Res<BlockTextures>, hotbar: Res<Hotbar>) {
    const SLOT_SIZE: f32 = 50.0;
    const SLOT_PADDING: f32 = 4.0;
    const BORDER_WIDTH: f32 = 2.0;

    // Main hotbar container
    commands
        .spawn((
            HotbarUi,
            Node {
                width: Val::Auto,
                height: Val::Px(SLOT_SIZE + SLOT_PADDING * 2.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Percent(50.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(SLOT_PADDING),
                padding: UiRect::all(Val::Px(SLOT_PADDING)),
                // Center horizontally
                margin: UiRect {
                    left: Val::Px(-((SLOT_SIZE + SLOT_PADDING) * 4.5)),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            for i in 0..9 {
                let block = hotbar.slots[i];
                let is_selected = i == hotbar.selected_slot;

                // Individual slot container
                parent
                    .spawn((
                        HotbarSlot { slot_index: i },
                        Node {
                            width: Val::Px(SLOT_SIZE),
                            height: Val::Px(SLOT_SIZE),
                            border: UiRect::all(Val::Px(BORDER_WIDTH)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(if is_selected {
                            Color::WHITE
                        } else {
                            Color::srgba(0.3, 0.3, 0.3, 0.8)
                        }),
                        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.9)),
                    ))
                    .with_children(|slot_parent| {
                        // Block icon/texture
                        if let Some(block_texture) = textures.get_block_texture(block) {
                            slot_parent.spawn((
                                Node {
                                    width: Val::Px(SLOT_SIZE - 8.0),
                                    height: Val::Px(SLOT_SIZE - 8.0),
                                    ..default()
                                },
                                ImageNode {
                                    image: block_texture,
                                    ..default()
                                },
                            ));
                        }
                    });
            }
        });
}

pub fn update_hotbar_selection(
    hotbar: Res<Hotbar>,
    mut slots: Query<(&HotbarSlot, &mut BorderColor)>,
) {
    if !hotbar.is_changed() {
        return;
    }

    for (slot, mut border) in slots.iter_mut() {
        border.set_all(if slot.slot_index == hotbar.selected_slot {
            Color::WHITE
        } else {
            Color::srgba(0.3, 0.3, 0.3, 0.8)
        });
    }
}

pub fn despawn_hotbar(mut commands: Commands, hotbar_query: Query<Entity, With<HotbarUi>>) {
    for entity in hotbar_query.iter() {
        commands.entity(entity).despawn_children().despawn();
    }
}
