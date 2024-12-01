macro_rules! day_mod {
    ($day: literal) => {
        paste::paste! {
            pub mod [< day_ $day _1>];
            pub mod [< day_ $day _2>];
        }
    };
}

macro_rules! mod_days {
    ($($days:literal),+) => {
        $(day_mod!($days);)+
    };
}

mod_days!(1);
