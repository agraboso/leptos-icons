#[cfg(feature = "WiLunarEclipse")]
use leptos::*;
#[cfg(feature = "WiLunarEclipse")]
///This icon requires the feature `WiLunarEclipse` to be enabled.
#[component]
pub fn LunarEclipse(
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
        "M9.8,14.62c0-0.93,0.23-1.8,0.7-2.6s1.1-1.44,1.9-1.91s1.67-0.7,2.6-0.7c0.94,0,1.81,0.23,2.61,0.7&#xA;	c0.8,0.47,1.43,1.1,1.9,1.9c0.47,0.8,0.7,1.67,0.7,2.61s-0.23,1.81-0.7,2.61c-0.47,0.8-1.1,1.43-1.9,1.9&#xA;	c-0.8,0.47-1.67,0.7-2.61,0.7s-1.8-0.23-2.6-0.7c-0.8-0.47-1.43-1.1-1.9-1.9C10.03,16.43,9.8,15.56,9.8,14.62z M14.25,11.22&#xA;	c0.87,0.11,1.6,0.49,2.19,1.15c0.59,0.66,0.89,1.44,0.89,2.33c0,0.83-0.26,1.56-0.78,2.2s-1.18,1.04-1.98,1.21&#xA;	c0.2,0.02,0.34,0.04,0.43,0.04c0.98,0,1.81-0.35,2.5-1.04c0.69-0.69,1.04-1.52,1.04-2.5c0-0.96-0.35-1.78-1.04-2.47&#xA;	c-0.69-0.68-1.53-1.02-2.5-1.02C14.74,11.14,14.49,11.17,14.25,11.22z"
        /> < title > { title } < / title > < / svg >
    }
}
