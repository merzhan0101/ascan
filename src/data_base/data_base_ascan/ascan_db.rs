use std::path::Path;
use tiberius::{Query, QueryItem};
use anyhow::Result;
use futures_util::TryStreamExt; 
use std::collections::HashSet;
use dotenv::dotenv;
use std::env;
use crate::data_base::client_db::{client_t, client_6pr};
use crate::structures::ascan_struct::{DataFilter, SelectWhee, DataWheel, WheelPlace, WheelDataOneLine, WheelDataTwoLine, WheelData6PRLine};
use crate::utils::utils_metod as utils;

pub async fn batch_list_kpc()->Result<Vec<String>>{
    dotenv().ok();

    let table_one_line = env::var("NAME_TABLE_ONE_LINE")?;

    let table_two_line = env::var("NAME_TABLE_TWO_LINE")?;

    let mut client = client_t().await?;
    let mut one_line:Vec<String> = Vec::new();
    let mut two_line:Vec<String> = Vec::new();
    let select_one_line_batch = Query::new(format!("select Distinct DocNumber from {}",table_one_line));
    let mut stream_one_line_batch = select_one_line_batch.query(&mut client).await?;
    while let Some(row) = stream_one_line_batch.try_next().await? {
        if let QueryItem::Row(r) = row {
            one_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    drop(stream_one_line_batch);
    // println!("1");
    let select_two_line_batch = Query::new(format!("select Distinct DocNumber from {}",table_two_line));
    let mut stream_two_line_batch = select_two_line_batch.query(&mut client).await?;
    while let Some(row) = stream_two_line_batch.try_next().await? {
        if let QueryItem::Row(r) = row {
            two_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    drop(stream_two_line_batch);

    let unique_values: HashSet<_> = one_line
        .into_iter()
        .chain(two_line.into_iter())
        .collect();

    let merged_list: Vec<_> = unique_values.into_iter().collect();

    Ok(merged_list)
}

pub async fn batch_list_6pr()->Result<Vec<String>>{
    let mut client = client_6pr().await?;
    dotenv().ok();
    let name_table = env::var("NAME_TABLE_6PR")?;
    let mut batch_list:Vec<String> = Vec::new();
    let select_batch = Query::new(format!("select Distinct Batch_number from {}", name_table));
    let mut stream_batch = select_batch.query(&mut client).await?;
    while let Some(row) = stream_batch.try_next().await? {
        if let QueryItem::Row(r) = row {
            batch_list.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    
    Ok(batch_list)
}

pub async fn malting_list_kpc()->Result<Vec<String>>{
    let mut client = client_t().await?;
    let mut one_line:Vec<String> = Vec::new();
    let mut two_line:Vec<String> = Vec::new();
    dotenv().ok();
    let table_one_line = env::var("NAME_TABLE_ONE_LINE")?;
    let table_two_line = env::var("NAME_TABLE_TWO_LINE")?;

    let select_one_line_malting = Query::new(format!("select Distinct BatchNumber from {}", table_one_line));
    let mut stream_one_line_malting = select_one_line_malting.query(&mut client).await?;
    while let Some(row) = stream_one_line_malting.try_next().await? {
        if let QueryItem::Row(r) = row {
            one_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    drop(stream_one_line_malting);
    
    let select_two_line_malting = Query::new(format!("select Distinct BatchNumber from {}", table_two_line));
    let mut stream_two_line_malting = select_two_line_malting.query(&mut client).await?;
    while let Some(row) = stream_two_line_malting.try_next().await? {
        if let QueryItem::Row(r) = row {
            two_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    drop(stream_two_line_malting);

    let unique_values: HashSet<_> = one_line
        .into_iter()
        .chain(two_line.into_iter())
        .collect();

    let merged_list: Vec<_> = unique_values.into_iter().collect();
    
    Ok(merged_list)
}

pub async fn malting_list_6pr()->Result<Vec<String>>{
    let mut client = client_6pr().await?;
    dotenv().ok();
    let name_table = env::var("NAME_TABLE_6PR")?;
    let mut malting_list:Vec<String> = Vec::new();
    let select_malting = Query::new(format!("select Distinct Treatment_number from {}", name_table));
    let mut stream_malting = select_malting.query(&mut client).await?;
    while let Some(row) = stream_malting.try_next().await? {
        if let QueryItem::Row(r) = row {
            malting_list.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
        }
    }
    
    Ok(malting_list)
}

pub async fn data_wheel_for_batch(data: DataFilter)->Result<WheelPlace>{
    let mut client = client_t().await?;
    let mut client_6pr = client_6pr().await?;
    let mut wheels = WheelPlace::default();
    
    dotenv().ok();
    let batch_like = format!("%{}%", data.batch_number.unwrap());

    let mut one_line = WheelDataOneLine::default();
    let mut two_line = WheelDataTwoLine::default();
    
    let mut file_name =String::new();

    if data.place.as_str() == "КПЦ"{
        let root_dir_one_line = env::var("ASCAN_ONE_LINE_DIR")?;
        let root_dir_two_line = env::var("ASCAN_TWO_LINE_DIR")?;
        let one_line_table = env::var("NAME_TABLE_ONE_LINE")?;
        let two_line_table = env::var("NAME_TABLE_TWO_LINE")?;

        // let mut select_wheel_one_line = Query::new(format!("select WheelHotNumber FROM {} where DocNumber Like @P1", one_line_table)); 
        // select_wheel_one_line.bind(&batch_like);
        {
            let mut select_wheel_one_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE, DATE_AND_TIME, DocNumber FROM {} where DocNumber Like @P1", one_line_table)); 
            select_wheel_one_line.bind(&batch_like);
            let mut stream_one_line = select_wheel_one_line.query(&mut client).await?;
            while let Some(row) = stream_one_line.try_next().await? {
                if let QueryItem::Row(r) = row {
                    one_line.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                    one_line.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                    one_line.task_number.push(r.get(2).map(|s:&str| s.trim().to_string()));
                    let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                    let path = Path::new(path_wheel);
                    if let Some(value) = path.file_name() {
                        file_name = value.to_os_string().into_string().unwrap();
                        one_line.path_img.push(utils::find_file_by_date_path(&root_dir_one_line, &file_name)); 

                    }
                    one_line.date_time.push(r.get(4).map(|s:&str| s.trim().to_string()));
                    one_line.batch_number.push(r.get(5).map(|s:&str| s.trim().to_string()));

                }
            }
        }

        {
            let mut select_wheel_two_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE, DATE_AND_TIME, DocNumber FROM {} where DocNumber Like @P1", two_line_table)); 
            select_wheel_two_line.bind(&batch_like);
            let mut stream_two_line = select_wheel_two_line.query(&mut client).await?;
            while let Some(row) = stream_two_line.try_next().await? {
                if let QueryItem::Row(r) = row {
                    two_line.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                    two_line.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                    two_line.task_number.push(r.get(2).map(|s:&str| s.trim().to_string()));
                    let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                    let path = Path::new(path_wheel);
                    if let Some(value) = path.file_name() {
                        file_name = value.to_os_string().into_string().unwrap();
                        // println!("one_line {}", file_name);
                        two_line.path_img.push(utils::find_file_by_date_path(&root_dir_two_line, &file_name)); 

                    }
                    two_line.date_time.push(r.get(4).map(|s:&str| s.trim().to_string()));
                    two_line.batch_number.push(r.get(5).map(|s:&str| s.trim().to_string()));
                }
            }
        }

//         let mut select_wheel_two_line = Query::new(format!("select WheelHotNumber FROM {} where DocNumber Like @P1", two_line_table)); 
//         select_wheel_two_line.bind(&batch_like);
        
//         let mut stream_one_line = select_wheel_one_line.query(&mut client).await?;
//         while let Some(row) = stream_one_line.try_next().await? {
//             if let QueryItem::Row(r) = row {
//                 // list_one_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
//                 wheels.line_one.push(r.get(0).map(|s:&str| s.trim().to_string()));
//             }
//         }
//         drop(stream_one_line);
// //      stream второй линии 
//         let mut stream_two_line = select_wheel_two_line.query(&mut client).await?;
//         while let Some(row) = stream_two_line.try_next().await? {
//             if let QueryItem::Row(r) = row {
//                 // list_two_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
//                 wheels.line_two.push(r.get(0).map(|s:&str| s.trim().to_string()));
//             }
//         }

        // println!("{:?}", wheels);
        wheels.line_one = Some(one_line);
        wheels.line_two = Some(two_line);
        wheels.pr_6 = None;
        return Ok(wheels);  
    
    }else {
        let root_dir_6pr = env::var("ASCAN_6PR_DIR")?;
        // let mut barch_list:Vec<String> = Vec::new();
        let mut pr6 = WheelData6PRLine::default();

        let name_table = env::var("NAME_TABLE_6PR")?;
        let mut select_wheel = Query::new(format!("select Barcode, Treatment_number, Path, Batch_number from {} where Batch_number like @P1", name_table));
        select_wheel.bind(&batch_like);

        let mut stream_wheel = select_wheel.query(&mut client_6pr).await?;
        while let Some(row) = stream_wheel.try_next().await? {
            if let QueryItem::Row(r) = row {
                pr6.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                pr6.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                pr6.date_time.push(r.get(2).map(|s:&str| s.trim().to_string()));
                let mut path_wheel = r.get(2).map(|s:&str| s.trim().to_string()).unwrap();
                println!("{}", path_wheel);
                path_wheel = format!("{}.png", path_wheel);
                let path = Path::new(&path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
                pr6.path_img.push(utils::find_file_by_date_path(&root_dir_6pr, &file_name));
                pr6.batch_number.push(r.get(3).map(|s:&str| s.trim().to_string()));

            }
        }
        
        wheels.line_one = None;
        wheels.line_two = None;
        wheels.pr_6 = Some(pr6);

        return Ok(wheels);
    }

}
pub async fn data_wheel_for_malting(data:DataFilter)->Result<WheelPlace>{

        let mut client = client_t().await?;
    let mut client_6pr = client_6pr().await?;
    let mut wheels = WheelPlace::default();
    
    dotenv().ok();
    let like_malting = format!("%{}%", data.malting.unwrap());

    let mut one_line = WheelDataOneLine::default();
    let mut two_line = WheelDataTwoLine::default();
    
    let mut file_name =String::new();

    if data.place.as_str() == "КПЦ"{
        let root_dir_one_line = env::var("ASCAN_ONE_LINE_DIR")?;
        let root_dir_two_line = env::var("ASCAN_TWO_LINE_DIR")?;

        let one_line_table = env::var("NAME_TABLE_ONE_LINE")?;
        let two_line_table = env::var("NAME_TABLE_TWO_LINE")?;

        {
            let mut select_wheel_one_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE, DATE_AND_TIME, DocNumber FROM {} where BatchNumber Like @P1", one_line_table)); 
            select_wheel_one_line.bind(&like_malting);
            let mut stream_one_line = select_wheel_one_line.query(&mut client).await?;
            while let Some(row) = stream_one_line.try_next().await? {
                if let QueryItem::Row(r) = row {
                    one_line.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                    one_line.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                    one_line.task_number.push(r.get(2).map(|s:&str| s.trim().to_string()));
                    let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                    let path = Path::new(path_wheel);
                    if let Some(value) = path.file_name() {
                        file_name = value.to_os_string().into_string().unwrap();
                        one_line.path_img.push(utils::find_file_by_date_path(&root_dir_one_line, &file_name)); 

                    }
                    one_line.date_time.push(r.get(4).map(|s:&str| s.trim().to_string()));
                    one_line.batch_number.push(r.get(5).map(|s:&str| s.trim().to_string()));

                }
            }
        }

        {
            let mut select_wheel_two_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE, DATE_AND_TIME, DocNumber FROM {} where BatchNumber Like @P1", two_line_table)); 
            select_wheel_two_line.bind(&like_malting);
            let mut stream_two_line = select_wheel_two_line.query(&mut client).await?;
            while let Some(row) = stream_two_line.try_next().await? {
                if let QueryItem::Row(r) = row {
                    two_line.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                    two_line.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                    two_line.task_number.push(r.get(2).map(|s:&str| s.trim().to_string()));
                    let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                    let path = Path::new(path_wheel);
                    if let Some(value) = path.file_name() {
                        file_name = value.to_os_string().into_string().unwrap();
                        two_line.path_img.push(utils::find_file_by_date_path(&root_dir_two_line, &file_name)); 
                    }
                    two_line.date_time.push(r.get(4).map(|s:&str| s.trim().to_string()));
                    two_line.batch_number.push(r.get(5).map(|s:&str| s.trim().to_string()));
                }
            }
        }

        wheels.line_one = Some(one_line);
        wheels.line_two = Some(two_line);
        wheels.pr_6 = None;
        return Ok(wheels);  
    
    }else {
        let root_dir_6pr = env::var("ASCAN_6PR_DIR")?;
        let mut pr6 = WheelData6PRLine::default();

        let name_table = env::var("NAME_TABLE_6PR")?;
        let mut select_wheel = Query::new(format!("select Barcode, Treatment_number, Path, Batch_number from {} where Treatment_number like @P1", name_table));
        select_wheel.bind(&like_malting);

        let mut stream_wheel = select_wheel.query(&mut client_6pr).await?;
        while let Some(row) = stream_wheel.try_next().await? {
            if let QueryItem::Row(r) = row {
                pr6.hot_number.push(r.get(0).map(|s:&str| s.trim().to_string()));
                pr6.malting_namber.push(r.get(1).map(|s:&str| s.trim().to_string()));
                pr6.date_time.push(r.get(2).map(|s:&str| s.trim().to_string()));
                let mut path_wheel = r.get(2).map(|s:&str| s.trim().to_string()).unwrap();
                println!("{}", path_wheel);
                path_wheel = format!("{}.png", path_wheel);
                let path = Path::new(&path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
                pr6.path_img.push(utils::find_file_by_date_path(&root_dir_6pr, &file_name));
                pr6.batch_number.push(r.get(3).map(|s:&str| s.trim().to_string()));

            }
        }
        
        wheels.line_one = None;
        wheels.line_two = None;
        wheels.pr_6 = Some(pr6);

        return Ok(wheels);
    }

}
// pub async fn data_wheel_for_malting(data:DataFilter)->Result<WheelPlace>{
//     dotenv().ok();
//     let mut client = client_t().await?;
//     let mut client_6pr = client_6pr().await?;
//     let mut wheels = WheelPlace::default();
    
//     let like_malting = format!("%{}%", data.malting.unwrap());

//     if data.place.as_str() == "КПЦ"{
//         // let mut list_one_line:Vec<String> = Vec::new();
//         // let mut list_two_line:Vec<String> = Vec::new();
        
//         // println!("{}", &like_malting);
//         let one_line_table = env::var("NAME_TABLE_ONE_LINE")?;
//         let two_line_table = env::var("NAME_TABLE_TWO_LINE")?;

//         let mut select_wheel_one_line = Query::new(format!("select WheelHotNumber FROM {} where BatchNumber Like @P1", one_line_table)); 
//         select_wheel_one_line.bind(&like_malting);
        
//         let mut select_wheel_two_line = Query::new(format!("select WheelHotNumber FROM {} where BatchNumber Like @P1", two_line_table)); 
//         select_wheel_two_line.bind(&like_malting);
        
//         let mut stream_one_line = select_wheel_one_line.query(&mut client).await?;
//         while let Some(row) = stream_one_line.try_next().await? {
//             if let QueryItem::Row(r) = row {
//                 // list_one_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
//                 wheels.line_one.push(r.get(0).map(|s:&str| s.trim().to_string()));
//             }
//         }
//         drop(stream_one_line);
// //      stream второй линии 
//         let mut stream_two_line = select_wheel_two_line.query(&mut client).await?;
//         while let Some(row) = stream_two_line.try_next().await? {
//             if let QueryItem::Row(r) = row {
//                 // list_two_line.push(r.get(0).map(|s:&str| s.trim().to_string()).unwrap());
//                 wheels.line_two.push(r.get(0).map(|s:&str| s.trim().to_string()));
//             }
//         }

//         return Ok(wheels);
    
//     }else {
//         // let mut barch_list:Vec<String> = Vec::new();
//         let name_table = env::var("NAME_TABLE_6PR")?;
        
//         let mut select_wheel = Query::new(format!("select Wheel_number from {} where Treatment_number Like @P1", name_table));
//         select_wheel.bind(&like_malting);

//         let mut stream_wheel = select_wheel.query(&mut client_6pr).await?;
//         while let Some(row) = stream_wheel.try_next().await? {
//             if let QueryItem::Row(r) = row {
//                 wheels.pr_6.push(r.get(0).map(|s:&str| s.trim().to_string()));

//             }
//         }
//         return Ok(wheels);
//     }
// }

pub async fn data_for_wheel_select_batch(data: SelectWhee)-> Result<DataWheel>{

    dotenv().ok();

    let mut client = client_t().await?;
    let mut client_6pr = client_6pr().await?;

    let mut data_wheel = DataWheel::default();

    let batch_like = format!("%{}%", data.batch_number.clone().unwrap());
    let wheel_like = format!("%{}%", data.wheel_number);

    let mut file_name =String::new();

    if data.place.as_str() == "КПЦ" && data.line == Some(1){

        let root_dir_one_line = env::var("ASCAN_ONE_LINE_DIR")?;
        let one_line_table = env::var("NAME_TABLE_ONE_LINE")?;

        let mut select_data_wheel_one_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE from {} where DocNumber Like @P1 and WheelHotNumber Like @P2", one_line_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_one_line.bind(&batch_like);
        select_data_wheel_one_line.bind(&wheel_like);
        let mut stream_data_wheel_one_line = select_data_wheel_one_line.query(&mut client).await?;
        while let Some(row) = stream_data_wheel_one_line.try_next().await? {

            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.malting_namber = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.task_number = r.get(2).unwrap();
                let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                let path = Path::new(path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
            }
        }

        data_wheel.batch_number = data.batch_number.unwrap();

        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_one_line, &file_name).unwrap();

        return Ok(data_wheel);

    } else if data.place.as_str() == "КПЦ" && data.line == Some(2) {
        
        let root_dir_two_line = env::var("ASCAN_TWO_LINE_DIR")?;
        let two_line_table = env::var("NAME_TABLE_TWO_LINE")?;

        let mut select_data_wheel_two_line = Query::new(format!("select WheelHotNumber, BatchNumber, TaskNumber, DATACODE from {} where DocNumber Like @P1 and WheelHotNumber Like @P2",two_line_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_two_line.bind(&batch_like);
        select_data_wheel_two_line.bind(&wheel_like);
            
        let mut stream_data_wheel_two_line = select_data_wheel_two_line.query(&mut client).await?;
        while let Some(row) = stream_data_wheel_two_line.try_next().await? {
            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.malting_namber = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.task_number = r.get(2).unwrap();
                let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                let path = Path::new(path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("two_line {},", file_name);
                }
            }
        }

        data_wheel.batch_number = data.batch_number.unwrap();
        
        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_two_line, &file_name).unwrap();
        
        return Ok(data_wheel);

    }else {

        let root_dir_6pr = env::var("ASCAN_6PR_DIR")?;
        let name_table = env::var("NAME_TABLE_6PR")?;

        let mut select_data_wheel_one_line = Query::new(format!("select Barcode, Treatment_number, Path from {} where Batch_number Like @P1 and Wheel_number Like @P2", name_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_one_line.bind(&batch_like);
        select_data_wheel_one_line.bind(&wheel_like);
        let mut stream_data_wheel_one_line = select_data_wheel_one_line.query(&mut client_6pr).await?;
        while let Some(row) = stream_data_wheel_one_line.try_next().await? {

            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.malting_namber = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                // data_wheel.task_number = r.get(2).map(|s:&str| s.trim().to_string()).unwrap();
                println!("{}", data_wheel.malting_namber);
                let mut path_wheel = r.get(2).map(|s:&str| s.trim().to_string()).unwrap();
                println!("{}", path_wheel);
                path_wheel = format!("{}.png", path_wheel);
                let path = Path::new(&path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
            }
        }

        // data_wheel.task_number = "0".to_string();
        data_wheel.batch_number = data.batch_number.unwrap();

        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_6pr, &file_name).unwrap();

        return Ok(data_wheel);
    }

}

pub async fn data_for_wheel_select_malting(data: SelectWhee)-> Result<DataWheel>{
    dotenv().ok();

    let mut client = client_t().await?;
    let mut client_6pr = client_6pr().await?;

    let mut data_wheel = DataWheel::default();

    let batch_like = format!("%{}%", data.malting_number.clone().unwrap());
    let wheel_like = format!("%{}%", data.wheel_number);

    let mut file_name =String::new();

    if data.place.as_str() == "КПЦ" && data.line == Some(1){

        let root_dir_one_line = env::var("ASCAN_ONE_LINE_DIR")?;
        let one_line_table = env::var("NAME_TABLE_ONE_LINE")?;

        let mut select_data_wheel_one_line = Query::new(format!("select WheelHotNumber, DocNumber, TaskNumber, DATACODE from {} where BatchNumber Like @P1 and WheelHotNumber Like @P2", one_line_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_one_line.bind(&batch_like);
        select_data_wheel_one_line.bind(&wheel_like);
        let mut stream_data_wheel_one_line = select_data_wheel_one_line.query(&mut client).await?;
        while let Some(row) = stream_data_wheel_one_line.try_next().await? {

            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.batch_number = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.task_number = r.get(2).unwrap();
                let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                let path = Path::new(path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
            }
        }

        data_wheel.malting_namber = data.malting_number.unwrap();

        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_one_line, &file_name).unwrap();

        return Ok(data_wheel);

    } else if data.place.as_str() == "КПЦ" && data.line == Some(2) {
        
        let root_dir_two_line = env::var("ASCAN_TWO_LINE_DIR")?;
        let two_line_table = env::var("NAME_TABLE_TWO_LINE")?;

        let mut select_data_wheel_two_line = Query::new(format!("select WheelHotNumber, DocNumber, TaskNumber, DATACODE from {} where BatchNumber Like @P1 and WheelHotNumber Like @P2", two_line_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_two_line.bind(&batch_like);
        select_data_wheel_two_line.bind(&wheel_like);
            
        let mut stream_data_wheel_two_line = select_data_wheel_two_line.query(&mut client).await?;
        while let Some(row) = stream_data_wheel_two_line.try_next().await? {
            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.batch_number = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.task_number = r.get(2).unwrap();
                let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                let path = Path::new(path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("two_line {},", file_name);
                }
            }
        }
            
        data_wheel.malting_namber = data.malting_number.unwrap();

        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_two_line, &file_name).unwrap();
        
        return Ok(data_wheel);

    }else {

        let root_dir_6pr = env::var("ASCAN_6PR_DIR")?;
        let name_table = env::var("NAME_TABLE_6PR")?;

        let mut select_data_wheel_one_line = Query::new(format!("select Barcode, Batch_number, Path from {} where Treatment_number Like @P1 and Wheel_number Like @P2", name_table)); //WheelHotNumber => горячая маркеровка, BatchNumber => Плавка, TaskNumber => номер задания, DATACODE => путь хранения 
        select_data_wheel_one_line.bind(&batch_like);
        select_data_wheel_one_line.bind(&wheel_like);
        let mut stream_data_wheel_one_line = select_data_wheel_one_line.query(&mut client_6pr).await?;
        while let Some(row) = stream_data_wheel_one_line.try_next().await? {

            if let QueryItem::Row(r) = row {
                data_wheel.hot_number = r.get(0).map(|s:&str| s.trim().to_string()).unwrap();
                data_wheel.batch_number = r.get(1).map(|s:&str| s.trim().to_string()).unwrap();
                // data_wheel.task_number = r.get(2).map(|s:&str| s.trim().to_string()).unwrap();
                let path_wheel:&str = r.get(3).map(|s:&str| s.trim()).unwrap();
                let path = Path::new(path_wheel);
                if let Some(value) = path.file_name() {
                    file_name = value.to_os_string().into_string().unwrap();
                    println!("one_line {}", file_name);
                }
            }
        }

        data_wheel.task_number = 0;
        data_wheel.malting_namber = data.malting_number.unwrap();

        data_wheel.path_img = utils::find_file_by_date_path(&root_dir_6pr, &file_name).unwrap();

        return Ok(data_wheel);
    }
}

