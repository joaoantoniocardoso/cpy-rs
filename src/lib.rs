#[macro_export]
macro_rules! export_cpy {
    (mod $module_name:ident { $($item:tt)* }) => {
        export_cpy!(@inner $($item)*);

        #[cfg(feature = "python")]
        #[pymodule]
        fn $module_name(_py: Python, m: &PyModule) -> PyResult<()> {
            export_cpy!(@add_classes m, $($item)*);
            Ok(())
        }
    };
    (@inner) => {};
    (@inner enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        export_cpy!(@enum $name { $($variant,)* });
        export_cpy!(@inner $($rest)*);
    };
    (@inner struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        export_cpy!(@struct $name { $($field : $ftype,)* });
        export_cpy!(@inner $($rest)*);
    };
    (@inner fn $name:ident() -> $ret:ty $body:block $($rest:tt)*) => {
        export_cpy!(@fn $name() -> $ret $body);
        export_cpy!(@inner $($rest)*);
    };
    (@add_classes $m:ident,) => {};
    (@add_classes $m:ident, enum $name:ident { $($variant:ident,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_classes $m, $($rest)*);
    };
    (@add_classes $m:ident, struct $name:ident { $($field:ident : $ftype:ty,)* } $($rest:tt)*) => {
        $m.add_class::<$name>()?;
        export_cpy!(@add_classes $m, $($rest)*);
    };
    (@add_classes $m:ident, fn $name:ident() -> $ret:ty $body:block $($rest:tt)*) => {
        $m.add_wrapped(wrap_pyfunction!($name))?;
        export_cpy!(@add_classes $m, $($rest)*);
    };
    (@enum $name:ident { $($variant:ident,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
        pub enum $name {
            $(
                $variant,
            )*
        }
    };
    (@struct $name:ident { $($field:ident : $ftype:ty,)* }) => {
        #[derive(Clone, Debug)]
        #[repr(C)]
        #[cfg_attr(feature = "python", pyo3::prelude::pyclass(get_all, set_all))]
        pub struct $name {
            $(
                pub $field: $ftype,
            )*
        }
    };
    (@fn $name:ident() -> $ret:ty $body:block) => {
        #[no_mangle]
        #[cfg_attr(feature = "python", pyfunction)]
        pub extern "C" fn $name() -> $ret {
            $body
        }
    };
}
