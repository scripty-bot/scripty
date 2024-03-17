use std::fmt::Display;

use num::Num;
use thousands::Separable;

pub fn separate_num<T: Num + Display>(num: T) -> String {
	num.separate_with_commas()
}
