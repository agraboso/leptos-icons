#[cfg(feature = "IoVolumeHigh")]
use leptos::*;
#[cfg(feature = "IoVolumeHigh")]
///This icon requires the feature `IoVolumeHigh` to be enabled.
#[component]
pub fn VolumeHigh(
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
        "M232,416a23.88,23.88,0,0,1-14.2-4.68,8.27,8.27,0,0,1-.66-.51L125.76,336H56a24,24,0,0,1-24-24V200a24,24,0,0,1,24-24h69.75l91.37-74.81a8.27,8.27,0,0,1,.66-.51A24,24,0,0,1,256,120V392a24,24,0,0,1-24,24ZM125.82,336Zm-.27-159.86Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,336a16,16,0,0,1-14.29-23.19c9.49-18.87,14.3-38,14.3-56.81,0-19.38-4.66-37.94-14.25-56.73a16,16,0,0,1,28.5-14.54C346.19,208.12,352,231.44,352,256c0,23.86-6,47.81-17.7,71.19A16,16,0,0,1,320,336Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,384a16,16,0,0,1-13.86-24C373.05,327.09,384,299.51,384,256c0-44.17-10.93-71.56-29.82-103.94a16,16,0,0,1,27.64-16.12C402.92,172.11,416,204.81,416,256c0,50.43-13.06,83.29-34.13,120A16,16,0,0,1,368,384Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M416,432a16,16,0,0,1-13.39-24.74C429.85,365.47,448,323.76,448,256c0-66.5-18.18-108.62-45.49-151.39a16,16,0,1,1,27-17.22C459.81,134.89,480,181.74,480,256c0,64.75-14.66,113.63-50.6,168.74A16,16,0,0,1,416,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
