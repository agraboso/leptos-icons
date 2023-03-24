#[cfg(feature = "IoCloudyNightSharp")]
use leptos::*;
#[cfg(feature = "IoCloudyNightSharp")]
///This icon requires the feature `IoCloudyNightSharp` to be enabled.
#[component]
pub fn CloudyNightSharp(
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
        "M340,480H106c-29.5,0-54.92-7.83-73.53-22.64C11.23,440.44,0,415.35,0,384.8c0-29.44,12.09-54.25,35-71.74,14.55-11.13,33.41-18.87,53.2-22,6.06-36.92,21.92-68.53,46.29-92A139.82,139.82,0,0,1,232,160c32.33,0,62.15,10.65,86.24,30.79a142.41,142.41,0,0,1,40.83,57.05c27.18,4.48,51.59,15.68,69.56,32.08C451.77,301,464,329.82,464,363.2c0,32.85-13.13,62.87-37,84.52C404.11,468.54,373.2,480,340,480Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M381.55,219.93c26.5,6.93,50,19.32,68.65,36.34q3.89,3.56,7.47,7.34c25.41-18.4,45.47-44.92,54.33-71.38-16.24,7.07-35.31,9.85-54.15,9.85-73.42,0-115.93-42.51-115.93-115.93,0-18.84,2.78-37.91,9.85-54.15-40.41,13.53-81,53.19-92.52,98.13a162.61,162.61,0,0,1,79.52,36.12A173,173,0,0,1,381.55,219.93Z"
        /> < title > { title } < / title > < / svg >
    }
}
