#[cfg(feature = "TbScaleOff")]
use leptos::*;
#[cfg(feature = "TbScaleOff")]
///This icon requires the feature `TbScaleOff` to be enabled.
#[component]
pub fn ScaleOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-scale-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 20h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.452 5.425l2.548 -.425l6 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v5m0 4v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 12l-3 -6l-3 6a3 3 0 0 0 6 0" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M18.873 14.871a3 3 0 0 0 2.127 -2.871l-3 -6l-2.677 5.355" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
