#[cfg(feature = "HiMdSolidCodeBracketSquare")]
use leptos::*;
#[cfg(feature = "HiMdSolidCodeBracketSquare")]
///This icon requires the feature `HiMdSolidCodeBracketSquare` to be enabled.
#[component]
pub fn CodeBracketSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M4.25 2C3.00736 2 2 3.00736 2 4.25V15.75C2 16.9926 3.00736 18 4.25 18H15.75C16.9926 18 18 16.9926 18 15.75V4.25C18 3.00736 16.9926 2 15.75 2H4.25ZM8.28033 8.28033C8.57322 7.98744 8.57322 7.51256 8.28033 7.21967C7.98744 6.92678 7.51256 6.92678 7.21967 7.21967L4.96967 9.46967C4.67678 9.76256 4.67678 10.2374 4.96967 10.5303L7.21967 12.7803C7.51256 13.0732 7.98744 13.0732 8.28033 12.7803C8.57322 12.4874 8.57322 12.0126 8.28033 11.7197L6.56066 10L8.28033 8.28033ZM12.7803 7.21967C12.4874 6.92678 12.0126 6.92678 11.7197 7.21967C11.4268 7.51256 11.4268 7.98744 11.7197 8.28033L13.4393 10L11.7197 11.7197C11.4268 12.0126 11.4268 12.4874 11.7197 12.7803C12.0126 13.0732 12.4874 13.0732 12.7803 12.7803L15.0303 10.5303C15.3232 10.2374 15.3232 9.76256 15.0303 9.46967L12.7803 7.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
