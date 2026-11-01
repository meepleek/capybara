use bevy::{
    asset::{embedded_asset, load_embedded_asset},
    image::{ImageLoaderSettings, ImageSampler},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use iyes_progress::ProgressTracker;

use crate::{UpdateSystems, assets::InitialScreen, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    // Spawn splash screen.
    app.insert_resource(ClearColor(SPLASH_BACKGROUND_COLOR));
    app.add_systems(OnEnter(Screen::Splash), spawn_splash_screen);

    // Animate splash screen.
    app.add_systems(
        Update,
        (
            tick_fade_in_out.in_set(UpdateSystems::TickTimers),
            apply_fade_in_out.in_set(UpdateSystems::Last),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // Add splash timer.
    app.add_systems(OnEnter(Screen::Splash), insert_splash_timer);
    app.add_systems(OnExit(Screen::Splash), remove_splash_timer);
    app.add_systems(
        Update,
        (
            tick_splash_timer.in_set(UpdateSystems::TickTimers),
            check_progress.in_set(UpdateSystems::Last),
        )
            .run_if(in_state(Screen::Splash)),
    );

    // embedded imgs
    embedded_asset!(app, "../../../assets/splash/bevy.png");
}

const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);
const SPLASH_DURATION_SECS: f32 = 1.8;
const SPLASH_FADE_DURATION_SECS: f32 = 0.6;

fn spawn_splash_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    initial_screen: Res<InitialScreen>,
) {
    if !initial_screen.show_splash {
        return;
    }

    let ass = asset_server.into_inner();
    let splash_bevy = load_embedded_asset!(
        ass,
        "../../../assets/splash/bevy.png",
        |settings: &mut ImageLoaderSettings| {
            // `ImagePlugin::default_nearest()` is used for pixel art.
            settings.sampler = ImageSampler::linear();
        }
    );

    commands.spawn((
        widget::ui_root("Splash Screen"),
        BackgroundColor(SPLASH_BACKGROUND_COLOR),
        DespawnOnExit(Screen::Splash),
        children![(
            Name::new("Splash image"),
            Node {
                margin: UiRect::all(Val::Auto),
                width: percent(70),
                ..default()
            },
            ImageNode::new(splash_bevy),
            ImageNodeFadeInOut {
                total_duration: SPLASH_DURATION_SECS,
                fade_duration: SPLASH_FADE_DURATION_SECS,
                t: 0.0,
            },
        )],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ImageNodeFadeInOut {
    /// Total duration in seconds.
    total_duration: f32,
    /// Fade duration in seconds.
    fade_duration: f32,
    /// Current progress in seconds, between 0 and [`Self::total_duration`].
    t: f32,
}

impl ImageNodeFadeInOut {
    fn alpha(&self) -> f32 {
        // Normalize by duration.
        let t = (self.t / self.total_duration).clamp(0.0, 1.0);
        let fade = self.fade_duration / self.total_duration;

        // Regular trapezoid-shaped graph, flat at the top with alpha = 1.0.
        ((1.0 - (2.0 * t - 1.0).abs()) / fade).min(1.0)
    }
}

fn tick_fade_in_out(time: Res<Time>, mut animation_query: Query<&mut ImageNodeFadeInOut>) {
    for mut anim in &mut animation_query {
        anim.t += time.delta_secs();
    }
}

fn apply_fade_in_out(mut animation_query: Query<(&ImageNodeFadeInOut, &mut ImageNode)>) {
    for (anim, mut image) in &mut animation_query {
        image.color.set_alpha(anim.alpha())
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Reflect)]
#[reflect(Resource)]
struct SplashTimer(Timer);

impl Default for SplashTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(SPLASH_DURATION_SECS, TimerMode::Once))
    }
}

fn insert_splash_timer(mut commands: Commands) {
    commands.init_resource::<SplashTimer>();
}

fn remove_splash_timer(mut commands: Commands) {
    commands.remove_resource::<SplashTimer>();
}

fn tick_splash_timer(time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    timer.0.tick(time.delta());
}

fn check_progress(
    timer: ResMut<SplashTimer>,
    mut next_screen: ResMut<NextState<Screen>>,
    initial_screen: Res<InitialScreen>,
    progress: Res<ProgressTracker<Screen>>,
) {
    let splash_done = timer.0.just_finished() || !initial_screen.show_splash;
    let assets_loaded = progress.is_ready();

    if splash_done && assets_loaded {
        next_screen.set(initial_screen.initial);
    }
}
