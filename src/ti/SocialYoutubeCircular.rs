#[cfg(feature = "TiSocialYoutubeCircular")]
use leptos::*;
#[cfg(feature = "TiSocialYoutubeCircular")]
///This icon requires the feature `TiSocialYoutubeCircular` to be enabled.
#[component]
pub fn SocialYoutubeCircular(
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
        stroke_witdh = "0" style = style version = "1.1" id = "Layer_1" width = "24px"
        height = "24px" viewBox = "0 0 24 24" width = { size.clone() } height = { size }
        > < polygon xmlns = "http://www.w3.org/2000/svg" points =
        "8.48,13.14 9.21,13.14 9.21,16.75 9.91,16.75 9.91,13.14 10.64,13.14 10.64,12.53 8.48,12.53 "
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.17,16c-0.12,0.14-0.53,0.42-0.53,0.02v-2.39h-0.62v2.61c0,0.79,0.79,0.58,1.16,0.17v0.34h0.62v-3.12h-0.62V16H12.17z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.48,13.61c-0.36,0-0.59,0.27-0.59,0.27v-1.36h-0.63v4.23h0.63v-0.24c0,0,0.21,0.28,0.59,0.28c0.33,0,0.58-0.29,0.58-0.69&#xD;&#xA;	c0,0,0-1.26,0-1.73S14.84,13.61,14.48,13.61z M14.41,16.02c0,0.23-0.16,0.34-0.37,0.25c-0.05-0.02-0.1-0.06-0.15-0.11v-1.94&#xD;&#xA;	c0.04-0.04,0.09-0.07,0.13-0.1c0.22-0.11,0.39,0.06,0.39,0.29L14.41,16.02L14.41,16.02z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.72,15.86c0,0.24-0.13,0.4-0.28,0.41c-0.16,0.01-0.32-0.12-0.32-0.41v-0.59h1.19v-0.8c0-0.29-0.11-0.51-0.26-0.66&#xD;&#xA;	c-0.17-0.16-0.4-0.24-0.63-0.24c-0.22,0-0.45,0.07-0.63,0.21c-0.19,0.15-0.31,0.38-0.31,0.69v1.4c0,0.28,0.09,0.5,0.23,0.66&#xD;&#xA;	c0.17,0.18,0.4,0.28,0.64,0.29c0.29,0.01,0.6-0.11,0.78-0.36c0.11-0.15,0.18-0.35,0.18-0.59v-0.16h-0.59&#xD;&#xA;	C16.72,15.71,16.72,15.76,16.72,15.86z M16.12,14.47c0-0.17,0.1-0.37,0.29-0.37s0.31,0.18,0.31,0.37s0,0.32,0,0.32h-0.6&#xD;&#xA;	C16.12,14.78,16.12,14.64,16.12,14.47z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.97,3c-4.97,0-9,4.03-9,9s4.03,9,9,9s9-4.03,9-9S17.94,3,12.97,3z M14.55,6.37h0.8v2.68c0,0.17,0.08,0.17,0.11,0.17&#xD;&#xA;	c0.12,0,0.3-0.13,0.39-0.22V6.36h0.8V9.9h-0.8V9.59c-0.11,0.1-0.22,0.18-0.33,0.24c-0.15,0.08-0.29,0.11-0.43,0.11&#xD;&#xA;	c-0.18,0-0.31-0.06-0.41-0.17c-0.09-0.11-0.13-0.28-0.13-0.49V6.37z M12,7.3c0-0.55,0.45-1,1-1s1,0.45,1,1V9c0,0.55-0.45,1-1,1&#xD;&#xA;	s-1-0.45-1-1V7.3z M9.92,5.15l0.48,1.76l0.49-1.76h0.91l-0.94,2.78V9.9H9.97V7.93L9.01,5.15H9.92z M17.82,17.69&#xD;&#xA;	c-0.51,0.5-4.83,0.51-4.83,0.51s-4.31-0.01-4.83-0.51c-0.51-0.5-0.51-2.99-0.51-3.01c0-0.01,0-2.5,0.51-3.01&#xD;&#xA;	c0.51-0.5,4.83-0.51,4.83-0.51s4.31,0.01,4.83,0.51c0.51,0.5,0.52,2.99,0.52,3.01C18.34,14.68,18.34,17.18,17.82,17.69z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.98,9.35c0.17,0,0.25-0.1,0.26-0.26v-1.9c0-0.13-0.13-0.24-0.25-0.24s-0.25,0.1-0.25,0.24v1.9&#xD;&#xA;	C12.74,9.24,12.81,9.34,12.98,9.35z"
        /> < title > { title } < / title > < / svg >
    }
}
