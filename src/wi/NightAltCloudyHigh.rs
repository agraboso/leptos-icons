#[cfg(feature = "WiNightAltCloudyHigh")]
use leptos::*;
#[cfg(feature = "WiNightAltCloudyHigh")]
///This icon requires the feature `WiNightAltCloudyHigh` to be enabled.
#[component]
pub fn NightAltCloudyHigh(
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
        "M3.57,13.43c0-1.15,0.36-2.18,1.08-3.08s1.63-1.48,2.73-1.74C7.7,7.24,8.4,6.12,9.5,5.24s2.35-1.31,3.76-1.31&#xA;	c1.38,0,2.61,0.43,3.69,1.28s1.78,1.95,2.1,3.29h0.33c0.9,0,1.73,0.22,2.49,0.65c0.76,0.43,1.37,1.03,1.81,1.79&#xA;	c0.44,0.76,0.67,1.58,0.67,2.48c0,1.15-0.35,2.18-1.06,3.08c0.64,0.55,1.4,0.84,2.26,0.87l0.66,0.06c0.12,0,0.18,0.06,0.18,0.19&#xA;	v0.77c0.01,1.01-0.24,1.95-0.73,2.8c-0.49,0.86-1.17,1.53-2.02,2.03c-0.85,0.5-1.78,0.75-2.79,0.75c-0.77,0-1.5-0.15-2.19-0.44&#xA;	c-0.69-0.29-1.28-0.69-1.78-1.19c-0.49-0.5-0.89-1.09-1.18-1.78c-0.29-0.69-0.44-1.41-0.44-2.17H8.37c-1.34-0.06-2.47-0.57-3.4-1.53&#xA;	S3.57,14.76,3.57,13.43z M5.28,13.43c0,0.87,0.3,1.62,0.9,2.26s1.33,0.98,2.19,1.02h11.19c0.86-0.04,1.59-0.38,2.19-1.02&#xA;	c0.6-0.64,0.9-1.39,0.9-2.26c0-0.88-0.32-1.63-0.97-2.28c-0.65-0.64-1.42-0.97-2.31-0.97h-1.62c-0.11,0-0.17-0.06-0.17-0.17&#xA;	l-0.07-0.58c-0.11-1.08-0.58-1.99-1.4-2.71s-1.77-1.09-2.86-1.09c-1.1,0-2.05,0.36-2.86,1.09S9.13,8.35,9.03,9.43l-0.07,0.58&#xA;	c0,0.11-0.07,0.17-0.2,0.17H8.23c-0.84,0.1-1.54,0.46-2.1,1.07S5.28,12.59,5.28,13.43z M16.71,18.39c0,0.79,0.2,1.52,0.6,2.17&#xA;	c0.4,0.65,0.91,1.15,1.54,1.5c0.63,0.35,1.29,0.52,1.99,0.52c0.62,0,1.23-0.15,1.82-0.45c0.6-0.3,1.12-0.75,1.58-1.36&#xA;	s0.75-1.31,0.86-2.1c-1.08-0.22-1.98-0.65-2.72-1.3c-0.84,0.65-1.78,0.99-2.82,1.01H16.71z"
        /> < title > { title } < / title > < / svg >
    }
}
