#[cfg(feature = "RiWeatherLineHaze2")]
use leptos::*;
#[cfg(feature = "RiWeatherLineHaze2")]
///This icon requires the feature `RiWeatherLineHaze2` to be enabled.
#[component]
pub fn Haze2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path fill - rule = "nonzero" d =
        "M12 19a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm7.5 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm-15 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zM17 7a5 5 0 0 1 0 10c-1.844 0-3.51-1.04-5-3.122C10.51 15.96 8.844 17 7 17A5 5 0 0 1 7 7c1.844 0 3.51 1.04 5 3.122C13.49 8.04 15.156 7 17 7zM7 9a3 3 0 0 0 0 6c1.254 0 2.51-.875 3.759-2.854l.089-.147-.09-.145c-1.197-1.896-2.4-2.78-3.601-2.85L7 9zm10 0c-1.254 0-2.51.875-3.759 2.854l-.09.146.09.146c1.198 1.896 2.4 2.78 3.602 2.85L17 15a3 3 0 0 0 0-6zm-5-7a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zM4.5 2a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3zm15 0a1.5 1.5 0 1 1 0 3 1.5 1.5 0 0 1 0-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
