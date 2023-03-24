#[cfg(feature = "TbAlarmSnooze")]
use leptos::*;
#[cfg(feature = "TbAlarmSnooze")]
///This icon requires the feature `TbAlarmSnooze` to be enabled.
#[component]
pub fn AlarmSnooze(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-alarm-snooze" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 13m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 11h4l-4 4h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 4l-2.75 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 4l2.75 2" /> < title > { title } < / title
        > < / svg >
    }
}
