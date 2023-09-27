use crate::{GameState, StructureCache};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct RoomBuilderPlugin;

impl Plugin for RoomBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gameplay), build_a_room);
    }
}

#[derive(Debug)]
pub struct ColliderProperties {
    pub size: Vec3,
    pub transform: Transform,
    pub tile_type: TileType,
}

#[derive(Debug, Default)]
pub struct ColliderChild {
    pub collider: Collider,
    pub transform: Transform,
}

fn build_a_room(mut commands: Commands, structures: Res<StructureCache>) {
    let room = Room::from_str(
        "
            xxxxxxxxxxxxxxxxx
            x    x     d    x
            xxxxxxxxddxxxxdxx
            x    xoooooox   x
            xxxxxxxxxxxxxxxdx
        ",
        Vec3::ZERO,
    );

    println!("Width: {}, Height: {}", room.width, room.height);

    room.build(commands, &structures);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Door,
    Empty,
}

impl TileType {
    fn is_wall_like(self) -> bool {
        matches!(self, Self::Wall | Self::Door)
    }
}

type Tile = Option<TileType>;

pub enum GridDirection {
    North,
    South,
    West,
    East,
}

#[derive(Default)]
pub struct TileCross {
    pub north: Tile,
    pub south: Tile,
    pub west: Tile,
    pub east: Tile,
}

impl TileCross {
    fn wall_from_surrounding(self) -> RenderableParts {
        use RenderableParts::*;
        use TileType::*;

        // Multi-Corner
        if let (Some(north), Some(south), Some(east), Some(west)) =
            (self.north, self.south, self.east, self.west)
        {
            if north.is_wall_like()
                && south.is_wall_like()
                && east.is_wall_like()
                && west.is_wall_like()
            {
                return MultiCorner;
            }
        }
        // T-Corners
        if let (Some(south), Some(east), Some(west)) = (self.south, self.east, self.west) {
            if south.is_wall_like() && east.is_wall_like() && west.is_wall_like() {
                return NorthT;
            }
        }

        if let (Some(north), Some(east), Some(west)) = (self.north, self.east, self.west) {
            if north.is_wall_like() && east.is_wall_like() && west.is_wall_like() {
                return SouthT;
            }
        }

        if let (Some(south), Some(north), Some(west)) = (self.south, self.north, self.west) {
            if south.is_wall_like() && north.is_wall_like() && west.is_wall_like() {
                return EastT;
            }
        }
        if let (Some(south), Some(east), Some(north)) = (self.south, self.east, self.north) {
            if south.is_wall_like() && east.is_wall_like() && north.is_wall_like() {
                return WestT;
            }
        }

        // Normal Corner
        if let (Some(north), Some(east)) = (self.north, self.east) {
            if north.is_wall_like() && east.is_wall_like() {
                return SWestCorner;
            }
        }

        if let (Some(north), Some(west)) = (self.north, self.west) {
            if north.is_wall_like() && west.is_wall_like() {
                return SEastCorner;
            }
        }

        if let (Some(south), Some(east)) = (self.south, self.east) {
            if south.is_wall_like() && east.is_wall_like() {
                return NWestCorner;
            }
        }

        if let (Some(south), Some(west)) = (self.south, self.west) {
            if south.is_wall_like() && west.is_wall_like() {
                return NEastCorner;
            }
        }

        // Normal Wall
        if let (Some(east), Some(west)) = (self.east, self.west) {
            if east.is_wall_like() && west.is_wall_like() {
                if self.south.is_none() || Some(Empty) == self.south {
                    return SouthWall;
                }

                if self.north.is_none() || Some(Empty) == self.north {
                    return NorthWall;
                }
            }
        }

        if let (Some(north), Some(south)) = (self.north, self.south) {
            if north.is_wall_like() && south.is_wall_like() {
                if self.west.is_none() || Some(Empty) == self.west {
                    return WestWall;
                }

                if self.east.is_none() || Some(Empty) == self.east {
                    return EastWall;
                }
            }
        }

        return NoPart;
    }

