#[cfg(feature = "IoLogoWindows")]
use leptos::*;
#[cfg(feature = "IoLogoWindows")]
///This icon requires the feature `IoLogoWindows` to be enabled.
#[component]
pub fn LogoWindows(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d = "M480,265H232V444l248,36V265Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M216,265H32V415l184,26.7V265Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M480,32,232,67.4V249H480V32Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M216,69.7,32,96V249H216V69.7Z" /> < title > {
        title } < / title > < / svg >
    }
}
