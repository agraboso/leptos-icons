#[cfg(feature = "WiSmoke")]
use leptos::*;
#[cfg(feature = "WiSmoke")]
///This icon requires the feature `WiSmoke` to be enabled.
#[component]
pub fn Smoke(
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
        "M6.34,12.48c0-0.94,0.3-1.78,0.89-2.52s1.34-1.21,2.25-1.41C9.73,7.43,10.3,6.5,11.2,5.78s1.92-1.08,3.08-1.08&#xA;	c1.12,0,2.13,0.35,3.02,1.05c0.89,0.7,1.46,1.6,1.73,2.69h0.27c1.12,0,2.08,0.39,2.88,1.18c0.79,0.78,1.19,1.74,1.19,2.85&#xA;	c0,0.6-0.12,1.17-0.37,1.7c-0.25,0.53-0.59,0.99-1.03,1.37v0.03c0,0.59-0.19,1.12-0.56,1.59c-0.37,0.47-0.84,0.76-1.4,0.89&#xA;	c-0.14,0.62-0.45,1.15-0.91,1.58c-0.46,0.43-1.01,0.7-1.63,0.8c0.29,0.34,0.43,0.72,0.43,1.13c0,0.48-0.17,0.89-0.51,1.24&#xA;	c-0.34,0.34-0.75,0.52-1.23,0.52c-0.48,0-0.89-0.17-1.23-0.52c-0.34-0.34-0.51-0.76-0.51-1.24c0-0.19,0.03-0.38,0.1-0.57h-0.1&#xA;	c-0.58,0-1.08-0.21-1.5-0.63c-0.42-0.42-0.63-0.92-0.63-1.5c0-0.4,0.1-0.76,0.3-1.07c-0.52-0.29-0.89-0.7-1.12-1.25h-1.28v-0.01&#xA;	c-1.07-0.07-1.98-0.49-2.73-1.27S6.34,13.56,6.34,12.48z M7.74,12.23c0,0.8,0.28,1.48,0.84,2.04s1.24,0.84,2.03,0.84&#xA;	c0.49,0,0.95-0.11,1.37-0.34c0.12,0.74,0.47,1.36,1.04,1.86s1.25,0.74,2.02,0.74c0.87,0,1.61-0.31,2.22-0.92&#xA;	c0.41,0.48,0.92,0.71,1.54,0.71c0.57,0,1.05-0.2,1.46-0.6c0.4-0.4,0.6-0.89,0.6-1.46c0.4-0.27,0.72-0.61,0.95-1.04&#xA;	c0.23-0.42,0.35-0.88,0.35-1.37c0-0.79-0.28-1.47-0.85-2.02c-0.57-0.55-1.25-0.83-2.05-0.83c-0.56,0-1.07,0.15-1.53,0.44&#xA;	c0.06-0.24,0.08-0.51,0.08-0.79c0-0.96-0.34-1.78-1.03-2.46c-0.69-0.68-1.52-1.01-2.49-1.01c-0.94,0-1.75,0.33-2.43,0.97&#xA;	s-1.04,1.44-1.07,2.37c-0.02,0-0.05,0-0.08,0c-0.04,0-0.07,0-0.09,0c-0.79,0-1.46,0.28-2.03,0.84S7.74,11.45,7.74,12.23z"
        /> < title > { title } < / title > < / svg >
    }
}
