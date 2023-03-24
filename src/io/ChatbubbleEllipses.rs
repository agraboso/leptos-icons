#[cfg(feature = "IoChatbubbleEllipses")]
use leptos::*;
#[cfg(feature = "IoChatbubbleEllipses")]
///This icon requires the feature `IoChatbubbleEllipses` to be enabled.
#[component]
pub fn ChatbubbleEllipses(
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
        "M398,81.84A227.4,227.4,0,0,0,255.82,32C194.9,32,138,55.47,95.46,98.09,54.35,139.33,31.82,193.78,32,251.37A215.66,215.66,0,0,0,67.65,370.13l.19.27c.28.41.57.82.86,1.22s.65.92.73,1.05l.22.4c1.13,2,2,4.44,1.23,6.9L52.46,446.63a29.13,29.13,0,0,0-1.2,7.63A25.69,25.69,0,0,0,76.83,480a29.44,29.44,0,0,0,10.45-2.29l67.49-24.36.85-.33a14.75,14.75,0,0,1,5.8-1.15,15.12,15.12,0,0,1,5.37,1c1.62.63,16.33,6.26,31.85,10.6,12.9,3.6,39.74,9,60.77,9,59.65,0,115.35-23.1,156.83-65.06C457.36,365.77,480,310.42,480,251.49a213.5,213.5,0,0,0-4.78-45C464.88,157.87,437.46,113.59,398,81.84ZM87.48,380h0ZM160,288a32,32,0,1,1,32-32A32,32,0,0,1,160,288Zm96,0a32,32,0,1,1,32-32A32,32,0,0,1,256,288Zm96,0a32,32,0,1,1,32-32A32,32,0,0,1,352,288Z"
        /> < title > { title } < / title > < / svg >
    }
}
