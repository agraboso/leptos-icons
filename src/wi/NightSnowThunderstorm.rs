#[cfg(feature = "WiNightSnowThunderstorm")]
use leptos::*;
#[cfg(feature = "WiNightSnowThunderstorm")]
///This icon requires the feature `WiNightSnowThunderstorm` to be enabled.
#[component]
pub fn NightSnowThunderstorm(
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
        "M4.23,16.88c0,1.12,0.33,2.12,1,3s1.53,1.47,2.58,1.76l-0.66,1.7c-0.05,0.14,0,0.22,0.14,0.22h2.13L8,27.77h0.29l4.36-5.66&#xA;	c0.04-0.04,0.04-0.09,0.02-0.14c-0.02-0.05-0.07-0.07-0.14-0.07h-2.19l2.49-4.65c0.07-0.14,0.03-0.22-0.14-0.22H9.72&#xA;	c-0.09,0-0.17,0.05-0.23,0.15l-1.07,2.88C7.7,19.88,7.11,19.5,6.64,18.9c-0.47-0.59-0.7-1.26-0.7-2.02c0-0.84,0.28-1.57,0.84-2.18&#xA;	c0.56-0.61,1.27-0.97,2.11-1.07l0.51-0.03c0.12,0,0.19-0.05,0.22-0.14l0.08-0.59c0.11-1.08,0.56-1.99,1.37-2.72s1.76-1.1,2.86-1.1&#xA;	c1.09,0,2.04,0.37,2.86,1.1s1.29,1.64,1.4,2.72l0.08,0.59c0,0.11,0.06,0.17,0.18,0.17h1.61c0.89,0,1.66,0.32,2.31,0.96&#xA;	s0.97,1.4,0.97,2.29c0,0.87-0.3,1.62-0.9,2.26s-1.32,0.98-2.18,1.03c-0.13,0-0.2,0.06-0.2,0.18v1.34c0,0.11,0.07,0.17,0.2,0.17&#xA;	c0.88-0.02,1.69-0.26,2.42-0.72c0.73-0.45,1.31-1.06,1.74-1.81s0.64-1.57,0.64-2.45c0-0.73-0.14-1.4-0.43-2.02&#xA;	c0.76-0.96,1.14-2.06,1.14-3.29c0-0.95-0.24-1.83-0.71-2.64c-0.47-0.81-1.11-1.45-1.92-1.92c-0.81-0.47-1.69-0.71-2.64-0.71&#xA;	c-0.72,0-1.42,0.15-2.1,0.45c-0.68,0.3-1.26,0.72-1.76,1.25c-0.81-0.43-1.71-0.65-2.72-0.65c-1.42,0-2.68,0.44-3.77,1.32&#xA;	s-1.8,2-2.1,3.37c-1.11,0.26-2.02,0.84-2.74,1.74C4.59,14.7,4.23,15.73,4.23,16.88z M13.82,22.96c0,0.24,0.08,0.44,0.24,0.59&#xA;	c0.16,0.16,0.36,0.24,0.58,0.24c0.24,0,0.44-0.08,0.61-0.24s0.25-0.36,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61s-0.37-0.26-0.61-0.26&#xA;	c-0.22,0-0.41,0.09-0.58,0.26S13.82,22.72,13.82,22.96z M13.82,19.32c0,0.24,0.08,0.43,0.24,0.58c0.16,0.16,0.36,0.24,0.58,0.24&#xA;	c0.24,0,0.45-0.08,0.61-0.23s0.25-0.35,0.25-0.59c0-0.23-0.08-0.43-0.25-0.6s-0.37-0.25-0.61-0.25c-0.23,0-0.42,0.08-0.58,0.25&#xA;	S13.82,19.09,13.82,19.32z M13.82,26.63c0,0.22,0.08,0.41,0.24,0.57c0.17,0.17,0.36,0.25,0.58,0.25c0.24,0,0.44-0.08,0.61-0.24&#xA;	c0.17-0.16,0.25-0.35,0.25-0.59c0-0.24-0.08-0.44-0.25-0.61c-0.17-0.17-0.37-0.26-0.61-0.26c-0.22,0-0.41,0.09-0.58,0.26&#xA;	C13.9,26.19,13.82,26.4,13.82,26.63z M17.05,21.02c0,0.24,0.08,0.44,0.25,0.6s0.36,0.25,0.6,0.25c0.23,0,0.43-0.08,0.59-0.25&#xA;	c0.16-0.17,0.24-0.37,0.24-0.6c0-0.22-0.08-0.42-0.24-0.58c-0.16-0.16-0.35-0.24-0.59-0.24c-0.23,0-0.43,0.08-0.6,0.24&#xA;	S17.05,20.79,17.05,21.02z M17.05,24.66c0,0.23,0.08,0.42,0.24,0.58c0.16,0.16,0.36,0.24,0.6,0.24c0.24,0,0.43-0.08,0.59-0.24&#xA;	c0.16-0.16,0.23-0.35,0.23-0.59c0-0.24-0.08-0.43-0.23-0.59c-0.16-0.16-0.35-0.23-0.59-0.23c-0.24,0-0.44,0.08-0.6,0.23&#xA;	C17.13,24.22,17.05,24.42,17.05,24.66z M18.06,9.02c0.67-0.64,1.48-0.97,2.45-0.97c0.98,0,1.82,0.34,2.51,1.03&#xA;	c0.69,0.68,1.04,1.52,1.04,2.5c0,0.66-0.16,1.26-0.47,1.81c-0.96-0.96-2.13-1.44-3.52-1.44h-0.31C19.46,10.76,18.89,9.78,18.06,9.02&#xA;	z"
        /> < title > { title } < / title > < / svg >
    }
}
