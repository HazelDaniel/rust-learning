pub mod utils {
    pub struct Person {
        pub job: Option<Job>,
    }

    #[derive(Clone, Copy)]
    pub struct Job {
        pub phone_number: Option<PhoneNumber>,
    }

    #[derive(Clone, Copy)]
    pub struct PhoneNumber {
        pub area_code: Option<i32>,
        pub number: u32,
    }

    #[derive(Debug)]
    pub enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    pub struct Peeled(pub Food);

    #[derive(Debug)]
    pub struct Chopped(pub Food);

    #[derive(Debug)]
    pub struct Cooked(pub Food);

    impl Person {
        pub fn work_phone_area_code(&self) -> Option<i32> {
            self.job?.phone_number?.area_code
        }
    }
}
