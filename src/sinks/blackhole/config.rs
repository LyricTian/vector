use crate::config::{DataType, GenerateConfig, SinkConfig, SinkContext};
use crate::sinks::blackhole::sink::BlackholeSink;
use crate::sinks::{Healthcheck, VectorSink};
use futures::{future, FutureExt};
use serde::{Deserialize, Serialize};

const fn default_print_interval_secs() -> u64 {
    1
}

#[derive(Clone, Debug, Derivative, Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
#[derivative(Default)]
pub struct BlackholeConfig {
    #[derivative(Default(value = "1"))]
    #[serde(default = "default_print_interval_secs")]
    pub print_interval_secs: u64,
    pub rate: Option<usize>,
}

#[async_trait::async_trait]
#[typetag::serde(name = "blackhole")]
impl SinkConfig for BlackholeConfig {
    async fn build(&self, cx: SinkContext) -> crate::Result<(VectorSink, Healthcheck)> {
        let sink = BlackholeSink::new(self.clone(), cx.acker());
        let healthcheck = future::ok(()).boxed();

        Ok((VectorSink::Stream(Box::new(sink)), healthcheck))
    }

    fn input_type(&self) -> DataType {
        DataType::all()
    }

    fn sink_type(&self) -> &'static str {
        "blackhole"
    }
}

impl GenerateConfig for BlackholeConfig {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(&Self::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::sinks::blackhole::config::BlackholeConfig;

    #[test]
    fn generate_config() {
        crate::test_util::test_generate_config::<BlackholeConfig>();
    }
}
