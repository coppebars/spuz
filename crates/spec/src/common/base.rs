use cfg_if::cfg_if;

pub type Str = Box<str>;
pub type Size = u32;

cfg_if! {
	if #[cfg(feature = "url")] {
		pub type Url = url::Url;
	} else {
		pub type Url = Box<str>;
	}
}
