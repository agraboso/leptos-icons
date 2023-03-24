#[cfg(feature = "IoServer")]
use leptos::*;
#[cfg(feature = "IoServer")]
///This icon requires the feature `IoServer` to be enabled.
#[component]
pub fn Server(
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
        "M256,428C203.65,428,144.61,416.39,98.07,397,81,389.81,66.38,378.18,54.43,369A4,4,0,0,0,48,372.18v12.58c0,28.07,23.49,53.22,66.14,70.82C152.29,471.33,202.67,480,256,480s103.7-8.67,141.86-24.42C440.51,438,464,412.83,464,384.76V372.18a4,4,0,0,0-6.43-3.18C445.62,378.17,431,389.81,413.92,397,367.38,416.39,308.35,428,256,428Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M464,126.51c-.81-27.65-24.18-52.4-66-69.85C359.74,40.76,309.34,32,256,32S152.26,40.76,114.09,56.66c-41.78,17.41-65.15,42.11-66,69.69L48,144c0,6.41,5.2,16.48,14.63,24.73,11.13,9.73,27.65,19.33,47.78,27.73C153.24,214.36,207.67,225,256,225s102.76-10.68,145.59-28.58c20.13-8.4,36.65-18,47.78-27.73C458.8,160.49,464,150.42,464,144Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M413.92,226C367.39,245.43,308.35,257,256,257S144.61,245.43,98.07,226C81,218.85,66.38,207.21,54.43,198A4,4,0,0,0,48,201.22V232c0,6.41,5.2,14.48,14.63,22.73,11.13,9.74,27.65,19.33,47.78,27.74C153.24,300.34,207.67,311,256,311s102.76-10.68,145.59-28.57c20.13-8.41,36.65-18,47.78-27.74C458.8,246.47,464,238.41,464,232V201.22a4,4,0,0,0-6.43-3.18C445.62,207.21,431,218.85,413.92,226Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M413.92,312C367.38,331.41,308.35,343,256,343S144.61,331.41,98.07,312C81,304.83,66.38,293.19,54.43,284A4,4,0,0,0,48,287.2V317c0,6.41,5.2,14.47,14.62,22.71,11.13,9.74,27.66,19.33,47.79,27.74C153.24,385.32,207.66,396,256,396s102.76-10.68,145.59-28.57c20.13-8.41,36.65-18,47.78-27.74C458.8,331.44,464,323.37,464,317V287.2a4,4,0,0,0-6.43-3.18C445.62,293.19,431,304.83,413.92,312Z"
        /> < title > { title } < / title > < / svg >
    }
}
