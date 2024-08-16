mod status_struct;

pub mod status_enum {
    use super::super::mantra::mantra_constents;
    use super::status_struct::status_structs::Sleep as Sleep;
    use super::status_struct::status_structs::Eat as Eat;
    use super::status_struct::status_structs::PlayBasketball as PlayBasketball;
    use super::status_struct::status_structs::DaoGuan as DaoGuan;


    pub enum Status {
        Sleeping(Sleep),
        Eating(Eat),
        PlayingBasketball(PlayBasketball),
        DaoGuan(DaoGuan),
    }

    impl Status {
        pub fn start_sleeping() -> Option<Status> {
            let start_time = "22:00".to_string();
            let position = "bed".to_string();
            println!("start_sleeping from {} at {}", start_time, position);
            Some(Status::Sleeping(Sleep::new(start_time, position)))
        }

        pub fn start_eating() -> Option<Status> {
            let food = "noodles".to_string();
            let position = "all_in_one canteen".to_string();
            println!("start_eating {} at {}", food, position);
            Some(Status::Eating(Eat::new(food, position)))
        }

        pub fn start_playing_basketball() -> Option<Status> {
            let teammate = vec!["dqr".to_string(), "zjq".to_string()];
            let position = "court".to_string();
            println!("start_playing_basketball with {:?} at {}", teammate, position);
            Some(Status::PlayingBasketball(PlayBasketball::new(teammate, position)))
        }

        pub fn start_dao_guan() -> Option<Status> {
            let video = "wq's video".to_string();
            println!("start_dao_guan {}", video);
            Some(Status::DaoGuan(DaoGuan::new(video)))
        }

        pub fn start_life() -> Option<Status> {
            println!("{}-{}", mantra_constents::SELAN, mantra_constents::NAME);
            None
        }
    }
}