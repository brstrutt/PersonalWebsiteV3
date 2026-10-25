use leptos::web_sys;

pub struct LoggerWithDuration {
    last_call_time_ms: f64,
}

impl LoggerWithDuration {
    pub fn new() -> LoggerWithDuration {
        LoggerWithDuration {
            last_call_time_ms: Self::now_in_ms(),
        }
    }

    pub fn log_time(&mut self, msg: &str) {
        let current_time = Self::now_in_ms();
        log::info!(
            "Time taken: {}ms, Msg: {}",
            current_time - self.last_call_time_ms,
            msg
        );
        self.last_call_time_ms = current_time;
    }

    fn now_in_ms() -> f64 {
        web_sys::window()
            .expect("no global `window` exists")
            .performance()
            .expect("Couldnt get the window performance object")
            .now()
    }
}
