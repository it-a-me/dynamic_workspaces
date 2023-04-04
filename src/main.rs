use anyhow::Result;
use hyprland::shared::{HyprData, HyprDataActive};
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut outputs = HashMap::with_capacity(10);
    for i in 1..10 {
        outputs.insert(i, Output::new(i));
    }
    let windows = hyprland::data::Workspaces::get()?;
    let active_id = hyprland::data::Workspace::get_active()?.id;
    for window in windows {
        outputs.insert(
            window.id,
            Output {
                id: window.id,
                has_windows: true,
                is_active: window.id == active_id,
            },
        );
    }
    let mut outputs = outputs.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    outputs.sort_by_key(|o| o.id);
    println!("{}", serde_json::to_string(&outputs)?);
    Ok(())
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Output {
    id: i32,
    has_windows: bool,
    is_active: bool,
}
impl Output {
    fn new(id: i32) -> Self {
        Self {
            id,
            has_windows: false,
            is_active: false,
        }
    }
}
