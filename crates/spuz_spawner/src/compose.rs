use crate::{LaunchMod, Layer};

macro_rules! impl_compose {
  ($($ty:ident),*) => {
	  #[allow(non_snake_case)]
    impl<$($ty),*> Layer for ($($ty),*) where $($ty: Layer),*{
			fn apply(self, launch_mod: &mut LaunchMod) {
				let ($($ty),*) = self;
				$($ty.apply(launch_mod);)*
			}
		}
  };
}

impl_compose!(T0, T1);
impl_compose!(T0, T1, T2);
impl_compose!(T0, T1, T2, T3);
impl_compose!(T0, T1, T2, T3, T4);
impl_compose!(T0, T1, T2, T3, T4, T5);
impl_compose!(T0, T1, T2, T3, T4, T5, T6);
