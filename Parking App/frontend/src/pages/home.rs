use leptos::*;
use leptos_router::*;
use crate::types::*;
use crate::services::api;

#[component]
pub fn Home() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (loading, set_loading) = create_signal(false);
    let (results, set_results) = create_signal(Vec::<ParkingResult>::new());
    let (user_location, set_user_location) = create_signal(None::<(f64, f64)>);

    // Get user location on mount
    create_effect(move |_| {
        // Get user location
        set_user_location(Some((37.7749, -122.4194))); // Default to SF
    });

    let search = move |_| {
        let location = user_location.get().unwrap_or((37.7749, -122.4194));
        let query = search_query.get();
        
        set_loading.set(true);

        spawn_local(async move {
            let request = SearchRequest {
                latitude: location.0,
                longitude: location.1,
                radius_m: Some(500),
                query: if query.is_empty() { None } else { Some(query) },
                filters: None,
            };

            match api::search_parking(request).await {
                Ok(response) => {
                    set_results.set(response.results);
                    set_loading.set(false);
                }
                Err(e) => {
                    log::error!("Search failed: {}", e);
                    set_loading.set(false);
                }
            }
        });
    };

    view! {
        <div class="relative min-h-screen">
            <!-- Map Container -->
            <div class="absolute inset-0 z-0 bg-gray-100">
                <div class="w-full h-full flex items-center justify-center">
                    <div class="text-center">
                        <svg class="w-16 h-16 text-gray-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                        <p class="text-gray-600">Map integration using OpenStreetMap</p>
                        <p class="text-sm text-gray-500 mt-2">Parking locations will be displayed here</p>
                    </div>
                </div>
            </div>

            <!-- Search Bar -->
            <div class="absolute top-4 left-4 right-4 z-10">
                <div class="bg-white rounded-2xl shadow-lg p-4">
                    <div class="flex items-center gap-3">
                        <input
                            type="text"
                            placeholder="Search parking..."
                            class="flex-1 outline-none text-gray-800"
                            prop:value=search_query
                            on:input=move |e| set_search_query.set(event_target_value(&e))
                            on:keypress=move |e| {
                                if event_key(&e) == "Enter" {
                                    search(());
                                }
                            }
                        />
                        <button
                            class="bg-purple-600 text-white px-4 py-2 rounded-xl hover:bg-purple-700 transition"
                            on:click=search
                        >
                            "Search"
                        </button>
                    </div>
                </div>
            </div>

            <!-- Current Location Button -->
            <button class="absolute bottom-32 right-4 z-10 bg-white rounded-full shadow-lg p-3 hover:bg-gray-50 transition">
                <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                </svg>
            </button>

            <!-- Profile Button -->
            <button class="absolute top-20 right-4 z-10 bg-white rounded-full shadow-lg p-3 hover:bg-gray-50 transition">
                <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                </svg>
            </button>

            <!-- Results Bottom Sheet -->
            <div class="absolute bottom-0 left-0 right-0 z-10 bg-white rounded-t-3xl shadow-2xl max-h-[70vh] overflow-hidden">
                <div class="p-4">
                    <div class="w-12 h-1 bg-gray-300 rounded-full mx-auto mb-4"></div>
                    
                    if loading.get() {
                        <div class="text-center py-8">
                            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-purple-600"></div>
                            <p class="mt-2 text-gray-600">Searching...</p>
                        </div>
                    } else if results.get().is_empty() {
                        <div class="text-center py-8">
                            <p class="text-gray-600">No parking found nearby</p>
                        </div>
                    } else {
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-bold text-gray-900">Nearby Parking</h2>
                            <A href="/profile" class="text-purple-600 font-semibold">Profile</A>
                        </div>
                        
                        <div class="space-y-3 overflow-y-auto max-h-[50vh]">
                            {results.get().into_iter().map(|result| {
                                view! {
                                    <div class="bg-gray-50 rounded-xl p-4 hover:bg-gray-100 transition cursor-pointer">
                                        <div class="flex items-start gap-3">
                                            <div class=format!(
                                                "w-14 h-14 rounded-xl flex flex-col items-center justify-center {}",
                                                match result.availability.level {
                                                    AvailabilityLevel::High => "bg-green-100",
                                                    AvailabilityLevel::Moderate => "bg-yellow-100",
                                                    AvailabilityLevel::Low => "bg-red-100",
                                                }
                                            )>
                                                <span class=format!(
                                                    "text-lg font-bold {}",
                                                    match result.availability.level {
                                                        AvailabilityLevel::High => "text-green-600",
                                                        AvailabilityLevel::Moderate => "text-yellow-600",
                                                        AvailabilityLevel::Low => "text-red-600",
                                                    }
                                                )>
                                                    {result.availability.score}%
                                                </span>
                                                <span class=format!(
                                                    "text-xs {}",
                                                    match result.availability.level {
                                                        AvailabilityLevel::High => "text-green-600",
                                                        AvailabilityLevel::Moderate => "text-yellow-600",
                                                        AvailabilityLevel::Low => "text-red-600",
                                                    }
                                                )>
                                                    {format!("{:?}", result.availability.level)}
                                                </span>
                                            </div>
                                            <div class="flex-1">
                                                <h3 class="font-semibold text-gray-900">{result.location.name}</h3>
                                                <p class="text-sm text-gray-600">{result.location.address}</p>
                                                <div class="flex items-center gap-4 mt-2 text-sm text-gray-500">
                                                    <span>{format!("{} m", result.distance_m as i32)}</span>
                                                    <span>•</span>
                                                    <span>{format!("{} min", result.walking_time_seconds.unwrap_or(0) / 60)}</span>
                                                    <span>•</span>
                                                    <span>{result.location.rating.unwrap_or(0.0)} ★</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
