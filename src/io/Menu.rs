#[cfg(feature = "IoMenu")]
use leptos::*;
#[cfg(feature = "IoMenu")]
///This icon requires the feature `IoMenu` to be enabled.
#[component]
pub fn Menu(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < line xmlns =
        "http://www.w3.org/2000/svg" x1 = "88" y1 = "152" x2 = "424" y2 = "152" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "88" y1 = "256" x2 = "424" y2
        = "256" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "88" y1 = "360" x2 = "424" y2
        = "360" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:48px"
        /> < title > { title } < / title > < / svg >
    }
}
