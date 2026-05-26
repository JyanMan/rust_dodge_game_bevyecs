use bevy_ecs::prelude::*;

use super::*;
use crate::components::*;
use crate::components::states::*;

pub struct WeaponContext<'a, 'w, 's> {
    pub self_e: Entity,
    pub combat: &'a mut Combat,
    pub weapon_d: &'a mut WeaponConfig,
    pub commands: &'a mut Commands<'w, 's>,
    pub grav: &'a mut GravityAffected,
    pub vel: &'a mut Velocity,
    pub sprite: &'a mut Sprite,
    pub trans: &'a mut Transform,
    pub local: &'a mut LocalTransform,
    pub anim_player: &'a mut AnimationPlayer,
    pub obb: &'a mut OBB
}

type WeaponFn = fn(&mut WeaponContext);

#[derive(Component)]
pub struct WeaponFns {
    pub start_attack: WeaponFn,
    pub start_dodge_attack: WeaponFn,
    pub while_attacking: WeaponFn,
    pub while_dodge_attacking: WeaponFn,
    pub after_effect: WeaponFn,
    pub after_dodge_effect: WeaponFn,
    pub end_attack: WeaponFn,
    pub end_dodge_attack: WeaponFn,
}

pub struct WeaponIdleContext<'a> {
    pub local: &'a mut LocalTransform,
    pub trans: &'a mut Transform,
    pub sprite: &'a mut Sprite,
    pub user_vel: &'a Velocity,
}

#[derive(Component, Clone)]
pub struct WeaponConfig {
    // pub state: WeaponState,
    pub damage: i32,
    pub knock_force: f32,
    pub attacking: bool,
    pub can_attack: bool,
    pub attack_dir: Vector2, // for animation
    pub knock_dir: Vector2, // actual knock for hit targets
    // pub after_effect: bool,
    // pub after_effect_timer: f32,
    // pub after_effect_duration: f32,
    pub attack_timer: Timer,
    pub after_effect_timer: Timer,
    pub attack_cd_timer: Timer,
    // owner_attack_cd: f32,
}

// impl Default for WeaponData {
//     fn default() -> Self {
//         Self {
//             // state: WeaponState::default(),
//             damage: 1,
//             knock_force: 5.0,
//             attack_timer: 0.0,
//             attack_duration: 0.5,
//             attack_cd: 0.1,
//             after_effect_timer: 0.0,
//             after_effect: false,
//             after_effect_duration: 0.0,
//             attacking: false,
//             attack_dir: Vector2::zero(),
//             knock_dir: Vector2::zero(),
//             can_attack: true,
//             // cd_timer: 0.0,
//             // owner_attack_cd: 0.0,
//         }
//     }
// }

impl WeaponConfig {
    // pub fn new(damage: i32, knock_force: f32, attack_duration: f32, attack_cd: f32, after_effect_duration: f32,  state: WeaponState, w_type: WeaponType) -> Self {
    //     Self {
    //         damage,
    //         knock_force,
    //         // attack_duration,
    //         after_effect: false,
    //         // after_effect_timer: 0.0,
    //         after_effect_duration,
    //         attack_dir: Vector2::zero(),
    //         knock_dir: Vector2::zero(),
    //         // attack_timer: 0.0,
    //         attacking: false,
    //         can_attack: true,
    //         attack_timer: Timer::new(attack_duration),
    //         attack_cd_timer: Timer::new(attack_cd),
    //         after_effect_timer: Timer::new(after_effect_duration),
    //         // attack_cd,
    //         // cd_timer: 0.0,
    //         owner_attack_cd: 0.0,
    //     }
    // }

    // pub fn attack_timer(&mut self, delta_time: f32, state: &mut StateMachine<WeaponState>) {
    //     assert!(self.attacking);
    //     self.attack_timer += delta_time;
    //     if self.attack_timer >= self.attack_duration {
    //         self.attack_timer = 0.0;
    //         self.attacking = false;
    //         // self.state = WeaponState::EndAttack;
    //         state.set_state(WeaponState::AfterEffectAttack);
    //         self.after_effect = true;
    //         // self.can_attack = false;
    //     }
    // }
    // pub fn attack_cd_timer(&mut self, delta_time: f32, state: &mut StateMachine<WeaponState>) {
    //     assert!(!self.can_attack);
    //     self.cd_timer += delta_time;
    //     if self.cd_timer >= (self.attack_cd + self.owner_attack_cd) {
    //         self.cd_timer = 0.0;
    //         self.owner_attack_cd = 0.0;
    //         self.can_attack = true;
    //         // self.can_attack = false;
    //     }
    // }

    pub fn temporary_attack_disable(&mut self) {
        // this waits for the timer for can_attack
        // see weapon_system attack_cd_timer usage
        self.can_attack = false;
    }

    // pub fn after_effect_timer(&mut self, delta_time: f32, state: &mut StateMachine<WeaponState>) {
    //     assert!(self.after_effect);
    //     self.after_effect_timer += delta_time;
    //     if self.after_effect_timer >= self.after_effect_duration {
    //         self.after_effect_timer = 0.0;
    //         self.after_effect = false;
    //         state.set_state(WeaponState::EndAttack);
    //     }
    // }

    pub fn attack(&mut self, owner_attack_cd: f32) {
        // state.set_state(WeaponState::StartAttack);
        // self.attack_timer = 0.0;
        self.attack_timer.reset();
        self.after_effect_timer.reset();
        self.attacking = true;
        self.can_attack = false;
        // self.owner_attack_cd = owner_attack_cd;
    }
}

// #[derive(Clone, Default, PartialEq)]
// pub enum WeaponState {
//     #[default]
//     Idle,
//     Running,
//     Aired,
// }

#[derive(Default, Clone)]
#[repr(usize)]
pub enum WeaponAnim {
    #[default]
    Attack,
}

impl WeaponAnim {
    pub const COUNT: usize = 1;
    pub fn usize(self) -> usize {
        self as usize
    }
}
