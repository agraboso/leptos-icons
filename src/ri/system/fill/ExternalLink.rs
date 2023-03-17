#[cfg(feature = "RiSystemFillExternalLink")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillExternalLink")]
/// *This icon requires the feature* `RiSystemFillExternalLink` *to be enabled*.
#[component]
pub fn ExternalLink(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 6v2H5v11h11v-5h2v6a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h6zm11-3v9l-3.794-3.793-5.999 6-1.414-1.414 5.999-6L12 3h9z" /></g></svg>
   }
}