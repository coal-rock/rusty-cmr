use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub struct Parser {
    pub input: Vec<u8>,
    pub position: usize,
    pub cube_count: i32,
    pub shader_param_names: HashSet<String>,
}

#[derive(Debug)]
pub struct Map {
    header: MapHeader,
    vars: Vec<Variable>,
    game_ident: String,
    texture_mru: Vec<u16>,
    entities: Vec<Entity>,
    vslots: Vec<Box<VSlot>>,
    map: Vec<Box<Option<Cube>>>,
}

#[derive(Debug)]
pub struct MapHeader {
    pub magic_field: String,
    pub version: u32,
    pub header_size: u32,
    pub world_size: u32,
    pub number_ents: u32,
    pub number_pvs: u32,
    pub number_lightmaps: u32,
    pub blend_map: u32,
    pub number_vars: u32,
    pub number_vslots: u32,
}

#[derive(Debug)]
pub enum VariableType {
    Int(u32),
    Float(f32),
    String(u16, String),
}

#[derive(Debug)]
pub struct Variable {
    var_type: VariableType,
    name_len: u16,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Entity {
    position: Position,
    attr1: u16,
    attr2: u16,
    attr3: u16,
    attr4: u16,
    attr5: u16,
    ent_type: EntityType,
}

#[derive(Debug, Clone)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

#[derive(Debug, Clone)]
pub enum EntityType {
    Empty,
    Light,
    MapModel,
    PlayerStart,
    EnvMap,
    Particles,
    Sound,
    Spotlight,
    IHealth,
    IAmmo,
    RaceStart,
    RaceFinish,
    RaceCheckpoint,
    PH4,
    PH5,
    PH6,
    PH7,
    PH8,
    PH9,
    Teleport,
    TeleDest,
    PH10,
    PH11,
    JumpPad,
    Base,
    PH12,
    PH13,
    PH14,
    PH15,
    PH16,
    Flag,
    MaxEntTypes,
}
#[derive(Debug, Clone)]
pub struct VSlot {
    slot: Option<Slot>,
    next: Box<Option<VSlot>>,
    index: i32,
    changed: i32,
    params: Vec<SlotShaderParam>,
    linked: bool,
    scale: f32,
    rotation: i32,
    offset: Vector2<i32>,
    scroll: Vector2<f32>,
    layer: i32,
    alpha_front: f32,
    alpha_back: f32,
    color_scale: Vector3<f32>,
    glow_color: Vector3<f32>,
}

impl VSlot {
    pub fn new(slot: Option<Slot>, index: i32) -> VSlot {
        VSlot {
            slot,
            next: Box::new(None),
            index,
            changed: 0,
            params: vec![],
            linked: false,
            scale: 1.0,
            rotation: 0,
            offset: Vector2::<i32> { x: 0, y: 0 },
            scroll: Vector2::<f32> { x: 0.0, y: 0.0 },
            layer: 0,
            alpha_front: 0.5,
            alpha_back: 0.0,
            color_scale: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            glow_color: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Slot {
    slot: Box<Option<Slot>>,
    index: i32,
    sts: Vec<Tex>,
    shader: Shader,
    params: Vec<SlotShaderParam>,
    variants: Vec<VSlot>,
    loaded: bool,
    tex_mask: u32,
    auto_grass: i8,
    grass_tex: Texture,
    thumbnail: Texture,
    layer_mask_name: String,
    layer_mask_mode: i32,
    layer_mask_scale: f32,
    layer_mask: ImageData,
}

#[derive(Debug, Clone)]
pub struct ImageData {
    width: i32,
    h: i32,
    bpp: i32,
    levels: i32,
    align: i32,
    pitch: i32,
    compressed: u32,
    // void *owner;
    // void (*freefunc)(void *);
}

#[derive(Debug, Clone)]
pub struct Tex {
    tex_type: i32,
    texture: Texture,
    name: String,
    combined: i32,
}

#[derive(Debug, Clone)]
pub struct Texture {
    name: String,
    tex_type: i32,
    width: i32,
    height: i32,
    xs: i32,
    ys: i32,
    bpp: i32,
    clamp: i32,
    mipmap: bool,
    can_reduce: bool,
    id: u32, //GLuint
    alpha_mask: String,
}

#[derive(Debug, Clone)]
pub struct Shader {
    last_shader: Box<Option<Shader>>,
    name: String,
    vs_str: String,
    ps_str: String,
    defer: String,
    shader_type: i32,
    program: u32, // GLuint
    vs_obj: u32,  // GLuint
    ps_obj: u32,  // GLuint
    default_params: Vec<SlotShaderParamState>,
    global_params: Vec<GlobalShaderParamUse>,
    local_params: Vec<LocalShaderParamState>,
    local_param_remap: Vec<u8>,
    detail_shader: Box<Option<Shader>>,
    variant_shader: Box<Option<Shader>>,
    alt_shader: Box<Option<Shader>>,
    fast_shader: (
        Box<Option<Shader>>,
        Box<Option<Shader>>,
        Box<Option<Shader>>,
    ), // 3 fields because MAXSHADERDETAIL = 3
    variants: Vec<Box<Option<Shader>>>,
    variant_rows: u16,
    standard: bool,
    forced: bool,
    used: bool,
    reuse_vs: Box<Option<Shader>>,
    reuse_ps: Box<Option<Shader>>,
    uniform_locs: Vec<Box<Option<UniformLoc>>>,
    attrib_locs: Vec<Box<Option<AttribLoc>>>,
    //const void *owner;
}

#[derive(Debug, Clone)]
pub struct SlotShaderParamState {
    value: (f32, f32, f32, f32),
    name: String,
    location: i32,
    size: i32,
    format: u32, // GLenum
}

#[derive(Debug, Clone)]
pub struct GlobalShaderParamState {
    name: String,
    value: GlobalShaderParamStateValue,
    version: i32,
    next_version: i32,
}

#[derive(Debug, Clone)]
pub enum GlobalShaderParamStateValue {
    Float(Vec<f32>), // 32
    Int(Vec<i32>),   // 31
    UInt(Vec<u32>),  // 32
    UChar(Vec<u8>),  // 32 * sizeof(float)
}

#[derive(Debug, Clone)]
pub struct GlobalShaderParamUse {
    param: GlobalShaderParamState,
    version: i32,
    location: i32,
    size: i32,
    format: u32, // GLenum
}

#[derive(Debug, Clone)]
struct LocalShaderParamState {
    name: String,
    location: i32,
    size: i32,
    format: u32, // GLenum
}

#[derive(Debug)]
pub struct ShaderParamBinding {
    location: i32,
    size: i32,
    format: u32, // GLenum
}

#[derive(Debug, Clone)]
struct AttribLoc {
    name: String,
    loc: i32,
}

#[derive(Debug, Clone)]
struct UniformLoc {
    name: String,
    block_name: String,
    loc: i32,
    verison: i32,
    binding: i32,
    stride: i32,
    offset: i32,
    size: i32,
    //void *data;
}

pub enum TextureType {
    Image,
    CubeMap,
    Type,

    Stub,
    Transient,
    Compressed,
    Alpha,
    Mirror,
    Flags,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SlotShaderParam {
    name: String,
    loc: i32,
    values: (f32, f32, f32, f32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cube {
    children: Vec<Box<Option<Cube>>>, // "points to 8 cube structures which are its children, or NULL. -Z first, then -Y, -X"
    edge_face: EdgeFace,
    textures: [u16; 6], // "one for each face. same order as orient." (6 entries)
    material: u16,      // empty-space material
    merged: u8,         // merged faces of the cube
    escaped_visible: EscapedVisible,
    cube_ext: Option<CubeExtInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubeExtInfo {
    max_verts: u8,
    tjoints: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeFace {
    Edge([u8; 12]), // edges of the cube, each uchar is 2 4bit values denoting the range (there should be 12 entries here)
    Face([u32; 3]), // 4 edges of each dimension together representing 2 perpendicular faces (there should be 3 entries here)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EscapedVisible {
    Escaped(u8),
    Visible(u8),
}

#[derive(Debug)]
pub enum GeometryType {
    Chidren,
    Empty,
    Solid,
    Normal,
    LODCube,
}

#[derive(Debug)]
pub enum MaterialType {
    Air,
    Water,
    Lava,
    Glass,
}

#[derive(Debug)]
pub enum MaterialClipping {
    NoClip,
    Clip,
    GameSpecificClip,
}

pub struct Geometry {
    geo_type: GeometryType,
    mat_type: MaterialType,
    death: bool,
    edit_only: bool,
}

pub struct LightMap {
    map_type: i32,
    bpp: i32,
    tex: i32,
    offset_x: i32,
    offset_y: i32,
    lightmaps: u8,
    lumels: u8,
    unlit_x: i32,
    unlit_y: i32,
    data: Vec<u8>,
}

impl LightMap {
    pub fn new() -> LightMap {
        LightMap {
            map_type: 0, // LM_DIFFUSE = 0
            bpp: 3,
            tex: -1,
            offset_x: -1,
            offset_y: -1,
            lightmaps: 0,
            lumels: 0,
            unlit_x: -1,
            unlit_y: -1,
            data: vec![],
        }
    }
}

#[derive(Clone)]
struct WaterPlane {
    height: i32,
    material_surfaces: Option<Vec<MaterialSurface>>,
}

impl WaterPlane {
    pub fn new() -> WaterPlane {
        WaterPlane {
            height: 0,
            material_surfaces: None,
        }
    }
}

#[derive(Clone)]
struct MaterialSurface {
    pos: Vector3<i32>,
    c_size: u16,
    r_size: u16,
    material: u16,
    skip: u16,
    orient: u8,
    visible: u8,
    index_depth: IndexDepth,
    light_envmap_ends: LightEnvMapEnds,
}

#[derive(Clone)]
enum IndexDepth {
    Index(i16),
    Depth(i16),
}

#[derive(Clone)]
enum LightEnvMapEnds {
    Light(Entity),
    EnvMap(u16),
    Ends(u8),
}

pub struct PVSData {
    offset: i32,
    len: i32,
}

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        Parser {
            input,
            position: 0,
            cube_count: 0,
            shader_param_names: HashSet::new(),
        }
    }

    pub fn parse_map(&mut self) -> Map {
        let header = self.parse_header();

        let mut vars = Vec::new();

        for _ in 0..header.number_vars {
            let variable = self.parse_variable();
            // println!("{:#?}", variable);
            vars.push(variable);
        }

        let game_ident = self.parse_game_ident();

        self.read_byte();
        self.read_byte();
        self.read_byte();
        self.read_byte();

        let texture_mru = self.parse_texture_mru();

        let mut entities = vec![];

        for _ in 0..header.number_ents {
            let entity = self.parse_entity();
            // println!("{:#?}", entity);
            entities.push(entity);
        }

        let mut vslot_num = header.number_vslots.clone() as i32;
        let vslots = self.parse_vslots(&mut vslot_num);

        let map = self.parse_children(
            &Vector3::<i32> { x: 0, y: 0, z: 0 },
            header.world_size as i32 >> 1,
            &mut false,
        );

        // self.parse_lightmaps(header.number_lightmaps);

        return Map {
            header,
            vars,
            game_ident,
            texture_mru,
            entities,
            vslots,
            map,
        };
    }

    fn parse_header(&mut self) -> MapHeader {
        MapHeader {
            magic_field: self.parse_to_string(4),
            version: self.parse_to_u32(),
            header_size: self.parse_to_u32(),
            world_size: self.parse_to_u32(),
            number_ents: self.parse_to_u32(),
            number_pvs: self.parse_to_u32(),
            number_lightmaps: self.parse_to_u32(),
            blend_map: self.parse_to_u32(),
            number_vars: self.parse_to_u32(),
            number_vslots: self.parse_to_u32(),
        }
    }

    fn parse_variable(&mut self) -> Variable {
        let var_type_byte = self.read_byte();

        let name_len = self.parse_to_u16();
        let name = self.parse_to_string(name_len);

        let var_type = match var_type_byte {
            0 => VariableType::Int(self.parse_to_u32()),
            1 => VariableType::Float(self.parse_to_f32()),
            2 => {
                let str_len = self.parse_to_u16();
                VariableType::String(str_len, self.parse_to_string(str_len))
            }
            _ => todo!("{}", var_type_byte),
        };

        Variable {
            var_type,
            name_len,
            name,
        }
    }

    fn parse_game_ident(&mut self) -> String {
        let str_len = self.read_byte();
        let str = self.parse_to_string(str_len.into());
        self.read_byte();

        str
    }

    fn parse_texture_mru(&mut self) -> Vec<u16> {
        let texture_mru_len = self.parse_to_u16();

        let mut texture_mru = vec![];

        for _ in 0..texture_mru_len {
            texture_mru.push(self.parse_to_u16());
        }

        texture_mru
    }

    fn parse_entity(&mut self) -> Entity {
        let ent = Entity {
            position: Position {
                x: self.parse_to_f32(),
                y: self.parse_to_f32(),
                z: self.parse_to_f32(),
            },
            attr1: self.parse_to_u16(),
            attr2: self.parse_to_u16(),
            attr3: self.parse_to_u16(),
            attr4: self.parse_to_u16(),
            attr5: self.parse_to_u16(),
            ent_type: match self.read_byte() {
                0 => EntityType::Empty,
                1 => EntityType::Light,
                2 => EntityType::MapModel,
                3 => EntityType::PlayerStart,
                4 => EntityType::EnvMap,
                5 => EntityType::Particles,
                6 => EntityType::Sound,
                7 => EntityType::Spotlight,
                8 => EntityType::IHealth,
                9 => EntityType::IAmmo,
                10 => EntityType::RaceStart,
                11 => EntityType::RaceFinish,
                12 => EntityType::RaceCheckpoint,
                13 => EntityType::PH4,
                14 => EntityType::PH5,
                15 => EntityType::PH6,
                16 => EntityType::PH7,
                17 => EntityType::PH8,
                18 => EntityType::PH9,
                19 => EntityType::Teleport,
                20 => EntityType::TeleDest,
                21 => EntityType::PH10,
                22 => EntityType::PH11,
                23 => EntityType::JumpPad,
                24 => EntityType::Base,
                25 => EntityType::PH12,
                26 => EntityType::PH13,
                27 => EntityType::PH14,
                28 => EntityType::PH15,
                29 => EntityType::PH16,
                30 => EntityType::Flag,
                31 => EntityType::MaxEntTypes,
                _ => todo!("{:#?}", self.input[self.position - 1]),
            },
        };

        // skip over reserved
        self.read_byte();

        ent
    }

    fn parse_vslots(&mut self, vslot_count: &mut i32) -> Vec<Box<VSlot>> {
        let mut prev = vec![-1; *vslot_count as usize];
        let mut vslots: Vec<Box<VSlot>> = vec![];

        while vslot_count > &mut 0 {
            let changed = self.parse_to_i32();

            if changed < 0 {
                println!("Changed: {}", changed);
                for _ in 0..changed.abs() {
                    vslots.push(Box::new(VSlot::new(None, vslots.len() as i32)));
                }
                *vslot_count += changed;
            } else {
                prev[vslots.len()] = self.parse_to_i32();
                vslots.push(self.parse_vslot(vslots.len() as i32, changed));
                *vslot_count -= 1;
            }
        }

        for pos in 0..vslots.len() {
            if prev[pos] >= 0 && prev[pos] < vslots.len() as i32 {
                vslots[prev[pos as usize] as usize].next =
                    Box::new(Some(*vslots[pos as usize].clone()));
                // FIXME: calling clone on a possibly recursive struct doesn't sound very
                // O(log(n)) to me :(
            }
        }

        vslots
    }

    fn parse_vslot(&mut self, vslot_length: i32, changed: i32) -> Box<VSlot> {
        let mut vslot = VSlot::new(None, vslot_length);
        vslot.changed = changed;

        // VSLOT_SHPARAM = 0
        if vslot.changed & (1 << 0) != 0 {
            let num_params = self.parse_to_u16();
            let mut name = String::new();

            for _ in 0..num_params {
                // TODO: implement MAXSTRLEN
                let nlen = self.parse_to_u16();
                name = self.parse_to_string(nlen);
                // name.push('\0');

                name = self.get_shader_param_name(name, true);

                let param = SlotShaderParam {
                    name,
                    loc: -1,
                    values: (
                        self.parse_to_f32(),
                        self.parse_to_f32(),
                        self.parse_to_f32(),
                        self.parse_to_f32(),
                    ),
                };

                vslot.params.push(param);
            }
        }

        // VSLOT_SCALE = 1
        if vslot.changed & (1 << 1) != 0 {
            vslot.scale = self.parse_to_f32();
        }

        // VSLOT_ROTATION = 2
        if vslot.changed & (1 << 2) != 0 {
            vslot.rotation = self.parse_to_i32().clamp(0, 7);
        }

        // VSLOT_OFFSET = 3
        if vslot.changed & (1 << 3) != 0 {
            vslot.offset.x = self.parse_to_i32();
            vslot.offset.y = self.parse_to_i32();
        }

        // VSLOT_SCROLL = 4
        if vslot.changed & (1 << 4) != 0 {
            vslot.scroll.x = self.parse_to_f32();
            vslot.scroll.y = self.parse_to_f32();
        }

        // VSLOT_LAYER = 5
        if vslot.changed & (1 << 5) != 0 {
            vslot.layer = self.parse_to_i32();
        }

        // VSLOT_ALPHA = 6
        if vslot.changed & (1 << 6) != 0 {
            vslot.alpha_front = self.parse_to_f32();
            vslot.alpha_back = self.parse_to_f32();
        }

        // VSLOT_COLOR = 7
        if vslot.changed & (1 << 7) != 0 {
            vslot.color_scale = Vector3::<f32> {
                x: self.parse_to_f32(),
                y: self.parse_to_f32(),
                z: self.parse_to_f32(),
            }
        }

        Box::new(vslot)
    }

    fn parse_lightmaps(&mut self, lightmap_count: u32) -> Vec<LightMap> {
        let mut lightmaps = vec![];

        for i in 0..lightmap_count {
            let mut lightmap = LightMap::new();

            let map_type = self.read_byte();

            lightmap.map_type = (map_type & 0x7F) as i32;
            lightmap.unlit_x = 0;
            lightmap.unlit_y = 0;

            if (map_type & 0x80) != 0 {
                lightmap.unlit_x = self.parse_to_u16() as i32;
                lightmap.unlit_y = self.parse_to_u16() as i32;
            }

            // LM_ALPHA = 16 (1 << 4)
            // LM_TYPE = 15
            // LM_BUMPMAP1 = 2
            if (lightmap.map_type & 16) != 0 && (lightmap.map_type & 15) != 2 {
                lightmap.bpp = 4;
            }

            // LM_PACKW = 512
            // LM_PACKH = 512
            for _ in 0..(lightmap.bpp * 512 * 512) {
                lightmap.data.push(self.read_byte());
            }

            lightmaps.push(lightmap);
        }

        lightmaps
    }

    fn parse_pvs(&mut self, pvs_count: i32) {
        let mut total_len = self.parse_to_u32();
        let mut num_water_planes = 0;
        let mut water_planes: Vec<WaterPlane> = vec![WaterPlane::new(); 32];
        let mut pvs: Vec<PVSData> = vec![];

        if (total_len & 0x80000000) != 0 {
            total_len &= !0x80000000;
            num_water_planes = self.parse_to_u32();

            for i in 0..num_water_planes {
                water_planes[i as usize].height = self.parse_to_i32();
            }
        }

        let mut offset = 0;

        for i in 0..pvs_count {
            let len = self.parse_to_u16();

            pvs.push(PVSData {
                offset: offset,
                len: len as i32,
            });

            offset += len as i32;
        }
    }

    // FIXME:
    // baggage from cardboard, looks weird bc works around odd
    // behaivior of HashSets in cardboard
    fn get_shader_param_name(&mut self, name: String, insert: bool) -> String {
        let exists = self.shader_param_names.get(&name.clone());

        if exists.is_some() || !insert {
            return exists.unwrap().to_string();
        } else {
            self.shader_param_names.insert(name.clone());
            name.clone()
        }
    }

    // copied almost verbatim from cardboard
    fn parse_cube<'a>(
        &'a mut self,
        cube: Box<Option<Cube>>,
        co: &Vector3<i32>,
        size: u32,
        failed: &mut bool,
    ) -> Box<Option<Cube>> {
        let mut has_children = false;
        let oct_sav = self.read_byte();

        let mut cube = cube.unwrap();

        // FIXME: none of the data read here is actually interpreted
        // the minimum required to traverse the file is stored, but everything
        // else is simply ignored
        match oct_sav & 0x7 {
            // Children
            0 => {
                cube.children = self.parse_children(co, size as i32 >> 1, failed);
                return Box::new(Some(cube));
            }
            // Empty
            1 => cube.edge_face = EdgeFace::Face([0x00000000; 3]),
            // Solid
            2 => cube.edge_face = EdgeFace::Face([0x80808080; 3]),
            // Normal
            3 => {
                let mut edges = vec![];

                for _ in 0..12 {
                    edges.push(self.read_byte());
                }

                cube.edge_face = EdgeFace::Edge(edges.try_into().unwrap());
            }
            // LODCube
            4 => has_children = true,
            _ => {
                *failed = false;
                return Box::new(Some(cube));
            }
        }

        for i in 0..6 {
            cube.textures[i] = self.parse_to_u16();
        }

        if (oct_sav & 0x40) != 0 {
            cube.material = self.parse_to_u16();
        }

        if (oct_sav & 0x80) != 0 {
            cube.merged = self.read_byte();
        }

        // holy fucking bingle
        if (oct_sav & 0x20) != 0 {
            let surface_mask: u8 = self.read_byte();
            let total_verts: u8 = self.read_byte().max(0);

            let mut offset = 0;

            for i in 0..6 {
                if surface_mask & (1 << i) != 0 {
                    // fields of surface mask struct
                    let surf_lmid: (u8, u8) = (self.read_byte(), self.read_byte());
                    let mut surf_verts = self.read_byte();
                    let surf_num_verts = self.read_byte();

                    let vert_mask: i32 = surf_verts as i32;

                    let num_verts = if surf_num_verts & (1 << 7) != 0 {
                        (surf_num_verts & 15) * 2
                    } else {
                        surf_num_verts & 15
                    };

                    if num_verts == 0 {
                        surf_verts = 0;
                        continue;
                    }

                    surf_verts = offset;
                    offset += num_verts;

                    let layer_verts = surf_num_verts & 15;

                    let mut has_xyz = vert_mask & 0x04 != 0;
                    let mut has_uv = vert_mask & 0x40 != 0;
                    let mut has_norm = vert_mask & 0x80 != 0;

                    if has_xyz {
                        //do stuff
                    } else {
                        // do other stuff
                    }

                    if layer_verts == 4 {
                        if has_xyz && (vert_mask & 0x01) != 0 {
                            self.parse_to_u16();
                            self.parse_to_u16();
                            self.parse_to_u16();
                            self.parse_to_u16();

                            has_xyz = false;
                        }
                        if has_uv && (vert_mask & 0x02) != 0 {
                            self.parse_to_u16();
                            self.parse_to_u16();
                            self.parse_to_u16();
                            self.parse_to_u16();

                            if surf_num_verts & (1 << 7) != 0 {
                                self.parse_to_u16();
                                self.parse_to_u16();
                                self.parse_to_u16();
                                self.parse_to_u16();
                            }

                            has_uv = false;
                        }
                    }

                    if has_norm && (vert_mask & 0x08) != 0 {
                        self.parse_to_u16();
                        has_norm = false;
                    }

                    if has_xyz || has_uv || has_norm {
                        for _ in 0..layer_verts {
                            if has_xyz {
                                self.parse_to_u16();
                                self.parse_to_u16();
                            }
                            if has_uv {
                                self.parse_to_u16();
                                self.parse_to_u16();
                            }
                            if has_norm {
                                self.parse_to_u16();
                            }
                        }
                    }
                    if surf_num_verts & (1 << 7) != 0 {
                        for _ in 0..layer_verts {
                            self.read_byte();
                            self.read_byte();
                        }
                    }
                }
            }
        }

        cube.children = if has_children {
            self.parse_children(co, size as i32 >> 1, failed)
        } else {
            vec![
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
                Box::new(None),
            ]
        };

        Box::new(Some(cube))
    }

    fn new_cubes(face: Option<u32>, material: Option<u16>) -> [Box<Option<Cube>>; 8] {
        let face = face.unwrap_or(0); // F_EMPTY
        let material = material.unwrap_or(0); // MAT_AIR

        let mut cubes: [Box<Option<Cube>>; 8] = [
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
        ];

        for i in 0..8 {
            let cube = Cube {
                children: vec![
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                    Box::new(None),
                ], // Cube cannot implement the copy trait, we cannot use [x; n]
                edge_face: EdgeFace::Face([face, face, face]),
                textures: [1, 1, 1, 1, 1, 1],
                material,
                merged: 0,
                escaped_visible: EscapedVisible::Visible(0),
                cube_ext: None,
            };

            cubes[i] = Box::new(Some(cube));
        }

        cubes
    }

    fn parse_children<'a>(
        &mut self,
        co: &Vector3<i32>,
        size: i32,
        failed: &mut bool,
    ) -> Vec<Box<Option<Cube>>> {
        let cubes = Parser::new_cubes(None, None);

        let mut parsed_cubes: Vec<Box<Option<Cube>>> = vec![
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
            Box::new(None),
        ];

        for (i, cube) in cubes.into_iter().enumerate() {
            self.cube_count += 1;
            parsed_cubes[i] = self.parse_cube(cube, co, size.try_into().unwrap(), failed);
            if *failed {
                break;
            };
        }

        parsed_cubes
    }

    fn parse_to_string(&mut self, byte_count: u16) -> String {
        let mut string = String::new();

        for _ in 0..byte_count {
            string.push(self.read_byte().into());
        }

        string
    }

    // TODO:
    // make these generic
    fn parse_to_i32(&mut self) -> i32 {
        i32::from_le_bytes([
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ])
    }

    fn parse_to_u32(&mut self) -> u32 {
        u32::from_le_bytes([
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ])
    }

    fn parse_to_f32(&mut self) -> f32 {
        f32::from_le_bytes([
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
            self.read_byte(),
        ])
    }

    fn parse_to_u16(&mut self) -> u16 {
        u16::from_le_bytes([self.read_byte(), self.read_byte()])
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.input[self.position];
        self.position += 1;
        byte
    }
}
