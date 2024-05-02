use super::point::Point;

///An implementation of a zone that is better than the default zone.
#[derive(Debug, Clone, PartialEq)]
pub struct Zone {
    pub start: Point,
    pub end: Point,
    pub size: f64,
}
#[allow(dead_code)]

impl Zone {
    pub fn new(start: Point, end: Point) -> Self {
        let mut zone = Self {
            start,
            end,
            size: 0.0,
        };
        zone.size = zone.start.distance(&zone.end);
        zone
    }
}
#[allow(dead_code)]

impl Zone {
    pub fn area(&self, max_x: u32, max_y: u32) -> (u32, u32, u32, u32) {
        let min_x = self.start.x.min(self.end.x);
        let min_y = self.start.y.min(self.end.y);
        let max_x = self.start.x.max(self.end.x).min(max_x);
        let max_y = self.start.y.max(self.end.y).min(max_y);
        (min_x, min_y, max_x, max_y)
    }

    pub fn includes(&self, zone: Zone) -> bool {
        let (min_x, min_y, max_x, max_y) = zone.area(800, 600);
        let (min_x2, min_y2, max_x2, max_y2) = self.area(800, 600);
        min_x >= min_x2 && min_y >= min_y2 && max_x <= max_x2 && max_y <= max_y2
    }

    pub fn is_inside(&self, point: &Point) -> bool {
        point.is_inside(&self.start, &self.end)
    }

    pub fn extend(&mut self, zone: &mut Zone) -> Option<&Zone> {
        let (min_x, min_y, max_x, max_y) = zone.area(800, 600);
        let (min_x2, min_y2, max_x2, max_y2) = self.area(800, 600);
        if self.start == zone.start {
            if self.end.x < zone.end.x {
                self.end.x = zone.end.x;
            }
            if self.end.y < zone.end.y {
                self.end.y = zone.end.y;
            }
            return Some(self);
        } else if self.end == zone.end {
            if self.start.x > zone.start.x {
                self.start.x = zone.start.x;
            }
            if self.start.y > zone.start.y {
                self.start.y = zone.start.y;
            }
            return Some(self);
        } else if self.start == zone.end {
            if self.end.x < zone.start.x {
                self.end.x = zone.start.x;
            }
            if self.end.y < zone.start.y {
                self.end.y = zone.start.y;
            }
            return Some(self);
        } else if self.end == zone.start {
            if self.start.x > zone.end.x {
                self.start.x = zone.end.x;
            }
            if self.start.y > zone.end.y {
                self.start.y = zone.end.y;
            }
            return Some(self);
        } else if min_x >= min_x2 && min_y >= min_y2 && max_x <= max_x2 && max_y <= max_y2 {
            return Some(self);
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZoneManager {
    zones: Vec<Zone>,
    width: u32,
    height: u32,
}
#[allow(dead_code)]

impl ZoneManager {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            zones: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_zone(&mut self, zone: Zone) {
        let exist_zone = self.zones.iter().find(|z| *z == &zone);
        if exist_zone.is_none() {
            self.zones.push(zone);
        }
    }

    pub fn extend_zones(&mut self) {
        let zones = self.zones.clone();
        self.zones.clear();
        for mut zone in zones {
            let mut found = false;
            for z in &mut self.zones {
                if z.extend(&mut zone).is_some() {
                    found = true;
                    break;
                }
            }
            if !found {
                self.zones.push(zone);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone() {
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        assert_eq!(zone.size, 14.142135623730951);
    }

    #[test]
    fn test_zone_area() {
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let (min_x, min_y, max_x, max_y) = zone.area(800, 600);
        assert_eq!(min_x, 0);
        assert_eq!(min_y, 0);
        assert_eq!(max_x, 10);
        assert_eq!(max_y, 10);
    }

    #[test]
    fn test_zone_includes() {
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let zone2 = Zone::new(Point::new(0, 0), Point::new(10, 10));
        assert_eq!(zone.includes(zone2), true);

        let zone3 = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let zone4 = Zone::new(Point::new(0, 0), Point::new(5, 5));
        assert_eq!(zone3.includes(zone4), true);
    }

    #[test]
    fn test_zone_is_inside() {
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let point = Point::new(5, 5);
        assert_eq!(zone.is_inside(&point), true);
    }

    #[test]
    fn test_zone_extend() {
        let mut zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let mut zone2 = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let mut zone3 = Zone::new(Point::new(0, 0), Point::new(5, 5));
        assert_eq!(zone.extend(&mut zone2).is_some(), true);
        assert_eq!(zone.extend(&mut zone3).is_some(), true);
    }

    #[test]
    fn test_zone_manager() {
        let mut zone_manager = ZoneManager::new(800, 600);
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        zone_manager.add_zone(zone);
        zone_manager.extend_zones();
        assert_eq!(zone_manager.zones.len(), 1);
    }

    #[test]
    fn test_zone_manager_extend() {
        let mut zone_manager = ZoneManager::new(800, 600);
        let zone = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let zone2 = Zone::new(Point::new(0, 0), Point::new(5, 5));

        zone_manager.add_zone(zone);
        zone_manager.add_zone(zone2);
        zone_manager.extend_zones();
        assert_eq!(zone_manager.zones.len(), 1);
        assert_eq!(zone_manager.zones[0].start, Point::new(0, 0));
        assert_eq!(zone_manager.zones[0].end, Point::new(10, 10));

        let zone3 = Zone::new(Point::new(0, 0), Point::new(5, 5));
        let zone4 = Zone::new(Point::new(0, 0), Point::new(10, 10));
        let zone5 = Zone::new(Point::new(0, 0), Point::new(15, 15));
        let zone6 = Zone::new(Point::new(0, 0), Point::new(20, 20));
        let zone7 = Zone::new(Point::new(5, 5), Point::new(25, 25));

        zone_manager.add_zone(zone3);
        zone_manager.add_zone(zone4);
        zone_manager.add_zone(zone5);
        zone_manager.add_zone(zone6);
        zone_manager.add_zone(zone7);
        zone_manager.extend_zones();

        assert_eq!(zone_manager.zones.len(), 1);
    }
}
