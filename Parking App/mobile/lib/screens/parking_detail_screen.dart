import 'package:flutter/material.dart';
import 'package:parking_app/utils/theme.dart';

class ParkingDetailScreen extends StatelessWidget {
  final String parkingId;

  const ParkingDetailScreen({
    super.key,
    required this.parkingId,
  });

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: CustomScrollView(
        slivers: [
          // App Bar
          SliverAppBar(
            expandedHeight: 200,
            pinned: true,
            backgroundColor: Colors.white,
            flexibleSpace: FlexibleSpaceBar(
              title: const Text(
                'Central Parking Garage',
                style: TextStyle(
                  color: AppTheme.primaryTextColor,
                  fontWeight: FontWeight.bold,
                ),
              ),
              background: Container(
                color: AppTheme.surfaceColor,
                child: const Center(
                  child: Icon(
                    Icons.local_parking,
                    size: 64,
                    color: AppTheme.primaryColor,
                  ),
                ),
              ),
            ),
          ),

          // Content
          SliverToBoxAdapter(
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  // Availability Card
                  Container(
                    padding: const EdgeInsets.all(20),
                    decoration: BoxDecoration(
                      color: AppTheme.successColor.withOpacity(0.1),
                      borderRadius: BorderRadius.circular(16),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        const Text(
                          'Estimated Availability',
                          style: TextStyle(
                            fontSize: 14,
                            color: AppTheme.secondaryTextColor,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Row(
                          children: [
                            Text(
                              '82%',
                              style: const TextStyle(
                                fontSize: 36,
                                fontWeight: FontWeight.bold,
                                color: AppTheme.successColor,
                              ),
                            ),
                            const SizedBox(width: 12),
                            const Text(
                              'High Chance',
                              style: TextStyle(
                                fontSize: 16,
                                fontWeight: FontWeight.w600,
                                color: AppTheme.successColor,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        const Text(
                          'Based on historical data and recent reports',
                          style: TextStyle(
                            fontSize: 12,
                            color: AppTheme.secondaryTextColor,
                          ),
                        ),
                      ],
                    ),
                  ),

                  const SizedBox(height: 24),

                  // Details
                  _buildDetailRow(
                    icon: Icons.location_on,
                    label: 'Distance',
                    value: '180 m',
                  ),
                  _buildDetailRow(
                    icon: Icons.directions_car,
                    label: 'Driving time',
                    value: '3 min',
                  ),
                  _buildDetailRow(
                    icon: Icons.directions_walk,
                    label: 'Walking time',
                    value: '2 min',
                  ),
                  _buildDetailRow(
                    icon: Icons.star,
                    label: 'Rating',
                    value: '4.5 (234 reviews)',
                  ),
                  _buildDetailRow(
                    icon: Icons.access_time,
                    label: 'Hours',
                    value: 'Open 24/7',
                  ),
                  _buildDetailRow(
                    icon: Icons.attach_money,
                    label: 'Price',
                    value: '\$5/hour',
                  ),

                  const SizedBox(height: 24),

                  // Action Buttons
                  SizedBox(
                    width: double.infinity,
                    child: ElevatedButton(
                      onPressed: () {
                        // Navigate
                      },
                      child: const Text('Navigate'),
                    ),
                  ),
                  const SizedBox(height: 12),
                  Row(
                    children: [
                      Expanded(
                        child: OutlinedButton(
                          onPressed: () {
                            // Save
                          },
                          child: const Text('Save'),
                        ),
                      ),
                      const SizedBox(width: 12),
                      Expanded(
                        child: OutlinedButton(
                          onPressed: () {
                            // Report
                          },
                          child: const Text('Report'),
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

  Widget _buildDetailRow({
    required IconData icon,
    required String label,
    required String value,
  }) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: Row(
        children: [
          Icon(
            icon,
            color: AppTheme.secondaryTextColor,
            size: 20,
          ),
          const SizedBox(width: 12),
          Text(
            label,
            style: const TextStyle(
              color: AppTheme.secondaryTextColor,
              fontSize: 14,
            ),
          ),
          const Spacer(),
          Text(
            value,
            style: const TextStyle(
              color: AppTheme.primaryTextColor,
              fontSize: 14,
              fontWeight: FontWeight.w500,
            ),
          ),
        ],
      ),
    );
  }
}
