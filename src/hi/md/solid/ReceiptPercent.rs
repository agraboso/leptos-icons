#[cfg(feature = "HiMdSolidReceiptPercent")]
use leptos::*;
#[cfg(feature = "HiMdSolidReceiptPercent")]
///This icon requires the feature `HiMdSolidReceiptPercent` to be enabled.
#[component]
pub fn ReceiptPercent(
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
        "M4.93005 1.31046C6.5916 1.10551 8.28365 1 10 1C11.7163 1 13.4084 1.10551 15.07 1.31046C16.1942 1.44913 17 2.41374 17 3.51661V18.25C17 18.5078 16.8676 18.7475 16.6494 18.8848C16.4312 19.0221 16.1578 19.0377 15.9255 18.9261L13.125 17.5819L10.3245 18.9261C10.1194 19.0246 9.88061 19.0246 9.67545 18.9261L6.875 17.5819L4.07455 18.9261C3.84215 19.0377 3.56875 19.0221 3.35057 18.8848C3.13239 18.7475 3 18.5078 3 18.25V3.51661C3 2.41374 3.80579 1.44913 4.93005 1.31046ZM13.7803 7.28033C14.0732 6.98744 14.0732 6.51256 13.7803 6.21967C13.4874 5.92678 13.0126 5.92678 12.7197 6.21967L6.21967 12.7197C5.92678 13.0126 5.92678 13.4874 6.21967 13.7803C6.51256 14.0732 6.98744 14.0732 7.28033 13.7803L13.7803 7.28033ZM9 8C9 8.55228 8.55228 9 8 9C7.44772 9 7 8.55228 7 8C7 7.44772 7.44772 7 8 7C8.55228 7 9 7.44772 9 8ZM12 13C12.5523 13 13 12.5523 13 12C13 11.4477 12.5523 11 12 11C11.4477 11 11 11.4477 11 12C11 12.5523 11.4477 13 12 13Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
