pub mod li_guo_qing {
    use std::thread::sleep;
    use std::time::Duration;
    use super::super::status::status_enum::Status as Status;
    use super::super::mantra::mantra_constents as Mantra;

    pub struct LiGuoQing {
        pub status: Option<Status>
    }

    impl Default for LiGuoQing {
        fn default() -> Self {
            Self::new()
        }
    }

    impl LiGuoQing {

        pub fn new() -> LiGuoQing {
            LiGuoQing {
                status: Status::start_life(),
            }
        }

        pub fn live_a_day(&mut self) {
            loop {
                match &self.status {
                    None => {
                        println!("刚起床，{}", Mantra::WHATEVER);
                        self.status = Status::start_eating();
                    }
                    Some(Status::Sleeping(_)) => {
                        sleep(Duration::from_secs(2));
                        println!("刚起床，{}", Mantra::WHATEVER);
                        self.status = Status::start_eating();
                    },
                    Some(Status::Eating(_)) => {
                        sleep(Duration::from_secs(1));
                        self.status = Status::start_playing_basketball();
                    },
                    Some(Status::PlayingBasketball(_)) => {
                        sleep(Duration::from_secs(2));
                        self.status = Status::start_dao_guan();
                    },
                    Some(Status::DaoGuan(_)) => {
                        sleep(Duration::from_micros(200));
                        self.status = Status::start_sleeping();
                        println!("开始睡觉 : {}", Mantra::OBITO);
                    } 
                }
            }
        }
    }
}

