use chrono::{DateTime, TimeDelta, Utc};

pub struct TTLCache<T> {
    data: Option<T>,
    last_read_at: Option<DateTime<Utc>>,
    ttl: TimeDelta,
}

impl<T> TTLCache<T> {
    pub fn new(ttl: TimeDelta) -> Self {
        Self {
            data: None,
            last_read_at: None,
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.last_read_at.is_none() || Utc::now() - self.last_read_at.unwrap() > self.ttl
    }

    fn update(&mut self, data: T) {
        self.data = Some(data);
        self.last_read_at = Some(Utc::now());
    }

    pub fn get<'a>(&'a mut self, mut get_origin_data: Box<dyn 'a + FnMut() -> T>) -> &T {
        if self.data.is_none() || self.is_expired() {
            self.update(get_origin_data());
        }
        self.data.as_ref().unwrap()
    }
}
