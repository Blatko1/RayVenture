use crate::{models::ModelID, raycaster::PointXZ, textures::TextureID};

use super::portal::{DummyPortal, Portal};

#[derive(Debug, Clone, Copy)]
pub struct BlueprintID(pub usize);

// TODO Use a struct or type for dimensions instead
// TODO rename all "blueprints" to "blueprints"
/// A map blueprint (room) with immutable data.
/// You can mutate room data in a [`Room`] struct.
#[derive(Debug)]
pub struct Blueprint {
    pub(super) id: BlueprintID,
    /// Room dimensions (width, height).
    pub(super) dimensions: (u64, u64),
    pub(super) tiles: Vec<Tile>,
    pub(super) unlinked_portals: Vec<Portal>,
    //pub(super) object_placeholders: Vec<Option<ModelID>>,
    pub(super) skybox: SkyboxTextureIDs,
    pub(super) repeatable: bool,
    pub(super) ambient_light_intensity: f32,
}

impl Blueprint {
    pub fn new(
        id: BlueprintID,
        dimensions: (u64, u64),
        tiles: Vec<Tile>,
        skybox: SkyboxTextureIDs,
        repeatable: bool,
        ambient_light_intensity: f32,
    ) -> Self {
        // Create unlinked Portals from DummyPortals
        let unlinked_portals = tiles
            .iter()
            .filter(|tile| tile.portal.is_some())
            .map(|tile| {
                let dummy = tile.portal.unwrap();
                Portal {
                    id: dummy.id,
                    direction: dummy.orientation,
                    position: tile.position,
                    center: PointXZ::new(
                        tile.position.x as f32 + 0.5,
                        tile.position.z as f32 + 0.5,
                    ),
                    ground_level: tile.ground_height,
                    link: None,
                }
            })
            .collect();

        let object_placeholders =
            tiles.iter().filter(|tile| tile.object.is_some()).count();
        Self {
            id,
            dimensions,
            unlinked_portals,
            tiles,
            //object_placeholders: vec![None; object_placeholders],
            skybox,
            repeatable,
            ambient_light_intensity,
        }
    }

    /// Returns the value at the provided map coordinates.
    /// Parsed arguments are assumed to be in map bound and correct.
    /// This game assumes that the y-axis points upwards, the z-axis forwards
    /// and the x-axis to the right so `x` represents moving left or right
    /// and `z` represents moving forward or backward on the map.
    /// Returns [`Tile::Void`] if coordinates are out of bounds.
    #[inline]
    pub fn get_tile_checked(&self, x: i64, z: i64) -> Option<&Tile> {
        // TODO do something about i64 arguments and 'if' conditions
        if x >= self.dimensions.0 as i64
            || x < 0
            || z >= self.dimensions.1 as i64
            || z < 0
        {
            return None;
        }
        self.tiles
            .get(z as usize * self.dimensions.0 as usize + x as usize)
    }

    // TODO do check while in loop like if x > 'bound' {break;}
    #[inline]
    pub fn get_tile(&self, x: usize, z: usize) -> Option<&Tile> {
        self.tiles.get(z * self.dimensions.0 as usize + x)
    }

    pub fn get_tile_unchecked(&self, x: usize, z: usize) -> &Tile {
        &self.tiles[z * self.dimensions.0 as usize + x]
    }

    pub fn dimensions_i64(&self) -> (i64, i64) {
        (self.dimensions.0 as i64, self.dimensions.1 as i64)
    }
}

// TODO maybe rename all with 'level' to 'height'
// TODO try removing Clone and Copy
#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub position: PointXZ<u64>,
    /// Texture of the bottom wall.
    pub bottom_wall_tex: TextureID,
    /// Texture of the top wall.
    pub top_wall_tex: TextureID,
    /// Texture of the bottom platform.
    pub ground_tex: TextureID,
    /// Texture of the top platform.
    pub ceiling_tex: TextureID,
    /// `Y-level` - starting lower bound of the bottom wall;
    /// level from which the bottom wall stretches.
    pub bottom_height: f32,
    /// `Y-level` - ending upper bound of the bottom wall;
    /// area/platform on which the player is walking.
    pub ground_height: f32,
    /// `Y-level` - starting lower bound of the top wall; the ceiling.
    pub ceiling_height: f32,
    /// `Y-level` - ending upper bound of the top wall;
    /// level to which the top wall stretches.
    pub top_height: f32,
    /// If the current tile should be a portal to different blueprint (map).
    pub portal: Option<DummyPortal>,

    pub object: Option<ObjectID>,
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectID(pub usize);

#[derive(Copy, Clone, Debug)]
pub struct SkyboxTextureIDs {
    pub north: TextureID,
    pub east: TextureID,
    pub south: TextureID,
    pub west: TextureID,
    pub top: TextureID,
    pub bottom: TextureID,
}
