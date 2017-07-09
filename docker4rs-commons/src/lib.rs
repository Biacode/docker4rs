#![feature(core_intrinsics)]

use std::intrinsics::type_name;

pub fn type_info<T>(_: T) {
    println!("{:?}", unsafe { type_name::<T>() });
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
