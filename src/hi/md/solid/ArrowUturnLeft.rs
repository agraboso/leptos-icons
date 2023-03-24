#[cfg(feature = "HiMdSolidArrowUturnLeft")]
use leptos::*;
#[cfg(feature = "HiMdSolidArrowUturnLeft")]
///This icon requires the feature `HiMdSolidArrowUturnLeft` to be enabled.
#[component]
pub fn ArrowUturnLeft(
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
        "M7.79252 2.23214C8.07852 2.53177 8.06748 3.00651 7.76786 3.29252L3.62192 7.25H13.625C16.5935 7.25 19 9.65647 19 12.625C19 15.5935 16.5935 18 13.625 18H10.75C10.3358 18 10 17.6642 10 17.25C10 16.8358 10.3358 16.5 10.75 16.5H13.625C15.7651 16.5 17.5 14.7651 17.5 12.625C17.5 10.4849 15.7651 8.75 13.625 8.75H3.62192L7.76786 12.7075C8.06748 12.9935 8.07852 13.4682 7.79252 13.7679C7.50651 14.0675 7.03177 14.0785 6.73214 13.7925L1.23214 8.54252C1.08388 8.401 1 8.20496 1 8C1 7.79504 1.08388 7.59901 1.23214 7.45748L6.73214 2.20748C7.03177 1.92148 7.50651 1.93252 7.79252 2.23214Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
