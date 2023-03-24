#[cfg(feature = "HiLgSolidLink")]
use leptos::*;
#[cfg(feature = "HiLgSolidLink")]
///This icon requires the feature `HiLgSolidLink` to be enabled.
#[component]
pub fn Link(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M19.9017 4.09835C18.4372 2.63388 16.0628 2.63388 14.5984 4.09835L10.0983 8.59835C8.63388 10.0628 8.63388 12.4372 10.0983 13.9017C10.4092 14.2125 10.7598 14.4565 11.133 14.6348C11.5068 14.8134 11.665 15.2611 11.4865 15.6349C11.3079 16.0086 10.8602 16.1669 10.4865 15.9883C9.96169 15.7376 9.47063 15.3953 9.03769 14.9623C6.98744 12.9121 6.98744 9.58794 9.03769 7.53769L13.5377 3.03769C15.5879 0.987437 18.9121 0.987437 20.9623 3.03769C23.0126 5.08794 23.0126 8.41206 20.9623 10.4623L19.2053 12.2193C18.9124 12.5122 18.4376 12.5122 18.1447 12.2193C17.8518 11.9264 17.8518 11.4515 18.1447 11.1586L19.9017 9.40165C21.3661 7.93718 21.3661 5.56282 19.9017 4.09835ZM12.5135 8.36513C12.6921 7.99138 13.1398 7.83313 13.5135 8.01167C14.0383 8.26236 14.5294 8.60475 14.9623 9.03769C17.0126 11.0879 17.0126 14.4121 14.9623 16.4623L10.4623 20.9623C8.41206 23.0126 5.08794 23.0126 3.03769 20.9623C0.987437 18.9121 0.987437 15.5879 3.03769 13.5377L4.79466 11.7807C5.08755 11.4878 5.56243 11.4878 5.85532 11.7807C6.14821 12.0736 6.14821 12.5485 5.85532 12.8414L4.09835 14.5984C2.63388 16.0628 2.63388 18.4372 4.09835 19.9017C5.56282 21.3661 7.93718 21.3661 9.40165 19.9017L13.9017 15.4016C15.3661 13.9372 15.3661 11.5628 13.9017 10.0983C13.5908 9.78748 13.2402 9.54347 12.867 9.36517C12.4932 9.18662 12.335 8.73889 12.5135 8.36513Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
