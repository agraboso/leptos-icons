#[cfg(feature = "WiMoonWaningGibbous1")]
use leptos::*;
#[cfg(feature = "WiMoonWaningGibbous1")]
///This icon requires the feature `WiMoonWaningGibbous1` to be enabled.
#[component]
pub fn MoonWaningGibbous1(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" x = "0px" y =
        "0px" viewBox = "0 0 30 30" style = "enable-background:new 0 0 30 30;" space =
        "preserve" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M3.74,14.49c0,1.22,0.19,2.4,0.56,3.54s0.91,2.17,1.6,3.09s1.5,1.72,2.42,2.42s1.95,1.23,3.09,1.6s2.32,0.56,3.54,0.56&#xA;	c5.03-1.4,7.54-5.14,7.54-11.22c0-1.18-0.14-2.3-0.42-3.37s-0.65-2.01-1.13-2.83s-1.04-1.57-1.68-2.24s-1.34-1.24-2.06-1.68&#xA;	s-1.47-0.81-2.26-1.07c-1.52,0-2.98,0.3-4.37,0.89S8.02,5.57,7.02,6.57s-1.8,2.19-2.39,3.57S3.74,12.97,3.74,14.49z"
        /> < title > { title } < / title > < / svg >
    }
}
