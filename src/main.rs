use eframe::egui;
use serde::Deserialize;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
}

struct HnApp {
    items: Vec<Story>,
}

impl HnApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let client = reqwest::blocking::Client::new();

        let top_ids: Vec<u64> = client
            .get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .send()
            .expect("Failed to fetch top stories")
            .json()
            .expect("Failed to parse story IDs");

        let mut items: Vec<Story> = Vec::new();

        for id in top_ids.iter().take(10) {
            let url =
                format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

            let story: Story = client
                .get(&url)
                .send()
                .expect("Failed to fetch story")
                .json()
                .expect("Failed to parse story");

            items.push(story);
        }

        Self { items }
    }
}

impl eframe::App for HnApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Top 10 Hacker News Stories");
            ui.add_space(20.0);

            for item in &self.items {
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.hyperlink_to(
                        &item.title,
                        item.url
                            .as_deref()
                            .unwrap_or("https://news.ycombinator.com"),
                    );
ui.label(format!(
            "⭐ {} points | 👤 {}",
            item.score,
            item.by
        ));                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "HN List",
        native_options,
        Box::new(|cc| Ok(Box::new(HnApp::new(cc)))),
    )
}
