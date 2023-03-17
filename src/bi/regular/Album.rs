#[cfg(feature = "BiRegularAlbum")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlbum")]
/// *This icon requires the feature* `BiRegularAlbum` *to be enabled*.
#[component]
pub fn Album(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="11.99" cy="11.99" r="2.01" /><path d="M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 18a8 8 0 1 1 8-8 8 8 0 0 1-8 8z" /><path d="M12 6a6 6 0 0 0-6 6h2a4 4 0 0 1 4-4z" /></svg>
   }
}