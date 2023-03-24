#[cfg(feature = "TbStereoGlasses")]
use leptos::*;
#[cfg(feature = "TbStereoGlasses")]
///This icon requires the feature `TbStereoGlasses` to be enabled.
#[component]
pub fn StereoGlasses(
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
        "icon icon-tabler icon-tabler-stereo-glasses" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 3h-2l-3 9" />< path xmlns = "http://www.w3.org/2000/svg" d = "M16 3h2l3 9"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 12v7a1 1 0 0 0 1 1h4.586a1 1 0 0 0 .707 -.293l2 -2a1 1 0 0 1 1.414 0l2 2a1 1 0 0 0 .707 .293h4.586a1 1 0 0 0 1 -1v-7h-18z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 16h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 16h1" /> < title > { title } < / title > <
        / svg >
    }
}