    fn door_from_surrounding(self) -> RenderableParts {
        use RenderableParts::*;
        use TileType::*;
        if let (Some(east), Some(west)) = (self.east, self.west) {
            if east.is_wall_like() && west.is_wall_like() {
                if self.south.is_none() || Some(Empty) == self.south {
                    return SouthDoor;
                }

                if self.north.is_none() || Some(Empty) == self.north {
                    return NorthDoor;
                }
            }
        }

        if let (Some(north), Some(south)) = (self.north, self.south) {
            if north.is_wall_like() && south.is_wall_like() {
                if self.west.is_none() || Some(Empty) == self.west {
                    return WestDoor;
                }

                if self.east.is_none() || Some(Empty) == self.east {
                    return EastDoor;
                }
            }
        }
        println!("Invalid space for a door, skipping");
        return NoPart;
    }
}

#[derive(Debug)]
pub struct Room {
    pub map: Vec<Tile>,
    width: usize,
    height: usize,
    pub origin: Vec3,
}
impl Room {
    fn from_str(input: &str, origin: Vec3) -> Room {
        let mut width: usize = 0;
        let mut height: usize = 0;
        let mut map: Vec<Tile>;
        let mut tile_vec: Vec<(Tile, usize, usize)> = Vec::default();
        input
            .trim()
            .lines()
            .inspect(|_| height += 1)
            .enumerate()
            .for_each(|(y, row)| {
                row.trim()
                    .chars()
                    .inspect(|_| {
                        if y == 0 {
                            width += 1
                        }
                    })
                    .enumerate()
                    .for_each(|(x, character)| {
                        if character == 'x' {
                            tile_vec.push((Some(TileType::Wall), x, y))
                        } else if character == 'd' {
                            tile_vec.push((Some(TileType::Door), x, y))
                        } else {
                            tile_vec.push((Some(TileType::Empty), x, y))
                        }
                    });
            });

        map = vec![None; width * height];

        for (t_type, x, y) in tile_vec {
            map[(width * y) + x] = t_type;
        }

        Room {
            map,
            width,
            height,
            origin,
        }
    }

    pub fn get_adjacent(&self, x: usize, y: usize, direction: GridDirection) -> Tile {
        use GridDirection::*;

        match direction {
            North => {
                if y == 0 {
                    None
                } else {
                    self.get(x, y - 1)
                }
            }
            South => {
                if y >= self.height - 1 {
                    None
                } else {
                    self.get(x, y + 1)
                }
            }
            East => {
                if x >= self.width - 1 {
                    None
                } else {
                    self.get(x + 1, y)
                }
            }
            West => {
                if x == 0 {
                    None
                } else {
                    self.get(x - 1, y)
                }
            }
        }
    }

    fn get_surrounding(&self, x: usize, y: usize) -> TileCross {
        use GridDirection::*;
        TileCross {
            north: self.get_adjacent(x, y, North),
            south: self.get_adjacent(x, y, South),
            west: self.get_adjacent(x, y, West),
            east: self.get_adjacent(x, y, East),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        self.map[(self.width * y) + x]
    }
    pub fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.map[(self.width * y) + x] = tile;
    }

    pub fn get_translation(&self, x: usize, y: usize) -> Vec3 {
        Vec3::new(x as f32 * 4.0, 0.0, y as f32 * 4.0) + self.origin
    }

    pub fn get_physical_size(&self) -> Vec3 {
        Vec3::new(self.width as f32 * 4.0, 0.0, self.height as f32 * 4.0)
    }

    pub fn get_center(&self) -> Vec3 {
        (self.get_physical_size() * 0.5) + self.origin
    }

