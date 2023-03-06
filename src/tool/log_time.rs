//! The log time struct support the time procedures which are used by others.
//! 
//! 
use chrono;
use regex::Regex;
use mockall::{self, automock};

/// A log timer that implements some time procesdures logic
pub struct LogTime {

}

#[automock]
impl LogTime {
    /// return a formated date 
    pub fn standard_date() -> String {
        chrono::Local::now().format("%Y%m%d").to_string()
    }

    /// replace the time var with current time
    /// 0, {d} will be replaced by [year][month][day], mylog{d}.log will be mylog20230306.log
    /// 1, {y} will be replaced by [year], mylog{y}.log will be mylog2023.log
    /// 2, {m} will be replaced by [month], mylog{m}.log will be mylog03.log
    /// 3, {D} will be replaced by [day], mylog{m}.log will be mylog06.log
    pub fn expand_time_var(time_var: &str) -> String {
        let current_time = chrono::Local::now();

        // {d} = [year][month][day], ex: 20230306
        let re = Regex::new(r"\{d\}").unwrap();
        let expanded_time = re.replace_all(time_var, current_time.format("%Y%m%d").to_string()).to_string();

        // {y} = [year], ex: 2023
        let re = Regex::new(r"\{y\}").unwrap();
        let expanded_time = re.replace_all(&expanded_time, current_time.format("%Y").to_string()).to_string();

        // {m} = [month], ex: 03
        let re = Regex::new(r"\{m\}").unwrap();
        let expanded_time = re.replace_all(&expanded_time, current_time.format("%m").to_string()).to_string();

        // {D} = [day], ex: 06
        let re = Regex::new(r"\{D\}").unwrap();
        let expanded_time = re.replace_all(&expanded_time, current_time.format("%d").to_string()).to_string();

        expanded_time
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn mock_standard_date() {
        use mockall_double::double;
        #[double]
        use super::LogTime;

        let ctx = MockLogTime::standard_date_context();
        ctx.expect().returning(|| {
            "19980618".to_string()
        });
        
        assert_eq!("19980618", LogTime::standard_date());
    }

    #[test]
    fn standard_date() {
        assert_eq!(chrono::Local::now().format("%Y%m%d").to_string(), LogTime::standard_date());
    }

    #[test]
    fn expand_time_var() {
        let current_time = chrono::Local::now();

        let s = "mylog{d}.log";
        assert_eq!(format!("mylog{}.log", current_time.format("%Y%m%d")), LogTime::expand_time_var(s));

        let s = "mylog{y}.log";
        assert_eq!(format!("mylog{}.log", current_time.format("%Y")), LogTime::expand_time_var(s));

        let s = "mylog{m}.log";
        assert_eq!(format!("mylog{}.log", current_time.format("%m")), LogTime::expand_time_var(s));

        let s = "mylog{D}.log";
        assert_eq!(format!("mylog{}.log", current_time.format("%d")), LogTime::expand_time_var(s));

        let s = "mylog{y}-{m}-{D}.log";
        assert_eq!(format!("mylog{}-{}-{}.log", 
                    current_time.format("%Y"),
                    current_time.format("%m"),
                    current_time.format("%d")), 
                    LogTime::expand_time_var(s));
    } 
}