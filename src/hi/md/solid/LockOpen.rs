#[cfg(feature = "HiMdSolidLockOpen")]
use leptos::*;
#[cfg(feature = "HiMdSolidLockOpen")]
///This icon requires the feature `HiMdSolidLockOpen` to be enabled.
#[component]
pub fn LockOpen(
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
        "M14.5 1C12.0147 1 10 3.01472 10 5.5V9H3C1.89543 9 1 9.89543 1 11V17C1 18.1046 1.89543 19 3 19H13C14.1046 19 15 18.1046 15 17V11C15 9.89543 14.1046 9 13 9H11.5V5.5C11.5 3.84315 12.8431 2.5 14.5 2.5C16.1569 2.5 17.5 3.84315 17.5 5.5V8.25C17.5 8.66421 17.8358 9 18.25 9C18.6642 9 19 8.66421 19 8.25V5.5C19 3.01472 16.9853 1 14.5 1Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
