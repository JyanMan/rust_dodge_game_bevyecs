use super::*;

#[derive(Clone, Default, PartialEq)]
pub enum WeaponState {
    #[default]
    Unowned,
    Owned,
    Idle,
    Attacking,
}

#[derive(Clone)]
pub struct WeaponData {
    pub w_type: WeaponType,
    pub state: WeaponState,
    pub damage: i32,
    pub knock_force: f32,
    pub attacking: bool,
    pub can_attack: bool,
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
            attacking: false,
            can_attack: true,
        }
    }
}

impl WeaponData {
    pub fn new(damage: i32, knock_force: f32, attack_duration: f32, state: WeaponState, w_type: WeaponType) -> Self {
        Self {
            w_type: w_type,
            state: state,
            damage: damage,
            knock_force: knock_force,
            attack_duration: attack_duration,
            attack_timer: 0.0,
            attacking: false,
            can_attack: true,
        }
    }

    pub fn attack_timer(&mut self, delta_time: f32) {
        assert!(self.attacking);
        self.attack_timer += delta_time;
        if self.attack_timer >= self.attack_duration {
            self.attack_timer = 0.0;
            self.attacking = false;
            // self.can_attack = false;
        }
    }

    pub fn attack(&mut self) {
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
    Idle,
    Attack,
}

impl WeaponAnim {
    pub const COUNT: usize = 4;
    pub fn usize(self) -> usize {
        self as usize
    }
}
