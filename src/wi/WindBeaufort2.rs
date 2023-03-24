#[cfg(feature = "WiWindBeaufort2")]
use leptos::*;
#[cfg(feature = "WiWindBeaufort2")]
///This icon requires the feature `WiWindBeaufort2` to be enabled.
#[component]
pub fn WindBeaufort2(
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
        "M4.94,13.5c0,0.18,0.06,0.31,0.17,0.4c0.12,0.11,0.26,0.17,0.42,0.17h10.4c0.17,0,0.33,0.06,0.46,0.19&#xA;	c0.13,0.12,0.2,0.28,0.2,0.46s-0.07,0.34-0.2,0.47s-0.28,0.2-0.46,0.2c-0.18,0-0.34-0.07-0.47-0.21c-0.12-0.11-0.26-0.16-0.4-0.16&#xA;	c-0.16,0-0.3,0.05-0.41,0.16c-0.11,0.11-0.16,0.24-0.16,0.39c0,0.16,0.06,0.3,0.17,0.41c0.36,0.36,0.78,0.53,1.27,0.53&#xA;	s0.91-0.17,1.26-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.92-0.52-1.27c-0.35-0.35-0.77-0.53-1.26-0.53H5.53&#xA;	c-0.16,0-0.3,0.06-0.42,0.17C4.99,13.21,4.94,13.34,4.94,13.5z M4.94,11.48c0,0.17,0.06,0.3,0.17,0.39&#xA;	c0.12,0.11,0.26,0.16,0.42,0.16h13.81c0.49,0,0.92-0.18,1.27-0.52c0.35-0.35,0.52-0.77,0.52-1.27c0-0.49-0.17-0.91-0.52-1.26&#xA;	s-0.77-0.52-1.27-0.52c-0.49,0-0.91,0.17-1.27,0.51c-0.11,0.12-0.16,0.27-0.16,0.42c0,0.16,0.05,0.3,0.16,0.4&#xA;	c0.11,0.1,0.24,0.15,0.4,0.15c0.15,0,0.29-0.05,0.41-0.16C19,9.66,19.15,9.6,19.34,9.6c0.17,0,0.33,0.06,0.46,0.18&#xA;	c0.13,0.12,0.2,0.27,0.2,0.45c0,0.18-0.07,0.34-0.2,0.47c-0.13,0.13-0.28,0.2-0.46,0.2H5.53c-0.16,0-0.3,0.06-0.42,0.17&#xA;	C4.99,11.18,4.94,11.32,4.94,11.48z M17.66,21.85h6.62l0.4-1.89H21v-0.03c0.2-0.09,0.49-0.22,0.86-0.37&#xA;	c0.38-0.15,0.69-0.28,0.95-0.38s0.55-0.25,0.87-0.44s0.57-0.38,0.77-0.57c0.19-0.19,0.36-0.44,0.5-0.75s0.21-0.64,0.21-1&#xA;	c0-0.56-0.14-1.02-0.43-1.4s-0.65-0.65-1.08-0.81c-0.43-0.16-0.92-0.24-1.45-0.24c-0.97,0-1.76,0.26-2.38,0.78&#xA;	c-0.62,0.52-0.98,1.29-1.1,2.31h2.09c0-0.37,0.11-0.68,0.32-0.94c0.22-0.26,0.52-0.38,0.91-0.38c0.3,0,0.52,0.08,0.67,0.24&#xA;	s0.23,0.34,0.23,0.54c0,0.12-0.01,0.23-0.03,0.32s-0.07,0.19-0.15,0.28s-0.15,0.16-0.21,0.22s-0.17,0.13-0.34,0.23&#xA;	c-0.17,0.09-0.3,0.17-0.4,0.22c-0.1,0.05-0.27,0.13-0.53,0.25c-0.88,0.43-1.43,0.71-1.64,0.83c-0.8,0.48-1.35,1.07-1.66,1.78&#xA;	C17.82,21.01,17.71,21.41,17.66,21.85z"
        /> < title > { title } < / title > < / svg >
    }
}