    pub fn row_iter(&self, row: usize) -> impl Iterator<Item = Tile> {
        let mut resulting_vec: Vec<Tile> = Vec::new();

        for i in 0..self.width {
            resulting_vec.push(self.map[i + (self.width * row)]);
        }
        resulting_vec.into_iter()
    }

    pub fn column_iter(&self, column: usize) -> impl Iterator<Item = Tile> {
        let mut resulting_vec: Vec<Tile> = Vec::new();

        for i in 0..self.height {
            resulting_vec.push(self.map[(i * self.width) + column]);
        }
        resulting_vec.into_iter()
    }

    pub fn get_collider_data(&self) -> (Vec<ColliderProperties>, Vec<usize>) {
        let mut walls: Vec<ColliderProperties> = Vec::new();
        let doors: Vec<usize> = Vec::new();

        for y in 0..self.height {
            let mut start: i8 = -1;
            let mut end: i8 = -1;
            for x in 0..self.width {
                if let Some(tile_type) = self.get(x, y) {
                    if tile_type == TileType::Wall {
                        if start != -1 {
                            end = x as i8;
                        } else {
                            start = x as i8;
                            end = x as i8 + 1;
                        }
                    } else {
                        if start != -1 {
                            let length = (end - start) as f32 * 4.0;
                            let size = Vec3::new(length, 4.0, 1.0) * 0.5;
                            let x_offset = if length == 4.0 {
                                (x as f32 * 4.0) - 4.0
                            } else {
                                (x as f32 * 4.0) - (length * 0.5) - 4.0
                            };
                            let offset = Vec3::new(x_offset, 1.0, y as f32 * 4.0);

                            let rotation = if length == 4.0 {
                                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians())
                            } else {
                                Quat::default()
                            };
                            let transform = Transform::from_translation(offset + self.origin)
                                .with_rotation(rotation);

                            walls.push(ColliderProperties {
                                size,
                                transform,
                                tile_type: TileType::Wall,
                            });
                        }
                    }
                    if x == self.width - 1 && start != -1 {
                        let length = (end - start) as f32 * 4.0;
                        let size = Vec3::new(length, 4.0, 1.0) * 0.5;
                        let x_offset = if length == 4.0 {
                            (x as f32 * 4.0) - 4.0
                        } else {
                            (x as f32 * 4.0) - (length * 0.5) - 4.0
                        };
                        let offset = Vec3::new(x_offset, 1.0, y as f32 * 4.0);

                        let rotation = if length == 4.0 {
                            Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians())
                        } else {
                            Quat::default()
                        };
                        let transform = Transform::from_translation(offset + self.origin)
                            .with_rotation(rotation);

                        walls.push(ColliderProperties {
                            size,
                            transform,
                            tile_type: TileType::Wall,
                        });
                    }
                }
            }
        }
        (walls, doors)
    }

    fn build(&self, mut commands: Commands, structures: &Res<StructureCache>) {
        use RenderableParts::*;
        use TileType::*;

        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(self.origin),
                    visibility: Visibility::Visible,
                    ..default()
                },
                Name::from("Room"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TransformBundle {
                        local: Transform::from_translation(self.get_center()),
                        ..default()
                    },
                    Collider::cuboid(
                        (self.width as f32 / 2.0) * 4.0,
                        0.5,
                        (self.height as f32 / 2.0) * 4.0,
                    ),
                    RigidBody::Fixed,
                ));

                for x in 0..self.width {
                    for y in 0..self.height {
                        let mut renderable: RenderableParts = NoPart;
                        let surrounding = self.get_surrounding(x, y);
                        match self.get(x, y) {
                            Some(tile) => match tile {
                                Wall => {
                                    renderable = surrounding.wall_from_surrounding();
                                }
                                Door => {
                                    renderable = surrounding.door_from_surrounding();
                                }
                                Empty => (),
                            },
                            None => (),
                        }
                        let (handle, rotation, colliders) = renderable.render(structures);
                        if handle != Handle::default() {
                            parent
                                .spawn((
                                    SceneBundle {
                                        scene: handle,
                                        transform: Transform::from_translation(
                                            self.get_translation(x, y),
                                        )
                                        .with_rotation(rotation),
                                        ..default()
                                    },
                                    Name::from(format!("Part: {},{}", y, x)),
                                ))
                                .with_children(|child| {
                                    for components in colliders {
                                        child.spawn((
                                            TransformBundle {
                                                local: Transform::from_translation(
                                                    components.transform.translation
                                                        + (Vec3::Y * 2.0),
                                                ),
                                                ..default()
                                            },
                                            components.collider,
                                            RigidBody::Fixed,
                                        ));
                                    }
                                });
                        }
                    }
                }
            });
    }
}

