use crate::components::controller::SkillKey;
use crate::components::skills::skill::Skills;
use crate::components::status::{
    ApplyStatusComponent, ApplyStatusInAreaComponent, RemoveStatusComponent,
};
use crate::components::{ApplyForceComponent, AreaAttackComponent, AttackComponent};
use crate::consts::{JobId, MonsterId};
use crate::video::{DynamicVertexArray, GlTexture};
use crate::{DeltaTime, ElapsedTime, MapRenderData, RenderMatrices, Shaders, SpriteResource, Tick};
use nphysics2d::object::ColliderHandle;
use std::collections::HashMap;
use std::time::Instant;

pub mod atk_calc;
pub mod char_state_sys;
pub mod control_sys;
pub mod input;
pub mod phys;
pub mod render;
pub mod skill_sys;
pub mod ui;

pub struct EffectSprites {
    pub torch: SpriteResource,
    pub fire_wall: SpriteResource,
    pub fire_ball: SpriteResource,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

pub struct Sprites {
    pub cursors: SpriteResource,
    pub numbers: GlTexture,
    pub character_sprites: HashMap<JobId, [SpriteResource; 2]>,
    pub mounted_character_sprites: HashMap<JobId, [SpriteResource; 2]>,
    pub head_sprites: [Vec<SpriteResource>; 2],
    pub monster_sprites: HashMap<MonsterId, SpriteResource>,
    pub effect_sprites: EffectSprites,
}

pub struct Texts {
    pub skill_name_texts: HashMap<Skills, GlTexture>,
    pub skill_key_texts: HashMap<SkillKey, GlTexture>,
    pub attack_absorbed: GlTexture,
    pub attack_blocked: GlTexture,
}

pub struct SystemVariables {
    pub sprites: Sprites,
    pub shaders: Shaders,
    pub tick: Tick,
    /// seconds the last frame required
    pub dt: DeltaTime,
    /// extract from the struct?
    pub time: ElapsedTime,
    pub matrices: RenderMatrices,
    pub map_render_data: MapRenderData,
    pub texts: Texts,
    pub skill_icons: HashMap<Skills, GlTexture>,
    pub attacks: Vec<AttackComponent>,
    pub area_attacks: Vec<AreaAttackComponent>,
    pub pushes: Vec<ApplyForceComponent>,
    pub apply_statuses: Vec<ApplyStatusComponent>,
    pub apply_area_statuses: Vec<ApplyStatusInAreaComponent>,
    pub remove_statuses: Vec<RemoveStatusComponent>,
    // Todo: put it into the new Graphic module if it is ready
    pub str_effect_vao: DynamicVertexArray,
}

pub struct Collision {
    pub character_coll_handle: ColliderHandle,
    pub other_coll_handle: ColliderHandle,
}

pub struct CollisionsFromPrevFrame {
    pub collisions: Vec<Collision>,
}

pub struct SystemFrameDurations(pub HashMap<&'static str, u32>);

impl SystemFrameDurations {
    pub fn system_finished(&mut self, started: Instant, name: &'static str) {
        let duration = Instant::now().duration_since(started).as_millis() as u32;
        self.0.insert(name, duration);
    }

    pub fn start_measurement(&mut self, name: &'static str) -> SystemStopwatch {
        SystemStopwatch::new(name, self)
    }
}

pub struct SystemStopwatch<'a> {
    // let now_ms = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
    started: Instant,
    name: &'static str,
    times: &'a mut SystemFrameDurations,
}

impl<'a> SystemStopwatch<'a> {
    pub fn new(name: &'static str, times: &'a mut SystemFrameDurations) -> SystemStopwatch<'a> {
        SystemStopwatch {
            started: Instant::now(),
            name,
            times,
        }
    }
}

impl<'a> Drop for SystemStopwatch<'a> {
    fn drop(&mut self) {
        self.times.system_finished(self.started, self.name);
    }
}
