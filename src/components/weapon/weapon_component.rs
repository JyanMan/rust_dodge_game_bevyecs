use bevy_ecs::prelude::*;

use super::*;
use crate::components::*;

#[derive(Clone, Default, PartialEq)]
pub enum WeaponState {
    #[default]
    Unowned,
    Owned,
    Idle,
    StartAttack,
    Attacking,
    EndAttack,
    AfterEffectAttack,
}

#[derive(Component)]
pub struct WeaponFns {
    pub start_attack: fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform),
    pub while_attacking: fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform),
    pub after_effect: fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform),
    pub end_attack: fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform),
}

#[derive(Component, Clone)]
pub struct WeaponData {
    pub w_type: WeaponType,
    pub state: WeaponState,
    pub damage: i32,
    pub knock_force: f32,
    pub attacking: bool,
    pub can_attack: bool,
    pub attack_dir: Vector2,
    pub after_effect: bool,
    pub after_effect_timer: f32,
    pub after_effect_duration: f32,
    attack_cd: f32,
    cd_timer: f32,
    attack_timer: f32,
    attack_duration: f32,
}

impl Default for WeaponData {
    fn default() -> Self {
        Self {
            w_type: WeaponType::default(),
            state: WeaponState::default(),
            damage: 1,
            knock_force: 5.0,
            attack_timer: 0.0,
            attack_duration: 0.5,
            attack_cd: 0.1,
            after_effect_timer: 0.0,
            after_effect: false,
            after_effect_duration: 0.0,
            attacking: false,
            attack_dir: Vector2::zero(),
            can_attack: true,
            cd_timer: 0.0
        }
    }
}

impl WeaponData {
    pub fn new(damage: i32, knock_force: f32, attack_duration: f32, attack_cd: f32, after_effect_duration: f32,  state: WeaponState, w_type: WeaponType) -> Self {
        Self {
            w_type: w_type,
            state: state,
            damage: damage,
            knock_force: knock_force,
            attack_duration: attack_duration,
            after_effect: false,
            after_effect_timer: 0.0,
            after_effect_duration,
            attack_dir: Vector2::zero(),
            attack_timer: 0.0,
            attacking: false,
            can_attack: true,
            attack_cd,
            cd_timer: 0.0
        }
    }

    pub fn attack_timer(&mut self, delta_time: f32) {
        assert!(self.attacking);
        self.attack_timer += delta_time;
        if self.attack_timer >= self.attack_duration {
            self.attack_timer = 0.0;
            self.attacking = false;
            // self.state = WeaponState::EndAttack;
            self.state = WeaponState::AfterEffectAttack;
            self.after_effect = true;
            // self.can_attack = false;
        }
    }
    pub fn attack_cd_timer(&mut self, delta_time: f32) {
        assert!(!self.can_attack);
        self.cd_timer += delta_time;
        if self.cd_timer >= self.attack_cd {
            self.cd_timer = 0.0;
            self.can_attack = true;
            // self.can_attack = false;
        }
    }

    pub fn after_effect_timer(&mut self, delta_time: f32) {
        assert!(self.after_effect);
        self.after_effect_timer += delta_time;
        if self.after_effect_timer >= self.after_effect_duration {
            self.after_effect_timer = 0.0;
            self.after_effect = false;
            self.state = WeaponState::EndAttack;
        }
    }

    pub fn attack(&mut self) {
        self.state = WeaponState::StartAttack;
        self.attack_timer = 0.0;
        self.attacking = true;
        self.can_attack = false;
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
