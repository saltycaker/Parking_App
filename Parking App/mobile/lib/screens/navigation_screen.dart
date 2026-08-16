import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:parking_app/utils/theme.dart';

class NavigationScreen extends StatefulWidget {
  final String parkingId;

  const NavigationScreen({
    super.key,
    required this.parkingId,
  });

  @override
  State<NavigationScreen> createState() => _NavigationScreenState();
}

class _NavigationScreenState extends State<NavigationScreen> {
  final MapController _mapController = MapController();
  List<LatLng> _routePoints = [];
  List<LatLng> _alternateRoutePoints = [];

  @override
  void initState() {
    super.initState();
    _initializeNavigation();
  }

  Future<void> _initializeNavigation() async {
    // Create route points (purple color for main route)
    setState(() {
      _routePoints = const [
        LatLng(37.7749, -122.4194), // Start
        LatLng(37.7755, -122.4185), // Waypoint
        LatLng(37.7762, -122.4178), // End
      ];

      // Create alternate route points (gray color)
      _alternateRoutePoints = const [
        LatLng(37.7749, -122.4194), // Start
        LatLng(37.7750, -122.4180), // Alternate waypoint
        LatLng(37.7762, -122.4178), // End
      ];
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          // Map
          FlutterMap(
            mapController: _mapController,
            options: MapOptions(
              initialCenter: LatLng(37.7755, -122.4185),
              initialZoom: 16.0,
              interactionOptions: const InteractionOptions(
                flags: InteractiveFlag.all & ~InteractiveFlag.rotate,
              ),
            ),
            children: [
              TileLayer(
                urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
                userAgentPackageName: 'com.example.parking_app',
              ),
              // Main route (purple)
              PolylineLayer(
                polylines: [
                  Polyline(
                    points: _routePoints,
                    strokeWidth: 5.0,
                    color: AppTheme.primaryColor,
                  ),
                ],
              ),
              // Alternate route (gray)
              PolylineLayer(
                polylines: [
                  Polyline(
                    points: _alternateRoutePoints,
                    strokeWidth: 3.0,
                    color: Colors.grey.shade300,
                  ),
                ],
              ),
              // Markers
              MarkerLayer(
                markers: [
                  // Start marker
                  Marker(
                    point: const LatLng(37.7749, -122.4194),
                    width: 40,
                    height: 40,
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.blue,
                        borderRadius: BorderRadius.circular(20),
                        border: Border.all(color: Colors.white, width: 3),
                      ),
                      child: const Icon(
                        Icons.location_on,
                        color: Colors.white,
                        size: 20,
                      ),
                    ),
                  ),
                  // End marker
                  Marker(
                    point: const LatLng(37.7762, -122.4178),
                    width: 40,
                    height: 40,
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.green,
                        borderRadius: BorderRadius.circular(20),
                        border: Border.all(color: Colors.white, width: 3),
                      ),
                      child: const Icon(
                        Icons.local_parking,
                        color: Colors.white,
                        size: 20,
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),

          // ETA Card
          Positioned(
            top: MediaQuery.of(context).padding.top + 16,
            left: 16,
            right: 16,
            child: Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(16),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.1),
                    blurRadius: 10,
                    offset: const Offset(0, 2),
                  ),
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Central Parking Garage',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: AppTheme.primaryTextColor,
                    ),
                  ),
                  const SizedBox(height: 8),
                  Row(
                    children: [
                      Text(
                        '3 min',
                        style: const TextStyle(
                          fontSize: 32,
                          fontWeight: FontWeight.bold,
                          color: AppTheme.primaryTextColor,
                        ),
                      ),
                      const SizedBox(width: 8),
                      Text(
                        '(0.8 km)',
                        style: TextStyle(
                          fontSize: 16,
                          color: AppTheme.secondaryTextColor,
                        ),
                      ),
                    ],
                  ),
                  const SizedBox(height: 4),
                  Text(
                    'Arrival at ${_formatArrivalTime()}',
                    style: TextStyle(
                      fontSize: 14,
                      color: AppTheme.secondaryTextColor,
                    ),
                  ),
                ],
              ),
            ),
          ),

          // Cancel Button
          Positioned(
            bottom: MediaQuery.of(context).padding.bottom + 20,
            left: 16,
            right: 16,
            child: SizedBox(
              width: double.infinity,
              child: ElevatedButton(
                onPressed: () {
                  Navigator.pop(context);
                },
                style: ElevatedButton.styleFrom(
                  backgroundColor: Colors.white,
                  foregroundColor: AppTheme.errorColor,
                  elevation: 0,
                ),
                child: const Text('Cancel Navigation'),
              ),
            ),
          ),
        ],
      ),
    );
  }

  String _formatArrivalTime() {
    final now = DateTime.now();
    final arrival = now.add(const Duration(minutes: 3));
    return '${arrival.hour}:${arrival.minute.toString().padLeft(2, '0')}';
  }
}
