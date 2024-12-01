macro_rules! day_mod {
    ($day: literal) => {
        paste::paste! {
            mod [< day_ $day _1>];
            pub use [< day_ $day _1>]::[< Day $day Part1 >];
            mod [< day_ $day _2>];
            pub use [< day_ $day _2>]::[< Day $day Part2 >];
        }
    };
}

macro_rules! mod_days {
    ($($days:literal),+) => {
        $(day_mod!($days);)+
    };
}

mod_days!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
);
