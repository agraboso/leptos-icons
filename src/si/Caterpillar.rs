#[cfg(feature = "SiCaterpillar")]
use leptos::*;
#[cfg(feature = "SiCaterpillar")]
///This icon requires the feature `SiCaterpillar` to be enabled.
#[component]
pub fn Caterpillar(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M11.901 11.554l.802-4.1.798 4.1zm2.869-6.52h-4.15L8.2 15.884l4.503-3.635 4.695 3.934zm-2.067 8.156l-7.509 6.072H19.95zM24 5.02v2.77h-2.066v11.45h-.882l-2.436-2.04V7.79h-2.057V5.02zM6.872 16.864c.548-.458.642-1.024.642-1.532V13.2h-2.98v2.894a.75.75 0 0 1-.748.751c-.414 0-.722-.336-.722-.75V7.893c0-.414.308-.75.722-.75a.75.75 0 0 1 .749.75v2.913H7.51V7.785c0-1.67-1.092-3.044-3.75-3.047-2.728 0-3.76 1.38-3.76 3.05v8.563c0 1.655 1.314 2.907 2.995 2.907h.922Z"
        /> < title > { title } < / title > < / svg >
    }
}
