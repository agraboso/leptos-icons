#[cfg(feature = "RiOthersFillCake2")]
use leptos::*;
#[cfg(feature = "RiOthersFillCake2")]
///This icon requires the feature `RiOthersFillCake2` to be enabled.
#[component]
pub fn Cake2(
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
        "none" d = "M0 0h24v24H0z" />< path d =
        "M8 6v3.999h3V6h2v3.999h3V6h2v3.999L19 10a3 3 0 0 1 2.995 2.824L22 13v1c0 1.014-.377 1.94-.999 2.645L21 21a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1v-4.36a4.025 4.025 0 0 1-.972-2.182l-.022-.253L2 14v-1a3 3 0 0 1 2.824-2.995L5 10l1-.001V6h2zm11 6H5a1 1 0 0 0-.993.883L4 13v.971l.003.147A2 2 0 0 0 6 16a1.999 1.999 0 0 0 1.98-1.7l.015-.153.005-.176c.036-1.248 1.827-1.293 1.989-.134l.01.134.004.147a2 2 0 0 0 3.992.031l.012-.282c.124-1.156 1.862-1.156 1.986 0l.012.282a2 2 0 0 0 3.99 0L20 14v-1a1 1 0 0 0-.883-.993L19 12zM7 1c1.32.871 1.663 2.088 1.449 2.888a1.5 1.5 0 0 1-2.898-.776C5.85 2.002 7 2.5 7 1zm5 0c1.32.871 1.663 2.088 1.449 2.888a1.5 1.5 0 1 1-2.898-.776C10.85 2.002 12 2.5 12 1zm5 0c1.32.871 1.663 2.088 1.449 2.888a1.5 1.5 0 1 1-2.898-.776C15.85 2.002 17 2.5 17 1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
