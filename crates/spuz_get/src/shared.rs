#[macro_export]
macro_rules! spawn_macro {
  ($($tt:tt)*) => {
    ::tokio::spawn(async move { $($tt)* })
  };
}

#[macro_export]
macro_rules! result_async_macro {
  ($($tt:tt)*) => {{
    let result: ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = $crate::Result<()>> + Send + Sync>> = ::std::boxed::Box::pin(async move { $($tt)* });
	  result
  }};
}

#[macro_export]
macro_rules! loop_select_macro {
  ($($tt:tt)*) => {
		loop {
			::tokio::select!($($tt)*)
		}
  };
}

pub use spawn_macro as spawn;
pub use result_async_macro as result_async;
pub use loop_select_macro as loop_select;
