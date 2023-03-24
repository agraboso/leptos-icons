#[cfg(feature = "HiMdSolidCog8Tooth")]
use leptos::*;
#[cfg(feature = "HiMdSolidCog8Tooth")]
///This icon requires the feature `HiMdSolidCog8Tooth` to be enabled.
#[component]
pub fn Cog8Tooth(
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
        "M8.33922 1.80388C8.43271 1.33646 8.84312 1 9.3198 1H10.6802C11.1569 1 11.5673 1.33646 11.6608 1.80388L11.9553 3.27675C12.4522 3.42101 12.9263 3.61886 13.3709 3.86363L14.6212 3.03014C15.0178 2.76572 15.5459 2.81802 15.883 3.15508L16.8449 4.11702C17.182 4.45409 17.2343 4.98221 16.9699 5.37883L16.1364 6.62908C16.3811 7.07369 16.579 7.54778 16.7232 8.04465L18.1961 8.33922C18.6635 8.43271 19 8.84312 19 9.3198V10.6802C19 11.1569 18.6635 11.5673 18.1961 11.6608L16.7232 11.9553C16.579 12.4522 16.3812 12.9262 16.1364 13.3708L16.97 14.6212C17.2344 15.0178 17.1821 15.5459 16.845 15.883L15.8831 16.8449C15.546 17.182 15.0179 17.2343 14.6213 16.9699L13.371 16.1363C12.9264 16.3811 12.4522 16.579 11.9554 16.7232L11.6608 18.1961C11.5673 18.6635 11.1569 19 10.6802 19H9.3198C8.84312 19 8.43271 18.6635 8.33922 18.1961L8.04465 16.7232C7.54778 16.579 7.0737 16.3811 6.62908 16.1364L5.37882 16.9699C4.9822 17.2343 4.45408 17.182 4.11701 16.8449L3.15507 15.883C2.81801 15.5459 2.76571 15.0178 3.03013 14.6212L3.86363 13.3709C3.61886 12.9263 3.42101 12.4522 3.27675 11.9554L1.80388 11.6608C1.33646 11.5673 1 11.1569 1 10.6802L1 9.3198C1 8.84312 1.33646 8.43271 1.80388 8.33922L3.27675 8.04465C3.42102 7.54774 3.61889 7.07363 3.86368 6.62898L3.03024 5.37882C2.76582 4.9822 2.81812 4.45408 3.15518 4.11701L4.11712 3.15507C4.45419 2.81801 4.98231 2.76571 5.37893 3.03013L6.62913 3.86359C7.07373 3.61884 7.5478 3.42101 8.04465 3.27675L8.33922 1.80388ZM13 10C13 11.6569 11.6569 13 10 13C8.34315 13 7 11.6569 7 10C7 8.34315 8.34315 7 10 7C11.6569 7 13 8.34315 13 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
