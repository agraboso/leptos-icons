#[cfg(feature = "HiMdSolidCloudArrowUp")]
use leptos::*;
#[cfg(feature = "HiMdSolidCloudArrowUp")]
///This icon requires the feature `HiMdSolidCloudArrowUp` to be enabled.
#[component]
pub fn CloudArrowUp(
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
        "M5.5 17C3.01472 17 1 14.9853 1 12.5C1 10.5184 2.28084 8.83595 4.05979 8.2354C4.02046 7.9961 4 7.75044 4 7.5C4 5.01472 6.01472 3 8.5 3C10.1404 3 11.5758 3.87771 12.3621 5.18913C12.7189 5.06655 13.1017 5 13.5 5C15.433 5 17 6.567 17 8.5C17 8.83334 16.9534 9.15579 16.8664 9.4612C18.1353 10.1318 19 11.4649 19 13C19 15.2091 17.2091 17 15 17H5.5ZM9.25 14.25C9.25 14.6642 9.58579 15 10 15C10.4142 15 10.75 14.6642 10.75 14.25V9.6599L12.7004 11.7603C12.9823 12.0639 13.4568 12.0814 13.7603 11.7996C14.0639 11.5177 14.0814 11.0432 13.7996 10.7397L10.5496 7.23966C10.4077 7.08684 10.2086 7 10 7C9.79145 7 9.59231 7.08684 9.45041 7.23966L6.20041 10.7397C5.91855 11.0432 5.93613 11.5177 6.23966 11.7996C6.54319 12.0814 7.01774 12.0639 7.2996 11.7603L9.25 9.6599V14.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
