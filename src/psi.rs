#[derive(Debug, PartialEq)]
pub struct CPUPressureStallInformation {
    some: PSIMetric,
}

#[derive(Debug, PartialEq)]
pub struct PSIMetric {
    avg10: f32,
    avg60: f32,
    avg300: f32,
    total: u64
}
