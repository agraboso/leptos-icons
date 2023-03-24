#[cfg(feature = "HiLgSolidSun")]
use leptos::*;
#[cfg(feature = "HiLgSolidSun")]
///This icon requires the feature `HiLgSolidSun` to be enabled.
#[component]
pub fn Sun(
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
        "http://www.w3.org/2000/svg" d =
        "M12 2.25C12.4142 2.25 12.75 2.58579 12.75 3V5.25C12.75 5.66421 12.4142 6 12 6C11.5858 6 11.25 5.66421 11.25 5.25V3C11.25 2.58579 11.5858 2.25 12 2.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.5 12C7.5 9.51472 9.51472 7.5 12 7.5C14.4853 7.5 16.5 9.51472 16.5 12C16.5 14.4853 14.4853 16.5 12 16.5C9.51472 16.5 7.5 14.4853 7.5 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.8943 6.16634C19.1872 5.87344 19.1872 5.39857 18.8943 5.10568C18.6014 4.81278 18.1266 4.81278 17.8337 5.10568L16.2427 6.69667C15.9498 6.98956 15.9498 7.46443 16.2427 7.75733C16.5356 8.05022 17.0105 8.05022 17.3034 7.75733L18.8943 6.16634Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21.75 12C21.75 12.4142 21.4142 12.75 21 12.75H18.75C18.3358 12.75 18 12.4142 18 12C18 11.5858 18.3358 11.25 18.75 11.25H21C21.4142 11.25 21.75 11.5858 21.75 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.8336 18.8943C18.1265 19.1871 18.6013 19.1871 18.8942 18.8943C19.1871 18.6014 19.1871 18.1265 18.8942 17.8336L17.3032 16.2426C17.0103 15.9497 16.5355 15.9497 16.2426 16.2426C15.9497 16.5355 15.9497 17.0104 16.2426 17.3033L17.8336 18.8943Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 18C12.4142 18 12.75 18.3358 12.75 18.75V21C12.75 21.4142 12.4142 21.75 12 21.75C11.5858 21.75 11.25 21.4142 11.25 21V18.75C11.25 18.3358 11.5858 18 12 18Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.7575 17.3033C8.0504 17.0104 8.0504 16.5355 7.7575 16.2426C7.46461 15.9497 6.98974 15.9497 6.69684 16.2426L5.10585 17.8336C4.81296 18.1265 4.81296 18.6014 5.10585 18.8943C5.39875 19.1872 5.87362 19.1872 6.16651 18.8943L7.7575 17.3033Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 12C6 12.4142 5.66421 12.75 5.25 12.75H3C2.58579 12.75 2.25 12.4142 2.25 12C2.25 11.5858 2.58579 11.25 3 11.25H5.25C5.66421 11.25 6 11.5858 6 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.69673 7.75732C6.98962 8.05021 7.4645 8.05021 7.75739 7.75732C8.05028 7.46443 8.05028 6.98955 7.75739 6.69666L6.1664 5.10567C5.87351 4.81278 5.39863 4.81278 5.10574 5.10567C4.81285 5.39856 4.81285 5.87344 5.10574 6.16633L6.69673 7.75732Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
