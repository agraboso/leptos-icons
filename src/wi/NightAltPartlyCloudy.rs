#[cfg(feature = "WiNightAltPartlyCloudy")]
use leptos::*;
#[cfg(feature = "WiNightAltPartlyCloudy")]
///This icon requires the feature `WiNightAltPartlyCloudy` to be enabled.
#[component]
pub fn NightAltPartlyCloudy(
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
        "M6.77,19.65c0-0.79,0.23-1.48,0.68-2.09c0.45-0.61,1.06-1.03,1.81-1.27c0.32-1.09,0.98-1.92,1.99-2.49v-0.35&#xA;	c0-1.46,0.46-2.74,1.38-3.85s2.09-1.8,3.5-2.06c0.36-0.06,0.72-0.09,1.08-0.09h0.03c0.21,0,0.44,0.02,0.7,0.05&#xA;	c0.26,0.02,0.5,0.06,0.73,0.11l0.91,0.28c0.13,0.07,0.18,0.16,0.16,0.26l-0.13,0.7C19.54,9.18,19.5,9.5,19.5,9.82&#xA;	c0,0.35,0.05,0.71,0.16,1.07c0.11,0.37,0.27,0.72,0.5,1.08s0.52,0.68,0.91,0.97c0.38,0.29,0.83,0.51,1.33,0.66l0.71,0.21&#xA;	c0.11,0.03,0.17,0.08,0.17,0.18c0,0.04,0,0.06-0.01,0.07l-0.18,0.68c-0.06,0.24-0.13,0.49-0.22,0.73c-0.15,0.44-0.38,0.89-0.7,1.37&#xA;	c0-0.01,0-0.01-0.01-0.01c-0.44,0.63-0.98,1.16-1.64,1.58c0.14,0.34,0.21,0.75,0.21,1.24c0,0.99-0.35,1.83-1.04,2.53&#xA;	c-0.69,0.7-1.52,1.05-2.49,1.05h-6.85c-0.97,0-1.81-0.35-2.52-1.06C7.13,21.46,6.77,20.62,6.77,19.65z M8.75,19.65&#xA;	c0,0.45,0.15,0.83,0.46,1.15s0.69,0.47,1.14,0.47h6.85c0.43,0,0.8-0.16,1.12-0.48c0.32-0.32,0.47-0.7,0.47-1.14&#xA;	c0-0.43-0.16-0.8-0.47-1.12s-0.69-0.47-1.12-0.47H15.9c-0.11,0-0.19-0.07-0.25-0.23l-0.08-0.64c-0.07-0.58-0.32-1.06-0.75-1.44&#xA;	s-0.93-0.58-1.51-0.58c-0.57,0-1.06,0.19-1.48,0.58c-0.42,0.39-0.66,0.87-0.73,1.44l-0.1,0.55c0,0.15-0.06,0.22-0.19,0.22&#xA;	l-0.63,0.08c-0.41,0.04-0.75,0.21-1.02,0.51C8.89,18.87,8.75,19.23,8.75,19.65z M13.18,13.25h0.12c0.93,0,1.75,0.26,2.49,0.78&#xA;	c0.73,0.52,1.25,1.22,1.54,2.1c0.77,0,1.45,0.24,2.03,0.72c0.69-0.43,1.2-1.02,1.53-1.75c-1.04-0.52-1.85-1.27-2.43-2.25&#xA;	s-0.88-2.01-0.88-3.11V9.39c-0.03,0-0.07,0-0.12,0c-0.05,0-0.09,0-0.12,0c-0.61,0-1.2,0.13-1.77,0.39&#xA;	c-0.57,0.26-1.05,0.64-1.44,1.12l-0.03-0.02C13.55,11.56,13.25,12.35,13.18,13.25z"
        /> < title > { title } < / title > < / svg >
    }
}
