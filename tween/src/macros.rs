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
