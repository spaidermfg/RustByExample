pub struct Person {
    pub job: Option<Job>,
}

#[derive(Clone, Copy)]
pub struct Job {
    pub phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
pub struct PhoneNumber {
    pub area_code: Option<u8>,
    pub number: u32,
}

impl Person {
    pub fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}