enum RenderableParts {
    NorthWall,
    SouthWall,
    EastWall,
    WestWall,
    NorthDoor,
    SouthDoor,
    EastDoor,
    WestDoor,
    NWestCorner,
    NEastCorner,
    SWestCorner,
    SEastCorner,
    NorthT,
    SouthT,
    EastT,
    WestT,
    MultiCorner,
    NoPart,
}

impl RenderableParts {
    fn render(self, structures: &Res<StructureCache>) -> (Handle<Scene>, Quat, Vec<ColliderChild>) {
        match self {
            RenderableParts::NorthWall => (
                structures.wall.clone_weak(),
                Quat::default(),
                vec![ColliderChild {
                    collider: Collider::cuboid(2.0, 2.0, 0.5),
                    ..default()
                }],
            ),
            RenderableParts::SouthWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
                vec![ColliderChild {
                    collider: Collider::cuboid(2.0, 2.0, 0.5),
                    ..default()
                }],
            ),
            RenderableParts::EastWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
                vec![ColliderChild {
                    collider: Collider::cuboid(2.0, 2.0, 0.5),
                    ..default()
                }],
            ),
            RenderableParts::WestWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
                vec![ColliderChild {
                    collider: Collider::cuboid(2.0, 2.0, 0.5),
                    ..default()
                }],
            ),
            RenderableParts::NorthDoor => {
                (structures.door.clone_weak(), Quat::default(), Vec::new())
            }
            RenderableParts::SouthDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
                Vec::new(),
            ),
            RenderableParts::EastDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
                Vec::new(),
            ),
            RenderableParts::WestDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
                Vec::new(),
            ),
            RenderableParts::NEastCorner => (
                structures.wall_corner.clone_weak(),
                Quat::default(),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(1.25, 2.0, 0.5),
                        transform: Transform::from_translation(Vec3::X * -0.75),
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::SWestCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(1.25, 2.0, 0.5),
                        transform: Transform::from_translation(Vec3::X * -0.75),
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::SEastCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(1.25, 2.0, 0.5),
                        transform: Transform::from_translation(Vec3::X * -0.75),
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::NWestCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(1.25, 2.0, 0.5),
                        transform: Transform::from_translation(Vec3::X * -0.75),
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::MultiCorner => (
                structures.multi_corner.clone_weak(),
                Quat::default(),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(2.0, 2.0, 0.5),
                        ..default()
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 2.0),
                        ..default()
                    },
                ],
            ),
            RenderableParts::NorthT => (
                structures.t_split.clone_weak(),
                Quat::default(),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(2.0, 2.0, 0.5),
                        ..default()
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::SouthT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(2.0, 2.0, 0.5),
                        ..default()
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::EastT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(2.0, 2.0, 0.5),
                        ..default()
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            RenderableParts::WestT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
                vec![
                    ColliderChild {
                        collider: Collider::cuboid(2.0, 2.0, 0.5),
                        ..default()
                    },
                    ColliderChild {
                        collider: Collider::cuboid(0.5, 2.0, 0.75),
                        transform: Transform::from_translation(Vec3::Z * 1.25),
                    },
                ],
            ),
            _ => (Handle::default(), Quat::default(), Vec::new()),
        }
    }
}
