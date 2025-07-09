use std::time::Duration;

use freya::prelude::*;

#[component]
pub fn Timer() -> Element {
    let mut elapsed = use_signal(|| 0.0);
    let mut duration = use_signal(|| 30.0);
    use_hook(move || {
        spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                if *elapsed.read() < *duration.read() {
                    elapsed.set(elapsed() + 0.1);
                }
            }
        });
    });


    rsx!(
        rect {
            direction: "vertical",
            width: "100%",
            content: "flex",
            padding: "24",
            spacing: "16",


            // ProgressBar.progress is an f32.
            // Slider value is an f64...
            // WHYYYYYYY?!?!?!?!?!
            rect {
                direction: "horizontal",
                spacing: "8",
                label { "Elapsed Time: "}
                ProgressBar {
                    show_progress: true,
                    progress: (*elapsed.read() / *duration.read()) * 100. ,
                }
            }
            rect {
                direction: "horizontal",
                label { "{elapsed}s"}
            }
            rect {
                direction: "horizontal",
                spacing: "8",
                label { "Duration: "}
                Slider {
                    size: "80%",
                    value: (*duration.read() as f64) / 60.0 * 100.0,
                    direction: "horizontal",
                    onmoved: move |v| {
                        duration.set((v as f32) / 100.0 * 60.0);
                    }
                }
            }
            Button {
                onpress: move |_| {
                    duration.set(30.0);
                    elapsed.set(0.0);
                },
                label {"Reset"}
            }
        }
    )
}
