use crate::abstractions::dbg_panic;
use crate::api::telemetry::metrics::MetricAttributes;
use anyhow::anyhow;
use fmt::Debug;
use opentelemetry::KeyValue;
use opentelemetry::metrics::Meter;
use opentelemetry_sdk::metrics::data::ResourceMetrics;
use opentelemetry_sdk::metrics::{InMemoryMetricExporter, SdkMeterProvider};
use std::{fmt, sync::Arc};
use temporal_sdk_core_api::telemetry::metrics::{
    CoreMeter, Counter, Gauge, GaugeF64, Histogram, HistogramDuration, HistogramF64,
    MetricParameters, NewAttributes,
};

#[derive(Debug, Clone)]
pub struct InMemoryMeter {
    pub(crate) meter: Meter,
    pub(crate) in_memory_exporter: InMemoryMetricExporter,
    pub(crate) mp: SdkMeterProvider,
}

// enum DurationHistogram {
//     Milliseconds(crate::telemetry::prometheus_meter::PromHistogramU64),
//     Seconds(crate::telemetry::prometheus_meter::PromHistogramF64),
// }

impl InMemoryMeter {
    pub(crate) fn new(
        in_memory_exporter: InMemoryMetricExporter,
        meter: Meter,
        mp: SdkMeterProvider,
    ) -> InMemoryMeter {
        Self {
            meter,
            in_memory_exporter,
            mp,
        }
    }

    // fn meter_provider(&self) -> SdkMeterProvider {
    //     self.mp.clone()
    // }

    // fn in_mem_exporter(&self) -> InMemoryMetricExporter {
    //     self.in_memory_exporter.clone()
    // }

    pub(crate) fn get_metrics(&self) -> Result<Vec<ResourceMetrics>, anyhow::Error> {
        self.mp
            .force_flush()
            .map_err(|e| anyhow!("failed to flush MeterProvider: {}", e))?;
        self.in_memory_exporter
            .get_finished_metrics()
            .map_err(Into::into)
    }
}

impl CoreMeter for InMemoryMeter {
    fn new_attributes(&self, attribs: NewAttributes) -> MetricAttributes {
        MetricAttributes::OTel {
            kvs: Arc::new(attribs.attributes.into_iter().map(KeyValue::from).collect()),
        }
    }

    fn extend_attributes(
        &self,
        existing: MetricAttributes,
        attribs: NewAttributes,
    ) -> MetricAttributes {
        if let MetricAttributes::OTel { mut kvs } = existing {
            Arc::make_mut(&mut kvs).extend(attribs.attributes.into_iter().map(Into::into));
            MetricAttributes::OTel { kvs }
        } else {
            dbg_panic!("Must use OTel attributes with an OTel metric implementation");
            existing
        }
    }

    fn counter(&self, params: MetricParameters) -> Counter {
        Counter::new(Arc::new(
            self.meter
                .u64_counter(params.name)
                .with_unit(params.unit)
                .with_description(params.description)
                .build(),
        ))
    }

    fn histogram(&self, params: MetricParameters) -> Histogram {
        Histogram::new(Arc::new(
            self.meter
                .u64_histogram(params.name)
                .with_unit(params.unit)
                .with_description(params.description)
                .build(),
        ))
    }

    fn histogram_f64(&self, params: MetricParameters) -> HistogramF64 {
        HistogramF64::new(Arc::new(
            self.meter
                .f64_histogram(params.name)
                .with_unit(params.unit)
                .with_description(params.description)
                .build(),
        ))
    }

    // TODO: fix, this just copies otel for now so things compile
    fn histogram_duration(&self, mut params: MetricParameters) -> HistogramDuration {
        params.unit = "s".into();
        HistogramDuration::new(Arc::new(
            crate::telemetry::otel::DurationHistogram::Seconds(
                self.meter
                    .f64_histogram(params.name)
                    .with_unit(params.unit)
                    .with_description(params.description)
                    .build(),
            ),
        ))
    }

    fn gauge(&self, params: MetricParameters) -> Gauge {
        Gauge::new(Arc::new(
            self.meter
                .u64_gauge(params.name)
                .with_unit(params.unit)
                .with_description(params.description)
                .build(),
        ))
    }

    fn gauge_f64(&self, params: MetricParameters) -> GaugeF64 {
        GaugeF64::new(Arc::new(
            self.meter
                .f64_gauge(params.name)
                .with_unit(params.unit)
                .with_description(params.description)
                .build(),
        ))
    }
}
