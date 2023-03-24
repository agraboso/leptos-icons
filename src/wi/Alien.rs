#[cfg(feature = "WiAlien")]
use leptos::*;
#[cfg(feature = "WiAlien")]
///This icon requires the feature `WiAlien` to be enabled.
#[component]
pub fn Alien(
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
        "M8.75,15.54c-1.12-2.4-0.95-4.66,0.52-6.79c1.03-1.48,2.6-2.39,4.73-2.72c0.16-0.04,0.34-0.07,0.54-0.08h0.63&#xA;	c2.91,0.09,5.05,1.38,6.4,3.88c0.64,1.18,0.8,2.48,0.48,3.91c-0.26,1.13-0.68,2.19-1.28,3.17c-1.29,2.01-2.63,3.64-4,4.88&#xA;	c-0.07,0.07-0.17,0.16-0.3,0.26c-0.46,0.35-0.89,0.53-1.28,0.54s-0.83-0.14-1.31-0.45c-0.29-0.17-0.53-0.37-0.74-0.59&#xA;	C11.18,19.55,9.71,17.55,8.75,15.54z M8.86,13.33c0.02,0.11,0.05,0.25,0.09,0.44s0.07,0.32,0.09,0.4c0.28,1.26,0.86,2.23,1.73,2.93&#xA;	c0.88,0.7,1.96,1.11,3.26,1.23c0.29,0.03,0.46,0.02,0.51-0.03s0.08-0.23,0.09-0.52c-0.01-0.08-0.03-0.21-0.05-0.39&#xA;	c-0.02-0.18-0.04-0.31-0.06-0.39c-0.25-1.34-0.88-2.32-1.9-2.93c-0.18-0.11-0.39-0.22-0.62-0.34s-0.44-0.2-0.61-0.27&#xA;	c-0.17-0.07-0.4-0.16-0.69-0.27c-0.29-0.11-0.5-0.19-0.63-0.25c-0.16-0.06-0.42-0.1-0.8-0.11C8.95,12.83,8.81,13,8.86,13.33z&#xA;	 M15.66,17.73c-0.02,0.31,0,0.49,0.06,0.56c0.07,0.07,0.25,0.08,0.55,0.04c0.38-0.04,0.78-0.12,1.2-0.22&#xA;	c1.07-0.27,1.94-0.84,2.62-1.71c0.34-0.41,0.6-0.86,0.77-1.34s0.34-1.05,0.47-1.72c0.05-0.23,0.04-0.38-0.03-0.46&#xA;	c-0.07-0.08-0.22-0.11-0.44-0.08c-0.59,0.1-1.12,0.23-1.59,0.4c-1.15,0.43-2.02,1.01-2.62,1.74C16.05,15.68,15.72,16.6,15.66,17.73z&#xA;	"
        /> < title > { title } < / title > < / svg >
    }
}
