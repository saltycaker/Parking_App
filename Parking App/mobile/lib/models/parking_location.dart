import 'package:json_annotation/json_annotation.dart';

part 'parking_location.g.dart';

@JsonSerializable()
class ParkingLocation {
  final String id;
  final String name;
  final String address;
  final double latitude;
  final double longitude;
  final String? placeId;
  final String parkingType;
  final bool? isCovered;
  final bool? hasEvCharging;
  final bool? isFree;
  final bool? isWheelchairAccessible;
  final double? heightRestrictionM;
  final double? rating;
  final int? reviewCount;
  final String? phone;
  final String? website;
  final Map<String, dynamic>? openingHours;
  final List<String>? photos;
  final DateTime createdAt;
  final DateTime updatedAt;

  ParkingLocation({
    required this.id,
    required this.name,
    required this.address,
    required this.latitude,
    required this.longitude,
    this.placeId,
    required this.parkingType,
    this.isCovered,
    this.hasEvCharging,
    this.isFree,
    this.isWheelchairAccessible,
    this.heightRestrictionM,
    this.rating,
    this.reviewCount,
    this.phone,
    this.website,
    this.openingHours,
    this.photos,
    required this.createdAt,
    required this.updatedAt,
  });

  factory ParkingLocation.fromJson(Map<String, dynamic> json) =>
      _$ParkingLocationFromJson(json);

  Map<String, dynamic> toJson() => _$ParkingLocationToJson(this);
}
