pub mod status_structs {
    pub struct Sleep {
        pub start_time: String,
        pub position: String,
    }

    impl Sleep {
        pub fn new(start_time: String, position: String) -> Sleep {
            Sleep {
                start_time,
                position,
            }
        }
    }

    pub struct Eat {
        pub food: String,
        pub position: String,
    }

    impl Eat {
        pub fn new(food: String, position: String) -> Eat {
            Eat {
                food,
                position,
            }
        }
    }

    pub struct PlayBasketball {
        pub teammate: Vec<String>,
        pub position: String,
    }

    impl PlayBasketball {
        pub fn new(teammate: Vec<String>, position: String) -> PlayBasketball {
            PlayBasketball {
                teammate,
                position,
            }
        }
    }

    pub struct DaoGuan {
        pub video: String,
    }

    impl DaoGuan {
        pub fn new(video: String) -> DaoGuan {
            DaoGuan {
                video,
            }
        }
    }
}
