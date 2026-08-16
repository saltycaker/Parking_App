import 'package:flutter/material.dart';
import 'package:parking_app/utils/theme.dart';

class ParkingCard extends StatelessWidget {
  final String name;
  final String address;
  final String distance;
  final int availability;
  final String availabilityLevel;
  final double rating;
  final String price;

  const ParkingCard({
    super.key,
    required this.name,
    required this.address,
    required this.distance,
    required this.availability,
    required this.availabilityLevel,
    required this.rating,
    required this.price,
  });

  Color get availabilityColor {
    switch (availabilityLevel.toLowerCase()) {
      case 'high':
        return AppTheme.successColor;
      case 'moderate':
        return AppTheme.warningColor;
      case 'low':
        return AppTheme.errorColor;
      default:
        return AppTheme.secondaryTextColor;
    }
  }

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 12),
      child: InkWell(
        onTap: () {
          // Navigate to parking detail
        },
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              // Availability Indicator
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  color: availabilityColor.withOpacity(0.1),
                  borderRadius: BorderRadius.circular(12),
                ),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Text(
                      '$availability%',
                      style: TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.bold,
                        color: availabilityColor,
                      ),
                    ),
                    Text(
                      availabilityLevel,
                      style: TextStyle(
                        fontSize: 10,
                        color: availabilityColor,
                      ),
                    ),
                  ],
                ),
              ),

              const SizedBox(width: 12),

              // Details
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      name,
                      style: const TextStyle(
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        color: AppTheme.primaryTextColor,
                      ),
                    ),
                    const SizedBox(height: 4),
                    Text(
                      address,
                      style: const TextStyle(
                        fontSize: 12,
                        color: AppTheme.secondaryTextColor,
                      ),
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                    const SizedBox(height: 8),
                    Row(
                      children: [
                        const Icon(
                          Icons.location_on,
                          size: 14,
                          color: AppTheme.secondaryTextColor,
                        ),
                        const SizedBox(width: 4),
                        Text(
                          distance,
                          style: const TextStyle(
                            fontSize: 12,
                            color: AppTheme.secondaryTextColor,
                          ),
                        ),
                        const SizedBox(width: 12),
                        const Icon(
                          Icons.star,
                          size: 14,
                          color: AppTheme.warningColor,
                        ),
                        const SizedBox(width: 4),
                        Text(
                          rating.toString(),
                          style: const TextStyle(
                            fontSize: 12,
                            color: AppTheme.secondaryTextColor,
                          ),
                        ),
                        const SizedBox(width: 12),
                        Text(
                          price,
                          style: const TextStyle(
                            fontSize: 12,
                            fontWeight: FontWeight.w600,
                            color: AppTheme.primaryColor,
                          ),
                        ),
                      ],
                    ),
                  ],
                ),
              ),

              const SizedBox(width: 8),

              // Arrow
              Icon(
                Icons.chevron_right,
                color: AppTheme.secondaryTextColor,
              ),
            ],
          ),
        ),
      ),
    );
  }
}
