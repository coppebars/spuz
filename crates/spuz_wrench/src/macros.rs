#[macro_export]
macro_rules! set_vars {
	($target:expr, $name:literal, $value:expr) => {
		for arg in &mut *$target {
		    *arg = arg.replace(concat!("${", $name, "}"), $value);
	    }
	};
    ($target:expr, { $($name:literal => $value:expr,)* }) => {
	    for arg in &mut *$target {
		    $(*arg = arg.replace(concat!("${", $name, "}"), $value);)*
	    }
    };
}
