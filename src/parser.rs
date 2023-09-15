pub struct Parser {
    pub input: Vec<u8>,
    pub position: usize,
}

#[derive(Debug)]
pub struct MapHeader {
    pub magic_field: String,
    pub version: u32,
    pub header_size: u32,
    pub world_size: u32,
    pub number_ents: u32,
    pub number_pvs: u32,
    pub light_maps: u32,
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

#[derive(Debug)]
pub struct Entity {
    position: Position,
    attr1: u16,
    attr2: u16,
    attr3: u16,
    attr4: u16,
    attr5: u16,
    ent_type: EntityType,
}

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
pub struct IVector {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
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
pub struct Cube<'a> {
    children: [Option<&'a mut Cube<'a>>; 8], // "points to 8 cube structures which are its children, or NULL. -Z first, then -Y, -X"
    edge_face: EdgeFace,
    textures: (u16, u16, u16, u16, u16, u16), // "one for each face. same order as orient." (6 entries)
    material: u16,                            // empty-space material
    merged: u8,                               // merged faces of the cube
    escaped_visible: EscapedVisible,
}

#[derive(Debug, Clone)]
pub enum EdgeFace {
    Edge(Vec<u8>), // edges of the cube, each uchar is 2 4bit values denoting the range (there should be 12 entries here)
    Face(Vec<u32>), // 4 edges of each dimension together representing 2 perpendicular faces (there should be 3 entries here)
}

#[derive(Debug, Copy, Clone)]
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

impl Parser {
    pub fn new(input: Vec<u8>) -> Self {
        Parser { input, position: 0 }
    }

    pub fn parse_map(&mut self) {
        let header = self.parse_header();
        println!("{:#?}", header);

        let mut vars = Vec::new();

        for _ in 0..header.number_vars {
            let variable = self.parse_variable();
            println!("{:#?}", variable);
            vars.push(variable);
        }

        let game_ident = self.parse_game_ident();
        println!("{:#?}", game_ident);

        // load bearing printlns
        println!("{}", self.read_byte());
        println!("{}", self.read_byte());
        println!("{}", self.read_byte());
        println!("{}", self.read_byte());

        let texture_mru = self.parse_texture_mru();
        // println!("{:#?}", texture_mru);

        println!("{:#?}", header);
        let mut entities = vec![];
        for _ in 0..header.number_ents {
            let entity = self.parse_entity();
            println!("{:#?}", entity);
            entities.push(entity);
        }

        let world_root =
            self.parse_children(&IVector { x: 0, y: 0, z: 0 }, header.world_size as i32 >> 1);

        // self.parse_cube(None, None);
    }

    fn parse_header(&mut self) -> MapHeader {
        MapHeader {
            magic_field: self.parse_to_string(4),
            version: self.parse_to_u32(),
            header_size: self.parse_to_u32(),
            world_size: self.parse_to_u32(),
            number_ents: self.parse_to_u32(),
            number_pvs: self.parse_to_u32(),
            light_maps: self.parse_to_u32(),
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
        println!("{}", texture_mru_len);
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
                _ => todo!("{:#?}", self.input[self.position - 3]),
            },
        };

        // skip over reserved
        self.read_byte();

        ent
    }

    // copied almost verbatim from cardboard
    fn parse_cube<'a>(
        &'a mut self,
        cube: Box<&'a mut Cube<'a>>,
        co: &IVector,
        size: u32,
    ) -> Box<&mut Cube> {
        let mut has_children = false;
        let oct_sav = self.read_byte();

        match oct_sav & 0x7 {
            // Children
            0 => {
                cube.children = self.parse_children(co, size.try_into().unwrap());
                return cube;
            }
            // Empty
            1 => cube.edge_face = EdgeFace::Face(vec![0x00000000; 3]),
            // Solid
            2 => cube.edge_face = EdgeFace::Face(vec![0x80808080; 3]),
            // Normal
            3 => {
                let mut edges = vec![];

                for _ in 0..12 {
                    edges.push(self.read_byte());
                }

                cube.edge_face = EdgeFace::Edge(edges);
            }
            // LODCube
            4 => has_children = true,
            _ => todo!(),
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
        }

        println!("{:#?}", cube);

        cube
    }

    fn new_cubes<'a>(face: Option<u32>, material: Option<u16>) -> [Option<&'a mut Cube<'a>>; 8] {
        let face = face.unwrap_or(0); // F_EMPTY
        let material = material.unwrap_or(0); // MAT_AIR

        let mut cubes: &mut Vec<Box<&mut Cube>> = &mut Vec::new();

        for i in 0..8 {
            let children: Vec<Option<&mut Cube>> = Vec::new();

            let mut cube = Cube {
                children: [None; 8],
                edge_face: EdgeFace::Face(vec![face, face, face, face]),
                textures: vec![1, 1, 1, 1, 1, 1],
                material,
                merged: 0,
                escaped_visible: EscapedVisible::Visible(0),
            };

            cubes.push(Box::new(&mut cube));
        }

        cubes
    }

    fn parse_children(&mut self, co: &IVector, size: i32) -> [Option<&mut Cube>; 8] {
        let cubes = Parser::new_cubes(None, None);

        for i in 0..8 {
            self.parse_cube(cubes[i], co, size.try_into().unwrap());
        }

        cubes.into()
    }

    fn parse_to_string(&mut self, byte_count: u16) -> String {
        let mut string = String::new();

        for _ in 0..byte_count {
            string.push(self.read_byte().into());
        }

        string
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
