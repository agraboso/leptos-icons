#[cfg(feature = "IoPartlySunny")]
use leptos::*;
#[cfg(feature = "IoPartlySunny")]
///This icon requires the feature `IoPartlySunny` to be enabled.
#[component]
pub fn PartlySunny(
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
        "M340,480H106c-29.5,0-54.92-7.83-73.53-22.64C11.23,440.44,0,415.35,0,384.8c0-26.66,10.08-49.8,29.14-66.91,15.24-13.68,36.17-23.21,59-26.84h0c.06,0,.08,0,.09-.05,6.44-39,23.83-72.09,50.31-95.68A140.24,140.24,0,0,1,232,160c30.23,0,58.48,9.39,81.71,27.17a142.24,142.24,0,0,1,42.19,53.21,16,16,0,0,0,11.19,9c26,5.61,48.4,17.29,65.17,34C453,304.11,464,331.71,464,363.2c0,32.85-13.13,62.87-37,84.52C404.11,468.54,373.2,480,340,480Zm19-232.18Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M387.89,221.68a168.8,168.8,0,0,1,34.76,14.71,4,4,0,0,0,5.82-2.44A97,97,0,0,0,432,207.27c-.39-52.43-43.48-95.22-95.91-95.27A95.46,95.46,0,0,0,281,129.33l-.06,0a3.38,3.38,0,0,0,1,6,162.45,162.45,0,0,1,51.28,26.4,173.92,173.92,0,0,1,45.32,52.51A16,16,0,0,0,387.89,221.68Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M496,224H464a16,16,0,0,1,0-32h32a16,16,0,0,1,0,32Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M336,96a16,16,0,0,1-16-16V48a16,16,0,0,1,32,0V80A16,16,0,0,1,336,96Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M245.49,133.49a15.92,15.92,0,0,1-11.31-4.69l-22.63-22.62a16,16,0,0,1,22.63-22.63l22.62,22.63a16,16,0,0,1-11.31,27.31Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M426.51,133.49a16,16,0,0,1-11.31-27.31l22.62-22.63a16,16,0,0,1,22.63,22.63L437.82,128.8A15.92,15.92,0,0,1,426.51,133.49Z"
        /> < title > { title } < / title > < / svg >
    }
}
