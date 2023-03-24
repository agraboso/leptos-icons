#[cfg(feature = "IoBaseball")]
use leptos::*;
#[cfg(feature = "IoBaseball")]
///This icon requires the feature `IoBaseball` to be enabled.
#[component]
pub fn Baseball(
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
        "M444,295.67l-.47-26.07a205.42,205.42,0,0,1-39.27-4.73l-5,17.35a14,14,0,1,1-26.88-7.81l5-17.38a206.47,206.47,0,0,1-36.05-17.35l-10.44,14.77a14,14,0,0,1-22.87-16.16l10.41-14.73a204.8,204.8,0,0,1-30-30L273.71,204a14,14,0,0,1-16.16-22.87l14.74-10.42a205.3,205.3,0,0,1-17.38-36.06l-17.32,5a14,14,0,1,1-7.81-26.89l17.3-5a205.88,205.88,0,0,1-4.43-32.59h0q-.17-3-.24-6l-18.22-.33a14,14,0,0,1-13.74-14A208,208,0,0,0,55,202.42a16,16,0,0,1,15,15.66l.44,24.43c1.58.05,3.16.11,4.73.2a205.88,205.88,0,0,1,32.59,4.43l5-17.3a14,14,0,0,1,26.89,7.81l-5,17.32a205.21,205.21,0,0,1,36,17.38L181,257.61a14,14,0,0,1,22.87,16.16L193.48,288.5a205.65,205.65,0,0,1,15.79,14.23,203.79,203.79,0,0,1,14.23,15.79l14.73-10.41A14,14,0,0,1,254.39,331l-14.76,10.43A206.86,206.86,0,0,1,257,377.47l17.38-5.05a14,14,0,0,1,7.81,26.89l-17.35,5a205.89,205.89,0,0,1,4.7,38.28l18.27.33a16,16,0,0,1,15.71,16.28,11.69,11.69,0,0,1-.08,1.19A208,208,0,0,0,456.83,309.36,14,14,0,0,1,444,295.67Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M301.49,56.2A14,14,0,0,1,287.5,70h-.25l-16.81-.3c.05,1.31.1,2.62.17,3.93h0a178.83,178.83,0,0,0,3.44,26.31l16.29-4.74A14,14,0,0,1,298.15,122l-16.27,4.73a178.06,178.06,0,0,0,13.33,27.69l13.84-9.78a14,14,0,1,1,16.16,22.87l-13.86,9.79q5.25,6.33,11.12,12.19c3.9,3.91,8,7.6,12.2,11.1l9.78-13.84A14,14,0,1,1,367.32,203l-9.76,13.8a178.83,178.83,0,0,0,27.68,13.33L390,213.85a14,14,0,0,1,26.89,7.81l-4.73,16.26a177.72,177.72,0,0,0,30.95,3.65l-.16-9a14,14,0,0,1,13.75-14.24h.25a14.67,14.67,0,0,1,2.59.25,208,208,0,0,0-158-163.51C301.48,55.47,301.5,55.83,301.49,56.2Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M208.44,457.55a16,16,0,0,1,16.28-15.71l16.76.29a178.49,178.49,0,0,0-3.62-29.95L221.6,416.9A14,14,0,1,1,213.79,390L230,385.3a177.92,177.92,0,0,0-13.33-27.68l-13.8,9.76a14,14,0,1,1-16.16-22.87l13.84-9.78c-3.5-4.22-7.19-8.3-11.1-12.2s-8-7.62-12.19-11.12l-9.79,13.86a14,14,0,1,1-22.87-16.16l9.78-13.84a177.16,177.16,0,0,0-27.69-13.33L122,298.21A14,14,0,1,1,95.1,290.4l4.73-16.29a177.32,177.32,0,0,0-26.31-3.44c-.89-.05-1.79-.08-2.68-.12L71,281.14a16,16,0,0,1-15.71,16.28H55a16,16,0,0,1-3.94-.51A208,208,0,0,0,208.71,460.78,15.72,15.72,0,0,1,208.44,457.55Z"
        /> < title > { title } < / title > < / svg >
    }
}
