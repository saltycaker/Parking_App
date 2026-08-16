import 'package:flutter/material.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:parking_web_flutter/utils/theme.dart';

class NavigationScreen extends StatelessWidget {
  final String parkingId;

  const NavigationScreen({
    super.key,
    required this.parkingId,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Navigation'),
        backgroundColor: Colors.white,
        foregroundColor: Colors.black,
        elevation: 0,
      ),
      body: Stack(
        children: [
          // Map with navigation route
          FlutterMap(
            options: MapOptions(
              initialCenter: LatLng(37.7749, -122.4194),
              initialZoom: 15.0,
              interactionOptions: const InteractionOptions(
                flags: InteractiveFlag.all & ~InteractiveFlag.rotate,
              ),
            ),
            children: [
              TileLayer(
                urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
                userAgentPackageName: 'com.example.parking_web_flutter',
              ),
              // Navigation route (purple line)
              PolylineLayer(
                polylines: [
                  Polyline(
                    points: [
                      LatLng(37.7749, -122.4194), // Start
                      LatLng(37.7760, -122.4180), // Waypoint
                      LatLng(37.7770, -122.4170), // End
                    ],
                    strokeWidth: 5.0,
                    color: Colors.black,
                  ),
                ],
              ),
              MarkerLayer(
                markers: [
                  // Start marker
                  Marker(
                    point: LatLng(37.7749, -122.4194),
                    width: 30,
                    height: 30,
                    child: Container(
                      decoration: BoxDecoration(
                        color: Colors.black,
                        borderRadius: BorderRadius.circular(15),
                        border: Border.all(color: Colors.white, width: 3),
                      ),
                      child: const Icon(
                        Icons.my_location,
                        color: Colors.white,
                        size: 16,
                      ),
                    ),
                  ),
                  // End marker
                  Marker(
                    point: LatLng(37.7770, -122.4170),
                    width: 30,
                    height: 30,
                    child: Container(
                      decoration: BoxDecoration(
                        color: AppTheme.successColor,
                        borderRadius: BorderRadius.circular(15),
                        border: Border.all(color: Colors.white, width: 3),
                      ),
                      child: const Icon(
                        Icons.local_parking,
                        color: Colors.white,
                        size: 16,
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),

          // Navigation info card
          Positioned(
            left: 16,
            right: 16,
            bottom: 16,
            child: Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: Colors.white,
                borderRadius: BorderRadius.circular(12),
                boxShadow: [
                  BoxShadow(
                    color: Colors.black.withOpacity(0.1),
                    blurRadius: 20,
                    offset: const Offset(0, -4),
                  ),
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    '5 min',
                    style: TextStyle(
                      fontSize: 24,
                      fontWeight: FontWeight.w600,
                      color: Colors.black,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '250m • Downtown Parking Garage',
                    style: TextStyle(
                      fontSize: 14,
                      color: AppTheme.secondaryTextColor,
                    ),
                  ),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      Expanded(
                        child: ElevatedButton(
                          onPressed: () {
                            Navigator.pop(context);
                          },
                          style: ElevatedButton.styleFrom(
                            backgroundColor: Colors.white,
                            foregroundColor: Colors.black,
                            elevation: 0,
                            side: const BorderSide(color: Colors.black),
                          ),
                          child: const Text('Cancel'),
                        ),
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: ElevatedButton(
                          onPressed: () {
                            // Start navigation
                          },
                          child: const Text('Start'),
                        ),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
