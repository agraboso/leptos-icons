#[cfg(feature = "VsSaveAs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsSaveAs")]
/// *This icon requires the feature* `VsSaveAs` *to be enabled*.
#[component]
pub fn SaveAs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M11.04 1.33L12.71 3l.29.71v.33h-.5l-.5.5v-.83l-1.67-1.67H10v4H4v-4H2v10h3l-.5 1H2l-1-1v-10l1-1h8.33l.71.29zM7 5h2V2H7v3zm6.5 0L15 6.5l-.02.69-5.5 5.5-.13.12-.37.37-.1.09-3 1.5-.67-.67 1.5-3 .09-.1.37-.37.12-.13 5.5-5.5h.71zm-6.22 7.24l-.52 1 1.04-.48-.52-.52zm.69-1.03l.79.79 5.15-5.15-.79-.79-5.15 5.15z" /></svg>
   }
}