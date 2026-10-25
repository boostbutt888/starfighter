//! Starfighter's shared Rust simulation. Native and WebAssembly use the same rules.
use macroquad::prelude::{vec2, Vec2};
use std::f32::consts::{PI, TAU};
pub const W: f32 = 600.;
pub const H: f32 = 840.;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Craft {
    Xwing,
    Ywing,
    Awing,
}
impl Craft {
    pub const ALL: [Self; 3] = [Self::Xwing, Self::Ywing, Self::Awing];
    pub fn name(self) -> &'static str {
        match self {
            Self::Xwing => "X-WING",
            Self::Ywing => "Y-WING",
            Self::Awing => "A-WING",
        }
    }
    pub fn speed(self) -> f32 {
        match self {
            Self::Xwing => 290.,
            Self::Ywing => 215.,
            Self::Awing => 390.,
        }
    }
    pub fn force(self) -> f32 {
        150.
    }
    pub fn bombs(self) -> u32 {
        match self {
            Self::Xwing => 7,
            Self::Ywing => 10,
            Self::Awing => 5,
        }
    }
    pub fn ports(self) -> &'static [f32] {
        match self {
            Self::Xwing => &[-18.75, -6.75, 6.75, 18.75],
            Self::Ywing => &[-3., 3.],
            Self::Awing => &[-12.75, 12.75],
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Rookie,
    Ace,
    Legend,
}
impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Rookie, Self::Ace, Self::Legend];
    pub fn name(self) -> &'static str {
        match self {
            Self::Rookie => "ROOKIE",
            Self::Ace => "PILOT",
            Self::Legend => "LEGEND",
        }
    }
    pub fn armor(self) -> f32 {
        match self {
            Self::Rookie => 2.,
            Self::Ace => 5.,
            Self::Legend => 10.,
        }
    }
    pub fn speed(self) -> f32 {
        match self {
            Self::Rookie => 0.75,
            Self::Ace => 1.,
            Self::Legend => 1.26,
        }
    }
    pub fn enemy_armor(self) -> f32 {
        match self {
            Self::Rookie => 1.,
            Self::Ace => 2.,
            Self::Legend => 3.,
        }
    }
    pub fn interval(self) -> f32 {
        match self {
            Self::Rookie => 1.35,
            Self::Ace => 1.,
            Self::Legend => 0.76,
        }
    }
    pub fn reserves(self) -> u32 {
        match self {
            Self::Rookie => 3,
            Self::Ace => 2,
            Self::Legend => 1,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hull {
    Barge,
    Destroyer,
    Executor,
    DeathStar,
    DeathStar2,
}
impl Hull {
    pub fn name(self) -> &'static str {
        match self {
            Self::Barge => "JABBA'S SAIL BARGE",
            Self::Destroyer => "STAR DESTROYER",
            Self::Executor => "EXECUTOR",
            Self::DeathStar => "DEATH STAR I",
            Self::DeathStar2 => "DEATH STAR II",
        }
    }
}
pub const MISSIONS: [&str; 5] = [
    "Encounter at Tatooine",
    "Siege of Hoth",
    "Shadows over Dagobah",
    "Assault on Coruscant",
    "Battle above Endor",
];
pub const PLACES: [&str; 5] = ["TATOOINE", "HOTH", "DAGOBAH", "CORUSCANT", "ENDOR ORBIT"];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Waves,
    Boss,
    FalseVictory,
    Ambush,
    Clear,
    Won,
    Lost,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickupKind {
    Force,
    Bomb,
    Spread,
    Cannon,
}
#[derive(Clone, Copy, Debug)]
pub struct Pickup {
    pub p: Vec2,
    pub kind: PickupKind,
    pub age: f32,
    pub dead: bool,
}
#[derive(Clone, Debug)]
pub struct Gun {
    pub offset: Vec2,
    pub hp: f32,
    pub max: f32,
    pub angle: f32,
    pub timer: f32,
    pub flash: f32,
    pub recoil: f32,
}
#[derive(Clone, Debug)]
pub struct BossShip {
    pub kind: Hull,
    pub p: Vec2,
    pub base: f32,
    pub size: Vec2,
    pub age: f32,
    pub seed: f32,
    pub guns: Vec<Gun>,
    pub bridge: Vec2,
    pub hp: f32,
    pub max: f32,
    pub bridge_timer: f32,
    pub dead: bool,
}
impl BossShip {
    pub fn arrival_y(&self) -> f32 {
        60. + self.size.y / 2.
    }
    pub fn in_position(&self) -> bool {
        self.p.y >= self.arrival_y() - 0.01
    }
    pub fn mass(&self) -> f32 {
        self.max + self.guns.iter().map(|g| g.max).sum::<f32>()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Shot {
    pub id: u64,
    pub p: Vec2,
    pub old: Vec2,
    pub v: Vec2,
    pub radius: f32,
    pub damage: f32,
    pub enemy: bool,
    pub green: bool,
    pub dead: bool,
    pub pierce: u8,
    pub hits: [u64; 8],
    pub hit_count: usize,
}
#[derive(Clone, Debug)]
pub struct Enemy {
    pub id: u64,
    pub kind: usize,
    pub p: Vec2,
    pub base: f32,
    pub age: f32,
    pub hp: f32,
    pub timer: f32,
    pub form: usize,
    pub seed: f32,
    pub angle: f32,
    pub dead: bool,
    pub path: FlightPath,
}
/// The dive / side sweep / turning hook paths match the earlier 194X simulation.
#[derive(Clone, Copy, Debug)]
pub enum FlightPath {
    Dive { x: f32, phase: f32 },
    Sweep { side: f32, lane: f32, phase: f32 },
    Hook { side: f32, lane: f32 },
}
impl FlightPath {
    pub fn position(self, age: f32, speed: f32) -> Vec2 {
        match self {
            Self::Dive { x, phase } => vec2(
                x + (age * 1.6 + phase).sin() * 45.,
                -70. + age * 145. * speed,
            ),
            Self::Sweep { side, lane, phase } => vec2(
                if side > 0. {
                    -70. + age * 175. * speed
                } else {
                    W + 70. - age * 175. * speed
                },
                lane + (age * 2. + phase).sin() * 70. + age * 24. * speed,
            ),
            Self::Hook { side, lane } => {
                let a = age * 0.8 * speed;
                vec2(
                    if side > 0. {
                        -55. + a.sin() * 430.
                    } else {
                        W + 55. - a.sin() * 430.
                    },
                    lane + age * 55. * speed,
                )
            }
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Blast {
    pub p: Vec2,
    pub size: f32,
    pub age: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct BombRun {
    pub origin: Vec2,
    pub age: f32,
}
impl BombRun {
    pub fn target(self) -> Vec2 {
        vec2(self.origin.x, (self.origin.y - 220.).max(115.))
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Laser(bool, bool),
    Explosion(f32),
    Pickup,
    Victory,
}
#[derive(Default)]
pub struct Input {
    pub movement: Vec2,
    pub drag: Vec2,
    pub fire: bool,
    pub bomb: bool,
    pub swap: bool,
    pub slow: bool,
}
pub struct Game {
    pub height: f32,
    pub craft: Craft,
    pub difficulty: Difficulty,
    pub stage: usize,
    pub started_stage: usize,
    pub phase: Phase,
    pub age: f32,
    pub time: f32,
    pub total: f32,
    pub player: Vec2,
    pub force: f32,
    pub bombs: u32,
    pub reserves: u32,
    pub score: u32,
    pub kills: u32,
    pub inv: f32,
    pub weapon: usize,
    pub levels: [u8; 2],
    pub fleet: Vec<BossShip>,
    pub enemies: Vec<Enemy>,
    pub shots: Vec<Shot>,
    pub pickups: Vec<Pickup>,
    pub blasts: Vec<Blast>,
    pub bomb_runs: Vec<BombRun>,
    pub events: Vec<Event>,
    pub notice: String,
    pub notice_age: f32,
    pub flash: f32,
    pub shake: f32,
    pub surprise: bool,
    fire: f32,
    spawn: f32,
    wave: usize,
    reinforce: f32,
    seed: u32,
    next_id: u64,
}
fn angle_delta(a: f32, b: f32) -> f32 {
    (a - b).sin().atan2((a - b).cos())
}
pub fn swept(a: Vec2, b: Vec2, p: Vec2, r: f32) -> bool {
    let d = b - a;
    let t = if d.length_squared() > 0. {
        ((p - a).dot(d) / d.length_squared()).clamp(0., 1.)
    } else {
        0.
    };
    (a + d * t).distance(p) < r
}
impl Game {
    pub fn new(craft: Craft, difficulty: Difficulty, stage: usize) -> Self {
        let stage = stage.min(4);
        Self {
            height: H,
            craft,
            difficulty,
            stage,
            started_stage: stage,
            phase: Phase::Waves,
            age: 0.,
            time: 0.,
            total: 0.,
            player: vec2(W / 2., H - 100.),
            force: craft.force(),
            bombs: craft.bombs(),
            reserves: difficulty.reserves(),
            score: 0,
            kills: 0,
            inv: 3.,
            weapon: 0,
            levels: [if stage > 1 { 2 } else { 1 }, 1],
            fleet: vec![],
            enemies: vec![],
            shots: vec![],
            pickups: vec![],
            blasts: vec![],
            bomb_runs: vec![],
            events: vec![],
            notice: MISSIONS[stage].to_uppercase(),
            notice_age: 4.,
            flash: 0.,
            shake: 0.,
            surprise: false,
            fire: 0.,
            spawn: 1.,
            wave: 0,
            reinforce: 7.,
            seed: 7419,
            next_id: 1,
        }
    }
    fn random(&mut self, a: f32, b: f32) -> f32 {
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        a + (b - a) * (self.seed >> 8) as f32 / 16777216.
    }
    fn id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    pub fn message(&mut self, t: impl Into<String>, duration: f32) {
        self.notice = t.into();
        self.notice_age = duration
    }
    pub fn resize_height(&mut self, height: f32) {
        let height = height.clamp(H, 1500.);
        self.player.y += height - self.height;
        self.height = height;
        self.player.y = self.player.y.clamp(100., height - 35.);
    }
    pub fn boss_arrival_time(&self) -> f32 {
        65. + self.stage as f32 * 3.
    }
    pub fn endor_approach(&self) -> f32 {
        (self.time / self.boss_arrival_time()).clamp(0., 1.)
    }
    pub fn explode(&mut self, p: Vec2, size: f32) {
        self.blasts.push(Blast { p, size, age: 0. });
        self.events.push(Event::Explosion(size));
        self.shake = self.shake.max(size * 3.)
    }
    fn shot(
        &mut self,
        p: Vec2,
        v: Vec2,
        r: f32,
        damage: f32,
        enemy: bool,
        green: bool,
        pierce: u8,
    ) {
        if enemy
            && self.shots.iter().filter(|s| s.enemy && !s.dead).count()
                >= if self.difficulty == Difficulty::Rookie {
                    120
                } else {
                    200
                }
        {
            return;
        }
        let id = self.id();
        self.shots.push(Shot {
            id,
            p,
            old: p,
            v,
            radius: r,
            damage,
            enemy,
            green,
            dead: false,
            pierce,
            hits: [0; 8],
            hit_count: 0,
        })
    }
    pub fn fire_player(&mut self) {
        let level = self.levels[self.weapon] as f32;
        self.fire = match self.craft {
            Craft::Xwing => 0.17,
            Craft::Ywing => 0.19,
            Craft::Awing => 0.13,
        } * if self.weapon == 1 {
            1.0 / (1. + (level - 1.).clamp(0., 3.) * 0.18)
        } else {
            1.
        };
        let ports = self.craft.ports();
        for port in ports {
            let p = self.player + vec2(*port, -25.);
            if self.weapon == 1 {
                self.shot(
                    p,
                    vec2(0., -650.),
                    3. + level * 0.7,
                    3. + level * 1.5,
                    false,
                    false,
                    (1 + (level as u8 - 1) / 2).min(4),
                )
            } else {
                let count = if level >= 2. {
                    1 + (level as usize - 1) * 2
                } else {
                    1
                };
                for n in 0..count {
                    let a = (n as f32 - (count - 1) as f32 / 2.) * 0.0455;
                    self.shot(
                        p,
                        vec2(a.sin() * 610., -a.cos() * 610.),
                        2.,
                        if count > 1 { 0.7 } else { 1. },
                        false,
                        true,
                        1,
                    )
                }
            }
        }
        self.events.push(Event::Laser(false, self.weapon == 1))
    }
    pub fn drop(&mut self, p: Vec2, kind: PickupKind) {
        self.pickups.push(Pickup {
            p,
            kind,
            age: 0.,
            dead: false,
        })
    }
    fn kill_enemy(&mut self, index: usize) {
        if self.enemies[index].dead {
            return;
        }
        self.enemies[index].dead = true;
        let p = self.enemies[index].p;
        self.score += if self.enemies[index].kind == 2 {
            250
        } else {
            120
        };
        self.kills += 1;
        self.explode(p, 0.6);
        if self.kills % 20 == 0 {
            self.drop(p, PickupKind::Force)
        } else if self.kills % 5 == 0 {
            let kind = if self.kills % 10 == 0 {
                PickupKind::Cannon
            } else {
                PickupKind::Spread
            };
            self.drop(p, kind)
        } else if self.kills % 13 == 0 {
            self.drop(p, PickupKind::Bomb)
        }
    }
    pub fn hurt(&mut self, damage: f32) {
        if self.inv > 0.
            || matches!(
                self.phase,
                Phase::Won | Phase::Lost | Phase::Clear | Phase::FalseVictory
            )
        {
            return;
        }
        self.force -= damage;
        self.inv = 1.1;
        self.explode(self.player, 0.5);
        if self.force <= 0. {
            self.explode(self.player, 1.5);
            self.shake = 0.;
            if self.reserves > 0 {
                self.reserves -= 1;
                self.force = self.craft.force();
                self.player = vec2(W / 2., self.height - 100.);
                self.inv = 3.;
                self.bombs = self.bombs.max(2);
                self.levels[self.weapon] = self.levels[self.weapon].saturating_sub(1).max(1);
                self.message("WINGMAN TAKING OVER / THE FORCE RESTORED", 3.)
            } else {
                self.phase = Phase::Lost;
                self.age = 0.;
                self.force = 0.
            }
        }
    }
    pub fn spawn_wave(&mut self) {
        let strength = (self.time / 16.) as usize + self.stage / 2;
        let n = 3 + strength.min(3);
        let form = self.wave % 5;
        let side = if self.wave % 2 == 0 { 1. } else { -1. };
        let lane = 150. + (self.wave % 3) as f32 * 95.;
        for i in 0..n {
            let x = 90. + i as f32 * (W - 180.) / (n - 1) as f32;
            let path = match form {
                1 | 3 => FlightPath::Sweep {
                    side,
                    lane,
                    phase: 0.,
                },
                2 => FlightPath::Hook { side, lane },
                _ => FlightPath::Dive {
                    x,
                    phase: self.wave as f32,
                },
            };
            let roster: &[usize] = if self.stage == 0 {
                &[0, 1, 2]
            } else {
                &[0, 2, 1, 3, 2]
            };
            let kind = roster[(self.wave + i / 3) % roster.len()];
            let age = -(i as f32) * 0.25;
            let id = self.id();
            let timer = 1. + i as f32 * 0.18;
            self.enemies.push(Enemy {
                id,
                kind,
                p: path.position(
                    age,
                    self.difficulty.speed() * (1. + self.stage as f32 * 0.07),
                ),
                base: x,
                age,
                hp: ([3., 3., 12., 6.][kind])
                    * (1. + self.stage as f32 * 0.12)
                    * self.difficulty.enemy_armor(),
                timer,
                form,
                seed: self.wave as f32,
                angle: PI,
                dead: false,
                path,
            })
        }
        self.wave += 1;
        self.spawn = (2.5 - strength as f32 * 0.18).max(1.35) * self.difficulty.interval()
            / (1. + self.stage as f32 * 0.12);
    }
    pub fn start_boss(&mut self, ambush: bool) {
        let kinds: Vec<Hull> = if ambush {
            vec![Hull::Destroyer, Hull::Executor, Hull::Destroyer]
        } else {
            match self.stage {
                0 => vec![Hull::Barge],
                1 => vec![Hull::Executor],
                2 => vec![Hull::Destroyer, Hull::Destroyer],
                3 => vec![Hull::Destroyer, Hull::DeathStar, Hull::Destroyer],
                _ => vec![Hull::DeathStar2],
            }
        };
        self.fleet.clear();
        self.enemies.clear();
        self.shots.retain(|p| !p.enemy);
        self.phase = if ambush { Phase::Ambush } else { Phase::Boss };
        self.age = 0.;
        let multi = kinds.len() > 1;
        for (i, kind) in kinds.iter().copied().enumerate() {
            let size = match kind {
                Hull::Barge => vec2(310., 520.),
                Hull::Executor => {
                    if multi {
                        vec2(190., 580.)
                    } else {
                        vec2(360., 600.)
                    }
                }
                Hull::Destroyer => {
                    if kinds.len() == 2 {
                        vec2(225., 470.)
                    } else {
                        vec2(170., 440.)
                    }
                }
                _ => {
                    if multi {
                        vec2(260., 260.)
                    } else {
                        vec2(460., 460.)
                    }
                }
            };
            let base = if kinds.len() == 1 {
                300.
            } else if kinds.len() == 2 {
                170. + i as f32 * 260.
            } else {
                90. + i as f32 * 210.
            };
            let count = match kind {
                Hull::Barge => 10,
                Hull::Executor => 16,
                Hull::DeathStar2 => 24,
                Hull::DeathStar => 16,
                _ => 12,
            };
            let station = if kind == Hull::DeathStar2 { 2. } else { 1. };
            let ghp = 3.
                * (10. + self.stage as f32 * 3. + if ambush { 7. } else { 0. })
                * self.difficulty.armor()
                * station;
            let bhp = (24. + self.stage as f32 * 10. + if ambush { 25. } else { 0. })
                * self.difficulty.armor()
                * station;
            let mut guns = vec![];
            for j in 0..count {
                let side = if j % 2 == 0 { -1. } else { 1. };
                let row = j / 2;
                let offset = if matches!(kind, Hull::DeathStar | Hull::DeathStar2) {
                    let a = j as f32 / count as f32 * TAU;
                    vec2(a.cos() * size.x * 0.32, a.sin() * size.y * 0.32)
                } else if kind == Hull::Barge {
                    vec2(
                        side * size.x * 0.16,
                        (-0.28 + row as f32 / (count / 2 - 1) as f32 * 0.56) * size.y,
                    )
                } else {
                    vec2(
                        side * size.x * (0.10 + row as f32 * 0.022),
                        (-0.03 + row as f32 / ((count / 2 - 1) as f32) * 0.30) * size.y,
                    )
                };
                guns.push(Gun {
                    offset: -offset,
                    hp: ghp,
                    max: ghp,
                    angle: PI,
                    timer: 1.5 + j as f32 * 0.3,
                    flash: 0.,
                    recoil: 0.,
                })
            }
            let bridge = vec2(
                0.,
                match kind {
                    Hull::Barge => size.y * 0.255,
                    Hull::DeathStar | Hull::DeathStar2 => 0.,
                    _ => size.y * 0.3,
                },
            );
            self.fleet.push(BossShip {
                kind,
                p: vec2(base, -size.y),
                base,
                size,
                age: 0.,
                seed: i as f32 * 1.7,
                guns,
                bridge: -bridge,
                hp: bhp,
                max: bhp,
                bridge_timer: 2.,
                dead: false,
            })
        }
        self.message(
            if ambush {
                "WARNING / IMPERIAL FLEET INBOUND"
            } else {
                "WARNING / BOSS INBOUND"
            },
            2.5,
        )
    }
    pub fn guns_left(&self) -> usize {
        self.fleet
            .iter()
            .filter(|b| !b.dead)
            .map(|b| b.guns.iter().filter(|g| g.hp > 0.).count())
            .sum()
    }
    pub fn fleet_ready(&self) -> bool {
        !self.fleet.is_empty() && self.fleet.iter().all(|b| b.dead || b.in_position())
    }
    pub fn gun_hit(&mut self, bi: usize, gi: usize, damage: f32) {
        if !self.fleet_ready() {
            return;
        }
        let b = &mut self.fleet[bi];
        let g = &mut b.guns[gi];
        if g.hp <= 0. {
            return;
        }
        g.hp = (g.hp - damage).max(0.);
        g.flash = 0.08;
        if g.hp == 0. {
            let p = b.p + g.offset;
            self.score += 300;
            self.explode(p, 0.9);
            if self.guns_left() == 0 {
                self.message(
                    if self.stage == 0 {
                        "JABBA EXPOSED / DESTROY THE COMMAND DECK"
                    } else {
                        "ALL GUNS DOWN / COMMAND BRIDGES EXPOSED"
                    },
                    4.,
                );
                self.events.push(Event::Pickup)
            }
        }
    }
    pub fn bridge_hit(&mut self, bi: usize, damage: f32) {
        if !self.fleet_ready() || self.guns_left() > 0 || self.fleet[bi].dead {
            return;
        }
        let b = &mut self.fleet[bi];
        b.hp = (b.hp - damage).max(0.);
        if b.hp == 0. {
            b.dead = true;
            let p = b.p;
            self.score += 2500 * (self.stage as u32 + 1);
            self.explode(p, 3.);
            for i in 0..4 {
                let dx = self.random(-50., 50.);
                let dy = self.random(-75., 75.);
                self.blasts.push(Blast {
                    p: p + vec2(dx, dy),
                    size: 1.5,
                    age: -i as f32 * 0.2,
                })
            }
            if self.fleet.iter().all(|b| b.dead) {
                self.fleet_defeated()
            }
        }
    }
    fn fleet_defeated(&mut self) {
        // Keep indices stable until the current swept-collision batch completes.
        for shot in &mut self.shots {
            if shot.enemy {
                shot.dead = true;
            }
        }
        for enemy in &mut self.enemies {
            enemy.dead = true;
        }
        self.age = 0.;
        if self.stage == 4 && !self.surprise {
            self.surprise = true;
            self.phase = Phase::FalseVictory;
            self.message("DEATH STAR II DESTROYED / EXTRACTION INBOUND", 5.)
        } else {
            self.phase = Phase::Clear;
            self.score += 1500 * (self.stage as u32 + 1);
            self.force = (self.force + 35.).min(self.craft.force());
            self.bombs = (self.bombs + 1).min(self.craft.bombs());
            if self.stage == 1 || self.stage == 3 {
                self.reserves += 1
            }
            self.message("SECTOR CLEAR / FORCE RESTORED / +1 BOMB", 4.);
            self.events.push(Event::Victory)
        }
    }
    pub fn bomb(&mut self) {
        if self.bombs == 0
            || matches!(
                self.phase,
                Phase::Clear | Phase::FalseVictory | Phase::Won | Phase::Lost
            )
        {
            return;
        }
        if self.bomb_runs.iter().any(|b| b.age < 0.65) {
            return;
        }
        self.bombs -= 1;
        self.bomb_runs.push(BombRun {
            origin: self.player,
            age: 0.,
        });
    }
    fn detonate(&mut self, target: Vec2) {
        self.flash = 0.7;
        self.explode(target, 2.8);
        self.shots.retain(|p| !p.enemy);
        for i in 0..self.enemies.len() {
            self.kill_enemy(i)
        }
        let shield = self.guns_left() > 0;
        for bi in 0..self.fleet.len() {
            if self.fleet[bi].dead {
                continue;
            }
            if shield {
                for gi in 0..self.fleet[bi].guns.len() {
                    self.gun_hit(bi, gi, 15.)
                }
            } else {
                self.bridge_hit(bi, 12.)
            }
        }
    }
    fn fleet_update(&mut self, dt: f32) {
        let shield = self.guns_left() > 0;
        let speed = self.difficulty.speed();
        let interval = self.difficulty.interval();
        let mut queued = vec![];
        for b in &mut self.fleet {
            b.age += dt;
            if b.dead {
                continue;
            }
            let target_y = b.arrival_y();
            b.p.y = (b.p.y + 150. * dt).min(target_y);
            b.p.x = b.base + (b.age * 0.6 + b.seed).sin() * if self.stage < 2 { 45. } else { 12. };
            for g in &mut b.guns {
                g.flash = (g.flash - dt).max(0.);
                g.recoil = (g.recoil - dt * 16.).max(0.);
                if g.hp <= 0. {
                    continue;
                }
                let p = b.p + g.offset;
                let target = (self.player - p).to_angle() + PI / 2.;
                g.angle += angle_delta(target, g.angle).clamp(-1.7 * dt, 1.7 * dt);
                g.timer -= dt;
                if b.p.y >= target_y && g.timer <= 0. {
                    g.timer = (2.7 - self.stage as f32 * 0.22)
                        * interval
                        * if self.phase == Phase::Ambush {
                            0.75
                        } else {
                            1.
                        };
                    g.flash = 0.14;
                    g.recoil = 3.;
                    let a = g.angle - PI / 2.;
                    for n in if self.stage >= 2 { -1..=1 } else { 0..=0 } {
                        let a = a + n as f32 * 0.12;
                        queued.push((
                            p,
                            vec2(a.cos(), a.sin()) * (165. + self.stage as f32 * 16.) * speed,
                            3.5,
                        ))
                    }
                    self.events.push(Event::Laser(true, true))
                }
            }
            if !shield {
                b.bridge_timer -= dt;
                if b.bridge_timer < 0. && b.p.y >= target_y {
                    b.bridge_timer = 1.9 * interval;
                    let p = b.p + b.bridge;
                    let a = (self.player - p).to_angle();
                    for n in -3..=3 {
                        let a = a + n as f32 * 0.12;
                        queued.push((p, vec2(a.cos(), a.sin()) * 190. * speed, 4.))
                    }
                    let burst_count = if b.kind == Hull::Barge { 16 } else { 12 };
                    for n in 0..burst_count {
                        let a = n as f32 / burst_count as f32 * TAU + b.age * 0.2;
                        queued.push((
                            p,
                            vec2(a.cos(), a.sin()) * (145. + self.stage as f32 * 12.) * speed,
                            3.5,
                        ));
                    }
                }
            }
        }
        for (p, v, r) in queued {
            self.shot(p, v, r, 14., true, false, 1)
        }
        self.reinforce -= dt;
        if self.reinforce <= 0. && self.stage > 0 {
            self.reinforce = 12. - self.stage as f32;
            self.spawn_wave()
        }
    }
    fn remember_hit(shot: &mut Shot, target: u64) {
        if shot.hit_count < shot.hits.len() {
            shot.hits[shot.hit_count] = target;
            shot.hit_count += 1
        }
        shot.pierce = shot.pierce.saturating_sub(1);
        if shot.pierce == 0 {
            shot.dead = true
        }
    }
    pub fn update(&mut self, dt: f32, input: Input) {
        self.events.clear();
        if matches!(self.phase, Phase::Won | Phase::Lost) {
            return;
        }
        self.total += dt;
        self.time += dt;
        self.age += dt;
        self.inv -= dt;
        self.fire -= dt;
        self.flash = (self.flash - dt * 1.4).max(0.);
        self.shake = (self.shake - dt * 15.).max(0.);
        self.notice_age = (self.notice_age - dt).max(0.);
        let mut detonations = vec![];
        for run in &mut self.bomb_runs {
            let old = run.age;
            run.age += dt;
            if old < 0.65 && run.age >= 0.65 {
                detonations.push(run.target())
            }
        }
        self.bomb_runs.retain(|b| b.age < 2.4);
        for target in detonations {
            self.detonate(target)
        }
        let movement = input.movement.normalize_or_zero()
            * self.craft.speed()
            * if input.slow { 0.45 } else { 1. };
        self.player += movement * dt + input.drag;
        self.player.x = self.player.x.clamp(28., W - 28.);
        self.player.y = self.player.y.clamp(100., self.height - 35.);
        if input.swap {
            self.weapon = 1 - self.weapon
        }
        if input.bomb {
            self.bomb()
        }
        if input.fire
            && self.fire <= 0.
            && !matches!(self.phase, Phase::Clear | Phase::FalseVictory)
        {
            self.fire_player()
        }
        if self.phase == Phase::Waves {
            self.spawn -= dt;
            if self.spawn <= 0. {
                self.spawn_wave();
            }
            if self.time >= self.boss_arrival_time() {
                self.start_boss(false)
            }
        }
        if matches!(self.phase, Phase::Boss | Phase::Ambush) {
            self.fleet_update(dt)
        }
        if self.phase == Phase::FalseVictory && self.age > 5. {
            self.start_boss(true);
            self.force = (self.force + 40.).min(self.craft.force());
            self.bombs = (self.bombs + 1).min(self.craft.bombs());
            self.inv = 4.
        }
        if self.phase == Phase::Clear && self.age > 4.5 {
            if self.stage == 4 {
                self.phase = Phase::Won;
                self.age = 0.;
                return;
            }
            self.stage += 1;
            self.phase = Phase::Waves;
            self.age = 0.;
            self.time = 0.;
            self.spawn = 1.;
            self.fleet.clear();
            self.enemies.clear();
            self.shots.clear();
            self.inv = 3.;
            self.message(MISSIONS[self.stage].to_uppercase(), 4.)
        }
        let mut queued = vec![];
        let mut collisions = vec![];
        for (i, e) in self.enemies.iter_mut().enumerate() {
            e.age += dt;
            let old = e.p;
            let speed = self.difficulty.speed()
                * (1. + self.stage as f32 * 0.07)
                * if e.kind == 2 { 0.72 } else { 1. };
            e.p = e.path.position(e.age, speed);
            if (e.p - old).length_squared() > 0.001 {
                e.angle = (e.p - old).to_angle() + PI / 2.;
            }
            if e.age < 0. {
                continue;
            }
            e.timer -= dt;
            if e.p.x > 12.
                && e.p.x < W - 12.
                && e.p.y > 85.
                && e.p.y < self.height - 30.
                && e.timer <= 0.
            {
                e.timer = 2.2 * self.difficulty.interval() / (1. + self.stage as f32 * 0.09);
                let aim = (self.player - e.p).normalize_or_zero() * 200. * speed;
                queued.push((e.p, aim));
                if e.kind == 2 {
                    let count = match self.difficulty {
                        Difficulty::Rookie => 8,
                        Difficulty::Ace => 12,
                        Difficulty::Legend => 16,
                    };
                    for n in 0..count {
                        let a = n as f32 / count as f32 * TAU + e.age * 0.25;
                        queued.push((e.p, vec2(a.cos(), a.sin()) * 130. * self.difficulty.speed()));
                    }
                } else if (e.kind == 1 || e.kind == 3) && self.difficulty != Difficulty::Rookie {
                    for side in [-1., 1.] {
                        queued.push((e.p, aim + vec2(side * 55., 0.)));
                    }
                }
            }
            if e.p.distance(self.player) < 22. {
                collisions.push(i)
            }
        }
        for (p, v) in queued {
            self.shot(p, v, 3., 14., true, false, 1);
            self.events.push(Event::Laser(true, false))
        }
        for i in collisions {
            self.hurt(25.);
            self.kill_enemy(i)
        }
        for p in &mut self.shots {
            p.old = p.p;
            p.p += p.v * dt
        }
        let mut hits = vec![];
        for pi in 0..self.shots.len() {
            let p = &self.shots[pi];
            if p.dead {
                continue;
            }
            if p.enemy {
                if swept(p.old, p.p, self.player, 7. + p.radius) {
                    hits.push((pi, 0, 0, 0))
                }
                continue;
            }
            for (ei, e) in self.enemies.iter().enumerate() {
                if !e.dead && !p.hits.contains(&e.id) && swept(p.old, p.p, e.p, 14. + p.radius) {
                    hits.push((pi, 1, ei, 0))
                }
            }
            for (bi, b) in self.fleet.iter().enumerate() {
                if b.dead {
                    continue;
                }
                for (gi, g) in b.guns.iter().enumerate() {
                    let id = 1_000_000 + bi as u64 * 100 + gi as u64;
                    if g.hp > 0.
                        && !p.hits.contains(&id)
                        && swept(p.old, p.p, b.p + g.offset, 10.5 + p.radius)
                    {
                        hits.push((pi, 2, bi, gi))
                    }
                }
                let id = 2_000_000 + bi as u64;
                if !p.hits.contains(&id) && swept(p.old, p.p, b.p + b.bridge, 20. + p.radius) {
                    hits.push((pi, 3, bi, 0))
                }
            }
        }
        for (pi, kind, a, b) in hits {
            if self.shots[pi].dead {
                continue;
            }
            let damage = self.shots[pi].damage;
            match kind {
                0 => {
                    self.shots[pi].dead = true;
                    self.hurt(14.)
                }
                1 => {
                    if self.enemies[a].dead {
                        continue;
                    }
                    let id = self.enemies[a].id;
                    Self::remember_hit(&mut self.shots[pi], id);
                    self.enemies[a].hp -= damage;
                    if self.enemies[a].hp <= 0. {
                        self.kill_enemy(a)
                    }
                }
                2 => {
                    if self.fleet[a].guns[b].hp <= 0. {
                        continue;
                    }
                    Self::remember_hit(&mut self.shots[pi], 1_000_000 + a as u64 * 100 + b as u64);
                    self.gun_hit(a, b, damage)
                }
                _ => {
                    Self::remember_hit(&mut self.shots[pi], 2_000_000 + a as u64);
                    self.bridge_hit(a, damage)
                }
            }
        }
        let mut collected = vec![];
        for p in &mut self.pickups {
            p.age += dt;
            p.p.y += 55. * dt;
            if p.p.distance(self.player) < 31. {
                let full = match p.kind {
                    PickupKind::Force => self.force >= 150.,
                    PickupKind::Bomb => self.bombs >= self.craft.bombs(),
                    PickupKind::Spread | PickupKind::Cannon => false,
                };
                if !full {
                    p.dead = true;
                    collected.push(p.kind)
                }
            }
        }
        for kind in collected {
            match kind {
                PickupKind::Force => {
                    self.force = (self.force + 35.).min(150.);
                    self.message("THE FORCE IS WITH YOU / +35 FORCE", 2.)
                }
                PickupKind::Bomb => {
                    self.bombs = (self.bombs + 1).min(self.craft.bombs());
                    self.message("PROTON PAYLOAD +1", 2.)
                }
                PickupKind::Spread | PickupKind::Cannon => {
                    self.weapon = if kind == PickupKind::Spread { 0 } else { 1 };
                    let cap = if kind == PickupKind::Cannon { 4 } else { 6 };
                    self.levels[self.weapon] = (self.levels[self.weapon] + 1).min(cap);
                    self.message(
                        format!(
                            "{} UPGRADED / LEVEL {}",
                            if self.weapon == 0 { "SPREAD" } else { "CANNON" },
                            self.levels[self.weapon]
                        ),
                        2.,
                    )
                }
            }
            self.events.push(Event::Pickup)
        }
        let hull_collision = self.fleet.iter().any(|b| {
            !b.dead
                && (self.player.x - b.p.x).abs() < b.size.x * 0.32
                && (self.player.y - b.p.y).abs() < b.size.y * 0.43
        });
        if hull_collision {
            self.hurt(30.)
        }
        for b in &mut self.blasts {
            b.age += dt
        }
        self.blasts.retain(|b| b.age < 1.7);
        self.enemies.retain(|e| {
            !e.dead
                && e.p.y < self.height + 100.
                && (e.age < 1. || (e.p.x > -130. && e.p.x < W + 130.))
        });
        self.shots.retain(|p| {
            !p.dead
                && p.p.y > -150.
                && p.p.y < self.height + 100.
                && p.p.x > -80.
                && p.p.x < W + 80.
        });
        self.pickups
            .retain(|p| !p.dead && p.p.y < self.height + 50.);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn game() -> Game {
        Game::new(Craft::Xwing, Difficulty::Ace, 0)
    }
    #[test]
    fn force_pickup_is_blue_health_replacement() {
        let mut g = game();
        g.force = 30.;
        g.drop(g.player, PickupKind::Force);
        g.update(0.01, Input::default());
        assert_eq!(g.force, 65.);
        assert_eq!(g.bombs, 7)
    }
    #[test]
    fn craft_loadouts_and_six_weapon_levels() {
        for craft in Craft::ALL {
            let mut g = Game::new(craft, Difficulty::Ace, 0);
            g.fire_player();
            assert_eq!(g.shots.len(), craft.ports().len());
            for _ in 0..9 {
                g.drop(g.player, PickupKind::Cannon);
                g.update(0.01, Input::default())
            }
            assert_eq!(g.levels[1], 4);
            assert_eq!(g.weapon, 1)
        }
        assert!(Craft::Awing.speed() > Craft::Xwing.speed());
        assert!(Craft::Ywing.bombs() > Craft::Xwing.bombs())
    }
    #[test]
    fn bosses_increase_armor_and_surprise_is_stronger() {
        let mut g = game();
        let mut old = 0.;
        for stage in 0..5 {
            g.stage = stage;
            g.start_boss(false);
            let mass = g.fleet.iter().map(BossShip::mass).sum::<f32>();
            assert!(mass > old, "stage {stage}: {mass} <= {old}");
            old = mass
        }
        g.start_boss(true);
        assert!(g.fleet.iter().map(BossShip::mass).sum::<f32>() > old)
    }
    #[test]
    fn shields_bombs_and_destroyed_tracking_guns() {
        let mut g = game();
        g.start_boss(false);
        assert_eq!(g.guns_left(), 10);
        let hp = g.fleet[0].hp;
        g.bridge_hit(0, 9999.);
        assert_eq!(g.fleet[0].hp, hp);
        g.fleet[0].p.y = g.fleet[0].arrival_y();
        let angle = g.fleet[0].guns[0].angle;
        g.player.x = 550.;
        g.fleet_update(0.2);
        assert_ne!(angle, g.fleet[0].guns[0].angle);
        for gun in &mut g.fleet[0].guns {
            gun.hp = 1.
        }
        g.bomb();
        assert_eq!(g.guns_left(), 10);
        g.update(0.66, Input::default());
        assert_eq!(g.guns_left(), 0);
        assert_eq!(g.fleet[0].hp, hp);
        let timer = g.fleet[0].guns[0].timer;
        g.fleet_update(0.3);
        assert_eq!(timer, g.fleet[0].guns[0].timer);
        g.bridge_hit(0, 9999.);
        assert_eq!(g.phase, Phase::Clear)
    }
    #[test]
    fn death_star_destruction_triggers_three_ship_ambush_before_victory() {
        let mut g = game();
        g.stage = 4;
        g.start_boss(false);
        for b in &mut g.fleet {
            b.p.y = b.arrival_y();
            for gun in &mut b.guns {
                gun.hp = 0.
            }
        }
        g.bridge_hit(0, 99999.);
        assert_eq!(g.phase, Phase::FalseVictory);
        g.age = 5.1;
        g.update(0.01, Input::default());
        assert_eq!(g.phase, Phase::Ambush);
        assert_eq!(
            g.fleet.iter().map(|b| b.kind).collect::<Vec<_>>(),
            vec![Hull::Destroyer, Hull::Executor, Hull::Destroyer]
        );
        for b in &mut g.fleet {
            b.p.y = b.arrival_y();
            for gun in &mut b.guns {
                gun.hp = 0.
            }
        }
        for i in 0..3 {
            g.bridge_hit(i, 99999.);
            if i < 2 {
                assert_eq!(g.phase, Phase::Ambush)
            }
        }
        assert_eq!(g.phase, Phase::Clear);
        g.age = 5.;
        g.update(0.01, Input::default());
        assert_eq!(g.phase, Phase::Won)
    }
    #[test]
    fn difficulties_change_durability_pressure_and_reserves() {
        assert!(Difficulty::Legend.armor() > Difficulty::Ace.armor());
        assert!(Difficulty::Rookie.interval() > Difficulty::Ace.interval());
        assert!(Difficulty::Legend.speed() > Difficulty::Ace.speed());
        assert!(Difficulty::Rookie.reserves() > Difficulty::Legend.reserves())
    }
    #[test]
    fn swept_projectile_does_not_tunnel() {
        assert!(swept(vec2(0., 100.), vec2(0., 0.), vec2(0., 50.), 5.));
        assert!(!swept(vec2(0., 100.), vec2(0., 0.), vec2(20., 50.), 5.))
    }
    #[test]
    fn side_entries_hooks_and_rotations_match_194x() {
        let left = FlightPath::Sweep {
            side: 1.,
            lane: 200.,
            phase: 0.,
        };
        let right = FlightPath::Sweep {
            side: -1.,
            lane: 200.,
            phase: 0.,
        };
        assert!(left.position(0., 1.).x < 0.);
        assert!(right.position(0., 1.).x > W);
        assert!(left.position(1., 1.).x > 0.);
        assert!(right.position(1., 1.).x < W);
        let hook = FlightPath::Hook {
            side: 1.,
            lane: 200.,
        };
        assert!(hook.position(1., 1.).x > hook.position(0., 1.).x);
        assert!(hook.position(4., 1.).x < hook.position(2., 1.).x);
    }
    #[test]
    fn armored_bombers_fire_starbursts_in_all_directions() {
        let mut g = game();
        g.spawn = 99.;
        g.enemies.push(Enemy {
            id: 1,
            kind: 2,
            p: vec2(300., 300.),
            base: 300.,
            age: 2.,
            hp: 12.,
            timer: 0.,
            form: 0,
            seed: 0.,
            angle: PI,
            dead: false,
            path: FlightPath::Dive { x: 300., phase: 0. },
        });
        g.update(0.01, Input::default());
        let shots: Vec<_> = g.shots.iter().filter(|s| s.enemy).collect();
        assert!(shots.len() >= 13);
        for direction in [vec2(1., 0.), vec2(-1., 0.), vec2(0., 1.), vec2(0., -1.)] {
            assert!(shots.iter().any(|p| p.v.dot(direction) > 50.));
        }
    }
    #[test]
    fn capital_command_decks_rotate_with_downward_hulls() {
        let mut g = game();
        g.start_boss(false);
        assert!(g.fleet[0].bridge.y < 0.);
        g.stage = 1;
        g.start_boss(false);
        assert!(g.fleet[0].bridge.y < 0.);
    }
    #[test]
    fn phone_height_keeps_fighter_near_bottom_and_allows_full_playfield() {
        let mut g = game();
        let margin = g.height - g.player.y;
        g.resize_height(1298.);
        assert!((g.height - g.player.y - margin).abs() < 0.01);
        g.update(
            1.,
            Input {
                movement: vec2(0., 1.),
                ..Input::default()
            },
        );
        assert!(g.player.y > H && g.player.y <= g.height - 35.);
    }
    #[test]
    fn endor_approach_and_final_boss_share_the_same_arrival_time() {
        let mut g = game();
        g.stage = 4;
        g.time = g.boss_arrival_time() - 0.1;
        assert!(g.endor_approach() > 0.99 && g.endor_approach() < 1.);
        g.update(0.11, Input::default());
        assert_eq!(g.phase, Phase::Boss);
        assert_eq!(g.endor_approach(), 1.);
    }
    #[test]
    fn destroying_bridge_during_a_projectile_batch_keeps_indices_stable() {
        let mut g = game();
        g.start_boss(false);
        g.fleet[0].p.y = 320.;
        for gun in &mut g.fleet[0].guns {
            gun.hp = 0.;
        }
        g.fleet[0].hp = 1.;
        let p = g.fleet[0].p + g.fleet[0].bridge;
        g.shot(vec2(10., 10.), vec2(0., 10.), 2., 1., true, true, 1);
        for _ in 0..2 {
            g.shot(p + vec2(0., 7.), vec2(0., -650.), 3., 5., false, false, 1);
        }
        g.update(0.02, Input::default());
        assert_eq!(g.phase, Phase::Clear);
    }
    #[test]
    fn incoming_fleets_ignore_lasers_and_bombs_until_every_ship_is_positioned() {
        let mut g = game();
        g.stage = 3;
        g.start_boss(false);
        let armor: Vec<_> = g.fleet.iter().map(|b| (b.hp, b.guns[0].hp)).collect();
        for i in 0..g.fleet.len() {
            g.gun_hit(i, 0, 9999.);
            g.bridge_hit(i, 9999.);
        }
        g.detonate(vec2(300., 200.));
        for (b, (hp, gun)) in g.fleet.iter().zip(&armor) {
            assert_eq!(b.hp, *hp);
            assert_eq!(b.guns[0].hp, *gun);
        }
        for b in &mut g.fleet {
            b.p.y = b.arrival_y();
        }
        g.fleet[1].p.y -= 1.;
        g.gun_hit(0, 0, 1.);
        assert_eq!(g.fleet[0].guns[0].hp, armor[0].1);
        g.fleet[1].p.y = g.fleet[1].arrival_y();
        assert!(g.fleet_ready());
        g.gun_hit(0, 0, 1.);
        assert_eq!(g.fleet[0].guns[0].hp, armor[0].1 - 1.);
        for b in &mut g.fleet {
            for gun in &mut b.guns {
                gun.hp = 0.;
            }
        }
        let hp = g.fleet[0].hp;
        g.fleet[1].p.y -= 1.;
        g.bridge_hit(0, 1.);
        assert_eq!(g.fleet[0].hp, hp);
        g.fleet[1].p.y = g.fleet[1].arrival_y();
        g.bridge_hit(0, 1.);
        assert_eq!(g.fleet[0].hp, hp - 1.);
    }
}
