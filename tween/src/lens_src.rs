use bevy::prelude::*;
use bevy_tweening::*;

use crate::{lens_src_relative, lens_src_struct};

pub trait LensSrcToLens {
    type Component: Component;
    type Lens: Lens<Self::Component> + Send + Sync;
    fn lens(&self, component: &Self::Component) -> Self::Lens;
}

lens_src_struct!(TransformPositionLensSrc, Vec2);
impl LensSrcToLens for TransformPositionLensSrc {
    type Component = Transform;
    type Lens = bevy_tweening::lens::TransformPositionLens;

    fn lens(&self, component: &Transform) -> Self::Lens {
        Self::Lens {
            start: self
                .start
                .map(|s| s.extend(component.translation.z))
                .unwrap_or(component.translation),
            end: self.end.extend(component.translation.z),
        }
    }
}

lens_src_relative!(TransformPositionMoveBy, Vec2);
impl LensSrcToLens for TransformPositionMoveBy {
    type Component = Transform;
    type Lens = bevy_tweening::lens::TransformPositionLens;

    fn lens(&self, component: &Transform) -> Self::Lens {
        Self::Lens {
            start: component.translation,
            end: component.translation + self.0.extend(0.),
        }
    }
}

lens_src_struct!(UiPositionLensSrc, UiRect);
impl LensSrcToLens for UiPositionLensSrc {
    type Component = Node;
    type Lens = bevy_tweening::lens::UiPositionLens;

    fn lens(&self, component: &Node) -> Self::Lens {
        Self::Lens {
            start: self.start.unwrap_or(UiRect {
                left: component.left,
                right: component.right,
                top: component.top,
                bottom: component.bottom,
            }),
            end: self.end,
        }
    }
}

lens_src_struct!(TransformRotationDegreesLensSrc, f32);
impl LensSrcToLens for TransformRotationDegreesLensSrc {
    type Component = Transform;
    type Lens = bevy_tweening::lens::TransformRotateZLens;

    fn lens(&self, component: &Transform) -> Self::Lens {
        Self::Lens {
            start: self
                .start
                .unwrap_or_else(|| component.rotation.to_euler(EulerRot::XYZ).2),
            end: self.end.to_radians(),
        }
    }
}

lens_src_relative!(TransformRotationByDegrees, f32);
impl LensSrcToLens for TransformRotationByDegrees {
    type Component = Transform;
    type Lens = bevy_tweening::lens::TransformRotateZLens;

    fn lens(&self, component: &Transform) -> Self::Lens {
        let start = component.rotation.to_euler(EulerRot::XYZ).2;
        Self::Lens {
            start,
            end: start + self.0.to_radians(),
        }
    }
}

lens_src_struct!(TransformScaleLensSrc, Vec2);
impl LensSrcToLens for TransformScaleLensSrc {
    type Component = Transform;
    type Lens = bevy_tweening::lens::TransformScaleLens;

    fn lens(&self, component: &Transform) -> Self::Lens {
        Self::Lens {
            start: self
                .start
                .map(|s| s.extend(component.scale.z))
                .unwrap_or(component.scale),
            end: self.end.extend(component.scale.z),
        }
    }
}

lens_src_struct!(UiTransformScaleLensSrc, Vec2);
impl LensSrcToLens for UiTransformScaleLensSrc {
    type Component = UiTransform;
    type Lens = bevy_tweening::lens::UiTransformScaleLens;

    fn lens(&self, component: &UiTransform) -> Self::Lens {
        Self::Lens {
            start: self.start.unwrap_or(component.scale),
            end: self.end,
        }
    }
}

lens_src_struct!(SpriteAlphaLensSrc, f32);
impl LensSrcToLens for SpriteAlphaLensSrc {
    type Component = Sprite;
    type Lens = bevy_tweening::lens::SpriteColorLens;

    fn lens(&self, component: &Sprite) -> Self::Lens {
        Self::Lens {
            start: component
                .color
                .with_alpha(self.start.unwrap_or(component.color.alpha())),
            end: component.color.with_alpha(self.end),
        }
    }
}

lens_src_struct!(SpriteColorLensSrc, Color);
impl LensSrcToLens for SpriteColorLensSrc {
    type Component = Sprite;
    type Lens = bevy_tweening::lens::SpriteColorLens;

    fn lens(&self, component: &Sprite) -> Self::Lens {
        Self::Lens {
            start: self.start.unwrap_or(component.color),
            end: self.end,
        }
    }
}

lens_src_struct!(TextAlphaLensSrc, f32);
impl LensSrcToLens for TextAlphaLensSrc {
    type Component = TextColor;
    type Lens = bevy_tweening::lens::TextColorLens;

    fn lens(&self, component: &Self::Component) -> Self::Lens {
        Self::Lens {
            start: component
                .0
                .with_alpha(self.start.unwrap_or(component.0.alpha())),
            end: component.0.with_alpha(self.end),
        }
    }
}

lens_src_struct!(TextColorLensSrc, Color);
impl LensSrcToLens for TextColorLensSrc {
    type Component = TextColor;
    type Lens = bevy_tweening::lens::TextColorLens;

    fn lens(&self, component: &Self::Component) -> Self::Lens {
        Self::Lens {
            start: self.start.unwrap_or(component.0),
            end: self.end,
        }
    }
}

lens_src_struct!(UiBgAlphaLensSrc, f32);
impl LensSrcToLens for UiBgAlphaLensSrc {
    type Component = BackgroundColor;
    type Lens = bevy_tweening::lens::UiBackgroundColorLens;

    fn lens(&self, component: &Self::Component) -> Self::Lens {
        Self::Lens {
            start: component
                .0
                .with_alpha(self.start.unwrap_or(component.0.alpha())),
            end: component.0.with_alpha(self.end),
        }
    }
}

lens_src_struct!(UiBgColorLensSrc, Color);
impl LensSrcToLens for UiBgColorLensSrc {
    type Component = BackgroundColor;
    type Lens = bevy_tweening::lens::UiBackgroundColorLens;

    fn lens(&self, component: &Self::Component) -> Self::Lens {
        Self::Lens {
            start: self.start.unwrap_or(component.0),
            end: self.end,
        }
    }
}

// #[derive(Component)]
// #[component(storage = "SparseSet")]
// pub struct TweenFactor<T: Send + Sync + 'static> {
//     timer: Timer,
//     delay: Option<Timer>,
//     ease: EaseFunction,
//     _phantom: PhantomData<T>,
// }

// impl<T: Send + Sync> TweenFactor<T> {
//     pub fn new(duration_ms: u64, ease: EaseFunction) -> Self {
//         Self {
//             timer: Timer::new(Duration::from_millis(duration_ms), TimerMode::Once),
//             delay: None,
//             ease,
//             _phantom: default(),
//         }
//     }

//     pub fn with_delay(mut self, delay_ms: u64) -> Self {
//         self.delay = Some(Timer::new(Duration::from_millis(delay_ms), TimerMode::Once));
//         self
//     }

//     pub fn factor(&self) -> f32 {
//         self.timer.fraction().calc(self.ease)
//     }
// }

// pub fn tween_factor<T: Send + Sync>(mut factor_q: Query<&mut TweenFactor<T>>, time: Res<Time>) {
//     for mut factor in &mut factor_q {
//         if let Some(delay) = factor.delay.as_mut() {
//             delay.tick(time.delta());
//             if delay.just_finished() {
//                 factor.delay.take();
//             }
//         } else if !factor.timer.finished() {
//             factor.timer.tick(time.delta());
//         }
//     }
// }
