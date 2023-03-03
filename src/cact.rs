use binrw::{BinRead};
use serde::{Serialize, Deserialize};
use crate::cfile::FileHeader;
use std::{str, mem};

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionHeader {
    data_version: u32,
    #[serde(skip_serializing)]
    action_count: i32,
    #[serde(skip_serializing)]
    anim_offset: u32,
    #[serde(skip_serializing)]
    anim_offset_2: u32,
    #[serde(skip_serializing)]
    camera_offset: u32,
    #[serde(skip_serializing)]
    bone_offset: u32,
}

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionDataInfo {
    category_id: i32,
    sub_category_id: i32,
    end_frame: i32,
    loop_back_frame: i32,
    option_flag: i32,
    reserve1: i32,
    reserve2: i32,
    reserve3: i32,
    reserve4: i32,
    #[serde(skip_serializing)]
    line_count: i32,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataBaseAnime
{
    motion_file_id: i32,
    motion_id: i32,
    motion_frame: f32,
    blend: f32,
    transparent: f32, 
    #[serde(skip_serializing)]
    padding: [u32; 5],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataFaceAnime
{
    motion_file_id: i32,
    motion_id: i32,
    motion_frame: f32,
    blend: f32,
    #[serde(skip_serializing)]
    padding: [u32; 6],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataUnkAnime
{
    motion_file_id: i32,
    motion_id: i32,
    motion_frame: f32,
    blend: f32,
    #[serde(skip_serializing)]
    padding: [u32; 6],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataCollision
{
    rect_id: i32,
    rect_attr: i32,
    flag: i32,
    branch_key: i32,
    bind_index: i32,
    push_rate: f32,
    rect: CollisionHitRect,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataUnkCollision
{
    rect_id: i32,
    rect_attr: i32,
    flag: i32,
    branch_key: i32,
    bind_index: i32,
    push_rate: f32,
    rect: CollisionHitRect,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Copy, Clone)]
struct CollisionHitRect
{
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataAttack
{
    dataid: i32,
    group_id: i32,
    #[serde(flatten)]
    rect: CollisionHitRect,
    flag: i32,
    #[serde(skip_serializing)]
    padding: [u32; 3],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataCancel
{
    flg1: i32,
    flg2: i32,
    flg3: i32,
    terms: i32,
    flg4: i32,
    precede_frame: i32,
    #[serde(skip_serializing)]
    padding: [u32; 4],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataBranch
{
    type_1: i32,
    param: f32,
    action_id: i32,
    action_frame: i32,
    type_2: i32,
    param_2: f32,
    #[serde(skip_serializing)]
    padding: [u32; 4],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataMove
{
    x: f32,
    y: f32,
    col_x: f32,
    col_y: f32,
    option_flag: i32,
    #[serde(skip_serializing)]
    padding: [u32; 5],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataOffset
{
    x: f32,
    y: f32,
    z: f32,
    #[serde(skip_serializing)]
    padding: [u32; 7],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataSpeed
{
    set_flag: i32,
    move_x: f32,
    move_y: f32,
    add_x: f32,
    add_y: f32,
    option_flag: i32,
    #[serde(skip_serializing)]
    padding: [u32; 4],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataActionFlag
{
    flag: i32,
    param1: i32,
    #[serde(skip_serializing)]
    padding: [u32; 8],
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Clone, Copy)]
struct EffectTypeSet {
    padding: [u8; 3],
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    pos_base: i16,
    act_file_id: i16,
    term_flag: i16,
    act_number: i16,
    option_flag: i32,
    unk_x: f32,
    unk_y: f32,
    unk_z: f32,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Clone, Copy)]
struct EffectTypeControl {
    padding: [u8; 3],
    reserve1: i32,
    reserve2: i32,
    reserve3: i32,
    pos_base: i16,
    act_file_id: i16,
    term_flag: i16,
    act_number: i16,
    reserve4: i32,
    padding2: [u32; 3],
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum EffectType {
    Set(EffectTypeSet),
    Control(EffectTypeControl),
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize)]
struct CharaActionDataEffect
{
    eff_type: EffectType,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Clone, Copy)]
struct SoundPlay
{
    volume: f32,
    common_id: i32,
    unique_id: i16,
    group_id: i16,
    priority: i32,
    option_flag: i16,
    tag_id: i16,
    option_param: f32,
}

#[repr(C, packed)]
#[derive(Serialize, Deserialize, Clone, Copy)]
struct SoundStop
{
    fade_sec: f32,
    reserve_1: i16,
    tag_id: i16,
    reserve2: i32,
    reserve3: i32,
    reserve4: i32,
    reserve5: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum SoundType {
    Play(SoundPlay),
    Stop(SoundStop),
    DefaultSound([u32; 6]),
}

impl Default for SoundType {
    fn default() -> Self {
        SoundType::DefaultSound([ 0u32; 6 ]) 
    }
}

#[derive(Serialize, Deserialize)]
struct CharaActionDataSound
{
    #[serde(skip_serializing)]
    data: [u32; 6],
    #[serde(skip_serializing)]
    padding: [u32; 3],
    #[serde(skip_serializing)]
    sound_type: u32,
    sound: SoundType,
}

#[binrw::parser(reader, endian)]
fn line_parser(count: usize, _: u8) -> binrw::BinResult<Vec<CharaActionDataActionLine>> {
    let mut line_vec: Vec<CharaActionDataActionLine> = Vec::new();
    for _ in 0..count {
        let mut line = <CharaActionDataActionLine>::read_options(reader, endian, ()).unwrap();
        for mut frame in &mut line.frame {
            match line.action_line_id {
                0 => {
                    let mut base_anime: CharaActionDataBaseAnime = unsafe { mem::zeroed() };
                    unsafe {
                        base_anime = mem::transmute::<[u32; 10], CharaActionDataBaseAnime>(frame.data);
                    }                
                    frame.line = Line::BaseAnime(base_anime)
                },
                1 => {
                    let mut face_anime: CharaActionDataFaceAnime = unsafe { mem::zeroed() };
                    unsafe {
                        face_anime = mem::transmute::<[u32; 10], CharaActionDataFaceAnime>(frame.data);
                    }                
                    frame.line = Line::FaceAnime(face_anime)
                },
                2 => {
                    let mut unk_anime: CharaActionDataUnkAnime = unsafe { mem::zeroed() };
                    unsafe {
                        unk_anime = mem::transmute::<[u32; 10], CharaActionDataUnkAnime>(frame.data);
                    }                
                    frame.line = Line::UnkAnime(unk_anime)
                },
                3 => {
                    let mut collision: CharaActionDataCollision = unsafe { mem::zeroed() };
                    unsafe {
                        collision = mem::transmute::<[u32; 10], CharaActionDataCollision>(frame.data);
                    }                
                    frame.line = Line::Collision(collision)
                },
                4 => {
                    let mut unk_collision: CharaActionDataUnkCollision = unsafe { mem::zeroed() };
                    unsafe {
                        unk_collision = mem::transmute::<[u32; 10], CharaActionDataUnkCollision>(frame.data);
                    }                
                    frame.line = Line::UnkCollision(unk_collision)
                },
                5 => {
                    let mut attack: CharaActionDataAttack = unsafe { mem::zeroed() };
                    unsafe {
                        attack = mem::transmute::<[u32; 10], CharaActionDataAttack>(frame.data);
                    }                
                    frame.line = Line::Attack(attack)
                },
                6 => {
                    let mut cancel: CharaActionDataCancel = unsafe { mem::zeroed() };
                    unsafe {
                        cancel = mem::transmute::<[u32; 10], CharaActionDataCancel>(frame.data);
                    }                
                    frame.line = Line::Cancel(cancel)
                },
                7 => {
                    let mut branch: CharaActionDataBranch = unsafe { mem::zeroed() };
                    unsafe {
                        branch = mem::transmute::<[u32; 10], CharaActionDataBranch>(frame.data);
                    }                
                    frame.line = Line::Branch(branch)
                },
                8 => {
                    let mut line_move: CharaActionDataMove = unsafe { mem::zeroed() };
                    unsafe {
                        line_move = mem::transmute::<[u32; 10], CharaActionDataMove>(frame.data);
                    }                
                    frame.line = Line::Move(line_move)
                },
                9 => {
                    let mut offset: CharaActionDataOffset = unsafe { mem::zeroed() };
                    unsafe {
                        offset = mem::transmute::<[u32; 10], CharaActionDataOffset>(frame.data);
                    }                
                    frame.line = Line::Offset(offset)
                },
                10 => {
                    let mut speed: CharaActionDataSpeed = unsafe { mem::zeroed() };
                    unsafe {
                        speed = mem::transmute::<[u32; 10], CharaActionDataSpeed>(frame.data);
                    }                
                    frame.line = Line::Speed(speed)
                },
                11 => {
                    let mut action_flag: CharaActionDataActionFlag = unsafe { mem::zeroed() };
                    unsafe {
                        action_flag = mem::transmute::<[u32; 10], CharaActionDataActionFlag>(frame.data);
                    }                
                    frame.line = Line::ActionFlag(action_flag)
                },
                14 => {
                    let mut effect: CharaActionDataEffect = unsafe { mem::zeroed() };
                    unsafe {
                        effect = mem::transmute::<[u32; 10], CharaActionDataEffect>(frame.data);
                    }
                    frame.line = Line::Effect(effect)
                },
                15 => {
                    let mut sound: CharaActionDataSound = unsafe { mem::zeroed() };
                    sound.data.clone_from_slice(&frame.data[..6]);
                    sound.sound_type = frame.data[6];
                    match sound.sound_type
                    {
                        0 => {
                            let mut play: SoundPlay = unsafe { mem::zeroed() };
                            unsafe {
                                play = mem::transmute::<[u32; 6], SoundPlay>(sound.data);
                            }                
                            sound.sound = SoundType::Play(play)
                        },
                        1 => {
                            let mut stop: SoundStop = unsafe { mem::zeroed() };
                            unsafe {
                                stop = mem::transmute::<[u32; 6], SoundStop>(sound.data);
                            }                
                            sound.sound = SoundType::Stop(stop)
                        },
                        _ => sound.sound = SoundType::DefaultSound(sound.data),
                    }
                    frame.line = Line::Sound(sound)
                },
                _ => frame.line = Line::DefaultLine(frame.data),
            }
        }
        line_vec.push(line);
    }
    Ok(line_vec)
}

#[derive(Serialize, Deserialize)]
enum Line {
    BaseAnime(CharaActionDataBaseAnime),
    FaceAnime(CharaActionDataFaceAnime),
    UnkAnime(CharaActionDataUnkAnime),
    Collision(CharaActionDataCollision),
    UnkCollision(CharaActionDataUnkCollision),
    Attack(CharaActionDataAttack),
    Cancel(CharaActionDataCancel),
    Branch(CharaActionDataBranch),
    Move(CharaActionDataMove),
    Offset(CharaActionDataOffset),
    Speed(CharaActionDataSpeed),
    ActionFlag(CharaActionDataActionFlag),
    Effect(CharaActionDataEffect),
    Sound(CharaActionDataSound),
    DefaultLine([u32; 10]),
}

impl Default for Line {
    fn default() -> Self {
        Line::DefaultLine([ 0u32; 10 ]) 
    }
}

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionLineFrame {
    frame: i32,
    #[serde(skip_serializing)]
    data: [u32; 10],
    #[br(ignore)]
    line: Line,
}

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionLine {
    #[serde(skip_serializing)]
    key_frame_count: i32,
    action_line_id: i32,
    #[br(count = key_frame_count)]
    frame: Vec<CharaActionDataActionLineFrame>,
}

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionData {
    info: CharaActionDataActionDataInfo,
    #[br(args(info.line_count as usize, 0))]
    #[br(parse_with = line_parser)]
    frame: Vec<CharaActionDataActionLine>,
}

#[binrw::parser(reader, endian)]
fn name_parser(count: usize, _: u8) -> binrw::BinResult<Vec<Name>> {
    let mut name_vec: Vec<Name> = Vec::new();
    for _ in 0..count {
        let mut name = Name::read_options(reader, endian, ()).unwrap();
        name.pretty_name = match str::from_utf8(&name.name) {
            Ok(pretty) => pretty.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        name_vec.push(name);
    }
    Ok(name_vec)
}

#[derive(BinRead, Serialize, Deserialize)]
struct Name {
    #[serde(skip_serializing)]
    name_size: u32,
    #[br(count = name_size)]
    #[serde(skip_serializing)]
    name: Vec<u8>,
    #[br(ignore)]
    pretty_name: String,
}

#[derive(BinRead, Serialize, Deserialize)]
struct NameList {
    #[serde(skip_serializing)]
    demo_count: u32,
    unk1: [u32; 9],
    #[br(args(demo_count as usize, 0))]
    #[br(parse_with = name_parser)]
    demo_names: Vec<Name>,
    unk2: u32,
    #[serde(skip_serializing)]
    chara_anim_count: u32,
    #[br(args(chara_anim_count as usize, 0))]
    #[br(parse_with = name_parser)]
    chara_anim_names: Vec<Name>,
    #[serde(skip_serializing)]
    cmn_anim_count: u32,
    #[br(args(cmn_anim_count as usize, 0))]
    #[br(parse_with = name_parser)]
    cmn_anim_names: Vec<Name>,
    #[serde(skip_serializing)]
    face_count: u32,
    #[br(args(face_count as usize, 0))]
    #[br(parse_with = name_parser)]
    face_names: Vec<Name>, 
    #[serde(skip_serializing)]
    face_count_2: u32,
    #[br(args(face_count_2 as usize, 0))]
    #[br(parse_with = name_parser)]
    face_names_2: Vec<Name>,
    unk3: u64,
    #[serde(skip_serializing)]
    chara_anim_count_2: u32,
    #[br(args(chara_anim_count_2 as usize, 0))]
    #[br(parse_with = name_parser)]
    chara_anim_names_2: Vec<Name>,
    unk4: u32,
    #[serde(skip_serializing)]
    mat_count: u32,
    #[br(args(mat_count as usize, 0))]
    #[br(parse_with = name_parser)]
    mat_names: Vec<Name>,
    unk5: u32,
    #[serde(skip_serializing)]
    camera_count: u32,
    #[br(args(camera_count as usize, 0))]
    #[br(parse_with = name_parser)]
    camera_names: Vec<Name>,
    #[serde(skip_serializing)]
    bone_count: u32,
    #[br(args(bone_count as usize, 0))]
    #[br(parse_with = name_parser)]
    bone_names: Vec<Name>,
}

#[derive(BinRead, Serialize, Deserialize)]
#[br(magic = b"\x00\x00\x00\x00")]
pub(crate) struct CharaActionData {
    #[serde(flatten)]
    file_header: FileHeader,
    #[serde(flatten)]
    data_header: CharaActionDataActionHeader,

    #[br(count = data_header.action_count)]
    frame: Vec<CharaActionDataActionData>,

    #[br(args(data_header.action_count as usize, 0))]
    #[br(parse_with = name_parser)]
    act_names: Vec<Name>,

    #[serde(flatten)]
    name_list: NameList,
}