#[cfg(feature = "HiMdSolidArrowDownOnSquareStack")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowDownOnSquareStack")]
///This icon requires the feature `HiMdSolidArrowDownOnSquareStack` to be enabled.
#[component]
pub fn ArrowDownOnSquareStack(
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
        "M8.00001 1C8.41422 1 8.75001 1.33579 8.75001 1.75V6H7.25001V1.75C7.25001 1.33579 7.58579 1 8.00001 1ZM7.25001 6V9.29553L6.30748 8.24828C6.03038 7.94039 5.55617 7.91543 5.24828 8.19253C4.9404 8.46962 4.91544 8.94384 5.19254 9.25172L7.44254 11.7517C7.58477 11.9098 7.78739 12 8.00001 12C8.21262 12 8.41525 11.9098 8.55748 11.7517L10.8075 9.25172C11.0846 8.94384 11.0596 8.46962 10.7517 8.19253C10.4438 7.91543 9.96963 7.94039 9.69254 8.24828L8.75001 9.29553V6H10.75C11.9926 6 13 7.00736 13 8.25V12.75C13 13.9926 11.9926 15 10.75 15H5.25C4.00736 15 3 13.9926 3 12.75V8.25C3 7.00736 4.00736 6 5.25 6H7.25001ZM7 16.75V16.5H10.75C12.8211 16.5 14.5 14.8211 14.5 12.75V10H14.75C15.9926 10 17 11.0074 17 12.25V16.75C17 17.9926 15.9926 19 14.75 19H9.25C8.00736 19 7 17.9926 7 16.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
