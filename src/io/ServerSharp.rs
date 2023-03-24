#[cfg(feature = "IoServerSharp")]
use leptos::*;
#[cfg(feature = "IoServerSharp")]
///This icon requires the feature `IoServerSharp` to be enabled.
#[component]
pub fn ServerSharp(
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
        "M409.43,389.87C362,410,305.4,421.05,256,421.05s-105.87-11.3-153.44-31.18S48,353.16,48,353.16v38.2c0,31.15,18,43.64,67.32,64.35C153.13,471.59,203.18,480,256,480s102.87-8.41,140.68-24.29C446,435,464,422.51,464,391.36v-38.2S456.86,369.75,409.43,389.87Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M63.69,173.22c11.23,9.84,27.82,19.49,48,27.92,42.48,17.76,96.45,28.37,144.36,28.37s101.88-10.61,144.36-28.37c20.13-8.43,36.72-18.08,47.95-27.92,6.06-5.31,10.85-10.12,13.47-12.85a8,8,0,0,0,2.22-5.54l0-26.06v-.1c-.84-28.79-24.71-54.41-67.21-72.14C358.83,40.71,308.84,32,256,32S153.17,40.71,115.26,56.53C72.85,74.22,49,99.78,48.05,128.5l0,26.33a8,8,0,0,0,2.21,5.54C52.84,163.1,57.62,167.91,63.69,173.22Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M409.43,221.91C365,241,305.4,253.09,256,253.09s-108.87-12.27-153.43-31.18S48,185.2,48,185.2v47.36c.08,7.52,5.5,16.2,15.69,25.13,11.24,9.84,27.82,19.5,48,27.92C154.12,303.38,208.09,314,256,314s101.88-10.6,144.36-28.37c20.13-8.42,36.72-18.08,47.95-27.92,10.25-9,15.68-17.71,15.69-25.27V185.2S453.87,202.82,409.43,221.91Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M409.43,306.38C362,326,305.4,337.56,256,337.56s-109.87-12.8-153.43-31.18S48,269.67,48,269.67v46.25c0,7.55,5.44,16.28,15.69,25.26,11.23,9.84,27.81,19.5,48,27.92,42.48,17.77,96.44,28.37,144.36,28.37s101.88-10.6,144.36-28.37c20.13-8.43,36.72-18.08,47.95-27.92,10.19-8.93,15.61-17.61,15.69-25.13V269.67S456.87,286.76,409.43,306.38Z"
        /> < title > { title } < / title > < / svg >
    }
}
