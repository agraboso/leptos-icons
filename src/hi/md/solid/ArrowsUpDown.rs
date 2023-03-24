#[cfg(feature = "HiMdSolidArrowsUpDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowsUpDown")]
///This icon requires the feature `HiMdSolidArrowsUpDown` to be enabled.
#[component]
pub fn ArrowsUpDown(
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
        "M2.23966 6.7996C2.5432 7.08145 3.01775 7.06387 3.2996 6.76034L5.25 4.6599L5.25 13.25C5.25 13.6642 5.58579 14 6 14C6.41422 14 6.75 13.6642 6.75 13.25V4.6599L8.70041 6.76034C8.98226 7.06387 9.45681 7.08145 9.76034 6.79959C10.0639 6.51774 10.0815 6.04319 9.7996 5.73966L6.5496 2.23966C6.40769 2.08684 6.20855 2 6 2C5.79145 2 5.59232 2.08684 5.45041 2.23966L2.20041 5.73966C1.91856 6.04319 1.93613 6.51774 2.23966 6.7996ZM10.2397 13.2004C9.93613 13.4823 9.91856 13.9568 10.2004 14.2603L13.4504 17.7603C13.5923 17.9132 13.7915 18 14 18C14.2086 18 14.4077 17.9132 14.5496 17.7603L17.7996 14.2603C18.0815 13.9568 18.0639 13.4823 17.7603 13.2004C17.4568 12.9186 16.9823 12.9361 16.7004 13.2397L14.75 15.3401V6.75C14.75 6.33579 14.4142 6 14 6C13.5858 6 13.25 6.33579 13.25 6.75V15.3401L11.2996 13.2397C11.0177 12.9361 10.5432 12.9186 10.2397 13.2004Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
