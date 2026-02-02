#[cfg(test)]
mod tests {
    use visualisation_module::{Config, Metrics, ModuleType};

    #[test]
    fn test_config_loading() {
        let pool_ip = Config::get_pool_ip();
        assert!(!pool_ip.is_empty());
        
        let pool_port = Config::get_pool_port();
        assert!(pool_port > 0);
    }

    #[test]
    fn test_metrics_creation() {
        let metrics = Metrics::new();
        assert!(metrics.avg_cpu() >= 0.0);
        assert!(metrics.avg_ram() >= 0);
    }

    #[test]
    fn test_metrics_fps() {
        let metrics = Metrics::new();
        metrics.update_fps(ModuleType::Screen, 60);
        metrics.update_fps(ModuleType::Screen, 55);
        
        let avg = metrics.avg_fps(ModuleType::Screen);
        assert!(avg > 0);
    }

    #[test]
    fn test_metrics_packets() {
        let metrics = Metrics::new();
        metrics.add_packets(ModuleType::Screen, 10);
        metrics.add_packets(ModuleType::Screen, 5);
        
        let total = metrics.get_packets(ModuleType::Screen);
        assert_eq!(total, 15);
    }

    #[test]
    fn test_summary() {
        let metrics = Metrics::new();
        metrics.update_fps(ModuleType::Screen, 60);
        metrics.add_packets(ModuleType::Screen, 100);
        
        let summary = metrics.get_summary();
        assert_eq!(summary.packets_screen, 100);
    }
}
