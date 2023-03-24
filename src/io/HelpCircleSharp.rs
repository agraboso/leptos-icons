#[cfg(feature = "IoHelpCircleSharp")]
use leptos::*;
#[cfg(feature = "IoHelpCircleSharp")]
///This icon requires the feature `IoHelpCircleSharp` to be enabled.
#[component]
pub fn HelpCircleSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M288.55,150.84c-8.09-3.86-20-6-32.72-5.82-18,.22-33.13,5.2-45,14.78-23,18.48-24.55,40.37-24.77,42.8a61.69,61.69,0,0,0-.09,12,3,3,0,0,0,3,2.69h21.23a3,3,0,0,0,3-3A65.7,65.7,0,0,1,214,204c0-.11,1.14-11.7,14.36-22.34,7-5.64,16.11-8.44,27.83-8.59,9.32-.11,16.93,1.47,20.34,3.09C291,183,298,192.31,298,204.57c0,18-10.9,26.23-30.18,39.18C247.08,257.68,237,275.1,237,297v11a3,3,0,0,0,3,3h22a3,3,0,0,0,3-3V297c0-9.16,2.23-19.13,18.44-30C303.39,253.59,326,238.4,326,204.57,326,181.43,312.7,162.34,288.55,150.84Z"
        style = "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,64C150,64,64,150,64,256s86,192,192,192,192-86,192-192S362,64,256,64Zm10.44,302H236.23a2.57,2.57,0,0,1-2.56-2.57v-30.2a2.57,2.57,0,0,1,2.56-2.57h30.21a2.57,2.57,0,0,1,2.56,2.57v30.2A2.57,2.57,0,0,1,266.44,366Zm17-99C267.23,277.88,265,287.85,265,297v11a3,3,0,0,1-3,3H240a3,3,0,0,1-3-3V297c0-21.91,10.08-39.33,30.82-53.26C287.1,230.8,298,222.6,298,204.57c0-12.26-7-21.57-21.49-28.46-3.41-1.62-11-3.2-20.34-3.09-11.72.15-20.82,2.95-27.83,8.59C215.12,192.25,214,203.84,214,204a65.7,65.7,0,0,0-.84,10.28,3,3,0,0,1-3,3H188.91a3,3,0,0,1-3-2.69,61.69,61.69,0,0,1,.09-12c.22-2.43,1.8-24.32,24.77-42.8,11.91-9.58,27.06-14.56,45-14.78,12.7-.15,24.63,2,32.72,5.82C312.7,162.34,326,181.43,326,204.57,326,238.4,303.39,253.59,283.44,267Z"
        /> < title > { title } < / title > < / svg >
    }
}
