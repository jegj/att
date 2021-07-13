pub fn autogenerate_task_name(current_date: chrono::Date<chrono::Local>) -> String {
    current_date.format("task_%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use chrono::TimeZone;

    #[test]
    fn test_autogenerate_task_name() {
        let mut dt = Local.ymd(1970, 1, 1);
        assert_eq!(autogenerate_task_name(dt), "task_19700101");
        dt = Local.ymd(2021, 5, 20);
        assert_eq!(autogenerate_task_name(dt), "task_20210520");
    }
}
