#[cfg(feature = "WiMoonAltWaningGibbous3")]
use leptos::*;
#[cfg(feature = "WiMoonAltWaningGibbous3")]
///This icon requires the feature `WiMoonAltWaningGibbous3` to be enabled.
#[component]
pub fn MoonAltWaningGibbous3(
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
        "M3.74,14.44c0-1.52,0.3-2.98,0.89-4.37s1.4-2.58,2.4-3.59s2.2-1.81,3.59-2.4s2.84-0.89,4.37-0.89s2.98,0.3,4.37,0.89&#xA;	s2.59,1.4,3.6,2.4s1.81,2.2,2.4,3.59s0.89,2.84,0.89,4.37s-0.3,2.98-0.89,4.37s-1.4,2.59-2.4,3.6s-2.2,1.81-3.6,2.4&#xA;	s-2.85,0.89-4.37,0.89s-2.98-0.3-4.37-0.89s-2.58-1.4-3.59-2.4s-1.81-2.2-2.4-3.6S3.74,15.97,3.74,14.44z M4.94,14.44&#xA;	c0,1.37,0.27,2.67,0.8,3.91s1.25,2.31,2.15,3.21s1.97,1.61,3.21,2.15s2.54,0.8,3.9,0.8c0.33,0,0.58,0,0.73-0.01&#xA;	c0.78-0.61,1.44-1.31,1.96-2.11s0.92-1.66,1.18-2.57s0.44-1.79,0.54-2.63s0.15-1.75,0.15-2.74c0-1.91-0.32-3.76-0.97-5.54&#xA;	s-1.65-3.28-3.02-4.49c-0.13-0.01-0.32-0.01-0.59-0.01c-1.36,0-2.66,0.27-3.9,0.8S8.79,6.44,7.89,7.34s-1.61,1.97-2.15,3.21&#xA;	S4.94,13.09,4.94,14.44z"
        /> < title > { title } < / title > < / svg >
    }
}
