#[macro_export]
macro_rules! lens_src_struct {
    ($name: ident, $type: ty) => {
        pub struct $name {
            start: Option<$type>,
            end: $type,
        }
        impl $name {
            pub fn new(end: impl Into<$type>) -> Self {
                Self {
                    start: None,
                    end: end.into(),
                }
            }

            pub fn absolute(start: impl Into<$type>, end: impl Into<$type>) -> Self {
                Self {
                    start: Some(start.into()),
                    end: end.into(),
                }
            }

            $crate::lens_src_duration_fn!($name);
        }
    };
}

#[macro_export]
macro_rules! lens_src_relative {
    ($name: ident, $type: ty) => {
        pub struct $name(pub $type);
        impl $name {
            pub fn new(value: impl Into<$type>) -> Self {
                Self(value.into())
            }

            $crate::lens_src_duration_fn!($name);
        }
    };
}

#[macro_export]
macro_rules! lens_src_duration_fn {
    ($name: ident) => {
        pub fn duration(
            self,
            duration: std::time::Duration,
        ) -> $crate::tween_builder::TweenBuilder {
            $crate::tween_builder::TweenBuilder::new(self, duration)
        }
    };
}

#[macro_export]
macro_rules! impl_lens_src {
    ($component_type_path: path, $lens_src_type_path: path, $lens_type_path: path, $field: tt,) => {
        impl $crate::lens_src::LensSrcToLens for $lens_src_type_path {
            type Component = $component_type_path;
            type Lens = $lens_type_path;

            fn lens(&self, component: &Self::Component) -> Self::Lens {
                Self::Lens {
                    start: self.start.unwrap_or(component.$field),
                    end: self.end,
                }
            }
        }
    };
    ($component_type_path: path, $lens_src_type_path: path, $lens_type_path: path,) => {
        impl_lens_src!(
            $component_type_path,
            $lens_src_type_path,
            $lens_type_path,
            0,
        );
    };
}
