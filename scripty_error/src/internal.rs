/// Syntax:
///
/// ```rust
/// error_fn_impl!(
///     ErrorType, ErrorEnum;
///     function_name, EnumVariantName, AttachedType;
///     function_name_attached, EnumVariant;
///     function_name_no_from_impl, EnumVariantFrom, AttachedType2 nofrom;
/// )
/// ```
macro_rules! error_fn_impl {
	($ty: ident, $en: ident; $name: ident, $enum_variant: ident) => {
		impl $ty {
			pub fn $name() -> $ty {
				$ty {
					bt:    Backtrace::new_unresolved(),
					error: $en::$enum_variant,
				}
			}
		}
	};

	($ty: ident, $en: ident; $name: ident, $enum_variant: ident, $error_type: ty, nofrom) => {
		impl $ty {
			pub fn $name(error: $error_type) -> $ty {
				$ty {
					bt:    Backtrace::new_unresolved(),
					error: $en::$enum_variant(error),
				}
			}
		}
	};

	($ty: ident, $en: ident; $name: ident, $enum_variant: ident, $error_type: ty) => {
		impl $ty {
			pub fn $name(error: $error_type) -> $ty {
				$ty {
					bt:    Backtrace::new_unresolved(),
					error: $en::$enum_variant(error),
				}
			}
		}

		impl From<$error_type> for $ty {
			fn from(error: $error_type) -> $ty {
				$ty::$name(error)
			}
		}
	};

	($ty: ident, $en: ident; $($name: ident, $enum_variant: ident$(, $error_type: ty $(, $nf: tt)?)?);* $(;)?) => {
		$(error_fn_impl!($ty, $en; $name, $enum_variant$(, $error_type $(, $nf)?)?);)*
	};
}
pub(super) use error_fn_impl;
