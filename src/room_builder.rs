use crate::{GameState, StructureCache};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::ops::Range;

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

fn build_a_room(mut commands: Commands, structures: Res<StructureCache>) {
    let room = StructureGrid::from_str(
        "
            xxxxxxxxxxxxxxxxx
            x    x     d    x
            xxxxxxxxddxxxxdxx
            x    xoooooox   x
            xxxxxxooooooxxxdx
        ",
        Vec3::ZERO,
    );

    build_room(room, commands, &structures);
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
pub struct StructureGrid(Vec<Vec<Tile>>, Vec3);

impl StructureGrid {
    fn get_adjacent(&self, cur_y: usize, cur_x: usize, direction: GridDirection) -> Tile {
        use GridDirection::*;
        match direction {
            North => {
                if cur_y == 0 {
                    None
                } else {
                    self.0[cur_y - 1][cur_x]
                }
            }
            South => {
                if cur_y >= self.0.len() - 1 {
                    None
                } else {
                    self.0[cur_y + 1][cur_x]
                }
            }
            West => {
                if cur_x == 0 {
                    None
                } else {
                    self.0[cur_y][cur_x - 1]
                }
            }
            East => {
                if cur_x >= self.0[cur_y].len() - 1 {
                    None
                } else {
                    self.0[cur_y][cur_x + 1]
                }
            }
        }
    }
    fn get_surrounding(&self, cur_x: usize, cur_y: usize) -> TileCross {
        use GridDirection::*;
        TileCross {
            north: self.get_adjacent(cur_x, cur_y, North),
            south: self.get_adjacent(cur_x, cur_y, South),
            west: self.get_adjacent(cur_x, cur_y, West),
            east: self.get_adjacent(cur_x, cur_y, East),
        }
    }

    fn get_translation(&self, cur_y: usize, cur_x: usize) -> Vec3 {
        Vec3::new(cur_x as f32 * 4.0, 0.0, cur_y as f32 * 4.0)
    }
    fn get_physical_size(&self) -> Vec3 {
        let height = (self.0.len() + 1) as f32 * 4.0;
        let width = (self.0[0].len() + 1) as f32 * 4.0;
        Vec3::new(width, 0.0, height)
    }

    pub fn get_center(&self) -> Vec3 {
        (self.get_physical_size() / 2.0) + self.1
    }

    pub fn find_longest_segments(&self) {
        let y = self.0.len();
        let x = self.0[0].len();
        //Find segments
        let mut wall_map = vec![false; x * y];
        for (y_index, row) in self.0.iter().enumerate() {
            for (x_index, tile) in row.iter().enumerate() {
                if let Some(tile_type) = tile {
                    match *tile_type {
                        TileType::Wall => {
                            wall_map[x_index * y_index] = true;
                        }
                        _ => (),
                    }
                }
            }
        }
        println!("{:?}", wall_map);
    }

    pub fn get_collider_data(&self) -> (Vec<ColliderProperties>, Vec<usize>) {
        self.find_longest_segments();
        let mut walls: Vec<ColliderProperties> = Vec::new();
        let mut doors: Vec<usize> = Vec::new();
        for (y_index, row) in self.0.iter().enumerate() {
            let mut start: i8 = -1;
            let mut end: i8 = -1;
            for (x_index, tile) in row.iter().enumerate() {
                if let Some(tile_type) = tile {
                    if *tile_type == TileType::Wall {
                        if start != -1 {
                            end = x_index as i8;
                        } else {
                            start = x_index as i8;
                            end = (x_index + 1) as i8;
                        }
                    } else {
                        if start != -1 {
                            let length = (end - start) as f32 * 4.0;
                            let size = Vec3::new(length, 4.0, 1.0) * 0.5;
                            let x_offset = if length == 4.0 {
                                (x_index as f32 * 4.0) - 4.0
                            } else {
                                (x_index as f32 * 4.0) - (length * 0.5) - 4.0
                            };
                            let offset = Vec3::new(x_offset, 1.0, y_index as f32 * 4.0);

                            let rotation = if length == 4.0 {
                                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians())
                            } else {
                                Quat::default()
                            };
                            let transform = Transform::from_translation(offset + self.1)
                                .with_rotation(rotation);

                            walls.push(ColliderProperties {
                                size,
                                transform,
                                tile_type: TileType::Wall,
                            });

                            start = -1;
                            end = 0;
                        }
                    }

                    if *tile_type == TileType::Door {
                        doors.push(x_index);
                    }
                }
            }
            if start != -1 {
                let length = (end - start) as f32 * 4.0;
                let size = Vec3::new(length, 4.0, 1.0) * 0.5;
                let x_offset = if length == 4.0 {
                    (end as f32 * 4.0) - 4.0
                } else {
                    (end as f32 * 4.0) - (length * 0.5) - 4.0
                };
                let offset = Vec3::new(x_offset, 1.0, y_index as f32 * 4.0);

                let rotation = if length == 4.0
                    && (start != row.len() as i8 - 1 || end != row.len() as i8 - 1)
                {
                    Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians())
                } else {
                    Quat::default()
                };
                let transform =
                    Transform::from_translation(offset + self.1).with_rotation(rotation);

                walls.push(ColliderProperties {
                    size,
                    transform,
                    tile_type: TileType::Wall,
                });

                start = -1;
                end = 0;
            }
        }
        (walls, doors)
    }

    fn from_str(input: &str, origin: Vec3) -> StructureGrid {
        let tiles: Vec<Vec<Tile>> = input
            .trim()
            .lines()
            .map(|string_row| {
                string_row
                    .trim()
                    .chars()
                    .map(|character| {
                        if character == 'x' {
                            return Some(TileType::Wall);
                        }
                        if character == 'd' {
                            return Some(TileType::Door);
                        }
                        return Some(TileType::Empty);
                    })
                    .collect()
            })
            .collect();
        StructureGrid(tiles, origin)
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
    fn render(self, structures: &Res<StructureCache>) -> (Handle<Scene>, Quat) {
        match self {
            RenderableParts::NorthWall => (structures.wall.clone_weak(), Quat::default()),
            RenderableParts::SouthWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
            ),
            RenderableParts::EastWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
            ),
            RenderableParts::WestWall => (
                structures.wall.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
            ),
            RenderableParts::NorthDoor => (structures.door.clone_weak(), Quat::default()),
            RenderableParts::SouthDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
            ),
            RenderableParts::EastDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
            ),
            RenderableParts::WestDoor => (
                structures.door.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
            ),
            RenderableParts::NEastCorner => (structures.wall_corner.clone_weak(), Quat::default()),
            RenderableParts::SWestCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
            ),
            RenderableParts::SEastCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
            ),
            RenderableParts::NWestCorner => (
                structures.wall_corner.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
            ),
            RenderableParts::MultiCorner => (structures.multi_corner.clone_weak(), Quat::default()),
            RenderableParts::NorthT => (structures.t_split.clone_weak(), Quat::default()),
            RenderableParts::SouthT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians()),
            ),
            RenderableParts::EastT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 270.0_f32.to_radians()),
            ),
            RenderableParts::WestT => (
                structures.t_split.clone_weak(),
                Quat::from_axis_angle(Vec3::Y, 90.0_f32.to_radians()),
            ),
            _ => (Handle::default(), Quat::default()),
        }
    }
}

pub fn build_room(grid: StructureGrid, mut commands: Commands, structures: &Res<StructureCache>) {
    use RenderableParts::*;
    use TileType::*;

    let (walls, doors) = grid.get_collider_data();

    for wall in walls {
        commands.spawn((
            TransformBundle {
                local: wall.transform,
                ..default()
            },
            Collider::cuboid(wall.size.x, wall.size.y, wall.size.z),
            RigidBody::Fixed,
            Name::from("Collider"),
        ));
    }

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(grid.1),
                visibility: Visibility::Visible,
                ..default()
            },
            Name::from("Room"),
        ))
        .with_children(|parent| {
            for y in 0..grid.0.len() {
                for x in 0..grid.0[0].len() {
                    let mut renderable: RenderableParts = NoPart;
                    let surrounding = grid.get_surrounding(y, x);
                    match grid.0[y][x] {
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
                    let (handle, rotation) = renderable.render(structures);
                    if handle != Handle::default() {
                        parent.spawn((
                            SceneBundle {
                                scene: handle,
                                transform: Transform::from_translation(grid.get_translation(y, x))
                                    .with_rotation(rotation),
                                ..default()
                            },
                            Name::from(format!("Part: {},{}", y, x)),
                        ));
                    }
                }
            }
        });
}
