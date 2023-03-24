#[cfg(feature = "IoMegaphone")]
use leptos::*;
#[cfg(feature = "IoMegaphone")]
///This icon requires the feature `IoMegaphone` to be enabled.
#[component]
pub fn Megaphone(
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
        "M48,176v.66a17.38,17.38,0,0,1-4.2,11.23l0,.05C38.4,194.32,32,205.74,32,224c0,16.55,5.3,28.23,11.68,35.91A19,19,0,0,1,48,272h0a32,32,0,0,0,32,32h8a8,8,0,0,0,8-8V152a8,8,0,0,0-8-8H80A32,32,0,0,0,48,176Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M452.18,186.55l-.93-.17a4,4,0,0,1-3.25-3.93V62c0-12.64-8.39-24-20.89-28.32-11.92-4.11-24.34-.76-31.68,8.53A431.18,431.18,0,0,1,344.12,93.9c-23.63,20-46.24,34.25-67,42.31a8,8,0,0,0-5.15,7.47V299a16,16,0,0,0,9.69,14.69c19.34,8.29,40.24,21.83,62,40.28a433.74,433.74,0,0,1,51.68,52.16A26.22,26.22,0,0,0,416.44,416a33.07,33.07,0,0,0,10.44-1.74C439.71,410,448,399.05,448,386.4V265.53a4,4,0,0,1,3.33-3.94l.85-.14C461.8,258.84,480,247.67,480,224S461.8,189.16,452.18,186.55Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,320V152a8,8,0,0,0-8-8H136a8,8,0,0,0-8,8V456a24,24,0,0,0,24,24h52.45a32.66,32.66,0,0,0,25.93-12.45,31.65,31.65,0,0,0,5.21-29.05c-1.62-5.18-3.63-11-5.77-17.19-7.91-22.9-18.34-37.07-21.12-69.32A32,32,0,0,0,240,320Z"
        /> < title > { title } < / title > < / svg >
    }
}
