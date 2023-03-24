#[cfg(feature = "WiNightCloudy")]
use leptos::*;
#[cfg(feature = "WiNightCloudy")]
///This icon requires the feature `WiNightCloudy` to be enabled.
#[component]
pub fn NightCloudy(
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
        "M4.3,16.89c0,0.89,0.22,1.72,0.66,2.48s1.03,1.36,1.79,1.8s1.58,0.67,2.48,0.67h10.81c0.89,0,1.72-0.22,2.48-0.67&#xA;	s1.36-1.05,1.8-1.8c0.44-0.76,0.67-1.59,0.67-2.48c0-0.64-0.14-1.3-0.42-2c0.76-0.93,1.13-2.03,1.13-3.3c0-0.95-0.23-1.83-0.69-2.63&#xA;	c-0.46-0.8-1.1-1.44-1.9-1.91c-0.8-0.47-1.68-0.7-2.63-0.7c-1.49,0-2.78,0.58-3.87,1.74c-0.76-0.43-1.66-0.65-2.69-0.65&#xA;	c-1.41,0-2.65,0.43-3.73,1.3s-1.77,1.98-2.08,3.35c-1.12,0.25-2.03,0.82-2.74,1.72C4.66,14.71,4.3,15.74,4.3,16.89z M6.01,16.89&#xA;	c0-0.83,0.28-1.55,0.83-2.16c0.56-0.61,1.26-0.96,2.1-1.06l0.68-0.03l0.07-0.72c0.14-1.08,0.61-1.99,1.41-2.71&#xA;	c0.8-0.73,1.74-1.09,2.81-1.09c1.09,0,2.05,0.37,2.86,1.1s1.27,1.63,1.38,2.71l0.1,0.75h1.78c0.88,0,1.64,0.32,2.28,0.95&#xA;	s0.96,1.39,0.96,2.26c0,0.9-0.32,1.67-0.95,2.32s-1.4,0.97-2.28,0.97H9.23c-0.87,0-1.62-0.32-2.26-0.97&#xA;	C6.33,18.55,6.01,17.78,6.01,16.89z M18.04,9.06c0.69-0.66,1.5-0.99,2.44-0.99c0.99,0,1.83,0.34,2.52,1.03&#xA;	c0.69,0.68,1.04,1.52,1.04,2.51c0,0.62-0.17,1.23-0.52,1.84C22.56,12.48,21.4,12,20.03,12h-0.31C19.45,10.89,18.89,9.91,18.04,9.06z&#xA;	"
        /> < title > { title } < / title > < / svg >
    }
}
