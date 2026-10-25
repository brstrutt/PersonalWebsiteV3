use web_sys::{js_sys::Promise, window};

pub async fn yield_to_browser() {
    let wait_for_browser = Promise::new(&mut |resolve, _| {
        window()
            .expect("Failed to get window when attempting to yield to browser")
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 0)
            .expect("Failed to set timeout when attempting to yield to browser");
    });

    // We don't care about the result of the timeout function, we just want the function to run as this yields to the browser
    #[allow(unused_must_use)]
    wait_for_browser.await;
}
