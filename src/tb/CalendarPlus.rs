#[cfg(feature = "TbCalendarPlus")]
use leptos::*;
#[cfg(feature = "TbCalendarPlus")]
///This icon requires the feature `TbCalendarPlus` to be enabled.
#[component]
pub fn CalendarPlus(
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
        "icon icon-tabler icon-tabler-calendar-plus" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 5m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M16 3l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 3l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 11l16 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 16l4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 14l0 4" /> < title > { title } < / title >
        < / svg >
    }
}
