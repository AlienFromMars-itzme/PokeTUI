pub fn hp_bar(current: i32, max: i32, width: usize) -> String {
    let fill = ((current.max(0) as f32 / max.max(1) as f32) * width as f32) as usize;
    format!(
        "{}{}",
        "#".repeat(fill),
        "-".repeat(width.saturating_sub(fill))
    )
}
