use leptos::*;
use leptos::*;
use web_sys::{HtmlElement, window};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "google.maps.Map")]
    pub type GoogleMap;

    #[wasm_bindgen(typescript_type = "google.maps.Marker")]
    pub type GoogleMarker;

    #[wasm_bindgen(typescript_type = "google.maps.LatLng")]
    pub type LatLng;

    #[wasm_bindgen(typescript_type = "google.maps.MapOptions")]
    pub type MapOptions;
}

#[component]
pub fn GoogleMapComponent(
    latitude: f64,
    longitude: f64,
    zoom: u32,
    on_marker_click: Callback<(f64, f64)>,
) -> impl IntoView {
    let map_ref = create_node_ref::<HtmlElement>();
    let (map_loaded, set_map_loaded) = create_signal(false);

    // Load Google Maps API
    create_effect(move |_| {
        if !map_loaded.get() {
            load_google_maps_api();
        }
    });

    // Initialize map when element is mounted
    create_effect(move |_| {
        if let Some(element) = map_ref.get() {
            if map_loaded.get() {
                initialize_map(&element, latitude, longitude, zoom);
            }
        }
    });

    view! {
        <div
            node_ref=map_ref
            class="w-full h-full"
            id="map"
        ></div>
    }
}

fn load_google_maps_api() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    
    // Check if API is already loaded
    if let Some(_) = document.get_element_by_id("google-maps-script") {
        return;
    }

    let script = document.create_element("script").unwrap();
    script.set_id("google-maps-script");
    script.set_attribute(
        "src",
        "https://maps.googleapis.com/maps/api/js?key=YOUR_API_KEY&callback=initMap"
    ).unwrap();
    
    // This would need to be set up properly with the actual API key
    document.body().unwrap().append_child(&script).unwrap();
}

fn initialize_map(element: &HtmlElement, lat: f64, lng: f64, zoom: u32) {
    // This would initialize the actual Google Maps instance
    // For now, this is a placeholder
    log::info!("Initializing map at coordinates: {}, {}", lat, lng);
}
