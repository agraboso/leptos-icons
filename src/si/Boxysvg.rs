#[cfg(feature = "SiBoxysvg")]
use leptos::*;
#[cfg(feature = "SiBoxysvg")]
///This icon requires the feature `SiBoxysvg` to be enabled.
#[component]
pub fn Boxysvg(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "m3.384 5.028 6.933-4.023L12.05 0l1.716 1.002 6.866 4.01 1.716 1.003v12.054l-1.71.988c-1.712.99-5.136 2.965-6.847 3.954L12.079 24l-1.735-1.002L3.4 18.992 1.665 17.99l-.002-1.992-.01-7.973-.001-1.992 1.732-1.005Zm14.68 9.478c.1.037.23.037.363.037.594 0 1.383-.333 1.779-.763.394-.392.76-1.152.76-1.778 0-.626-.366-1.386-.76-1.78-.396-.43-1.185-.762-1.779-.762-.134 0-.263 0-.398.038.135-.069.23-.169.299-.234.397-.392.76-1.187.76-1.811 0-.592-.363-1.381-.76-1.78-.397-.397-1.189-.755-1.783-.755-.625 0-1.414.358-1.81.755-.064.067-.13.165-.232.268.031-.103.031-.234.031-.368 0-.595-.325-1.38-.755-1.778-.394-.395-1.152-.757-1.78-.757-.626 0-1.384.362-1.781.757-.427.397-.752 1.183-.752 1.778 0 .134 0 .265.028.398-.066-.133-.165-.23-.23-.298-.395-.397-1.183-.755-1.809-.755-.593 0-1.386.358-1.781.755-.399.399-.76 1.188-.76 1.78 0 .624.361 1.419.76 1.811.069.065.162.13.263.234-.101-.038-.23-.038-.364-.038-.594 0-1.383.332-1.779.762-.397.394-.76 1.154-.76 1.78 0 .626.363 1.386.76 1.778.396.43 1.185.763 1.779.763.134 0 .263 0 .398-.037-.135.068-.228.168-.297.233-.399.39-.76 1.185-.76 1.811 0 .59.361 1.381.76 1.779.395.397 1.15.756 1.781.756.626 0 1.414-.36 1.81-.756.064-.066.13-.166.229-.267-.028.101-.028.234-.028.366 0 .59.325 1.381.752 1.778.397.398 1.155.759 1.781.759.628 0 1.386-.361 1.78-.759.43-.397.755-1.188.755-1.778 0-.132 0-.265-.03-.395.068.13.167.23.231.296.396.397 1.185.756 1.81.756.594 0 1.386-.36 1.783-.756.397-.398.76-1.189.76-1.779 0-.626-.363-1.42-.76-1.81-.069-.066-.164-.133-.263-.234Zm-1.547.591h.028c.794 0 1.425.628 1.425 1.453 0 .79-.631 1.416-1.425 1.416-.82 0-1.452-.626-1.452-1.416v-.034l-2.103-2.11v2.968c.263.263.459.621.459 1.054 0 .79-.659 1.419-1.45 1.419-.79 0-1.448-.63-1.448-1.419 0-.433.164-.79.461-1.054v-2.968l-2.11 2.11v.034c0 .79-.657 1.416-1.447 1.416a1.41 1.41 0 0 1-1.423-1.416c0-.825.63-1.453 1.423-1.453h.03l2.107-2.108H6.628a1.36 1.36 0 0 1-1.055.464c-.79 0-1.416-.659-1.416-1.451 0-.795.626-1.452 1.416-1.452.433 0 .792.2 1.055.463h2.964L7.485 8.906h-.03c-.794 0-1.423-.664-1.423-1.453 0-.79.63-1.417 1.423-1.417.82 0 1.447.628 1.447 1.417v.033l2.11 2.111v-2.97c-.263-.263-.46-.62-.46-1.054 0-.787.658-1.417 1.447-1.417.791 0 1.45.63 1.45 1.417 0 .434-.164.791-.46 1.055v2.97l2.104-2.112v-.033c0-.79.664-1.417 1.452-1.417.794 0 1.425.628 1.425 1.417 0 .79-.631 1.453-1.425 1.453h-.028l-2.109 2.107h2.965c.263-.263.623-.463 1.054-.463.79 0 1.416.657 1.416 1.452 0 .792-.626 1.451-1.416 1.451a1.36 1.36 0 0 1-1.054-.464h-2.965l2.109 2.108Z"
        /> < title > { title } < / title > < / svg >
    }
}
