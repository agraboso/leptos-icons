#[cfg(feature = "FaSolidHamsa")]
use leptos::*;
#[cfg(feature = "FaSolidHamsa")]
///This icon requires the feature `FaSolidHamsa` to be enabled.
#[component]
pub fn Hamsa(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M34.6 288H80c8.8 0 16-7.2 16-16V72c0-22.1 17.9-40 40-40s40 17.9 40 40V204c0 11 9 20 20 20s20-9 20-20V40c0-22.1 17.9-40 40-40s40 17.9 40 40V204c0 11 9 20 20 20s20-9 20-20V72c0-22.1 17.9-40 40-40s40 17.9 40 40V272c0 8.8 7.2 16 16 16h45.4c19.1 0 34.6 15.5 34.6 34.6c0 8.6-3.2 16.9-9 23.3L416.6 441c-41.1 45.2-99.4 71-160.6 71s-119.4-25.8-160.6-71L9 345.9c-5.8-6.4-9-14.7-9-23.3C0 303.5 15.5 288 34.6 288zM256 288c-38.4 0-76.8 35.8-90.6 50.2c-3.6 3.7-5.4 8.7-5.4 13.8s1.8 10.1 5.4 13.8C179.2 380.2 217.6 416 256 416s76.8-35.8 90.6-50.2c3.6-3.7 5.4-8.7 5.4-13.8s-1.8-10.1-5.4-13.8C332.8 323.8 294.4 288 256 288zm0 32a32 32 0 1 1 0 64 32 32 0 1 1 0-64z"
        /> < title > { title } < / title > < / svg >
    }
}
