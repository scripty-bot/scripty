use num::Num;
use std::fmt::Display;
use thousands::Separable;

pub fn separate_num<T: Num>(num: T) -> String
where
    T: Display,
{
    num.separate_with_commas()
}
