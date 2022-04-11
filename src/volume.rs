use cpp::cpp;

cpp!{{
    #include <iostream>    
    #include "lib/SetVolume.h"
}}


pub fn mute(m: bool) {
    unsafe {
        cpp!([m as "bool"] -> u32 as "int32_t" {

            set_mute(m);
            return 0;
        })
    };
}

pub fn change_volume(v: f32) {
    unsafe {
        cpp!([v as "float"] {

            set_volume(v);

        })
    };
}

pub fn get_volume() -> f32 {
    let r = unsafe {
        cpp!([] -> f32 as "float" {

            return get_volume();

        })
    };
    return r;
}
