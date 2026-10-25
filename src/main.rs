#![cfg_attr(not(target_arch = "wasm32"), windows_subsystem = "windows")]
use macroquad::prelude::*;
use starfighter::*;
use std::f32::consts::PI;
mod audio;
const INK: Color = Color::new(0.025, 0.055, 0.08, 1.);
const GOLD: Color = Color::new(0.96, 0.74, 0.4, 1.);
const CREAM: Color = Color::new(0.95, 0.96, 0.91, 1.);
const BLUE: Color = Color::new(0.35, 0.82, 1., 1.);
#[cfg(target_arch = "wasm32")]
extern "C" {
    fn starfighter_ready();
}
#[cfg(target_arch = "wasm32")]
static BACKGROUND: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(target_arch = "wasm32")]
static STATUS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
#[cfg(target_arch = "wasm32")]
static POSITION: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
/// Read-only state for automated browser control verification.
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn starfighter_status() -> u32 {
    STATUS.load(std::sync::atomic::Ordering::Relaxed)
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn starfighter_position() -> u32 {
    POSITION.load(std::sync::atomic::Ordering::Relaxed)
}
#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn starfighter_pause() {
    BACKGROUND.store(true, std::sync::atomic::Ordering::Relaxed)
}
fn conf() -> Conf {
    Conf {
        window_title: "Starfighter • Rebel Command".into(),
        window_width: 1040,
        window_height: 940,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}
struct Art {
    menu: Texture2D,
    bosses: Vec<Texture2D>,
    title: Texture2D,
    rebel: Texture2D,
    imperial: Texture2D,
    stations: Texture2D,
    barge: Texture2D,
    turrets: Texture2D,
    endor: Texture2D,
    backgrounds: Vec<Texture2D>,
    body: Font,
    display: Font,
}
macro_rules! texture {
    ($file:literal) => {
        Texture2D::from_file_with_format(
            include_bytes!(concat!("../assets/", $file)),
            Some(ImageFormat::Png),
        )
    };
}
// Isolate each boss onto its own texture and remove disconnected atlas debris.
// Filtering cannot remove actual opaque pixels from a neighboring sprite.
fn isolate_component(mut image: Image) -> Image {
    let w = image.width as usize;
    let h = image.height as usize;
    let mut visited = vec![false; w * h];
    let mut largest = Vec::new();
    for start in 0..w * h {
        if visited[start] || image.bytes[start * 4 + 3] < 24 {
            continue;
        }
        visited[start] = true;
        let mut component = vec![start];
        let mut cursor = 0;
        while cursor < component.len() {
            let i = component[cursor];
            cursor += 1;
            let x = i % w;
            let y = i / w;
            for (nx, ny) in [
                (x as isize - 1, y as isize),
                (x as isize + 1, y as isize),
                (x as isize, y as isize - 1),
                (x as isize, y as isize + 1),
            ] {
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    continue;
                }
                let n = ny as usize * w + nx as usize;
                if !visited[n] && image.bytes[n * 4 + 3] >= 24 {
                    visited[n] = true;
                    component.push(n);
                }
            }
        }
        if component.len() > largest.len() {
            largest = component;
        }
    }
    let mut keep = vec![false; w * h];
    for i in largest {
        keep[i] = true;
    }
    for (i, pixel) in image.bytes.chunks_exact_mut(4).enumerate() {
        if !keep[i] {
            pixel.fill(0);
        }
    }
    image
}
fn boss_images() -> Vec<Image> {
    let capital = Image::from_file_with_format(
        include_bytes!("../assets/rebel-capital-atlas.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let stations = Image::from_file_with_format(
        include_bytes!("../assets/death-stars.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    let mut barge = Image::from_file_with_format(
        include_bytes!("../assets/jabba-barge.png"),
        Some(ImageFormat::Png),
    )
    .unwrap();
    // Counter-rotate the whole throne compartment before the hull is turned
    // toward the player; this also avoids drawing a second portrait over Jabba.
    let (left, top, width, height) = (330usize, 1030usize, 360usize, 250usize);
    let stride = barge.width as usize;
    let source = barge.bytes.clone();
    for y in 0..height {
        for x in 0..width {
            let dst = ((top + y) * stride + left + x) * 4;
            let src = ((top + height - 1 - y) * stride + left + width - 1 - x) * 4;
            barge.bytes[dst..dst + 4].copy_from_slice(&source[src..src + 4]);
        }
    }
    [
        barge.sub_image(Rect::new(65., 0., 890., 1536.)),
        capital.sub_image(Rect::new(445., 750., 365., 504.)),
        capital.sub_image(Rect::new(919., 708., 270., 546.)),
        stations.sub_image(Rect::new(0., 0., 887., 887.)),
        stations.sub_image(Rect::new(887., 0., 887., 887.)),
    ]
    .into_iter()
    .map(isolate_component)
    .collect()
}
fn boss_textures() -> Vec<Texture2D> {
    boss_images()
        .into_iter()
        .map(|clean| {
            let t = Texture2D::from_image(&clean);
            t.set_filter(FilterMode::Linear);
            t
        })
        .collect()
}
impl Art {
    fn new() -> Self {
        let a = Self {
            bosses: boss_textures(),
            title: texture!("title.png"),
            menu: texture!("menu-action.png"),
            rebel: texture!("rebel-capital-atlas.png"),
            imperial: texture!("imperial-top-atlas.png"),
            stations: texture!("death-stars.png"),
            barge: texture!("jabba-barge.png"),
            turrets: texture!("turrets.png"),
            endor: texture!("endor-moon.png"),
            backgrounds: vec![
                texture!("tatooine.png"),
                texture!("hoth.png"),
                texture!("dagobah.png"),
                texture!("coruscant.png"),
                texture!("space-flight.png"),
            ],
            body: load_ttf_font_from_bytes(include_bytes!("../assets/fonts/Barlow-Regular.ttf"))
                .unwrap(),
            display: load_ttf_font_from_bytes(include_bytes!(
                "../assets/fonts/BarlowCondensed-SemiBold.ttf"
            ))
            .unwrap(),
        };
        // Atlas textures must use nearest sampling; linear filtering samples
        // neighboring hulls across crop boundaries and creates stray fragments.
        for t in [&a.rebel, &a.imperial, &a.stations] {
            t.set_filter(FilterMode::Nearest)
        }
        for t in [&a.title, &a.menu, &a.barge, &a.turrets]
            .into_iter()
            .chain(a.backgrounds.iter())
        {
            t.set_filter(FilterMode::Linear)
        }
        a
    }
    fn label(&self, t: &str, p: Vec2, size: f32, color: Color, center: bool, display: bool) {
        let font = if display { &self.display } else { &self.body };
        let w = if center {
            measure_text(t, Some(font), size as u16, 1.).width / 2.
        } else {
            0.
        };
        draw_text_ex(
            t,
            p.x - w,
            p.y,
            TextParams {
                font: Some(font),
                font_size: size as u16,
                color,
                ..Default::default()
            },
        );
    }
    fn image(&self, t: &Texture2D, source: Rect, p: Vec2, size: Vec2, angle: f32, color: Color) {
        draw_texture_ex(
            t,
            p.x - size.x / 2.,
            p.y - size.y / 2.,
            color,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(size),
                rotation: angle,
                ..Default::default()
            },
        )
    }
    fn craft(&self, kind: Craft, p: Vec2, size: f32, angle: f32, color: Color) {
        let i = match kind {
            Craft::Xwing => 0,
            Craft::Ywing => 1,
            Craft::Awing => 2,
        };
        self.image(
            &self.rebel,
            Rect::new(i as f32 * 418., 0., 418., 418.),
            p,
            vec2(size, size),
            angle,
            color,
        )
    }
    fn enemy(&self, e: &Enemy) {
        let i = e.kind;
        self.image(
            &self.imperial,
            Rect::new((i % 2) as f32 * 627., (i / 2) as f32 * 627., 627., 627.),
            e.p,
            vec2(if i == 2 { 44.25 } else { 39. }, 42.75),
            e.angle,
            WHITE,
        )
    }
    fn hull(&self, b: &BossShip, color: Color) {
        let index = match b.kind {
            Hull::Barge => 0,
            Hull::Destroyer => 1,
            Hull::Executor => 2,
            Hull::DeathStar => 3,
            Hull::DeathStar2 => 4,
        };
        let t = &self.bosses[index];
        let angle = if matches!(b.kind, Hull::DeathStar | Hull::DeathStar2) {
            0.
        } else {
            PI
        };
        self.image(
            t,
            Rect::new(0., 0., t.width(), t.height()),
            b.p,
            b.size,
            angle,
            color,
        );
    }
    fn backdrop(&self, stage: usize, time: f32, height: f32, approach: f32) {
        let t = &self.backgrounds[stage];
        if stage == 4 {
            let tile_h = W * t.height() / t.width();
            let offset = time * 24. % (tile_h * 2.);
            for k in -2..4 {
                draw_texture_ex(
                    t,
                    0.,
                    k as f32 * tile_h + offset - tile_h,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(W, tile_h)),
                        flip_y: k % 2 == 0,
                        ..Default::default()
                    },
                );
            }
            let p = vec2(W * (1.15 - approach * 0.27), -50. + approach * 270.);
            let size = 30. + approach * approach * 490.;
            draw_texture_ex(
                &self.endor,
                p.x - size / 2.,
                p.y - size / 2.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
            for i in 0..80 {
                let x = (i * 137 % 600) as f32;
                let y = ((i * 91) as f32 + time * (9. + (i % 4) as f32 * 8.)) % height;
                draw_rectangle(x, y, 1., 1.8, Color::new(0.8, 0.94, 1., 0.45))
            }
        } else {
            let h = W * t.height() / t.width();
            let offset = time * 22. % (h * 2.);
            for k in -2..3 {
                let y = k as f32 * h + offset - h;
                draw_texture_ex(
                    t,
                    0.,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(W, h)),
                        flip_y: k % 2 == 0,
                        ..Default::default()
                    },
                )
            }
            draw_rectangle(0., 0., W, height, Color::new(0.01, 0.025, 0.05, 0.16))
        }
    }
}
#[derive(Clone, Copy)]
struct View {
    origin: Vec2,
    size: Vec2,
    scale: f32,
}
impl View {
    fn new(_mobile: bool, height: f32) -> Self {
        let scale = (screen_width() / W)
            .min(screen_height() / height)
            .max(0.001);
        let size = vec2(W, height) * scale;
        Self {
            origin: (vec2(screen_width(), screen_height()) - size) / 2.,
            size,
            scale,
        }
    }
    fn point(&self, p: Vec2) -> Vec2 {
        (p - self.origin) / self.scale
    }
    fn button(&self, i: usize) -> Rect {
        let w = (self.size.x * 0.12).clamp(44., 62.);
        Rect::new(
            self.origin.x + self.size.x
                - w
                - 8.
                - if i > 0 { (i - 1) as f32 * (w + 5.) } else { 0. },
            if i == 0 {
                self.origin.y + self.size.y - w - 8.
            } else {
                self.origin.y + 8. * self.scale
            },
            w,
            w,
        )
    }
}
#[derive(Default)]
struct TouchState {
    drag: Option<(u64, Vec2)>,
    enabled: bool,
    pressed: Vec<u64>,
}
#[derive(Default)]
struct Actions {
    input: Input,
    pause: bool,
    mute: bool,
    menu_points: Vec<Vec2>,
}
impl TouchState {
    fn sample(&mut self, points: &[Touch], v: View, play: bool, paused: bool) -> Actions {
        let mut a = Actions::default();
        if !points.is_empty() {
            self.enabled = true
        }
        if let Some((id, old)) = self.drag {
            match points.iter().find(|p| p.id == id) {
                Some(p) if !matches!(p.phase, TouchPhase::Ended | TouchPhase::Cancelled) => {
                    a.input.drag = (p.position - old) / v.scale;
                    self.drag = Some((id, p.position))
                }
                _ => self.drag = None,
            }
        }
        // Mobile browsers can deliver both edges between two animation frames.
        // Handle an unobserved Ended as a tap; never run a button twice for one touch.
        for p in points
            .iter()
            .filter(|p| matches!(p.phase, TouchPhase::Started | TouchPhase::Ended))
        {
            if p.phase == TouchPhase::Ended {
                if let Some(index) = self.pressed.iter().position(|id| *id == p.id) {
                    self.pressed.swap_remove(index);
                    continue;
                }
            } else if !self.pressed.contains(&p.id) {
                self.pressed.push(p.id);
            }
            if play && v.button(0).contains(p.position) {
                a.input.bomb = true
            } else if play && v.button(1).contains(p.position) {
                a.pause = true
            } else if play && v.button(2).contains(p.position) {
                a.mute = true
            } else if paused {
                a.pause = true
            } else if play && self.drag.is_none() {
                self.drag = Some((p.id, p.position))
            } else if !play {
                a.menu_points.push(v.point(p.position))
            }
        }
        a.input.fire = self.enabled && play && !paused;
        a
    }
}
#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Intro,
    Menu,
    Play,
    Pause,
    End,
}
struct App {
    intro_age: f32,
    screen: Screen,
    craft: usize,
    difficulty: usize,
    stage: usize,
    game: Game,
    touch: TouchState,
    music_started: bool,
}
impl App {
    fn new() -> Self {
        Self {
            intro_age: 0.,
            screen: Screen::Intro,
            craft: 0,
            difficulty: 1,
            stage: 0,
            game: Game::new(Craft::Xwing, Difficulty::Ace, 0),
            touch: TouchState::default(),
            music_started: false,
        }
    }
    fn start(&mut self) {
        self.game = Game::new(
            Craft::ALL[self.craft],
            Difficulty::ALL[self.difficulty],
            self.stage,
        );
        self.screen = Screen::Play;
        self.touch.drag = None;
        self.music_started = true
    }
}
fn card(i: usize) -> Rect {
    Rect::new(32. + i as f32 * 181., 542., 174., 86.)
}
fn rating(i: usize) -> Rect {
    Rect::new(32. + i as f32 * 181., 650., 174., 36.)
}
fn deployment() -> Rect {
    Rect::new(32., 696., 536., 48.)
}
fn launch() -> Rect {
    Rect::new(32., 754., 536., 44.)
}
fn music_rect() -> Rect {
    Rect::new(330., 25., 112., 33.)
}
fn effects_rect() -> Rect {
    Rect::new(450., 25., 118., 33.)
}
fn panel(r: Rect, border: Color, fill: Color) {
    draw_rectangle(r.x, r.y, r.w, r.h, fill);
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 1., border)
}
fn draw_menu(a: &Art, app: &App, au: &audio::Audio, time: f32) {
    draw_texture_ex(
        &a.menu,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(W, 900.)),
            ..Default::default()
        },
    );
    draw_rectangle(0., 530., W, 310., Color::new(0.015, 0.03, 0.04, 0.35));
    a.label("REBEL COMMAND", vec2(32., 47.), 17., GOLD, false, false);
    a.label(
        "A GALAXY UNDER SIEGE. A REBELLION RISING.",
        vec2(34., 100.),
        13.,
        GOLD,
        false,
        false,
    );
    a.label("STARFIGHTER", vec2(30., 480.), 68., GOLD, false, true);
    a.label(
        "The odds are impossible. Fly anyway.",
        vec2(34., 514.),
        17.,
        CREAM,
        false,
        false,
    );
    for i in 0..3 {
        let r = card(i);
        let selected = i == app.craft;
        panel(
            r,
            if selected {
                GOLD
            } else {
                Color::new(0.4, 0.5, 0.55, 0.5)
            },
            Color::new(0.025, 0.055, 0.08, 0.78),
        );
        a.craft(Craft::ALL[i], vec2(r.x + 28., r.y + 27.), 42., 0., WHITE);
        a.label(
            Craft::ALL[i].name(),
            vec2(r.x + 57., r.y + 33.),
            20.,
            if selected { GOLD } else { CREAM },
            false,
            true,
        );
        a.label(
            [
                "BALANCED / WING CANNONS",
                "HEAVY / NOSE CANNONS",
                "FAST / EDGE CANNONS",
            ][i],
            vec2(r.x + 12., r.y + 59.),
            10.,
            CREAM,
            false,
            false,
        );
        a.label(
            &format!("BOMBS {}  /  SPEED {}", Craft::ALL[i].bombs(), [3, 2, 5][i]),
            vec2(r.x + 12., r.y + 76.),
            11.,
            BLUE,
            false,
            false,
        )
    }
    a.label("FLIGHT RATING", vec2(32., 644.), 11., GOLD, false, false);
    for i in 0..3 {
        let r = rating(i);
        panel(
            r,
            if i == app.difficulty { GOLD } else { GRAY },
            Color::new(0.02, 0.05, 0.07, 0.8),
        );
        a.label(
            Difficulty::ALL[i].name(),
            vec2(r.x + r.w / 2., r.y + 25.),
            20.,
            if i == app.difficulty { GOLD } else { CREAM },
            true,
            true,
        )
    }
    let r = deployment();
    panel(
        r,
        Color::new(0.4, 0.5, 0.55, 0.7),
        Color::new(0.02, 0.05, 0.07, 0.9),
    );
    a.label(
        "Q / E     DEPLOYMENT     TAP LEFT / RIGHT TO CHANGE",
        vec2(300., r.y + 13.),
        11.,
        BLUE,
        true,
        false,
    );
    a.label(
        &format!(
            "<  0{} / {}  >",
            app.stage + 1,
            MISSIONS[app.stage].to_uppercase()
        ),
        vec2(300., r.y + 36.),
        19.,
        CREAM,
        true,
        true,
    );
    let r = launch();
    draw_rectangle(r.x, r.y, r.w, r.h, GOLD);
    a.label(
        "LAUNCH CAMPAIGN",
        vec2(300., r.y + 30.),
        27.,
        INK,
        true,
        true,
    );
    a.label(
        "WASD / ARROWS  MOVE     SPACE / Z  FIRE     X / B  BOMB",
        vec2(300., 815.),
        10.,
        CREAM,
        true,
        false,
    );
    a.label(
        &format!(
            "v{} / BUILD {}",
            env!("CARGO_PKG_VERSION"),
            env!("STARFIGHTER_BUILD")
        ),
        vec2(300., 834.),
        10.,
        CREAM,
        true,
        false,
    );
    for (i, r) in [music_rect(), effects_rect()].into_iter().enumerate() {
        panel(
            r,
            Color::new(0.7, 0.75, 0.7, 0.5),
            Color::new(0.02, 0.05, 0.07, 0.7),
        );
        a.label(
            if i == 0 {
                if au.music {
                    "MUSIC ON"
                } else {
                    "MUSIC OFF"
                }
            } else if au.effects {
                "SOUND ON"
            } else {
                "SOUND OFF"
            },
            vec2(r.x + r.w / 2., r.y + 22.),
            13.,
            CREAM,
            true,
            false,
        )
    }
    let _ = time;
}
fn draw_gun(a: &Art, b: &BossShip, g: &Gun, time: f32) {
    let p = b.p + g.offset;
    if g.hp <= 0. {
        draw_ellipse(p.x, p.y, 12., 9., 0., Color::new(0.05, 0.045, 0.04, 0.95));
        draw_line(p.x - 7., p.y - 6., p.x + 7., p.y + 6., 2., DARKGRAY);
        draw_line(p.x + 7., p.y - 6., p.x - 7., p.y + 6., 2., DARKGRAY);
        for i in 0..3 {
            let age = (time * 0.4 + i as f32 * 0.3).fract();
            draw_circle(
                p.x + (time * 2. + i as f32).sin() * 5.,
                p.y - age * 30.,
                5. + age * 6.,
                Color::new(0.07, 0.075, 0.08, (1. - age) * 0.5),
            )
        }
        return;
    }
    let kind = match b.kind {
        Hull::Barge => 1,
        Hull::DeathStar | Hull::DeathStar2 => 2,
        _ => 0,
    };
    a.image(
        &a.turrets,
        Rect::new(kind as f32 * 724., 0., 724., 724.),
        p + vec2(0., g.recoil),
        vec2(25., 25.),
        g.angle,
        WHITE,
    );
    draw_circle_lines(p.x, p.y, 14., 1., Color::new(0.95, 0.75, 0.4, 0.5));
    draw_rectangle(p.x - 10., p.y + 17., 20., 2., INK);
    draw_rectangle(p.x - 10., p.y + 17., 20. * g.hp / g.max, 2., GOLD);
    if g.flash > 0. {
        let angle = g.angle - PI / 2.;
        let muzzle = p + vec2(angle.cos(), angle.sin()) * 18.;
        for j in (1..=4).rev() {
            draw_circle(
                muzzle.x,
                muzzle.y,
                j as f32 * 3.,
                Color::new(0.7, 1., 0.5, 0.12),
            )
        }
        draw_circle(muzzle.x, muzzle.y, 3., CREAM)
    }
}
fn draw_game(a: &Art, g: &Game) {
    a.backdrop(g.stage, g.total, g.height, g.endor_approach());
    for b in &g.bomb_runs {
        if b.age < 0.65 {
            let t = (b.age / 0.65).clamp(0., 1.);
            let p = b.origin.lerp(b.target(), t);
            draw_circle_lines(
                b.target().x,
                b.target().y,
                18. + (1. - t) * 20.,
                1.,
                GOLD.with_alpha(0.5),
            );
            draw_ellipse(p.x, p.y, 3.5, 9., 0., GOLD);
            draw_line(p.x, p.y + 8., p.x, p.y + 24., 3., BLUE.with_alpha(0.65));
        }
    }
    let shield = g.guns_left() > 0;
    for b in &g.fleet {
        a.hull(
            b,
            if b.dead {
                Color::new(0.3, 0.3, 0.3, 0.28)
            } else {
                WHITE
            },
        );
        if b.dead {
            continue;
        }
        for gun in &b.guns {
            draw_gun(a, b, gun, g.total)
        }
        let p = b.p + b.bridge;
        draw_circle_lines(
            p.x,
            p.y,
            22. + (g.total * 4.).sin(),
            1.5,
            if shield { BLUE } else { GOLD },
        );
        if shield {
            draw_circle_lines(p.x, p.y, 26., 1., Color::new(0.2, 0.7, 1., 0.3))
        } else {
            draw_rectangle(p.x - 21., p.y + 27., 42., 3., INK);
            draw_rectangle(p.x - 21., p.y + 27., 42. * b.hp / b.max, 3., GOLD)
        }
        a.label(
            if b.kind == Hull::Barge {
                "JABBA"
            } else if shield {
                "SHIELDED"
            } else {
                "BRIDGE"
            },
            p + vec2(0., 39.),
            9.,
            if shield { BLUE } else { GOLD },
            true,
            false,
        )
    }
    for e in &g.enemies {
        a.enemy(e)
    }
    if g.inv <= 0. || (g.inv * 12.) as i32 % 2 == 0 {
        let color = craft_color(g.craft);
        for port in g.craft.ports() {
            let p = g.player + vec2(*port * 0.35, 25.);
            draw_ellipse(
                p.x,
                p.y,
                3.,
                10. + (g.total * 40.).sin() * 3.,
                0.,
                Color::new(0.2, 0.65, 1., 0.6),
            );
            draw_ellipse(p.x, p.y, 1.5, 7., 0., CREAM)
        }
        a.craft(g.craft, g.player, 48., 0., WHITE);
        let _ = color;
    }
    for p in &g.pickups {
        let color = match p.kind {
            PickupKind::Force => BLUE,
            PickupKind::Bomb => GOLD,
            PickupKind::Spread => Color::new(0.9, 0.8, 0.4, 1.),
            PickupKind::Cannon => Color::new(0.55, 0.9, 1., 1.),
        };
        for r in (1..=5).rev() {
            draw_circle(
                p.p.x,
                p.p.y,
                r as f32 * 5.,
                Color::new(color.r, color.g, color.b, 0.07),
            )
        }
        draw_circle_lines(p.p.x, p.p.y, 14. + (p.age * 6.).sin() * 2., 1.5, color);
        a.label(
            match p.kind {
                PickupKind::Force => "+",
                PickupKind::Bomb => "B",
                PickupKind::Spread => "S",
                PickupKind::Cannon => "C",
            },
            p.p + vec2(0., 7.),
            23.,
            CREAM,
            true,
            true,
        );
        a.label(
            match p.kind {
                PickupKind::Force => "THE FORCE",
                PickupKind::Bomb => "BOMB",
                PickupKind::Spread => "SPREAD +",
                PickupKind::Cannon => "CANNON +",
            },
            p.p + vec2(0., 31.),
            11.,
            color,
            true,
            false,
        )
    }
    for p in &g.shots {
        let color = if p.enemy {
            Color::new(1., 0.16, 0.06, 1.)
        } else {
            if p.green {
                Color::new(0.2, 1., 0.4, 1.)
            } else {
                BLUE
            }
        };
        let end = p.p - p.v.normalize_or_zero() * if p.enemy { 13. } else { 19. };
        for j in (1..=3).rev() {
            draw_line(
                p.p.x,
                p.p.y,
                end.x,
                end.y,
                p.radius * 2. + j as f32 * 3.,
                Color::new(color.r, color.g, color.b, 0.08),
            )
        }
        draw_line(p.p.x, p.p.y, end.x, end.y, p.radius * 2., color);
        draw_line(p.p.x, p.p.y, end.x, end.y, p.radius * 0.6, CREAM)
    }
    for b in &g.blasts {
        if b.age < 0. {
            continue;
        }
        let alpha = (1. - b.age / 1.7).max(0.);
        let radius = (8. + b.age * 55.) * b.size;
        for i in (1..=9).rev() {
            let f = i as f32 / 9.;
            draw_circle(
                b.p.x,
                b.p.y,
                radius * f,
                Color::new(
                    1.,
                    0.25 + (1. - f) * 0.7,
                    0.05 + (1. - f) * 0.5,
                    alpha * 0.12,
                ),
            )
        }
        draw_circle_lines(
            b.p.x,
            b.p.y,
            radius * 1.25,
            1.,
            Color::new(0.8, 0.9, 1., alpha * 0.35),
        );
        for i in 0..25 {
            let angle = i as f32 * 2.399;
            let p = b.p
                + vec2(angle.cos(), angle.sin()) * b.age * (40. + (i % 6) as f32 * 18.) * b.size;
            draw_circle(p.x, p.y, 1. + alpha, GOLD.with_alpha(alpha))
        }
    }
    if g.flash > 0. {
        draw_rectangle(0., 0., W, g.height, Color::new(0.7, 0.9, 1., g.flash))
    }
    // HUD follows 194X: armor / reserves / score at upper left, mission beneath, controls upper right.
    draw_rectangle(0., 0., W, 82., Color::new(0.015, 0.04, 0.065, 0.88));
    a.label(
        &format!(
            "THE FORCE {:03}  |  WINGMEN {}",
            g.force.ceil() as i32,
            g.reserves
        ),
        vec2(12., 22.),
        13.,
        BLUE,
        false,
        false,
    );
    a.label(
        &format!("SCORE {:07}   BOMBS {}", g.score, g.bombs),
        vec2(12., 42.),
        12.,
        CREAM,
        false,
        false,
    );
    a.label(
        &format!("0{} / {}", g.stage + 1, MISSIONS[g.stage].to_uppercase()),
        vec2(12., 65.),
        13.,
        GOLD,
        false,
        false,
    );
    if !g.fleet.is_empty() {
        let max = g.fleet.iter().map(BossShip::mass).sum::<f32>();
        let hp = g
            .fleet
            .iter()
            .filter(|b| !b.dead)
            .map(|b| b.hp + b.guns.iter().map(|g| g.hp).sum::<f32>())
            .sum::<f32>();
        draw_rectangle(12., 77., 576., 3., Color::new(0.1, 0.2, 0.24, 0.7));
        draw_rectangle(12., 77., 576. * hp / max, 3., GOLD);
        a.label(
            &format!(
                "{} / {} TURRETS / {}",
                if g.phase == Phase::Ambush {
                    "LAST STAND"
                } else if g.fleet.len() == 1 {
                    g.fleet[0].kind.name()
                } else {
                    "IMPERIAL FLEET"
                },
                g.guns_left(),
                if !g.fleet_ready() {
                    "POSITIONING / SHIELDS ACTIVE"
                } else if shield {
                    "DESTROY ALL GUNS FIRST"
                } else {
                    "COMMAND TARGETS EXPOSED"
                }
            ),
            vec2(300., 96.),
            9.,
            if shield { BLUE } else { GOLD },
            true,
            false,
        )
    }
    if g.notice_age > 0. {
        let warning = g.notice.starts_with("WARNING");
        let size = if warning {
            28.
        } else if g.notice.len() > 48 {
            13.
        } else {
            20.
        };
        let p = vec2(
            W / 2.,
            if warning {
                g.height * 0.46
            } else {
                g.height * 0.4
            },
        );
        let alpha = (g.notice_age / 0.45).min(1.);
        a.label(
            &g.notice,
            p + vec2(1., 2.),
            size,
            INK.with_alpha(alpha * 0.8),
            true,
            true,
        );
        a.label(
            &g.notice,
            p,
            size,
            if warning {
                Color::new(1., 0.55, 0.35, alpha)
            } else {
                GOLD.with_alpha(alpha)
            },
            true,
            true,
        )
    }
    a.label(
        &format!(
            "{} L{}",
            if g.weapon == 0 { "SPREAD" } else { "CANNON" },
            g.levels[g.weapon]
        ),
        vec2(12., g.height - 15.),
        13.,
        CREAM,
        false,
        false,
    );
}
fn craft_color(c: Craft) -> Color {
    match c {
        Craft::Xwing => Color::new(1., 0.25, 0.35, 1.),
        Craft::Ywing => BLUE,
        Craft::Awing => Color::new(0.3, 1., 0.6, 1.),
    }
}
fn draw_controls(a: &Art, v: View, app: &App, au: &audio::Audio) {
    if !matches!(app.screen, Screen::Play | Screen::Pause) {
        return;
    }
    for i in 0..3 {
        let r = v.button(i);
        panel(
            r,
            if i == 0 {
                GOLD
            } else {
                Color::new(0.7, 0.8, 0.8, 0.6)
            },
            Color::new(0.02, 0.06, 0.09, 0.82),
        );
        let label = match i {
            0 => format!("BOMB {}", app.game.bombs),
            1 => {
                if app.screen == Screen::Pause {
                    "PLAY".into()
                } else {
                    "PAUSE".into()
                }
            }
            _ => {
                if au.music || au.effects {
                    "MUTE".into()
                } else {
                    "SOUND".into()
                }
            }
        };
        if i == 0 {
            draw_ellipse(r.x + r.w / 2., r.y + 17., 5., 9., 0., GOLD);
            draw_line(
                r.x + r.w / 2. - 5.,
                r.y + 8.,
                r.x + r.w / 2. + 5.,
                r.y + 8.,
                2.,
                CREAM,
            )
        }
        a.label(
            &label,
            vec2(
                r.x + r.w / 2.,
                r.y + if i == 0 { r.h - 7. } else { r.h / 2. + 4. },
            ),
            if i == 0 { 12. } else { 10. },
            if i == 0 { GOLD } else { CREAM },
            true,
            false,
        )
    }
}
fn draw_intro(a: &Art, time: f32) {
    clear_background(Color::from_hex(0x080e12));
    let fade = (time / 0.7).clamp(0., 1.) * ((3.6 - time) / 0.65).clamp(0., 1.);
    let y = H / 2. - 47. - (1. - (time / 0.9).clamp(0., 1.)).powi(3) * 12.;
    let green = Color::new(0.57, 0.72, 0.53, fade);
    let dark = Color::new(0.15, 0.24, 0.18, fade);
    let p = vec2(W / 2., y);
    draw_triangle(
        p + vec2(-22., -9.),
        p + vec2(-72., -20.),
        p + vec2(-34., 13.),
        green,
    );
    draw_triangle(
        p + vec2(22., -9.),
        p + vec2(72., -20.),
        p + vec2(34., 13.),
        green,
    );
    draw_triangle(
        p + vec2(-31., -7.),
        p + vec2(-58., -15.),
        p + vec2(-34., 4.),
        dark,
    );
    draw_triangle(
        p + vec2(31., -7.),
        p + vec2(58., -15.),
        p + vec2(34., 4.),
        dark,
    );
    draw_ellipse(p.x, p.y, 31., 30., 0., green);
    draw_ellipse(p.x - 12., p.y, 7., 4., -8., dark);
    draw_ellipse(p.x + 12., p.y, 7., 4., 8., dark);
    draw_line(p.x - 17., p.y - 9., p.x - 5., p.y - 7., 1.8, dark);
    draw_line(p.x + 17., p.y - 9., p.x + 5., p.y - 7., 1.8, dark);
    draw_line(p.x - 8., p.y - 19., p.x + 8., p.y - 19., 1., dark);
    draw_line(p.x - 5., p.y + 15., p.x + 5., p.y + 15., 1.2, dark);
    draw_triangle(p + vec2(0., 4.), p + vec2(-3., 9.), p + vec2(3., 9.), dark);
    a.label(
        "Dagobah Ventures",
        vec2(W / 2., y + 88.),
        32.,
        Color::new(0.88, 0.91, 0.88, fade),
        true,
        false,
    );
    draw_line(
        W / 2. - 19.,
        y + 115.,
        W / 2. + 19.,
        y + 115.,
        1.,
        Color::new(0.57, 0.72, 0.53, fade * 0.6),
    );
    a.label(
        "TAP OR ENTER TO SKIP",
        vec2(W / 2., H - 45.),
        10.,
        Color::new(0.6, 0.65, 0.65, fade * 0.45),
        true,
        false,
    );
}
fn input_keys(a: &mut Actions) {
    a.input.movement = vec2(
        (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) as i32 as f32
            - (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) as i32 as f32,
        (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) as i32 as f32
            - (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) as i32 as f32,
    );
    a.input.fire |= is_key_down(KeyCode::Space) || is_key_down(KeyCode::Z);
    a.input.slow = is_key_down(KeyCode::LeftShift);
    a.input.bomb |= is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::B);
    a.input.swap |= is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Tab);
    a.pause |= is_key_pressed(KeyCode::P) || is_key_pressed(KeyCode::Escape);
    a.mute |= is_key_pressed(KeyCode::M);
}
#[macroquad::main(conf)]
async fn main() {
    let art = Art::new();
    let mut target = render_target(1200, 1680);
    target.texture.set_filter(FilterMode::Linear);
    let mut audio = audio::Audio::new().await;
    let mut app = App::new();
    let mut fullscreen = false;
    #[cfg(target_arch = "wasm32")]
    unsafe {
        starfighter_ready()
    };
    #[cfg(not(target_arch = "wasm32"))]
    let capture = std::env::args().any(|a| a == "--capture");
    #[cfg(target_arch = "wasm32")]
    let capture = false;
    let mut capture_frame = 0usize;
    loop {
        let dt = get_frame_time().min(0.035);
        let mut touches: Vec<Touch> = touches()
            .into_iter()
            .map(|mut p| {
                p.position /= screen_dpi_scale().max(0.1);
                p
            })
            .collect();
        if touches.is_empty()
            && (is_mouse_button_down(MouseButton::Left)
                || is_mouse_button_released(MouseButton::Left))
        {
            touches.push(Touch {
                id: u64::MAX,
                position: vec2(mouse_position().0, mouse_position().1),
                phase: if is_mouse_button_pressed(MouseButton::Left) {
                    TouchPhase::Started
                } else if is_mouse_button_released(MouseButton::Left) {
                    TouchPhase::Ended
                } else {
                    TouchPhase::Moved
                },
            });
        }
        let mobile = !touches.is_empty() || app.touch.enabled || screen_width() < 700.;
        let playing = matches!(app.screen, Screen::Play | Screen::Pause);
        let scene_height = if playing {
            (screen_height() / screen_width() * W).clamp(H, 1500.)
        } else {
            H
        };
        if playing {
            app.game.resize_height(scene_height);
        }
        let view = View::new(mobile, scene_height);
        let target_height = (scene_height * 2.).ceil() as u32;
        if target.texture.height() as u32 != target_height {
            target = render_target(1200, target_height);
            target.texture.set_filter(FilterMode::Linear);
        }
        let mut act = app.touch.sample(
            &touches,
            view,
            matches!(app.screen, Screen::Play | Screen::Pause),
            app.screen == Screen::Pause,
        );
        input_keys(&mut act);
        #[cfg(target_arch = "wasm32")]
        if BACKGROUND.swap(false, std::sync::atomic::Ordering::Relaxed)
            && app.screen == Screen::Play
        {
            app.screen = Screen::Pause;
            app.touch.drag = None;
        }
        if is_key_pressed(KeyCode::F11) {
            fullscreen = !fullscreen;
            set_fullscreen(fullscreen)
        }
        if act.mute {
            let mute = audio.music || audio.effects;
            audio.music = !mute;
            audio.effects = !mute;
            app.music_started = true
        }
        if is_mouse_button_pressed(MouseButton::Left) && touches.is_empty() {
            let raw = vec2(mouse_position().0, mouse_position().1);
            if matches!(app.screen, Screen::Play | Screen::Pause) {
                for i in 0..3 {
                    if view.button(i).contains(raw) {
                        match i {
                            0 => act.input.bomb = true,
                            1 => act.pause = true,
                            _ => {
                                let muted = audio.music || audio.effects;
                                audio.music = !muted;
                                audio.effects = !muted
                            }
                        }
                    }
                }
            } else {
                act.menu_points.push(view.point(raw))
            }
        }
        match app.screen {
            Screen::Intro => {
                app.intro_age += dt;
                if app.intro_age >= 3.6
                    || (app.intro_age > 0.4
                        && (is_key_pressed(KeyCode::Enter)
                            || is_key_pressed(KeyCode::Space)
                            || !act.menu_points.is_empty()))
                {
                    app.screen = Screen::Menu;
                    app.touch.enabled = false;
                }
            }
            Screen::Menu => {
                if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::PageUp) {
                    app.stage = (app.stage + 4) % 5
                }
                if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::PageDown) {
                    app.stage = (app.stage + 1) % 5
                }
                if is_key_pressed(KeyCode::Key1) {
                    app.difficulty = 0
                }
                if is_key_pressed(KeyCode::Key2) {
                    app.difficulty = 1
                }
                if is_key_pressed(KeyCode::Key3) {
                    app.difficulty = 2
                }
                if is_key_pressed(KeyCode::Left) {
                    app.craft = (app.craft + 2) % 3
                }
                if is_key_pressed(KeyCode::Right) {
                    app.craft = (app.craft + 1) % 3
                }
                for p in act.menu_points {
                    for i in 0..3 {
                        if card(i).contains(p) {
                            app.craft = i
                        }
                        if rating(i).contains(p) {
                            app.difficulty = i
                        }
                    }
                    if deployment().contains(p) {
                        app.stage = if p.x < 300. {
                            (app.stage + 4) % 5
                        } else {
                            (app.stage + 1) % 5
                        }
                    }
                    if music_rect().contains(p) {
                        audio.music = !audio.music;
                        app.music_started = true
                    }
                    if effects_rect().contains(p) {
                        audio.effects = !audio.effects;
                        app.music_started = true
                    }
                    if launch().contains(p) {
                        app.start()
                    }
                }
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    app.start()
                }
            }
            Screen::Play => {
                if act.pause {
                    app.screen = Screen::Pause;
                    app.game.events.clear();
                    app.game.shake = 0.
                } else {
                    app.game.update(dt, act.input);
                    if matches!(app.game.phase, Phase::Won | Phase::Lost) {
                        app.screen = Screen::End;
                        app.game.shake = 0.
                    }
                }
            }
            Screen::Pause => {
                app.game.events.clear();
                if act.pause {
                    app.screen = Screen::Play;
                    app.touch.drag = None
                }
            }
            Screen::End => {
                app.game.age += dt;
                app.game.events.clear();
                if app.game.age > 1.
                    && (is_key_pressed(KeyCode::Enter)
                        || is_key_pressed(KeyCode::Space)
                        || !act.menu_points.is_empty())
                {
                    app.screen = Screen::Menu
                }
            }
        }
        if capture {
            let scene = capture_frame / 4;
            if capture_frame % 4 == 0 {
                app = App::new();
                app.screen = Screen::Menu;
                if scene > 0 {
                    app.start();
                    app.game.stage = (scene - 1).min(4);
                    app.game.start_boss(scene == 6);
                    for b in &mut app.game.fleet {
                        b.p.y = b.arrival_y();
                    }
                    app.game.notice_age = 0.;
                    app.game.inv = 0.;
                    if scene == 7 {
                        for b in &mut app.game.fleet {
                            for g in &mut b.guns {
                                g.hp = 0.
                            }
                        }
                    }
                    if scene == 8 {
                        app.game.phase = Phase::Won;
                        app.screen = Screen::End;
                    }
                }
            }
        }
        let active = app.music_started && app.screen != Screen::Pause;
        let boss = app.screen == Screen::Play
            && (matches!(
                app.game.phase,
                Phase::Boss | Phase::Ambush | Phase::FalseVictory
            ) || (app.game.phase == Phase::Waves
                && app.game.time >= app.game.boss_arrival_time() - 10.));
        audio.update(
            if app.screen == Screen::Play {
                &app.game.events
            } else {
                &[]
            },
            boss,
            active,
        );
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0., 0., W, scene_height))
        });
        clear_background(INK);
        if app.screen == Screen::Intro {
            draw_intro(&art, app.intro_age);
        } else if app.screen == Screen::Menu {
            draw_menu(&art, &app, &audio, get_time() as f32)
        } else {
            let shake = if app.screen == Screen::Play {
                app.game.shake
            } else {
                0.
            };
            if shake > 0. {
                set_camera(&Camera2D {
                    render_target: Some(target.clone()),
                    offset: vec2(
                        (get_time() as f32 * 57.).sin() * shake / W,
                        (get_time() as f32 * 71.).cos() * shake / H,
                    ),
                    ..Camera2D::from_display_rect(Rect::new(0., 0., W, scene_height))
                })
            }
            draw_game(&art, &app.game);
            if app.screen == Screen::Pause {
                draw_rectangle(0., 0., W, scene_height, Color::new(0.01, 0.035, 0.06, 0.8));
                art.label("FLIGHT PAUSED", vec2(300., 400.), 55., GOLD, true, true);
                art.label(
                    "P / ESC OR TAP TO RESUME",
                    vec2(300., 442.),
                    16.,
                    CREAM,
                    true,
                    false,
                )
            }
            if app.screen == Screen::End {
                draw_texture_ex(
                    &art.title,
                    -600.,
                    0.,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(1960., 840.)),
                        ..Default::default()
                    },
                );
                draw_rectangle(0., 0., W, H, Color::new(0.01, 0.03, 0.05, 0.84));
                art.label(
                    "REBEL COMMAND / FLIGHT REPORT",
                    vec2(300., 270.),
                    16.,
                    GOLD,
                    true,
                    false,
                );
                art.label(
                    if app.game.phase == Phase::Won {
                        "THE GALAXY HAS HOPE."
                    } else {
                        "THE REBELLION ENDURES."
                    },
                    vec2(300., 355.),
                    48.,
                    GOLD,
                    true,
                    true,
                );
                art.label(
                    &format!("{} / {}", app.game.craft.name(), app.game.difficulty.name()),
                    vec2(300., 420.),
                    20.,
                    CREAM,
                    true,
                    false,
                );
                art.label(
                    &format!("SCORE {:07}", app.game.score),
                    vec2(300., 465.),
                    35.,
                    CREAM,
                    true,
                    true,
                );
                art.label(
                    &format!("{} IMPERIALS DOWN", app.game.kills),
                    vec2(300., 500.),
                    17.,
                    BLUE,
                    true,
                    false,
                );
                art.label(
                    "TAP OR PRESS ENTER TO RETURN TO HANGAR",
                    vec2(300., 600.),
                    16.,
                    CREAM,
                    true,
                    false,
                )
            }
        }
        set_default_camera();
        clear_background(INK);
        draw_texture_ex(
            &target.texture,
            view.origin.x,
            view.origin.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(view.size),
                flip_y: true,
                ..Default::default()
            },
        );
        draw_controls(&art, view, &app, &audio);
        #[cfg(not(target_arch = "wasm32"))]
        if capture && capture_frame % 4 == 3 {
            std::fs::create_dir_all("target/preview").ok();
            target.texture.get_texture_data().export_png(&format!(
                "target/preview/scene-{:02}.png",
                capture_frame / 4
            ));
        }
        if capture {
            capture_frame += 1;
            if capture_frame >= 36 {
                break;
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let screen = match app.screen {
                Screen::Intro | Screen::Menu => 0,
                Screen::Play => 1,
                Screen::Pause => 2,
                Screen::End => 3,
            };
            STATUS.store(
                screen
                    | ((app.game.stage as u32) << 4)
                    | ((app.craft as u32) << 8)
                    | ((app.difficulty as u32) << 10)
                    | ((app.game.force.max(0.).ceil() as u32) << 16)
                    | (app.game.bombs.min(255) << 24),
                std::sync::atomic::Ordering::Relaxed,
            );
            POSITION.store(
                app.game.player.x as u32 | ((app.game.player.y as u32) << 16),
                std::sync::atomic::Ordering::Relaxed,
            );
        }
        next_frame().await;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menu_ship_selection_never_overlaps_launch() {
        for i in 0..3 {
            assert!(!card(i).overlaps(&launch()));
            assert!(!rating(i).overlaps(&launch()))
        }
        assert!(!deployment().overlaps(&launch()))
    }
    #[test]
    fn boss_atlas_debris_is_removed_and_clean_images_export() {
        let mut im = Image::gen_image_color(10, 10, Color::new(0., 0., 0., 0.));
        for x in 2..7 {
            for y in 2..7 {
                im.set_pixel(x, y, WHITE);
            }
        }
        im.set_pixel(9, 9, WHITE);
        let clean = isolate_component(im);
        assert_eq!(clean.bytes[(9 * 10 + 9) * 4 + 3], 0);
        assert_eq!(clean.bytes[(4 * 10 + 4) * 4 + 3], 255);
        std::fs::create_dir_all("target/preview").unwrap();
        for (i, im) in boss_images().iter().enumerate() {
            im.export_png(&format!("target/preview/clean-boss-{i}.png"));
        }
    }
    fn view() -> View {
        View {
            origin: vec2(0., 0.),
            size: vec2(390., 844.),
            scale: 0.65,
        }
    }
    #[test]
    fn quick_phone_taps_are_not_lost_and_buttons_do_not_repeat() {
        let mut t = TouchState::default();
        let p = vec2(200., 300.);
        let action = t.sample(
            &[Touch {
                id: 1,
                phase: TouchPhase::Ended,
                position: p,
            }],
            view(),
            false,
            false,
        );
        assert_eq!(action.menu_points.len(), 1);
        let action = t.sample(
            &[Touch {
                id: 2,
                phase: TouchPhase::Started,
                position: p,
            }],
            view(),
            false,
            false,
        );
        assert_eq!(action.menu_points.len(), 1);
        let action = t.sample(
            &[Touch {
                id: 2,
                phase: TouchPhase::Ended,
                position: p,
            }],
            view(),
            false,
            false,
        );
        assert!(action.menu_points.is_empty());
    }
    #[test]
    fn second_finger_bomb_preserves_relative_drag() {
        let mut t = TouchState::default();
        let v = view();
        let first = Touch {
            id: 1,
            phase: TouchPhase::Started,
            position: vec2(120., 600.),
        };
        let a = t.sample(&[first], v, true, false);
        assert_eq!(a.input.drag, Vec2::ZERO);
        let moved = Touch {
            id: 1,
            phase: TouchPhase::Moved,
            position: vec2(140., 580.),
        };
        let r = v.button(0);
        let bomb = Touch {
            id: 2,
            phase: TouchPhase::Started,
            position: vec2(r.x + r.w / 2., r.y + r.h / 2.),
        };
        let a = t.sample(&[moved, bomb], v, true, false);
        assert!(a.input.bomb);
        assert_eq!(t.drag.unwrap().0, 1);
        assert!(a.input.drag.x > 30. && a.input.drag.y < -30.);
    }
}
