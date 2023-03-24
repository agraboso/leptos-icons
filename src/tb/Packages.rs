#[cfg(feature = "TbPackages")]
use leptos::*;
#[cfg(feature = "TbPackages")]
///This icon requires the feature `TbPackages` to be enabled.
#[component]
pub fn Packages(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-packages"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 16.5l-5 -3l5 -3l5 3v5.5l-5 3z" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M2 13.5v5.5l5 3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 16.545l5 -3.03" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 16.5l-5 -3l5 -3l5 3v5.5l-5 3z" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M12 19l5 3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 16.5l5 -3" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 13.5v-5.5l-5 -3l5 -3l5 3v5.5" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M7 5.03v5.455" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 8l5 -3" /> < title > { title } < / title > < / svg >
    }
}
