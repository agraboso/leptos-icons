#[cfg(feature = "HiMdSolidClipboardDocumentCheck")]
use leptos::*;
#[cfg(feature = "HiMdSolidClipboardDocumentCheck")]
///This icon requires the feature `HiMdSolidClipboardDocumentCheck` to be enabled.
#[component]
pub fn ClipboardDocumentCheck(
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
        "M17.9999 5.25C17.9999 4.08761 17.1185 3.1311 15.9875 3.0124C15.8688 1.88145 14.9123 1 13.7499 1H12.2499C11.0875 1 10.131 1.88145 10.0123 3.0124C9.13721 3.10424 8.4115 3.69769 8.12793 4.5H10.9999C12.3806 4.5 13.4999 5.61929 13.4999 7V14H15.7499C16.9925 14 17.9999 12.9926 17.9999 11.75V5.25ZM12.2499 2.5C11.8357 2.5 11.4999 2.83579 11.4999 3.25V3.5H14.4999V3.25C14.4999 2.83579 14.1641 2.5 13.7499 2.5H12.2499Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M3 6C2.44772 6 2 6.44771 2 7V17C2 17.5523 2.44772 18 3 18H11C11.5523 18 12 17.5523 12 17V7C12 6.44772 11.5523 6 11 6H3ZM9.87404 10.166C10.1038 9.82138 10.0107 9.35573 9.66603 9.12596C9.32138 8.8962 8.85573 8.98933 8.62596 9.33397L6.13343 13.0728L5.28033 12.2197C4.98744 11.9268 4.51256 11.9268 4.21967 12.2197C3.92678 12.5126 3.92678 12.9874 4.21967 13.2803L5.71967 14.7803C5.87855 14.9392 6.1003 15.0185 6.3239 14.9963C6.5475 14.9742 6.7494 14.853 6.87404 14.666L9.87404 10.166Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
