use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::data_base::data_base_ascan::ascan_db as db;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAscan{
    pub batch_number_kpc:Vec<String>,
    pub batch_number_6pr:Vec<String>,
    pub malting_for_kpc:Vec<String>,
    pub malting_for_6pr:Vec<String>,
}

impl DataAscan {
    pub async fn new()-> Result<Self, Box<dyn std::error::Error>>{
        let batch_number_kpc = db::batch_list_kpc().await?;
        let batch_number_6pr = db::batch_list_6pr().await?;
        let malting_for_kpc = db::malting_list_kpc().await?;
        let malting_for_6pr = db::malting_list_6pr().await?;
        Ok(Self { 
            batch_number_kpc, 
            batch_number_6pr,
            malting_for_kpc,
            malting_for_6pr
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFilter{
    pub place: String,
    pub batch_number: Option<String>,
    pub malting: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMalting{
    pub place: String,
    pub malting_number: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelDataOneLine{
    pub hot_number: Vec<Option<String>>,
    pub malting_namber: Vec<Option<String>>,
    pub batch_number:Vec<Option<String>>,
    pub task_number:Vec<Option<String>>,
    pub path_img: Vec<Option<String>>,
    pub date_time: Vec<Option<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelDataTwoLine{
    pub hot_number: Vec<Option<String>>,
    pub malting_namber: Vec<Option<String>>,
    pub batch_number:Vec<Option<String>>,
    pub task_number:Vec<Option<String>>,
    pub path_img: Vec<Option<String>>,
    pub date_time: Vec<Option<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelData6PRLine{
    pub hot_number: Vec<Option<String>>,
    pub malting_namber: Vec<Option<String>>,
    pub batch_number:Vec<Option<String>>,
    pub task_number:Vec<Option<i64>>,
    pub path_img: Vec<Option<String>>,
    pub date_time: Vec<Option<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WheelPlace{
    pub pr_6: Option<WheelData6PRLine>,
    pub line_one: Option<WheelDataOneLine>,
    pub line_two: Option<WheelDataTwoLine>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectWhee{
    pub place: String,
    pub batch_number: Option<String>,
    pub malting_number: Option<String>,
    pub wheel_number: String,
    pub line: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataWheel{
    pub hot_number: String,
    pub malting_namber: String,
    pub batch_number:String,
    pub task_number:i64,
    pub path_img: String
}
