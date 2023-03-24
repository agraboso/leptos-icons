#[cfg(feature = "TbMoodAnnoyed2")]
use leptos::*;
#[cfg(feature = "TbMoodAnnoyed2")]
///This icon requires the feature `TbMoodAnnoyed2` to be enabled.
#[component]
pub fn MoodAnnoyed2(
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
        "icon icon-tabler icon-tabler-mood-annoyed-2" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 14c-2 0 -3 1 -3.5 2.05" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 9.25c-.5 1 -2.5 1 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 9.25c-.5 1 -2.5 1 -3 0" /> < title > {
        title } < / title > < / svg >
    }
}
