#[cfg(feature = "WiNightAltShowers")]
use leptos::*;
#[cfg(feature = "WiNightAltShowers")]
///This icon requires the feature `WiNightAltShowers` to be enabled.
#[component]
pub fn NightAltShowers(
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
        "M4.07,16.91c0,1.33,0.46,2.48,1.39,3.43s2.06,1.47,3.4,1.53c0.11,0,0.17-0.06,0.17-0.17v-1.37c0-0.12-0.06-0.18-0.17-0.18&#xA;	c-0.87-0.07-1.6-0.41-2.19-1.04c-0.59-0.62-0.89-1.36-0.89-2.21c0-0.83,0.28-1.54,0.84-2.16s1.26-0.97,2.1-1.07l0.53-0.07&#xA;	c0.13,0,0.2-0.06,0.2-0.17l0.07-0.52c0.11-1.08,0.56-1.99,1.37-2.71c0.81-0.73,1.76-1.09,2.85-1.09c1.09,0,2.04,0.36,2.85,1.09&#xA;	c0.81,0.72,1.28,1.63,1.39,2.72l0.08,0.58c0,0.12,0.06,0.18,0.18,0.18h1.61c0.9,0,1.67,0.32,2.32,0.95&#xA;	c0.64,0.63,0.97,1.39,0.97,2.28c0,0.85-0.3,1.59-0.89,2.21c-0.59,0.62-1.33,0.97-2.19,1.04c-0.13,0-0.2,0.06-0.2,0.18v1.37&#xA;	c0,0.11,0.07,0.17,0.2,0.17c1.33-0.04,2.46-0.55,3.38-1.51c0.92-0.96,1.38-2.11,1.38-3.45c0-0.87-0.22-1.68-0.65-2.43&#xA;	c0.81-0.73,1.34-1.6,1.58-2.62v-0.13l0.19-0.79l-0.76-0.21c-0.81-0.24-1.44-0.7-1.89-1.35c-0.45-0.66-0.67-1.34-0.67-2.03&#xA;	c0-0.26,0.03-0.52,0.08-0.78l0.2-0.8l-0.85-0.25L21.9,5.49c-0.47-0.09-0.88-0.14-1.25-0.14c-0.38,0-0.76,0.04-1.14,0.13&#xA;	c-0.39,0.09-0.79,0.22-1.2,0.41c-0.42,0.19-0.82,0.45-1.2,0.8c-0.38,0.35-0.72,0.76-1,1.23c-0.74-0.33-1.53-0.49-2.36-0.49&#xA;	c-1.41,0-2.66,0.44-3.75,1.31s-1.77,1.99-2.07,3.36c-1.12,0.26-2.05,0.83-2.77,1.72C4.43,14.73,4.07,15.76,4.07,16.91z M9.47,23.68&#xA;	c0,0.15,0.05,0.3,0.15,0.45c0.1,0.15,0.25,0.26,0.45,0.33c0.22,0.07,0.43,0.06,0.64-0.05s0.34-0.28,0.41-0.51l0.28-1.06&#xA;	c0.07-0.21,0.05-0.41-0.07-0.62c-0.12-0.21-0.29-0.34-0.51-0.41c-0.23-0.06-0.45-0.03-0.65,0.08s-0.34,0.3-0.42,0.53l-0.23,0.99&#xA;	C9.49,23.57,9.47,23.66,9.47,23.68z M10.77,18.95c0,0.11,0.03,0.23,0.1,0.36c0.07,0.17,0.25,0.3,0.53,0.38&#xA;	c0.24,0.06,0.46,0.04,0.66-0.06c0.19-0.1,0.33-0.28,0.4-0.52l0.28-1.03c0.07-0.23,0.05-0.45-0.07-0.64&#xA;	c-0.12-0.2-0.29-0.33-0.51-0.39c-0.24-0.06-0.47-0.04-0.67,0.07c-0.2,0.11-0.33,0.28-0.4,0.52l-0.27,1.01&#xA;	C10.79,18.78,10.77,18.88,10.77,18.95z M12.02,26.8c0,0.17,0.05,0.33,0.15,0.49c0.1,0.16,0.25,0.27,0.45,0.33&#xA;	c0.11,0.03,0.18,0.05,0.23,0.05c0.09,0,0.21-0.03,0.38-0.1c0.2-0.08,0.34-0.27,0.43-0.55l0.3-1.05c0.07-0.21,0.05-0.42-0.07-0.63&#xA;	c-0.12-0.21-0.29-0.34-0.51-0.41c-0.24-0.06-0.47-0.04-0.67,0.08c-0.2,0.12-0.34,0.29-0.41,0.53l-0.25,1.01&#xA;	C12.03,26.63,12.02,26.72,12.02,26.8z M13.35,22.03c0,0.15,0.05,0.3,0.15,0.45s0.25,0.26,0.46,0.33c0.22,0.07,0.44,0.05,0.64-0.06&#xA;	c0.2-0.11,0.33-0.28,0.4-0.52l0.27-1.04c0.07-0.21,0.05-0.42-0.06-0.62c-0.11-0.2-0.27-0.34-0.49-0.41&#xA;	c-0.24-0.06-0.47-0.03-0.68,0.08s-0.35,0.3-0.42,0.53l-0.24,1L13.35,22.03z M16.16,23.79c0,0.38,0.21,0.62,0.64,0.75&#xA;	c0.09,0.02,0.17,0.03,0.24,0.03c0.15,0,0.27-0.02,0.37-0.07c0.21-0.08,0.36-0.27,0.44-0.57l0.27-1.02c0.06-0.25,0.04-0.47-0.08-0.67&#xA;	s-0.29-0.32-0.53-0.37c-0.23-0.07-0.45-0.05-0.64,0.07s-0.33,0.29-0.4,0.51l-0.27,1.04c0,0.02-0.01,0.07-0.02,0.15&#xA;	C16.16,23.71,16.16,23.76,16.16,23.79z M17.55,18.98c0,0.16,0.05,0.31,0.15,0.46c0.1,0.15,0.26,0.26,0.46,0.32&#xA;	c0.14,0.03,0.22,0.05,0.23,0.05c0.09,0,0.21-0.03,0.38-0.1c0.21-0.08,0.35-0.27,0.44-0.55l0.28-1.04c0.06-0.22,0.03-0.43-0.08-0.63&#xA;	s-0.3-0.33-0.53-0.4c-0.22-0.07-0.43-0.05-0.63,0.07s-0.33,0.29-0.4,0.52l-0.26,1.06C17.56,18.83,17.55,18.92,17.55,18.98z&#xA;	 M17.58,8.81c0.32-0.56,0.76-1,1.33-1.31c0.57-0.31,1.17-0.47,1.81-0.47h0.21c-0.01,0.09-0.01,0.21-0.01,0.38&#xA;	c0,0.95,0.26,1.85,0.78,2.71c0.52,0.86,1.25,1.51,2.17,1.96c-0.22,0.43-0.48,0.8-0.78,1.1c-0.93-0.81-2.02-1.21-3.25-1.21h-0.32&#xA;	C19.27,10.78,18.63,9.73,17.58,8.81z"
        /> < title > { title } < / title > < / svg >
    }
}
