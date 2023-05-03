pub(crate) mod ipc;
pub(crate) mod scene;

use {
    crate::{entity::ecs::ECS, error::ZapataError, physics::Effect},
    std::{
        fmt::{Display, Formatter},
        result::Result,
        time,
        vec::Vec,
    },
};

#[derive(Default, Debug)]
pub struct SceneManager {
    running: bool,
    lifetime: Lifetime,
    scenes: Vec<Scene>,
}

#[derive(Debug)]
struct Lifetime {
    total_tick_time: time::Duration,
    total_delta_tick_time: time::Duration,
    last_tick_duration: time::Duration,
    last_tick_timestamp: time::Instant,
    ticks: u64,
}

pub struct Scene {
    name: String,
    lifetime: Lifetime,
    physics_effects: Vec<Effect>,

    pub ecs: ECS,
}

impl Lifetime {
    pub fn start(&mut self) {
        self.last_tick_timestamp = time::Instant::now();
    }

    pub fn stop(&mut self) {
        let tick_dur = time::Instant::now().duration_since(self.last_tick_timestamp);
        self.total_tick_time += tick_dur;

        if tick_dur > self.last_tick_duration {
            // If this tick took longer than the previous, add the dur to the total delta time
            self.total_delta_tick_time += tick_dur - self.last_tick_duration
        }

        self.last_tick_duration = tick_dur;
        self.ticks += 1;
    }

    pub fn new() -> Self {
        Self {
            total_tick_time: time::Duration::from_secs(0),
            total_delta_tick_time: time::Duration::from_secs(0),
            last_tick_duration: time::Duration::from_secs(0),
            last_tick_timestamp: time::Instant::now(),
            ticks: 0,
        }
    }
}

impl SceneManager {
    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn run(&mut self, epochs: Option<usize>) -> Result<(), ZapataError> {
        self.running = true;

        while self.running {
            self.lifetime.start();
            if let Some(stop) = epochs {
                if self.lifetime.ticks == stop as u64 {
                    break;
                }
            }

            for scene in self.scenes.iter_mut() {
                if let Err(e) = scene.update() {
                    return Err(e);
                }
            }

            self.lifetime.stop();
        }
        Ok(())
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn new(scenes: Vec<Scene>) -> Self {
        Self {
            scenes,
            lifetime: Lifetime::new(),
            running: false,
        }
    }
}

impl Default for Lifetime {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SceneManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, scene) in self.scenes.iter().enumerate() {
            if let Err(e) = f.write_fmt(format_args!(
                "Scene {}: {:?} | avg: {:?}\n",
                i,
                scene.lifetime,
                scene.average_tick()
            )) {
                return f.write_str("Couldn't build SceneManager display string.");
            }
        }
        f.write_str("")
    }
}
