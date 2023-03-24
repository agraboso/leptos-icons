#[cfg(feature = "TbWashDry2")]
use leptos::*;
#[cfg(feature = "TbWashDry2")]
///This icon requires the feature `TbWashDry2` to be enabled.
#[component]
pub fn WashDry2(
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
        "icon icon-tabler icon-tabler-wash-dry-2" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 3m0 3a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-6 0a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 12h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 12h.01" /> < title > { title } < / title >
        < / svg >
    }
}
