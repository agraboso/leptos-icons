#[cfg(feature = "WiNightPartlyCloudy")]
use leptos::*;
#[cfg(feature = "WiNightPartlyCloudy")]
///This icon requires the feature `WiNightPartlyCloudy` to be enabled.
#[component]
pub fn NightPartlyCloudy(
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
        "M6.77,19.61c0,0.97,0.35,1.81,1.06,2.52c0.71,0.71,1.55,1.06,2.52,1.06h6.85c0.97,0,1.8-0.35,2.49-1.05&#xA;	c0.69-0.7,1.04-1.54,1.04-2.53c0-0.48-0.07-0.89-0.21-1.23c0.83-0.53,1.49-1.24,1.97-2.12c0.48-0.88,0.73-1.83,0.73-2.84&#xA;	c0-0.81-0.16-1.59-0.47-2.33c-0.32-0.74-0.74-1.38-1.28-1.91S20.3,8.22,19.56,7.9c-0.74-0.32-1.51-0.48-2.32-0.48&#xA;	c-1.09,0-2.1,0.27-3.02,0.81s-1.65,1.27-2.18,2.18c-0.53,0.92-0.79,1.92-0.79,3.01v0.34c-1.01,0.57-1.67,1.41-1.99,2.49&#xA;	c-0.76,0.24-1.36,0.66-1.81,1.27C7,18.13,6.77,18.83,6.77,19.61z M8.75,19.61c0-0.42,0.13-0.78,0.4-1.08&#xA;	c0.27-0.3,0.61-0.47,1.02-0.51l0.63-0.08c0.12,0,0.19-0.08,0.19-0.23l0.1-0.56c0.07-0.58,0.31-1.06,0.73-1.44&#xA;	c0.42-0.39,0.91-0.58,1.48-0.58c0.58,0,1.09,0.19,1.51,0.58c0.43,0.39,0.68,0.87,0.75,1.44l0.08,0.65c0.06,0.15,0.15,0.23,0.25,0.23&#xA;	h1.31c0.43,0,0.8,0.16,1.12,0.47c0.32,0.31,0.47,0.68,0.47,1.12c0,0.45-0.16,0.83-0.47,1.15s-0.69,0.48-1.12,0.48h-6.85&#xA;	c-0.45,0-0.83-0.16-1.14-0.48S8.75,20.06,8.75,19.61z M13.18,13.22c0.07-1.09,0.49-2.01,1.27-2.76c0.78-0.74,1.71-1.12,2.8-1.12&#xA;	c1.11,0,2.06,0.4,2.84,1.19c0.78,0.79,1.17,1.76,1.17,2.89c0,0.7-0.17,1.35-0.51,1.95c-0.34,0.6-0.8,1.08-1.38,1.45&#xA;	c-0.59-0.49-1.27-0.73-2.03-0.73c-0.29-0.88-0.81-1.57-1.54-2.09c-0.73-0.52-1.56-0.78-2.49-0.78H13.18z"
        /> < title > { title } < / title > < / svg >
    }
}
