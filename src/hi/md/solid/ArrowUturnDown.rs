#[cfg(feature = "HiMdSolidArrowUturnDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUturnDown")]
///This icon requires the feature `HiMdSolidArrowUturnDown` to be enabled.
#[component]
pub fn ArrowUturnDown(
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
        "M2.23214 12.2075C2.53177 11.9215 3.00651 11.9325 3.29252 12.2321L7.25 16.3781V6.375C7.25 3.40647 9.65647 1 12.625 1C15.5935 1 18 3.40647 18 6.375V9.25C18 9.66421 17.6642 10 17.25 10C16.8358 10 16.5 9.66421 16.5 9.25V6.375C16.5 4.2349 14.7651 2.5 12.625 2.5C10.4849 2.5 8.75 4.2349 8.75 6.375V16.3781L12.7075 12.2321C12.9935 11.9325 13.4682 11.9215 13.7679 12.2075C14.0675 12.4935 14.0785 12.9682 13.7925 13.2679L8.54252 18.7679C8.401 18.9161 8.20496 19 8 19C7.79504 19 7.59901 18.9161 7.45748 18.7679L2.20748 13.2679C1.92148 12.9682 1.93252 12.4935 2.23214 12.2075Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
