#[cfg(feature = "HiMdSolidFire")]
use leptos::*;
#[cfg(feature = "HiMdSolidFire")]
///This icon requires the feature `HiMdSolidFire` to be enabled.
#[component]
pub fn Fire(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M13.4997 4.93762C16.8478 6.87062 17.9949 11.1518 16.0619 14.4998C14.1289 17.8479 9.84775 18.995 6.4997 17.062C3.15166 15.129 2.00453 10.8479 3.93753 7.4998C4.10592 7.20813 4.29214 6.93316 4.49401 6.67548C4.69562 6.41812 5.08463 6.45704 5.28714 6.71368C5.56487 7.06565 5.88119 7.38577 6.22971 7.66764C6.56235 7.93667 7.01647 7.61943 7.00304 7.19183C7.00103 7.12812 7.00003 7.06416 7.00003 6.99997C7.00003 6.08143 7.20643 5.2111 7.57539 4.43282C8.10854 3.30822 8.98111 2.37583 10.0608 1.76798C10.3078 1.62893 10.6112 1.7522 10.7378 2.00584C11.3297 3.1927 12.2651 4.2248 13.4997 4.93762ZM14 12C14 14.2091 12.2092 16 10 16C8.08674 16 6.4791 14.6016 6.09017 12.8183C5.9966 12.3894 6.52967 12.1749 6.90396 12.4045C7.38998 12.7025 7.93731 12.8964 8.50538 12.9685C8.80801 13.0068 9.03609 12.7289 9.01482 12.4246C9.00501 12.2844 9.00002 12.1428 9.00002 12C9.00002 10.5731 9.49812 9.26254 10.3299 8.23269C10.4337 8.10417 10.599 8.04108 10.7612 8.07233C12.6063 8.4278 14 10.0511 14 12Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
