use mysql as my;

pub struct Metric {
    pub name: String,
    pub sql: String,
}

#[derive(Serialize)]
pub struct MetricValue {
    pub metric_name: String,
    pub value: i32,
}

pub struct MetricsCollector {
    pool: my::Pool,
}

impl MetricsCollector {
    pub fn connect(connection_string: &str) -> MetricsCollector {
        let pool = my::Pool::new(connection_string)
            .expect("Cannot connect to mysql, check the connection string");

        MetricsCollector { pool }
    }

    pub fn collect_all(&self) -> Vec<MetricValue> {
        self.collect(&self.load_metrics_from_settings())
    }

    fn load_metrics_from_settings(&self) -> Vec<Metric> {
        let metrics_to_collect: Vec<Metric> = self.pool
            .prep_exec("select name, query from medidor_metrics", ())
            .map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    let (name, query) = my::from_row(row);
                    Metric { name, sql: query }
                }).collect()
            }).unwrap();
        
        metrics_to_collect
    }

    fn collect(&self, metrics: &[Metric]) -> Vec<MetricValue> {
        let mut collected = Vec::new();
        for metric in metrics {
            collected.push(self.collect_one(metric));
        }

        collected
    }

    fn collect_one(&self, metric: &Metric) -> MetricValue {
        let mut result = self.pool.prep_exec(&metric.sql, ()).unwrap();
        let value:i32 = my::from_row(result.next().unwrap().unwrap());

        MetricValue { metric_name: String::from(&metric.name), value }
    }
}